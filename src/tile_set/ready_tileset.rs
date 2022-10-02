use std::{fmt::Display, str::FromStr};

use crate::tile::Tile;
use anyhow::{anyhow, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;

use super::FullTileSet;

static TILESET_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"((ton|nan|shaa|pei|haku|chun|hatsu)|([1-9]+)([psm]))(\d+)?").unwrap()
});

#[derive(Debug)]
pub struct ReadyTileSet {
    pub(super) tiles: Vec<Tile>,
}

impl FromStr for ReadyTileSet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut tiles = vec![];
        for cap in TILESET_REGEX.captures_iter(s) {
            let num = cap
                .get(5)
                .map_or_else(|| 1, |s| s.as_str().parse().unwrap());
            let v = if let Some(s) = cap.get(2) {
                vec![Tile::from_str(s.as_str()).unwrap()]
            } else {
                cap[3]
                    .chars()
                    .map(|ch| Tile::from_str(&format!("{}{}", ch, &cap[4])).unwrap())
                    .collect()
            };
            for _ in 0..num {
                tiles.extend_from_slice(&v);
            }
        }
        if tiles.len() != 13 {
            Err(anyhow!("wrong number of tiles: {}", tiles.len()))
        } else {
            tiles.sort();
            Ok(ReadyTileSet { tiles })
        }
    }
}

impl Display for ReadyTileSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self
            .tiles
            .iter()
            .map(|tile| tile.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "{text}")
    }
}

impl ReadyTileSet {
    pub fn shanten(&self) -> u8 {
        todo!()
    }

    pub fn draw(self, tile: Tile) -> FullTileSet {
        let mut tiles = self.tiles;
        let res = tiles.binary_search(&tile);
        let drawed_index = match res {
            Ok(i) => i,
            Err(i) => i,
        };
        tiles.insert(drawed_index, tile);
        FullTileSet {
            tiles,
            drawed_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ready_tileset_from_str() {
        assert_eq!(
            ReadyTileSet::from_str("123p 4m3 5s1 6s haku1 chun2 nan1 shaa")
                .unwrap()
                .to_string(),
            "4m 4m 4m 5s 6s 1p 2p 3p nan shaa haku chun chun"
        );
        assert_eq!(
            ReadyTileSet::from_str("hatsu1 123p3 hatsu ton pei")
                .unwrap()
                .to_string(),
            "1p 1p 1p 2p 2p 2p 3p 3p 3p ton pei hatsu hatsu"
        );
        assert_eq!(
            ReadyTileSet::from_str("1112345678999p")
                .unwrap()
                .to_string(),
            "1p 1p 1p 2p 3p 4p 5p 6p 7p 8p 9p 9p 9p"
        );

        assert_eq!(
            ReadyTileSet::from_str("hatsu12 haku0 chun")
                .unwrap()
                .to_string(),
            "chun hatsu hatsu hatsu hatsu hatsu hatsu hatsu hatsu hatsu hatsu hatsu hatsu"
        );
    }
}
