#[macro_use]
extern crate stdweb;

use stdweb::web::{
    self,
	IEventTarget,
};

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashSet;

use stdweb::web::event::{
    IEvent,
    IKeyboardEvent,
    KeydownEvent,
    KeyupEvent,
};

use scene::Scene;
use game::GameObject;

macro_rules! enclose {
    ( [$( $x:ident ),*] $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum ButtonState {
	Left,
	Right,
	Space,
	Esc,
}

#[derive(Debug)]
pub struct WebGame {
	global_object_counter: u32,
	buttons: HashSet<ButtonState>,
	scene: Scene,
	player: Option<GameObject>,
	mouse_x: f32,
	mouse_y: f32,
	screen_w: i32,
	screen_h: i32,
	frustum_size: f32,
	test: Vec<GameObject>,
}

impl WebGame {
	fn new() -> WebGame {
		WebGame {
			global_object_counter: 0,
			buttons: HashSet::new(),
			scene: Scene {},
			player: None,
			mouse_x: 0.0,
			mouse_y: 0.0,
			screen_w: 0,
			screen_h: 0,
			frustum_size: 1000.0,
			test: Vec::new(),
		}
	}

	fn increment_global_object_counter(&mut self) -> u32 {
		let old_value = self.global_object_counter;
		self.global_object_counter += 1;
		old_value
	}

	fn on_button(&mut self, key: &str, pressed: bool) -> bool {
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

fn render(current_stamp: f64, dt: f64) {
	let seconds = current_stamp / 1000.0;
	STATE.with(|state| {
		let mut state = state.borrow_mut();
		// initial render call
		if dt == 0.0 {
			//state.scene.clear();

			let mut cube = GameObject::new(&mut state);
			state.scene.add(&cube);
			cube.z = 50.0;
			cube.y = -400.0;
			state.player = Some(cube);

			for i in -7..8 {
				for j in -8..9 {
					let mut cube = GameObject::new(&mut state);
					cube.x = i as f32 * 55.0;
					cube.y = j as f32 * 55.0;
					cube.z = -25.1;
					state.scene.add(&cube);
					state.test.push(cube);
				}
			}
		} else {
			let player_id = state.player.as_ref().unwrap().get_id();
			//js!(console.log("dt: " + @{format!("{} ms", dt)}));
			js!(
				var cube = gameObjects[@{player_id}];
				cube.rotation.x += @{dt};
				cube.rotation.y += @{dt};
				renderer.render(scene, camera);
			);
			/*
			if state.buttons.contains(&ButtonState::Left) {
				state.player.as_mut().unwrap().x -= (dt * 60.0) as f32;
			}
			if state.buttons.contains(&ButtonState::Right) {
				state.player.as_mut().unwrap().x += (dt * 60.0) as f32;
			}
			*/
			state.player.as_mut().unwrap().x = state.mouse_x;
			state.player.as_ref().unwrap().update_position();
			let offset = (seconds as f32).sin() * 50.0;
			for obj in &mut state.test {
				//obj.update_position_with_offset(offset);
			}
		}
	});
	web::window().request_animation_frame(move |stamp| {
		render(stamp, (stamp - current_stamp) / 1000.0);
	});
}

mod game {
	use super::WebGame;

	#[derive(Debug)]
	pub struct GameObject {
		id: u32,
		pub x: f32,
		pub y: f32,
		pub z: f32,
	}

	impl GameObject {
		pub fn new(state: &mut WebGame) -> GameObject {
			let id = state.increment_global_object_counter();
			js!(
				gameObjects[@{id}] = new THREE.Mesh(geometry, material);
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
}

mod scene {
	use super::game::GameObject;

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
		let proportion_h = state.screen_h as f32 / state.frustum_size;
		let half_w = state.screen_w as f32 / 2.0;
		let half_h = state.screen_h as f32 / 2.0;

		state.mouse_x = (x as f32 - half_w) / state.screen_w as f32 * real_width;		
		state.mouse_x = min(max(state.mouse_x, -state.frustum_size / 2.0), state.frustum_size / 2.0);

		state.mouse_y = (y as f32 - half_h) / state.screen_h as f32 * proportion_h;
	});
}

fn on_sceen_size_changed(w: i32, h: i32) {
	STATE.with(|state| {
		let mut state = state.borrow_mut();
		state.screen_w = w;
		state.screen_h = h;
	});
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