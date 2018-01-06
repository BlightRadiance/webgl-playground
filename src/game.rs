use std::collections::HashSet;

#[derive(Debug)]
pub struct WebGame {
	pub global_object_counter: u32,
	pub buttons: HashSet<ButtonState>,
	pub scene: Scene,
	pub player: Option<GameObject>,
	pub ball: Option<GameObject>,
	pub mouse_x: f32,
	pub mouse_y: f32,
	pub screen_w: i32,
	pub screen_h: i32,
	pub frustum_size: f32,
	pub test: Vec<GameObject>,
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
			ball: None,
			mouse_x: 0.0,
			mouse_y: 0.0,
			screen_w: 0,
			screen_h: 0,
			frustum_size: 1000.0,
			test: Vec::new(),
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
		js!(console.log("Got button: ", @{format!("{} - pressed: {}", key, pressed)}););
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
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl GameObject {
	pub fn new(id: u32, geometry: &Geometry, material: &Material) -> Self {
		js!(
			gameObjects[@{id}] = new THREE.Mesh(geometry[@{geometry.get_id()}], materials[@{material.get_id()}]);
		);
		GameObject {
			id: id,
			x: 0.0,
			y: 0.0,
			z: 0.0,
		}
	}

	pub fn update_position(&self) {
		js!(
			var obj = gameObjects[@{self.get_id()}];
			obj.position.x = @{self.x};
			obj.position.y = @{self.y};
			obj.position.z = @{self.z};
		);
	}

	pub fn update_position_with_offset(&self, offset: f32) {
		js!(
			var obj = gameObjects[@{self.get_id()}];
			obj.position.x = @{self.x + offset};
			obj.position.y = @{self.y + offset};
		);
	}

	pub fn get_id(&self) -> u32 {
		self.id
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
}

impl Materials {
	pub fn new() -> Self {
		Materials {
			default: Material::new(0),
			player: Material::new(1),
			ball: Material::new(2),
		}			
	}
}

#[derive(Debug)]
pub struct Geometries {
	pub box_geometry: Geometry,
	pub player_geometry: Geometry,
	pub sphere_geometry: Geometry,
}

impl Geometries {
	pub fn new() -> Self {
		Geometries {
			box_geometry: Geometry::new(0),
			player_geometry: Geometry::new(1),
			sphere_geometry: Geometry::new(2),
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

	pub fn add(&self, object: &GameObject) {
		js!(
			var obj = gameObjects[@{object.get_id()}];
			obj.position.x = @{object.x};
			obj.position.y = @{object.y};
			obj.position.z = @{object.z};
			scene.add(obj);
		);
	}

	pub fn remove(&self, object: &GameObject) {
		js!(
			scene.remove(gameObjects[@{object.get_id()}]);
		);
	}
}