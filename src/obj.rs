use raylib::math::*;

pub struct Turret {
	pub position: Vector2,
	pub speed: f32,
	pub health: u32,
	pub rec: Rectangle,
	pub startposx: f32,
	pub startposy: f32,
}
/// a struct that holds the values of the comet
pub struct Comet {
	pub position: Vector2,
	pub speed: f32,
	pub min: i32,
	pub max: i32,
	pub rec: Rectangle,
	pub startposx: f32,
	pub startposy: f32,
}
/// a struct that holds the values of the bullet
pub struct Bullet {
	pub position: Vector2,
	pub speed: f32,
	pub is_shot: bool,
	pub rec: Rectangle,
	pub startposx: f32,
	pub startposy: f32,
}