use number_renderer::NumberRenderer;
use opengl_graphics::GlGraphics;
use piston_window::*;
use rand::random;
use settings::Settings;
use std::collections::HashSet;
use tile::{Tile, TileState};

fn rgb2rgba(c: [f32; 3]) -> [f32; 4] {
    [c[0], c[1], c[2], 1.0]
}

pub struct Board<'a> {
    tiles: Vec<Tile<'a>>,
    score: i32,
    settings: &'a Settings,
}

impl<'a> Board<'a> {
    pub fn new(settings: &'a Settings) -> Board<'a> {
        let mut board = Board {
            tiles: Vec::<Tile>::new(),
            score: 0,
            settings: settings,
        };
        board.generate_tile();
        board.generate_tile();
        board
    }

    pub fn generate_tile(&mut self) {
        if self.tiles.len() == (self.settings.tile_width * self.settings.tile_height) as usize {
            return;
        }

        loop {
            let x = (random::<u32>() % self.settings.tile_width as u32) as i32;
            let y = (random::<u32>() % self.settings.tile_height as u32) as i32;

            if self.get_tile(x, y).is_none() {
                let score = if random::<u32>() % 10 == 0 { 4 } else { 2 };
                self.tiles.push(Tile::new(self.settings, score, x, y));
                break;
            }
        }
    }

    pub fn update(&mut self, dt: f64) {}

    pub fn render(&self, number_renderer: &NumberRenderer, c: &Context, gl: &mut GlGraphics) {}

    pub fn merge_from_bottom_to_top(&mut self) {}

    pub fn merge_from_top_to_bottom(&mut self) {}

    fn merge_col(&mut self, y_start: i32, y_end: i32, y_step: i32) {}

    pub fn merge_from_left_to_right(&mut self) {}

    pub fn merge_from_right_to_left(&mut self) {}

    fn merge_row(&mut self, x_start: i32, x_end: i32, x_step: i32) {}

    fn is_locking(&self) -> bool {
        for tile in self.tiles.iter() {
            if tile.status != TileState::TileStatic {
                return true;
            }
        }
        false
    }

    // returns next tile right besides (x, y)
    fn get_next_tile<'b>(
        &'b self,
        x: i32,
        y: i32,
        step_x: i32,
        step_y: i32,
    ) -> Option<&'b Tile<'a>> {
        let mut x = x + step_x;
        let mut y = y + step_y;
        while x >= 0 && x < self.settings.tile_width && y >= 0 && y < self.settings.tile_height {
            let tile = self.get_tile(x, y);
            if tile.is_some() {
                return tile;
            }

            x += step_x;
            y += step_y;
        }
        None
    }

    fn get_mut_next_tile<'b>(
        &'b mut self,
        x: i32,
        y: i32,
        step_x: i32,
        step_y: i32,
    ) -> Option<&'b mut Tile<'a>> {
        let mut x = x + step_x;
        let mut y = y + step_y;
        let mut found = false;
        while x >= 0 && x < self.settings.tile_width && y >= 0 && y < self.settings.tile_height {
            let tile = self.get_tile(x, y);

            if tile.is_some() {
                found = true;
                break;
            }

            x += step_x;
            y += step_y;
        }

        if found {
            self.get_mut_tile(x, y)
        } else {
            None
        }
    }

    fn get_tile<'b>(&'b self, x: i32, y: i32) -> Option<&'b Tile<'a>> {
        for tile in self.tiles.iter() {
            if tile.tile_x == x && tile.tile_y == y {
                return Some(tile);
            }
        }
        None
    }

    fn get_mut_tile<'b>(&'b mut self, x: i32, y: i32) -> Option<&'b mut Tile<'a>> {
        for tile in self.tiles.iter_mut() {
            if tile.tile_x == x && tile.tile_y == y {
                return Some(tile);
            }
        }
        None
    }

    fn get_tile_count(&self, x: i32, y: i32) -> i32 {
        let mut count = 0;
        for tile in self.tiles.iter() {
            if tile.tile_x == x && tile.tile_y == y {
                count += 1;
            }
        }
        count
    }

    fn render_board(&self, c: &Context, gl: &mut GlGraphics) {
        Rectangle::new(rgb2rgba(self.settings.label_color)).draw(
            [
                self.settings.board_padding,
                self.settings.board_padding + self.settings.board_offset_y,
                self.settings.board_size[0],
                self.settings.board_size[1],
            ],
            &DrawState::default(),
            c.transform,
            gl,
        );

        let mut x = self.settings.board_padding + self.settings.tile_padding;
        let mut y =
            self.settings.board_padding + self.settings.board_offset_y + self.settings.tile_padding;

        for _ in 0..self.settings.tile_height {
            for _ in 0..self.settings.tile_width {
                Rectangle::new(rgb2rgba(self.settings.tiles_colors[0])).draw(
                    [x, y, self.settings.tile_size, self.settings.tile_size],
                    &DrawState::default(),
                    c.transform,
                    gl,
                );

                x += self.settings.tile_padding + self.settings.tile_size;
            }

            x = self.settings.board_padding + self.settings.tile_padding;
            y += self.settings.tile_padding + self.settings.tile_size;
        }
    }

    fn render_tiles(&self, number_renderer: &NumberRenderer, c: &Context, gl: &mut GlGraphics) {
        for tile in self.tiles.iter() {
            tile.render(number_renderer, c, gl);
        }
    }

    fn add_score(&mut self, score: i32) {
        self.score += score;
        println!("Score: {}", self.score);
    }
}
