use crate::tile::Tile;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TilePattern {
    /// could be
    /// [1] * 14            kokushi
    /// [2] * 7             chiitoi
    /// [3] * 4 + [2] * 1   common
    pub(super) pattern: Vec<Vec<Tile>>,
    pub(super) last_draw: Tile,
}

impl TilePattern {
    pub fn new(pattern: Vec<Vec<Tile>>, last_draw: Tile) -> Self {
        assert_eq!(pattern.iter().flatten().count(), 14);
        Self { pattern, last_draw }
    }
}
