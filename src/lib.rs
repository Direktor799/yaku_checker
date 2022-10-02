//! Closed hand only
//! No Chii, Pon, Kan(opened or closed)

mod tile;
mod tile_pattern;
mod tile_set;
mod yaku;

pub use tile_set::FullTileSet;

#[cfg(test)]
mod tests {
    // use std::str::FromStr;

    // use crate::{tile::Tile, tile_set::ReadyTileSet, yaku::Yaku};

    // #[test]
    // fn test() {
    //     let tiles = ReadyTileSet::from_str("123m 123s 123p haku3 ton").unwrap();
    //     assert_eq!(tiles.shanten(), 1);
    //     let tiles = tiles.draw(Tile::from_str("pei").unwrap());
    //     assert!(tiles.yaku().is_none());
    //     let tiles = tiles.discard(&Tile::from_str("ton").unwrap()).unwrap();
    //     assert_eq!(tiles.shanten(), 1);
    //     let tiles = tiles.draw(Tile::from_str("pei").unwrap());
    //     assert_eq!(
    //         tiles.yaku().unwrap(),
    //         vec![
    //             Yaku::YakuhaiSangenpai(Tile::from_str("haku").unwrap()),
    //             Yaku::Sanshokudoujun
    //         ]
    //     );
    // }
}
