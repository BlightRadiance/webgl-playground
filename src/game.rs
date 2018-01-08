use std::collections::HashSet;
use vecmath::*;
use super::sat::*;

pub struct WebGame {
	pub global_object_counter: u32,
	pub buttons: HashSet<ButtonState>,

	pub scene: Scene,

	pub player: Option<CollidableObject>,
	pub balls: Vec<BallObject>,
	pub objects: Vec<Box<CollidableObject>>,

	pub mouse_x: f32,
	pub mouse_y: f32,
	pub screen_w: i32,
	pub screen_h: i32,
	pub frustum_size: f32,
	
	pub materials: Materials,
	pub geometries: Geometries,
	pub current_time_in_seconds: f64,
}

impl WebGame {
	pub fn new() -> Self {
		WebGame {
			global_object_counter: 0,
			buttons: HashSet::new(),
			scene: Scene {},
			player: None,
			balls: Vec::new(),
			objects: Vec::new(),
			mouse_x: 0.0,
			mouse_y: 0.0,
			screen_w: 0,
			screen_h: 0,
			frustum_size: 1000.0,
			materials: Materials::new(),
			geometries: Geometries::new(),
			current_time_in_seconds: 0.0,
		}
	}

	pub fn increment_global_object_counter(&mut self) -> u32 {
		let old_value = self.global_object_counter;
		self.global_object_counter += 1;
		old_value
	}

	pub fn on_button(&mut self, key: &str, pressed: bool) -> bool {
		//js!(console.log("Got button: ", @{format!("{} - pressed: {}", key, pressed)}););
		let button = match key {
			"a" => ButtonState::Left,
			"Left" => ButtonState::Left,
			"ArrowLeft" => ButtonState::Left,
			"d" => ButtonState::Right,
			"Right" => ButtonState::Right,
			"ArrowRight" => ButtonState::Right,
			" " => ButtonState::Space,
			"Escape" => ButtonState::Esc,
			_ => return false
		};
		if pressed {
			self.buttons.insert(button);
		} else {
			self.buttons.remove(&button);
		}
		return true;
	}
}

#[derive(Debug)]
pub struct GameObject {
	id: u32,
	pub position: Vector3<f32>,
	pub need_update: bool,
}

impl GameObject {
	pub fn new(id: u32, geometry: &Geometry, material: &Material) -> Self {
		js!(
			gameObjects[@{id}] = new THREE.Mesh(geometry[@{geometry.get_id()}], materials[@{material.get_id()}]);
		);
		GameObject {
			id: id,
			position: [0.0, 0.0, 0.0],
			need_update: true,
		}
	}

	pub fn get_id(&self) -> u32 {
		self.id
	}

	pub fn update(&mut self) {
		if self.need_update {
			js!(
				var obj = gameObjects[@{self.get_id()}];
				obj.position.x = @{self.position[0]};
				obj.position.y = @{self.position[1]};
				obj.position.z = @{self.position[2]};
			);
		}
	}
}

pub trait Collidable {
	fn on_collision(&mut self, scene: &mut Scene, disp: &Vector2<f32>,  normal: &Vector2<f32>);
}

pub struct CollidableObject {
	pub object: GameObject,
	shape: Box<Shape>,
	collision_response: Box<Collidable>,
}

impl CollidableObject {
	pub fn new(object: GameObject, shape: Box<Shape>, collision_response: Box<Collidable>) -> Self {
		CollidableObject {
			object: object,
			shape: shape,
			collision_response: collision_response,
		}
	}

	pub fn update(&mut self) {
		self.object.update();
		self.shape.set_position([self.object.position[0], self.object.position[1]]);
	}
}

#[derive(Debug)]
pub struct Material {
	id: u32,
}

impl Material {
	pub fn new(id: u32) -> Self {
		Material {
			id: id,
		}
	}

	pub fn get_id(&self) -> u32 {
		self.id
	}
}

#[derive(Debug)]
pub struct Geometry {
	id: u32,
}

impl Geometry {
	pub fn new(id: u32) -> Self {
		Geometry {
			id: id,
		}
	}

