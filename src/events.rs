use sdl2::event::WindowEvent;
use sdl2::{ event::Event, keyboard::Keycode, mouse::MouseButton };

use crate::screen_renderer::ScreenRenderer;

pub struct Events{
    // track if the mouse button is down
    pub is_dragging: bool,
    // track last mouse position
    pub last_mouse_pos: (i32, i32),
}


impl Events {
    pub fn new() -> Events {
        Events {
            is_dragging: false,
            last_mouse_pos: (0, 0),
        }
    }

    pub fn process_events(&mut self, screen_renderer: &mut ScreenRenderer) -> bool {
        for event in screen_renderer.context.event_pump.poll_iter() {
            // In your process_events() function...
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return false;
                }
                Event::Window { win_event: WindowEvent::Resized(w, h), .. } => {
                    // Force the window to maintain a 1:1 aspect ratio (width == height)
                    if w != h {
                        screen_renderer.context.canvas.window_mut().set_size(w as u32, w as u32).unwrap();
                    } else {
                        screen_renderer.context.canvas.window_mut().set_size(w as u32, h as u32).unwrap();
                    }
                }
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    self.is_dragging = true;
                    self.last_mouse_pos = (x, y);
                }
                // Event::MouseMotion { x, y, .. } if self.is_dragging => {
                //     // calculate the distance moved since the last mouse event
                //     let dx = x - self.last_mouse_pos.0;
                //     let dy = y - self.last_mouse_pos.1;
                //     // update the camera position in the renderer
                //     screen_renderer.camera.0 -= dx;
                //     screen_renderer.camera.1 -= dy;
                //     // remember the current mouse position for the next event
                //     self.last_mouse_pos = (x, y);
                // }
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, x, y, .. } => {
                    let col = (x as i32) / screen_renderer.tile_height as i32;
                    let row = (y as i32) / screen_renderer.tile_width as i32;
                    
                    if screen_renderer.cells.contains(&(col, row)) {
                        screen_renderer.cells.remove(&(col, row));
                    } else {
                        screen_renderer.cells.insert((col, row));
                    }
                    
                    self.is_dragging = false;
                }
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(Keycode::Space) => {
                            screen_renderer.update = !screen_renderer.update;
                        }
                        _ =>
                         {}
                    }
                }
                _ => {}
            }
        }

        return true;
    }
}
