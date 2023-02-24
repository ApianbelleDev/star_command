pub struct Engine{
	pub display_width: f32,
	pub display_height: f32,
	pub window_title: String,
	pub off_screen_x: f32,
	pub off_screen_y: f32,
	pub display_edge: f32,
	pub display_top: f32,
	pub is_game_over: bool
	
}

pub enum GameStates {
	Logo,
	Title,
	Gameplay,
	Game_Over
}