use crate::engine::game_application::GameApplication;

pub struct CoasterEngine {
	game_application: GameApplication,
}

impl CoasterEngine {
	pub fn new() -> CoasterEngine {
		let game_application = GameApplication::new();
		game_application.init();

		CoasterEngine {
			game_application,
		}
	}
}