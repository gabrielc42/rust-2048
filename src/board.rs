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

    pub fn generate_tile(&mut self) {}

    pub fn update(&mut self, dt: f64) {}

    pub fn render(&self, number_renderer: &NumberRenderer, c: &Context, gl: &mut GlGraphics) {}

    pub fn merge_from_bottom_to_top(&mut self) {}

    pub fn merge_from_top_to_bottom(&mut self) {}

    fn merge_col(&mut self, y_start: i32, y_end: i32, y_step: i32) {}

    pub fn merge_from_left_to_right(&mut self) {}

    pub fn merge_from_right_to_left(&mut self) {}

    fn merge_row(&mut self, x_start: i32, x_end: i32, x_step: i32) {}

    fn is_locking(&self) -> bool {}

    // returns next tile right besides (x, y)
    fn get_next_tile<'b>(
        &'b self,
        x: i32,
        y: i32,
        step_x: i32,
        step_y: i32,
    ) -> Option<&'b Tile<'a>> {
    }

    fn get_mut_next_tile<'b>() {}

    fn get_tile<'b>() {}

    fn get_mut_tile<'b>() {}

    fn get_tile_count() -> i32 {}

    fn render_board() {}

    fn render_tiles() {}

    fn add_score() {}
}
