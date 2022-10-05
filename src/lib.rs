//! Closed hand only
//! Focused on current tile set (No round wind, dora, discarded, etc)

mod full_set;
mod ready_set;
mod tile;
mod tile_block;
mod tile_pattern;
mod yaku;

pub use ready_set::ReadyTileSet;
pub use tile::*;
pub use yaku::Yaku;

// TODO: non-recursion pattern searching & cache result hashes for better pruning
