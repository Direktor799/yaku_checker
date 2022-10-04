use crate::{full_set::FullTileSet, tile::Tile, Yaku, ALL_TILES, T_INVALID};
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

#[derive(Debug)]
pub enum ReadyState {
    Tenpai(Vec<(Tile, Vec<Yaku>)>),
    Shanten((u8, Vec<Vec<Yaku>>)),
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
    /// a very heavy search
    pub fn shanten(&self) -> ReadyState {
        let tenpai_ret = ALL_TILES
            .into_iter()
            .filter_map(|draw_tile| {
                self.maybe_effective(draw_tile)
                    .then_some(draw_tile)
                    .and_then(|draw_tile| self.draw(draw_tile).yakus())
                    .map(|yakus| (draw_tile, yakus))
            })
            .collect::<Vec<_>>();
        if !tenpai_ret.is_empty() {
            return ReadyState::Tenpai(tenpai_ret);
        }

        // check kokushi shanten
        let mut yaochuus = self.tiles[..13]
            .iter()
            .filter(|&&tile| tile.is_terminal() || tile.is_honor())
            .collect::<Vec<_>>();
        let bonus = yaochuus.windows(2).any(|tiles| tiles[0] == tiles[1]);
        yaochuus.dedup();
        let distinct_yaochuu_num = yaochuus.len() as u8;
        let mut shanten_num = 13 - (distinct_yaochuu_num + bonus as u8);
        let mut shanten_ret = if bonus {
            vec![vec![Yaku::Kokushimusou]]
        } else {
            vec![vec![Yaku::Kokushimusou13]]
        };

        // check chiitoi shanten
        let mut pair_num = 0;
        let mut index = 0;
        while index < 12 {
            if self.tiles[index] == self.tiles[index + 1] {
                pair_num += 1;
                index += 2;
            } else {
                index += 1;
            }
        }
        if 6 - pair_num < shanten_num {
            // only to set max depth
            shanten_num = 6 - pair_num;
            // push later
            shanten_ret.clear();
        }

        let mut search_queue = VecDeque::new();
        search_queue.push_back((0u8, *self));
        while let Some((depth, ready_set)) = search_queue.pop_front() {
            if depth > shanten_num {
                break;
            }
            for draw_tile in ALL_TILES
                .into_iter()
                .filter(|&tile| ready_set.maybe_effective(tile))
            {
                let full_set = ready_set.draw(draw_tile);
                if let Some(yakus) = full_set.yakus() {
                    if depth < shanten_num {
                        shanten_ret.clear();
                    }
                    shanten_num = depth;
                    shanten_ret.push(yakus);
                } else {
                    for discard_tile in full_set
                        .tiles
                        .into_iter()
                        .filter(|&tile| tile != draw_tile && full_set.is_useless(tile))
                    {
                        let next_ready_set = full_set.discard(discard_tile).unwrap();
                        search_queue.push_back((depth + 1, next_ready_set));
                    }
                }
            }
        }
        shanten_ret.sort();
        shanten_ret.dedup();
        ReadyState::Shanten((shanten_num, shanten_ret))
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

    /// maybe forward shanten
    pub fn maybe_effective(&self, draw: Tile) -> bool {
        self.tiles[..13].iter().any(|&tile| {
            draw.tile_type() == tile.tile_type()
                && (draw.number() as i8 - tile.number() as i8).abs() <= 2
        })
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
