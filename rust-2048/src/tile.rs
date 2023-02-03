#[derive(Clone)]
pub struct Tile<'a> {
    pub score: i32,
    pub tile_x: i32,
    pub tile_y: i32,
    pub status: TileState,

    settings: &'a Settings,
}
