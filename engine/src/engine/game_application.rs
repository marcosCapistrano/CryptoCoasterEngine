/* 
The Game's application layer, each operating system has to have one of these

The application layer class is instantiated as a global singleton object and is referred to throughtout your code via a pointer, it is constructed globally too, since it has to be there from the entry point to the program termination.

It should:
 - Handle interfacing with the hardware and the operating system
 - Set up calls for SDL2 and OpenGL to work properly
 - Run the initialization sequence
 - Enter the main loop
 - Run shutdown code after the main loop exits

 It is meant to:
	- Be inherited by a game-specing application class that will extend it and define game-specific things, such as title, but also implementations for creating the game logic and game views and loading the initial state of the game

	The class acts as a container for other important members that manage the application layer:
		- A handle to the text resource. It cointains all the user-presented strings so the game can be localized
		- The game logic implementation
		- A data structure that holds game options
		- The resource cache, which is responsible for loading textures, meshes, and sounds
		- The main Event Manager, which allows different game subsystems to communicate with each other
		- The network communications manager.
*/

extern crate sdl2;

pub struct GameApplication {
	is_running: bool,
	is_editor_running: bool,
	quit_requested: bool,
	quitting: bool,
	screen_size: (u32, u32),

	/*
	pub game_logic: Option<GameLogic>,
	pub event_manager: Option<EventManager>
	pub resource_cache: Option<ResourceCache>
	*/
}

impl GameApplication {
	pub fn new() -> GameApplication {

		GameApplication {
			/*
			game_logic: None,
			event_manager: None,
			resource_cache: None,
			*/

			screen_size: (800, 600),
			is_running: false,
			is_editor_running: false,
			quit_requested: false,
			quitting: false
		}
	}

	pub fn init(&self) {
		/*
			First, check system resources:
				- Detect multiple instances of the application
				- Check secondary storage space and memory
				- Calculate CPU speed
				- Load the game's resource cache
				- Load strings that will be presented to the player
				- Create the LUA Script Manager
				- Create the game's Event Manager
				- Use the script manager to load initial game options
				- Initialize SDL2, the window, and OpenGL
				- Create the game logic and game views
				- Set the directory for save games and other temporary files
				- Preload selected resources from the resource cache

			Make sure everything is initialized before some other subsystem needs it to exist.
		*/


		// TODO: Resource Check

		// TODO: Register Engine Events
		// TODO: Register Game Events

		// TODO: Initialize Resource Cache

		// Initialize SDL2 and Create a Window
		let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

		let window = video_subsystem.window("Rust SDL2 Demo", self.screen_size.0, self.screen_size.1)
			.position_centered()
			.build()
			.unwrap();

		// TODO: Create Game Logic and Game View
	}

	pub fn screen_size(self) -> (u32, u32) {
		self.screen_size
	}

	pub fn is_editor_running(self) -> bool {
		self.is_editor_running
	}

	pub fn register_engine_events(self) {

	}
}