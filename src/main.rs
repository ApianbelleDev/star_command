#![allow(unused_imports)]

use raylib::prelude::*;
use crate::KeyboardKey::*;
use crate::engine::*;
use crate::obj::*;
use rand::prelude::*;
use gilrs::{Gilrs, Button, Event};
/// a struct that holds the values of the player

mod engine;
mod obj;
fn main(){
	let mut engine = Engine{
		display_width: 240.0,
		display_height: 160.0,
		window_title: "Star Command".to_string(),
		off_screen_x: -100.0,
		off_screen_y: -100.0,
		display_edge: 210.0,
		display_top: -0.0,
		is_game_over: false
	};
	let mut turret = Turret {
		position: Vector2::new(110.0, 125.0),
		speed: 80.0,
		health: 1,
		rec: Rectangle::new(engine.off_screen_x, engine.off_screen_y, 16.0, 16.0),
		startposx: 110.0;
		startposy: 125.0;
	};
	let mut comet = Comet {
		position: Vector2::new(engine.display_top, engine.display_top),
		speed: 35.0,
		//isDestroyed: false,
		min: 0 ,
		max: engine.display_edge as i32,
		rec: Rectangle::new(engine.off_screen_x, engine.off_screen_y, 32.0, 32.0),
		startposx: engine.display_top;
		startposy: engine.display_top;
		
	};

	let mut bullet = Bullet {
		position: Vector2::new(engine.off_screen_x, engine.off_screen_y),
		speed: 100.0,
		is_shot: false,	
		rec: Rectangle::new(engine.off_screen_x, engine.off_screen_y, 8.0, 8.0),
		startposx: engine.off_screen_x;
		startposy: engine.off_screen_y;
	};
	//Start
	let mut currentScreen = GameStates::Logo;
	let mut frameCounter: i32 = 0;

	//randomize comet position
	let rng = rand::thread_rng().gen_range(comet.min..=comet.max);
	comet.position.x = rng as f32; 

	//window creation
	let (mut rl, thread) = raylib::init()
	.size(engine.display_width as i32, engine.display_height as i32)
	.title(&engine.window_title)
	.build();

	while !rl.window_should_close(){
		//Update
		frameCounter += 1;
		match currentScreen{
			GameStates::Logo => {
				if frameCounter > 120 {
					currentScreen = GameStates::Title;
				}
			}
			GameStates::Title => {
				if rl.is_key_pressed(KEY_ENTER) {
					currentScreen = GameStates::Gameplay;
				}
				// set start positions for the objects here:

			}
			GameStates::Gameplay => {
				//turret start pos
				turret.rec.x = turret.startposx;
				turret.rec.y = turret.startposy;

				//comet start pos
				comet.rec.x = comet.startpos.x;
				comet.rec.y = comet.startpos.y;

				//bullet start pos
				bullet.rec.x = bullet.position.x;
				bullet.rec.y = bullet.position.y;

				//move comet down 
				comet.position.y += comet.speed * rl.get_frame_time();
				//player input
				if rl.is_key_down(KEY_RIGHT) {
					turret.position.x += turret.speed * rl.get_frame_time();
				}
					
				if rl.is_key_down(KEY_LEFT) {
					turret.position.x -= turret.speed * rl.get_frame_time();
				}
					
				//if the player hits either edge of the screen, stop the player before they pass the edge(screen bounds)
				if turret.position.x >= engine.display_edge {
					turret.position.x = engine.display_edge;
				}	
				else if turret.position.x <= 0.0 {
					turret.position.x = 0.0;
				}
				
				if rl.is_key_pressed(KEY_SPACE) {
					bullet.is_shot = true;
					bullet.position.y = turret.position.y;
					bullet.position.x = turret.position.x;
				}

				if bullet.is_shot == true {
					bullet.position.y -= bullet.speed * rl.
					get_frame_time();
				}

				// Check object collision and react accordingly
				if bullet.rec.check_collision_recs(&comet.rec) {
					//reset comet pos and randomize position
					comet.position = Vector2::new(engine.display_top, engine.display_top);

					let rng = rand::thread_rng().gen_range(comet.min..=comet.max);

					comet.position.x = rng as f32;
					//reset bullet pos
					bullet.position.x = engine.off_screen_x;
					bullet.position.y = engine.off_screen_y;
					bullet.is_shot = false;
				}

				if turret.rec.check_collision_recs(&comet.rec) || comet.rec.y >= 140.0 {
					engine.is_game_over = true;
					// reset comet position
					comet.position.x = engine.off_screen_x - 100.0;
					comet.position.y = engine.off_screen_y;

					//set turret position off screen
					turret.position.x = engine.off_screen_x;
					turret.position.y = engine.off_screen_y;

					bullet.position.x = engine.off_screen_x;
					bullet.position.y = engine.off_screen_y;
				}
				if engine.is_game_over == true {
					currentScreen = GameStates::Game_Over;
				}
			}
			GameStates::Game_Over => {
				if rl.is_key_pressed(KEY_ENTER){
					currentScreen = GameStates::Title;
					engine.is_game_over = false;	
				}
			}

		}

		
		
		let mut d = rl.begin_drawing(&thread);
			match currentScreen {
				GameStates::Logo => {
					d.clear_background(Color::BLACK);
					d.draw_text("Logo", 0, 0, 32, Color::WHITE);
				}
				GameStates::Title => {
					d.clear_background(Color::BLACK);
					d.draw_text("Title", 0, 0, 32, Color::WHITE);
				}
				GameStates::Gameplay => {
					d.clear_background(Color::BLACK);
					d.draw_rectangle_rec(turret.rec, Color::WHITE );
					d.draw_rectangle_rec(comet.rec, Color::WHITE);
					d.draw_rectangle_rec(bullet.rec, Color::WHITE);
					d.draw_fps(0, 0);
				}
				GameStates::Game_Over => {
					d.clear_background(Color::BLACK);
					d.draw_text("Game Over",  0, 0, 32, Color::WHITE);
					engine.is_game_over = false;
				}
			}
	}
	
}

