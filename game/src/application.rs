use coaster_engine::engine 

pub struct CryptoCoaster {
	coaster_engine: CoasterEngine,	
}

impl CryptoCoaster {
	pub fn new() {
		coaster_engine = coaster_engine.new();
	}

	pub fn run(&mut self) {
		self.coaster_engine.canvas.set_draw_color(Color::RGB(0, 255, 255));
		self.canvas.clear();
		self.canvas.present();
		let mut event_pump = self.sdl.event_pump().unwrap();
		let mut i = 0;
		'running: loop {
			i = (i + 1) % 255;
			self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
			self.canvas.clear();
			for event in event_pump.poll_iter() {
				match event {
					Event::Quit { .. }
					| Event::KeyDown {
						keycode: Some(Keycode::Escape),
						..
					} => break 'running,
					_ => {}
				}
			}
			// The rest of the game loop goes here...

			self.canvas.present();
			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
	}
}

pub mod crypto_coaster;
