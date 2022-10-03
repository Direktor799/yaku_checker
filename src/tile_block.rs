use crate::{Tile, T_INVALID};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TileBlock {
    tiles: [Tile; 3],
    len: u8,
}

impl TileBlock {
    pub fn new(tiles: &[Tile]) -> Self {
        let mut dst = [T_INVALID; 3];
        dst[..tiles.len()].clone_from_slice(tiles);
        TileBlock {
            tiles: dst,
            len: tiles.len() as u8,
        }
    }

    pub fn triplet(&self) -> Option<Tile> {
        if self.len == 3 && self.tiles[0] == self.tiles[1] && self.tiles[0] == self.tiles[2] {
            Some(self.tiles[0])
        } else {
            None
        }
    }

    pub fn sequence(&self) -> Option<Tile> {
        if self.len == 3
            && self.tiles[0].tile_type() == self.tiles[1].tile_type()
            && self.tiles[0].tile_type() == self.tiles[2].tile_type()
            && self.tiles[0].number() + 1 == self.tiles[1].number()
            && self.tiles[0].number() + 2 == self.tiles[2].number()
        {
            Some(self.tiles[0])
        } else {
            None
        }
    }

    pub fn pair(&self) -> Option<Tile> {
        if self.len == 2 && self.tiles[0] == self.tiles[1] && self.tiles[0] != self.tiles[2] {
            Some(self.tiles[0])
        } else {
            None
        }
    }

    pub fn len(&self) -> u8 {
        self.len
    }

    pub fn tiles(&self) -> &[Tile] {
        &self.tiles[..self.len as usize]
    }
}

impl Ord for TileBlock {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.tiles().cmp(other.tiles())
    }
}

impl PartialOrd for TileBlock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
