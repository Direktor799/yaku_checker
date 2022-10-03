//! Closed hand only
//! No Chii, Pon, Kan(opened or closed)

mod full_set;
mod ready_set;
mod tile;
mod tile_pattern;
mod yaku;

pub use ready_set::ReadyTileSet;
pub use tile::*;

// TODO: change visablity

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let tiles = "123m 123s 123p haku3 ton".parse::<ReadyTileSet>().unwrap();
        // assert_eq!(tiles.shanten(), 1);
        let tiles = tiles.draw(T_PEI.clone());
        assert!(tiles.yaku().is_none());
        let tiles = tiles.discard(&T_TON).unwrap();
        // assert_eq!(tiles.shanten(), 1);
        let _tiles = tiles.draw(T_PEI.clone());
        // assert_eq!(
        //     tiles.yaku().unwrap(),
        //     vec![Yaku::YakuhaiSangenpai(T_HAKU), Yaku::Sanshokudoujun]
        // );
    }
}
