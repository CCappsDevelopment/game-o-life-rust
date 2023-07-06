extern crate sdl2;

use std::time::Duration;

use events::Events;
use screen_renderer::ScreenRenderer;

use crate::{events, screen_renderer};

pub struct Game {
    pub screen_renderer: ScreenRenderer,
    pub event_handler: Events,
}

impl Game {
    pub fn new() -> Game {
        let screen_renderer = ScreenRenderer::new();
        let event_handler = Events::new();

        Game {
            screen_renderer,
            event_handler
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let button_font = ttf_context.load_font("./assets/fonts/LibreFranklin-Medium.ttf", 24)?;

        let mut fonts: Vec<&sdl2::ttf::Font> = Vec::new();
        fonts.push(&button_font);

        // Game loop
        'running: loop {
            // Handle events
            if !self.event_handler.process_events(
                &mut self.screen_renderer,
            ) {
                break 'running;
            }

            // Render the screen
            self.screen_renderer.draw(fonts.clone())?;

            // Set the framerate to 60fps
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}