	pub fn get_id(&self) -> u32 {
		self.id
	}
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ButtonState {
	Left,
	Right,
	Space,
	Esc,
}

#[derive(Debug)]
pub struct Materials {
	pub default: Material,
	pub player: Material,
	pub ball: Material,
	pub walls: Material,
}

impl Materials {
	pub fn new() -> Self {
		Materials {
			default: Material::new(0),
			player: Material::new(1),
			ball: Material::new(2),
			walls: Material::new(3),
		}			
	}
}

#[derive(Debug)]
pub struct Geometries {
	pub box_geometry: Geometry,
	pub player_geometry: Geometry,
	pub sphere_geometry: Geometry,
	pub side_walls_geometry: Geometry,
	pub top_down_walls_geometry: Geometry,
}

impl Geometries {
	pub fn new() -> Self {
		Geometries {
			box_geometry: Geometry::new(0),
			player_geometry: Geometry::new(1),
			sphere_geometry: Geometry::new(2),
			side_walls_geometry: Geometry::new(3),
			top_down_walls_geometry: Geometry::new(4),
		}			
	}
}

#[derive(Debug)]
pub struct Scene {
}

impl Scene {
	pub fn clear(&self) {
		js!(
			while(scene.children.length > 0) { 
				scene.remove(scene.children[0]); 
			}
		);
	}

	pub fn add(&self, object: &mut GameObject) {
		js!(
			var obj = gameObjects[@{object.get_id()}];
			scene.add(obj);
		);
		object.need_update = true;
		object.update();
	}

	pub fn remove(&self, object: &GameObject) {
		js!(
			scene.remove(gameObjects[@{object.get_id()}]);
		);
	}
}

#[derive(Debug)]
pub struct BallObject {
	object: GameObject,
	shape: CircleObject,
	v: Vector2<f32>,
}

impl BallObject {
	pub fn new(object: GameObject, radius: f32) -> Self {
		let shape = CircleObject::new([object.position[0], object.position[1]], radius);
		BallObject {
			object: object,
			shape: shape,
			v: [850.0, 850.0],
		}
	}

	pub fn update(&mut self, mut scene: &mut Scene, dt: f64, objects: &mut Vec<&mut CollidableObject>) {
		self.object.position[0] += self.v[0] * dt as f32;
		self.object.position[1] += self.v[1] * dt as f32;
		self.object.update();
		self.shape.set_position([self.object.position[0], self.object.position[1]]);
		for object in objects {
			if let Some((disp, normal)) = get_collision(&self.shape, object.shape.as_ref()) {
				object.collision_response.on_collision(&mut scene, &disp, &normal);

				//reflect
				let tmp = vec2_dot(self.v, normal) * 2.0;
				self.v = vec2_sub(self.v, vec2_mul([tmp, tmp], normal));
			
				//displace
				self.object.position = vec3_add(self.object.position, [disp[0], disp[1], 0.0]);
				self.object.update();
				// Note: no need to update shape's position
			}
		}
	}

	pub fn collide_with_other_balls(&mut self, balls: &[BallObject]) {
		for ball in balls {
			if let Some((disp, normal)) = get_collision(&self.shape, &ball.shape) {
				//reflect
				let tmp = vec2_dot(self.v, normal) * 2.0;
				self.v = vec2_sub(self.v, vec2_mul([tmp, tmp], normal));
			
				//displace
				self.object.position = vec3_add(self.object.position, [disp[0], disp[1], 0.0]);
				self.object.update();
				// Note: no need to update shape's position
			}
		}
	}
}

pub struct PlayerCollision {
}

impl Collidable for PlayerCollision {
	fn on_collision(&mut self, scene: &mut Scene, disp: &Vector2<f32>,  normal: &Vector2<f32>) {

	}
}

pub struct WallCollision {
}

impl Collidable for WallCollision {
	fn on_collision(&mut self, scene: &mut Scene, disp: &Vector2<f32>,  normal: &Vector2<f32>) {

	}
}