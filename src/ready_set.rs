use crate::{full_set::FullTileSet, tile::Tile, yaku::Han, Yaku, ALL_TILES, T_INVALID};
use anyhow::{anyhow, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::VecDeque, fmt::Display, str::FromStr};

static TILESET_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"((ton|nan|shaa|pei|haku|chun|hatsu)|([1-9]+)([psm]))(\d+)?").unwrap()
});

#[derive(Debug, Clone, Copy)]
pub struct ReadyTileSet {
    pub(crate) tiles: [Tile; 14],
}

impl FromStr for ReadyTileSet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut tiles = Vec::with_capacity(14);
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
            tiles.push(T_INVALID);
            Ok(ReadyTileSet {
                tiles: tiles.try_into().unwrap(),
            })
        }
    }
}

impl Display for ReadyTileSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self
            .tiles
            .iter()
            .take(13)
            .map(|tile| tile.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "{text}")
    }
}

impl ReadyTileSet {
    /// a very heavy search, return (shanten_num, Vec<(effective_income, yakus, han)>)
    pub fn shanten(&self) -> (u8, Vec<(Tile, Vec<Yaku>, Han)>) {
        let mut ret = vec![];
        let mut shanten_num = 6;
        let mut search_queue = VecDeque::new();
        search_queue.push_back((0u8, *self));
        while let Some((depth, ready_set)) = search_queue.pop_front() {
            if depth > shanten_num {
                break;
            }
            for tile in ALL_TILES {
                let full_set = ready_set.draw(tile);
                if let Some((yakus, han)) = full_set.yakus() {
                    shanten_num = depth;
                    ret.push((tile, yakus, han));
                } else {
                    for discard_tile in &full_set.tiles {
                        let next_ready_set = full_set.discard(discard_tile).unwrap();
                        search_queue.push_back((depth + 1, next_ready_set));
                    }
                }
            }
        }
        (shanten_num, ret)
    }

    pub fn draw(self, tile: Tile) -> FullTileSet {
        let mut tiles = self.tiles;
        let res = tiles.binary_search(&tile);
        let index = match res {
            Ok(i) => i,
            Err(i) => i,
        }
        .min(tiles.len() - 1);
        tiles[index..].rotate_right(1);
        tiles[index] = tile;
        FullTileSet {
            tiles,
            last_draw: tile,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(
            ReadyTileSet::from_str("123p 4m3 5s1 6s haku1 chun2 nan1 shaa")
                .unwrap()
                .to_string(),
            "4m 4m 4m 1p 2p 3p 5s 6s nan shaa haku chun chun"
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
            "hatsu hatsu hatsu hatsu hatsu hatsu hatsu hatsu hatsu hatsu hatsu hatsu chun"
        );
    }
}
