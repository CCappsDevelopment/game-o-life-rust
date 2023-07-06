use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub struct SdlContext {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub window_width: u32,
    pub window_height: u32,
    pub dragging: bool,
    pub drag_start: (i32, i32),
    pub screen_position: (i32, i32),
}

impl SdlContext {
    pub fn new() -> SdlContext {
        let (event_pump, canvas, window_width, window_height) = Self::init_sdl2().unwrap();

        SdlContext {
            event_pump,
            canvas,
            window_width,
            window_height,
            dragging: false,
            drag_start: (0, 0),
            screen_position: (0, 0),
        }
    }

    fn init_sdl2() -> Result<(EventPump, Canvas<Window>, u32, u32), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        // Get the current display mode so we can determine screen dimensions
        let display_mode = video_subsystem.current_display_mode(0)?;

        // Calculate window dimensions as percentages of screen dimensions
        let window_width: u32 = (((display_mode.w as f32) * 0.5) + 1.0) as u32;
        let window_height: u32 = (((display_mode.h as f32) * 0.8) + 1.0) as u32;

        let window: Window = video_subsystem
            .window("Game O' Life", window_width, window_height)
            .position_centered()
            //.resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            //.software() // turn off hardware acceleration
            .build()
            .map_err(|e| e.to_string())?;

        let event_pump = sdl_context.event_pump()?;

        Ok((event_pump, canvas, window_width, window_height))
    }
}
