#[macro_use]
extern crate stdweb;
extern crate vecmath;

use stdweb::web::{
    self,
	IEventTarget,
};

use std::cell::RefCell;
use std::rc::Rc;

use stdweb::web::event::{
    IEvent,
    IKeyboardEvent,
    KeydownEvent,
    KeyupEvent,
};

mod game;
mod sat;
mod utils;

use utils::{min, max, split_vec_mut_around};
use game::*;
use sat::{ConvexObject};

macro_rules! enclose {
    ( [$( $x:ident ),*] $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

thread_local!(
	static STATE: Rc<RefCell<WebGame>> = Rc::new(RefCell::new(WebGame::new()));
);

fn initial_render_call(state: &mut WebGame) {
	//state.scene.clear();

	// Player
	let mut cube = GameObject::new(state.increment_global_object_counter(), &state.geometries.player_geometry, &state.materials.player);
	state.scene.add(&mut cube);
	cube.position[2] = 25.0;
	cube.position[1] = -400.0;
	let verts = vec![[-75.0, 15.0], [75.0, 15.0], [75.0, -15.0], [-75.0, -15.0]];
	let player = ConvexObject::new([cube.position[0], cube.position[1]], verts.clone());
	state.player = Some(CollidableObject::new(cube, Box::new(player), Box::new(PlayerCollision {})));

	// Balls
	{
		let mut sphere = GameObject::new(state.increment_global_object_counter(), &state.geometries.sphere_geometry, &state.materials.ball);
		state.scene.add(&mut sphere);
		sphere.position[2] = 50.0;
		sphere.position[1] = -450.0;
		sphere.update();
		state.balls.push(BallObject::new(sphere, 25.0));
	}
	{
		let mut sphere = GameObject::new(state.increment_global_object_counter(), &state.geometries.sphere_geometry, &state.materials.ball);
		state.scene.add(&mut sphere);
		sphere.position[2] = 50.0;
		sphere.position[1] = -400.0;
		sphere.update();
		state.balls.push(BallObject::new(sphere, 25.0));
	}
	{
		let mut sphere = GameObject::new(state.increment_global_object_counter(), &state.geometries.sphere_geometry, &state.materials.ball);
		state.scene.add(&mut sphere);
		sphere.position[2] = 50.0;
		sphere.position[1] = -350.0;
		sphere.update();
		state.balls.push(BallObject::new(sphere, 25.0));
	}
	{
		let mut sphere = GameObject::new(state.increment_global_object_counter(), &state.geometries.sphere_geometry, &state.materials.ball);
		state.scene.add(&mut sphere);
		sphere.position[2] = 50.0;
		sphere.position[1] = -300.0;
		sphere.update();
		state.balls.push(BallObject::new(sphere, 25.0));
	}
	{
		let mut sphere = GameObject::new(state.increment_global_object_counter(), &state.geometries.sphere_geometry, &state.materials.ball);
		state.scene.add(&mut sphere);
		sphere.position[2] = 50.0;
		sphere.position[1] = -250.0;
		sphere.update();
		state.balls.push(BallObject::new(sphere, 25.0));
	}
	{
		let mut sphere = GameObject::new(state.increment_global_object_counter(), &state.geometries.sphere_geometry, &state.materials.ball);
		state.scene.add(&mut sphere);
		sphere.position[2] = 50.0;
		sphere.position[1] = -200.0;
		sphere.update();
		state.balls.push(BallObject::new(sphere, 25.0));
	}
	{
		let mut sphere = GameObject::new(state.increment_global_object_counter(), &state.geometries.sphere_geometry, &state.materials.ball);
		state.scene.add(&mut sphere);
		sphere.position[2] = 50.0;
		sphere.position[1] = -150.0;
		sphere.update();
		state.balls.push(BallObject::new(sphere, 25.0));
	}
	{
		let mut sphere = GameObject::new(state.increment_global_object_counter(), &state.geometries.sphere_geometry, &state.materials.ball);
		state.scene.add(&mut sphere);
		sphere.position[2] = 50.0;
		sphere.position[1] = -100.0;
		sphere.update();
		state.balls.push(BallObject::new(sphere, 25.0));
	}
	{
		let mut sphere = GameObject::new(state.increment_global_object_counter(), &state.geometries.sphere_geometry, &state.materials.ball);
		state.scene.add(&mut sphere);
		sphere.position[2] = 50.0;
		sphere.position[1] = -50.0;
		sphere.update();
		state.balls.push(BallObject::new(sphere, 25.0));
	}

	//Walls
	let verts = vec![[-25.0, 500.0], [25.0, 500.0], [25.0, -500.0], [-25.0, -500.0]];
	let mut left_wall_obj = GameObject::new(state.increment_global_object_counter(), &state.geometries.side_walls_geometry, &state.materials.walls);
	state.scene.add(&mut left_wall_obj);
	left_wall_obj.position[0] = -475.0;
	left_wall_obj.position[1] = 0.0;
	left_wall_obj.position[2] = 25.0;
	let left_wall = ConvexObject::new([left_wall_obj.position[0], left_wall_obj.position[1]], verts.clone());
	state.objects.push(Box::new(CollidableObject::new(left_wall_obj, Box::new(left_wall), Box::new(WallCollision {}))));

	let mut right_wall_obj = GameObject::new(state.increment_global_object_counter(), &state.geometries.side_walls_geometry, &state.materials.walls);
	state.scene.add(&mut right_wall_obj);
	right_wall_obj.position[0] = 475.0;
	right_wall_obj.position[1] = 0.0;
	right_wall_obj.position[2] = 25.0;
	let right_wall = ConvexObject::new([right_wall_obj.position[0], right_wall_obj.position[1]], verts.clone());
	state.objects.push(Box::new(CollidableObject::new(right_wall_obj, Box::new(right_wall), Box::new(WallCollision {}))));

	let verts = vec![[-500.0, 25.0], [500.0, 25.0], [500.0, -25.0], [-500.0, -25.0]];
	let mut top_wall_obj = GameObject::new(state.increment_global_object_counter(), &state.geometries.top_down_walls_geometry, &state.materials.walls);
	state.scene.add(&mut top_wall_obj);
	top_wall_obj.position[0] = 0.0;
	top_wall_obj.position[1] = 500.0;
	top_wall_obj.position[2] = 25.0;
	let top_wall = ConvexObject::new([top_wall_obj.position[0], top_wall_obj.position[1]], verts.clone());
	state.objects.push(Box::new(CollidableObject::new(top_wall_obj, Box::new(top_wall), Box::new(WallCollision {}))));

	let mut bottom_wall_obj = GameObject::new(state.increment_global_object_counter(), &state.geometries.top_down_walls_geometry, &state.materials.walls);
	state.scene.add(&mut bottom_wall_obj);
	bottom_wall_obj.position[0] = 0.0;
	bottom_wall_obj.position[1] = -500.0;
	bottom_wall_obj.position[2] = 25.0;
	let bottom_wall = ConvexObject::new([bottom_wall_obj.position[0], bottom_wall_obj.position[1]], verts.clone());
	state.objects.push(Box::new(CollidableObject::new(bottom_wall_obj, Box::new(bottom_wall), Box::new(WallCollision {}))));
}

fn update(mut state: &mut WebGame, dt: f64) {
	let mut player = state.player.as_mut().unwrap();
	player.object.position[0] = state.mouse_x;
	player.update();

	let mut objects_to_check_collision_against = Vec::new();
	objects_to_check_collision_against.push(player);
	for obj in &mut state.objects {
		obj.as_mut().update();
		objects_to_check_collision_against.push(obj.as_mut());
	}

	// Collide balls with collidable objects
	for ball in &mut state.balls {
		ball.update(&mut state.scene, dt, &mut objects_to_check_collision_against);
	}

	// Collide balls with other balls
	let balls_count = state.balls.len();
	for i in 0..balls_count {
		let (batch_1, ball, batch_2) = split_vec_mut_around(&mut state.balls, i as usize);
		ball.collide_with_other_balls(batch_1);
		ball.collide_with_other_balls(batch_2);
	}
}

fn render(current_stamp: f64, dt: f64) {
	STATE.with(|state| {
		state.borrow_mut().current_time_in_seconds = current_stamp / 1000.0;
		if dt == 0.0 {
			initial_render_call(&mut state.borrow_mut());
		} else {
			update(&mut state.borrow_mut(), min(dt, 0.03333333333));
			js!(
				renderer.render(scene, camera);
			);
		}
	});
	web::window().request_animation_frame(move |stamp| {
		render(stamp, (stamp - current_stamp) / 1000.0);
	});
}

fn on_mouse_move(x: i32, y: i32) {
	STATE.with(|state| {
		let mut state = state.borrow_mut();

		//Transform from top right corner to screen coordinates
		let aspect = state.screen_w as f32 / state.screen_h as f32;
		let real_width = state.frustum_size * aspect;
		let half_w = state.screen_w as f32 / 2.0;
		let half_h = state.screen_h as f32 / 2.0;

		state.mouse_x = (x as f32 - half_w) / state.screen_w as f32 * real_width;		
		state.mouse_x = min(max(state.mouse_x, (-state.frustum_size / 2.0) + 100.0), state.frustum_size / 2.0 - 100.0);

		// Does not matter for now
		state.mouse_y = y as f32 - half_h;
	});
}

fn on_sceen_size_changed(w: i32, h: i32) {
	STATE.with(|state| {
		let mut state = state.borrow_mut();
		state.screen_w = w;
		state.screen_h = h;
	});
}

fn prepare_button_listener(game: Rc<RefCell<WebGame>>) {
	stdweb::web::window().add_event_listener(enclose!([game] move |event: KeydownEvent| {
		if game.borrow_mut().on_button(&event.key(), true) {
			event.prevent_default();
		}
	}));
	stdweb::web::window().add_event_listener(enclose!([game] move |event: KeyupEvent| {
		if game.borrow_mut().on_button(&event.key(), false) {
			event.prevent_default();
		}
	}));
}

fn init() {
	STATE.with(|state| {
		prepare_button_listener(state.clone());
	});
	stdweb::web::window().request_animation_frame(move |stamp: f64| {
		render(stamp, 0.0);
	});
}

fn main() {
	stdweb::initialize();
	js! {
        Module.exports.on_mouse_move = @{on_mouse_move};
        Module.exports.on_sceen_size_changed = @{on_sceen_size_changed};
        Module.exports.init = @{init};
    }
}