use std::collections::HashSet;
use std::time::Instant;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{ /*BlendMode, Texture,*/ TextureQuery };
use sdl2::ttf::Font;

use crate::sdl_context::SdlContext;

pub struct ScreenRenderer {
    pub context: SdlContext,
    pub camera: (i32, i32),
    pub last_frame_ticks: Instant,
    pub window_width: i32,
    pub window_height: i32,
    pub tile_width: i32,
    pub tile_height: i32,
    pub grid_size: (i32, i32),
    pub cells: HashSet<(i32, i32)>,
    pub update: bool,
}

impl ScreenRenderer {
    pub fn new() -> ScreenRenderer {
        let context = SdlContext::new();

        let last_frame_ticks = Instant::now();

        // determine the grid size dynamically based on window size
        let grid_width = 32;
        let grid_height = 32;

        let window_width = context.window_width as i32;
        let window_height = context.window_height as i32;

        // calculate tile width and height based on window size and grid size
        let tile_width = window_width / grid_width;
        let tile_height = window_height / grid_height;

        ScreenRenderer {
            context,
            camera: (0, 0),
            last_frame_ticks,
            window_width,
            window_height,
            tile_width,
            tile_height,
            grid_size: (grid_width, grid_height),
            cells: HashSet::new(),
            update: false,
        }
    }

    // Render the screen
    pub fn draw(&mut self, fonts: Vec<&Font>) -> Result<(), String> {
        if self.update && self.last_frame_ticks.elapsed().as_millis() >= 160 {
            self.clear_off_screen_cells(25);
            self.update();
            
            self.last_frame_ticks = Instant::now();
        }

        self.draw_bg()?;
        self.draw_grid()?;
        self.draw_cells()?;
        self.draw_buttons(&fonts[0])?;

        self.context.canvas.present();

        Ok(())
    }

    fn draw_bg(&mut self) -> Result<(), String> {
        self.context.canvas.set_draw_color(Color::RGB(28, 28, 40));
        self.context.canvas.clear();

        Ok(())
    }

    pub fn draw_grid(&mut self) -> Result<(), String> {
        self.context.canvas.set_draw_color(Color::RGBA(228, 228, 240, 64));
        // Offset everything by the camera position
        let cam_x = self.camera.0 % self.window_width;
        let cam_y = self.camera.1 % self.window_height;

        // Calculate the number of visible grid lines in both directions
        let visible_x = self.window_width / self.tile_width + 1;
        let visible_y = self.window_height / self.tile_height + 1;

        let starting_x = if cam_x < 0 {
            (self.window_width + cam_x) / self.tile_width
        } else {
            cam_x / self.tile_width
        };

        let starting_y = if cam_y < 0 {
            (self.window_height + cam_y) / self.tile_height
        } else {
            cam_y / self.tile_height
        };

        for x in 0..visible_x {
            let actual_x = ((x + starting_x) * self.tile_width) % self.window_width;

            // Draw vertical line at this x
            self.context.canvas.draw_line((actual_x, 0), (actual_x, self.window_height)).unwrap();
        }

        // Do the same for y
        for y in 0..visible_y {
            let actual_y = ((y + starting_y) * self.tile_height) % self.window_height;

            self.context.canvas.draw_line((0, actual_y), (self.window_width, actual_y)).unwrap();
        }

        Ok(())
    }

    fn draw_cells(&mut self) -> Result<(), String> {
        // for each cell in the set of cells, draw a square at the cell's position
        for &(x, y) in &self.cells {
            // calculate the screen position of the cell
            let screen_x = x - self.camera.0;
            let screen_y = y - self.camera.1;

            // calculate the position of the cell on the screen
            let screen_x = screen_x * self.tile_width;
            let screen_y = screen_y * self.tile_height;

            // draw the cell
            self.context.canvas.set_draw_color(Color::RGB(255, 255, 255));
            self.context.canvas.fill_rect(
                Rect::new(screen_x, screen_y, self.tile_width as u32, self.tile_height as u32)
            )?;
        }

        Ok(())
    }

    fn draw_buttons(&mut self, font: &Font) -> Result<(), String> {
        let button_names = vec![""];
        let button_states = vec![""];

        for (_index, (_button_name, _button_state)) in button_names
            .iter()
            .zip(button_states)
            .enumerate() {
            self.draw_button(font)?;
        }

        Ok(())
    }

    fn draw_button(&mut self, font: &Font) -> Result<(), String> {
        let button_pressed = false;
        let x = self.window_width - 100;
        let y = self.window_height - 100;
        let width = 80;
        let height = 40;

        let text = "Start";

        // Set the button color based on its pressed state
        let button_color = if button_pressed == true {
            Color::RGB(225, 225, 225)
        } else {
            Color::RGB(125, 125, 125)
        };

        let border_color = Color::RGB(26, 28, 26);

        let border_rect = Rect::new(x, y, width, height);
        let button_rect = Rect::new(x + 1, y + 1, width - 1, height - 1);

        // Draw the button border
        self.context.canvas.set_draw_color(border_color);
        self.context.canvas.fill_rect(border_rect)?;

        // Fill the button with the button color
        self.context.canvas.set_draw_color(button_color);
        self.context.canvas.fill_rect(button_rect)?;

        // Render and draw the button text
        let surface = font
            .render(text)
            .blended(Color::RGB(0, 0, 0))
            .map_err(|e| e.to_string())?;

        let texture_creator = self.context.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery { width: texture_width, height: texture_height, .. } = texture.query();

        let target = Rect::new(
            button_rect.x() + ((button_rect.width() as i32) - (texture_width as i32)) / 2,
            button_rect.y() + ((button_rect.height() as i32) - (texture_height as i32)) / 2,
            texture_width,
            texture_height
        );
        self.context.canvas.copy(&texture, None, Some(target))?;

        Ok(())
    }

    pub fn clear_off_screen_cells(&mut self, off_screen_limit: i32) {
        self.cells.retain(|&(x, y)| {
            return 
                x >= -off_screen_limit &&
                x < self.grid_size.0 + off_screen_limit &&
                y >= -off_screen_limit &&
                y < self.grid_size.1 + off_screen_limit
        });
    }

    pub fn get_cell_neighbors(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut neighbors = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx != 0 || dy != 0 {  // exclude the cell itself
                    if self.cells.contains(&(x + dx, y + dy)) {
                        neighbors.push((x + dx, y + dy));
                    }
                }
            }
        }
        neighbors
    }

    pub fn update(&mut self) {
        // create a new set for the updated cells
        let mut updated_cells = HashSet::new();
    
        // add all currently alive cells to the set
        for &(x, y) in &self.cells {
            let neighbors = self.get_cell_neighbors(x, y);
            let alive_neighbors = neighbors.len();
    
            if alive_neighbors == 2 || alive_neighbors == 3 {
                updated_cells.insert((x, y));
            }
        }
    
        // check every cell in the grid
        for x in 0..self.grid_size.0 + 4{
            for y in 0..self.grid_size.1 + 4{
                if !self.cells.contains(&(x, y)) {
                    // if the cell is not currently alive, check if it should become alive
                    let neighbors = self.get_cell_neighbors(x, y);
                    let alive_neighbors = neighbors.len();
    
                    if alive_neighbors == 3 {
                        // if the cell has exactly three neighbors, it becomes alive
                        updated_cells.insert((x, y));
                    }
                }
            }
        }
    
        // replace the old set with the updated one
        self.cells = updated_cells;
    }
    
}
