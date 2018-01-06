#[macro_use]
extern crate stdweb;

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
use game::*;

macro_rules! enclose {
    ( [$( $x:ident ),*] $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn initial_render_call(state: &mut WebGame) {
	//state.scene.clear();
	let mut cube = GameObject::new(state.increment_global_object_counter(), &state.geometries.player_geometry, &state.materials.player);
	state.scene.add(&cube);
	cube.z = 50.0;
	cube.y = -400.0;
	state.player = Some(cube);

	let mut sphere = GameObject::new(state.increment_global_object_counter(), &state.geometries.sphere_geometry, &state.materials.ball);
	state.scene.add(&sphere);
	sphere.z = 50.0;
	sphere.y = -350.0;
	sphere.update_position();
	state.ball = Some(sphere);

	for i in -7..8 {
		for j in -8..9 {
			let mut cube = GameObject::new(state.increment_global_object_counter(), &state.geometries.box_geometry, &state.materials.default);
			cube.x = i as f32 * 55.0;
			cube.y = j as f32 * 55.0;
			cube.z = -25.1;
			state.scene.add(&cube);
			state.test.push(cube);
		}
	}
}

fn update(state: &mut WebGame, dt: f64) {
	state.player.as_mut().unwrap().x = state.mouse_x;
	state.player.as_ref().unwrap().update_position();
	let offset = (state.current_time_in_seconds as f32).sin() * 10.0;
	for obj in &mut state.test {
		obj.update_position_with_offset(offset);
	}
}

fn render(current_stamp: f64, dt: f64) {
	STATE.with(|state| {
		state.borrow_mut().current_time_in_seconds = current_stamp / 1000.0;
		if dt == 0.0 {
			initial_render_call(&mut state.borrow_mut());
		} else {
			update(&mut state.borrow_mut(), dt);
			js!(
				renderer.render(scene, camera);
			);
		}
	});
	web::window().request_animation_frame(move |stamp| {
		render(stamp, (stamp - current_stamp) / 1000.0);
	});
}

thread_local!(
	static STATE: Rc<RefCell<WebGame>> = Rc::new(RefCell::new(WebGame::new()));
);

fn min<T:PartialOrd>(a:T,b:T)->T { if a<b{a}else{b}}

fn max<T:PartialOrd>(a:T,b:T)->T { if a>b{a}else{b}}

fn on_mouse_move(x: i32, y: i32) {
	STATE.with(|state| {
		let mut state = state.borrow_mut();

		//Transform from top right corner to screen coordinates
		let aspect = state.screen_w as f32 / state.screen_h as f32;
		let real_width = state.frustum_size * aspect;
		let half_w = state.screen_w as f32 / 2.0;
		let half_h = state.screen_h as f32 / 2.0;

		state.mouse_x = (x as f32 - half_w) / state.screen_w as f32 * real_width;		
		state.mouse_x = min(max(state.mouse_x, -state.frustum_size / 2.0), state.frustum_size / 2.0);

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