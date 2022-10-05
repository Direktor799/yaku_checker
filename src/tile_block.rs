use crate::{Tile, T_INVALID};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileBlock {
    tiles: [Tile; 3],
    block_type: BlockType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum BlockType {
    Triplet,
    Sequence,
    Pair,
    Incompleted,
    Orphan,
}

impl TileBlock {
    pub fn new_triplet(tiles: [Tile; 3]) -> Result<Self> {
        if tiles[0] == tiles[1] && tiles[0] == tiles[2] {
            Ok(TileBlock {
                tiles,
                block_type: BlockType::Triplet,
            })
        } else {
            Err(anyhow!("{:?} is not a triplet", tiles))
        }
    }

    pub fn new_sequence(tiles: [Tile; 3]) -> Result<Self> {
        if tiles[0].tile_type() == tiles[1].tile_type()
            && tiles[0].tile_type() == tiles[2].tile_type()
            && tiles[0].number() + 1 == tiles[1].number()
            && tiles[0].number() + 2 == tiles[2].number()
        {
            Ok(TileBlock {
                tiles,
                block_type: BlockType::Sequence,
            })
        } else {
            Err(anyhow!("{:?} is not a sequence", tiles))
        }
    }

    pub fn new_pair(tiles: [Tile; 2]) -> Result<Self> {
        if tiles[0] == tiles[1] {
            Ok(TileBlock {
                tiles: [tiles[0], tiles[1], T_INVALID],
                block_type: BlockType::Pair,
            })
        } else {
            Err(anyhow!("{:?} is not a pair", tiles))
        }
    }

    pub fn new_incompleted(tiles: [Tile; 2]) -> Result<Self> {
        if tiles[0].is_related(tiles[1]) {
            Ok(TileBlock {
                tiles: [tiles[0], tiles[1], T_INVALID],
                block_type: BlockType::Incompleted,
            })
        } else {
            Err(anyhow!("{:?} is not a incompleted", tiles))
        }
    }

    pub fn new_orphan(tile: Tile) -> Result<Self> {
        Ok(TileBlock {
            tiles: [tile, T_INVALID, T_INVALID],
            block_type: BlockType::Orphan,
        })
    }

    pub fn new_unknown(tiles: &[Tile]) -> Result<Self> {
        match tiles.len() {
            1 => TileBlock::new_orphan(tiles[0]),
            2 => TileBlock::new_pair(tiles.try_into()?),
            3 => TileBlock::new_triplet(tiles.try_into()?)
                .or_else(|_| TileBlock::new_sequence(tiles.try_into()?)),
            _ => Err(anyhow!("{} is too long to be a block", tiles.len())),
        }
    }

    pub fn triplet(&self) -> Option<Tile> {
        if let BlockType::Triplet = self.block_type {
            Some(self.tiles[0])
        } else {
            None
        }
    }

    pub fn sequence(&self) -> Option<Tile> {
        if let BlockType::Sequence = self.block_type {
            Some(self.tiles[0])
        } else {
            None
        }
    }

    pub fn pair(&self) -> Option<Tile> {
        if let BlockType::Pair = self.block_type {
            Some(self.tiles[0])
        } else {
            None
        }
    }

    pub fn income(&self) -> Option<Vec<Tile>> {
        if let BlockType::Incompleted = self.block_type {
            Some(if self.tiles[0] == self.tiles[1] {
                vec![self.tiles[0]]
            } else if self.tiles[0].number() + 1 == self.tiles[1].number() {
                let mut v = vec![];
                if let Some(tile) = self.tiles[0].prev() {
                    v.push(tile);
                }
                if let Some(tile) = self.tiles[1].next() {
                    v.push(tile);
                }
                v
            } else {
                vec![self.tiles[0].next().unwrap()]
            })
        } else if let BlockType::Orphan = self.block_type {
            Some(vec![self.tiles[0]])
        } else {
            None
        }
    }

    pub fn len(&self) -> u8 {
        match self.block_type {
            BlockType::Triplet | BlockType::Sequence => 3,
            BlockType::Pair | BlockType::Incompleted => 2,
            BlockType::Orphan => 1,
        }
    }

    pub fn tiles(&self) -> &[Tile] {
        &self.tiles[..self.len() as usize]
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
