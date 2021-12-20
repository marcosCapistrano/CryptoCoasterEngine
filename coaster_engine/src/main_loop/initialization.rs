struct GameOptions {
	  level: String,
		screen_size: (u32, u32),
}

/*
Sets the configuration for the game, such as the level to load, the screen size, etc.
The "new" constructor initializes the options with sensible values,
whereas the init function would read a xml file to get the options
*/

impl GameOptions {
	pub fn new() -> GameOptions {
		GameOptions {
			level: "".to_string(),
			screen_size: (1024, 768),
		}
	}

	pub fn init(&self/*, String filename, String cmd*/) {
		self.level = 1;
		self.screen_size = (1024, 768);
	}
}