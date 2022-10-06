use crate::{full_set::FullTileSet, tile::Tile, tile_block::TileBlock, Yaku, ALL_TILES, T_INVALID};
use anyhow::{anyhow, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Display,
    str::FromStr,
};

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
    /// a very heavy search for all possible situation
    pub fn check(&self) -> (u8, Vec<(Tile, Vec<Yaku>)>) {
        // check tenpai
        let tenpai_ret = ALL_TILES
            .into_iter()
            .filter_map(|draw_tile| self.draw(draw_tile).yakus().map(|yakus| (draw_tile, yakus)))
            .collect::<Vec<_>>();
        if !tenpai_ret.is_empty() {
            return (0, tenpai_ret);
        }

        // check kokushi shanten
        let mut yaochuus = self.tiles[..13]
            .iter()
            .filter(|&&tile| tile.is_terminal() || tile.is_honor())
            .cloned()
            .collect::<Vec<_>>();
        let extras = yaochuus
            .windows(2)
            .filter_map(|tiles| {
                if tiles[0] == tiles[1] {
                    Some(tiles[0])
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        yaochuus.dedup();
        let distinct_yaochuu_num = yaochuus.len() as u8;

        // all three kind
        let kokushi_shanten_num = 13 - (distinct_yaochuu_num + extras.is_empty() as u8);
        let common_shanten_num = self.common_shanten();
        let chiitoi_shanten_num = self.chiitou_shanten();

        let shanten_num = kokushi_shanten_num
            .min(common_shanten_num)
            .min(chiitoi_shanten_num);
        let mut shanten_ret = vec![];
        if shanten_num > 2 {
            return (shanten_num, shanten_ret);
        }

        // kokushi
        if kokushi_shanten_num == shanten_num {
            let all_yaochuus = ALL_TILES
                .into_iter()
                .filter(|tile| tile.is_honor() || tile.is_terminal())
                .collect::<Vec<_>>();
            shanten_ret = if extras.is_empty() {
                all_yaochuus
                    .iter()
                    .map(|&tile| {
                        if yaochuus.iter().any(|&t| t == tile) {
                            (tile, vec![Yaku::Kokushimusou])
                        } else {
                            (tile, vec![Yaku::Kokushimusou13])
                        }
                    })
                    .collect::<Vec<_>>()
            } else {
                all_yaochuus
                    .iter()
                    .filter(|&&tile| !yaochuus.iter().any(|&t| t == tile))
                    .map(|&tile| (tile, vec![Yaku::Kokushimusou]))
                    .collect()
            };
        }

        // chiitoi
        if chiitoi_shanten_num == shanten_num {
            let mut search_queue = VecDeque::new();
            search_queue.push_back((chiitoi_shanten_num, T_INVALID, *self));
            while let Some((cur_shanten_num, first_income, ready_set)) = search_queue.pop_front() {
                for draw_tile in ALL_TILES
                    .into_iter()
                    .filter(|&tile| ready_set.maybe_effective(tile))
                {
                    let full_set = ready_set.draw(draw_tile);
                    let yakus = full_set.yakus();

                    if let Some(yakus) = yakus {
                        shanten_ret.push((first_income, yakus));
                    } else if cur_shanten_num != 0 {
                        for discard_tile in
                            full_set.tiles.into_iter().filter(|&tile| tile != draw_tile)
                        {
                            let next_ready_set = full_set.discard(discard_tile).unwrap();
                            let next_shanten_num = next_ready_set.chiitou_shanten();
                            if next_shanten_num < cur_shanten_num {
                                search_queue.push_back((
                                    next_shanten_num,
                                    if first_income == T_INVALID {
                                        draw_tile
                                    } else {
                                        first_income
                                    },
                                    next_ready_set,
                                ));
                            }
                        }
                    }
                }
            }
        }

        // common
        if common_shanten_num == shanten_num {
            let mut search_queue = VecDeque::new();
            search_queue.push_back((common_shanten_num, T_INVALID, *self));
            while let Some((cur_shanten_num, first_income, ready_set)) = search_queue.pop_front() {
                for draw_tile in ALL_TILES
                    .into_iter()
                    .filter(|&tile| ready_set.maybe_effective(tile))
                {
                    let full_set = ready_set.draw(draw_tile);
                    let yakus = full_set.yakus();

                    if let Some(yakus) = yakus {
                        shanten_ret.push((first_income, yakus));
                    } else if cur_shanten_num != 0 {
                        for discard_tile in
                            full_set.tiles.into_iter().filter(|&tile| tile != draw_tile)
                        {
                            let next_ready_set = full_set.discard(discard_tile).unwrap();
                            let next_shanten_num = next_ready_set.common_shanten();
                            if next_shanten_num < cur_shanten_num {
                                search_queue.push_back((
                                    next_shanten_num,
                                    if first_income == T_INVALID {
                                        draw_tile
                                    } else {
                                        first_income
                                    },
                                    next_ready_set,
                                ));
                            }
                        }
                    }
                }
            }
        }

        shanten_ret.sort();
        shanten_ret.dedup();
        (shanten_num, shanten_ret)
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

    /// calculate shanten num and tiles that could forward shanten in chiitoi pattern
    fn chiitou_shanten(&self) -> u8 {
        let mut pair_tile = vec![];
        let mut index = 0;
        while index < 12 {
            if self.tiles[index] == self.tiles[index + 1] {
                if let Some(&last) = pair_tile.last() {
                    if last != self.tiles[index] {
                        pair_tile.push(self.tiles[index]);
                    }
                } else {
                    pair_tile.push(self.tiles[index]);
                }
                index += 2;
            } else {
                index += 1;
            }
        }
        6 - pair_tile.len() as u8
    }

    /// calculate shanten num and tiles that could forward shanten in common pattern
    fn common_shanten(&self) -> u8 {
        let mut tile_left = BTreeMap::new();
        for &tile in &self.tiles[..13] {
            *tile_left.entry(tile).or_default() += 1;
        }

        let mut min_shanten = 8;
        Self::find_common_patterns(&mut tile_left, 0, 0, false, &mut min_shanten);
        min_shanten
    }

    fn find_common_patterns(
        tile_left: &mut BTreeMap<Tile, u8>,
        completed: u8,
        incompleted: u8,
        have_pair: bool,
        min_shanten: &mut u8,
    ) {
        if completed + incompleted - have_pair as u8 > 4 {
            return;
        }

        let kvs = tile_left
            .iter()
            .filter_map(|(&k, &v)| if v > 0 { Some((k, v)) } else { None })
            .collect::<Vec<_>>();

        let current_shanten = 8 - 2 * completed - incompleted;
        *min_shanten = (*min_shanten).min(current_shanten);
        if kvs.is_empty() {
            return;
        }

        let tile_left_num = kvs.iter().map(|(_, v)| *v).sum::<u8>();
        if tile_left_num / 3 * 2 + (tile_left_num % 3) / 2 < current_shanten - *min_shanten {
            return;
        }

        // continue with a triplet
        if *tile_left.get(&kvs[0].0).unwrap() >= 3 {
            *tile_left.get_mut(&kvs[0].0).unwrap() -= 3;
            Self::find_common_patterns(
                tile_left,
                completed + 1,
                incompleted,
                have_pair,
                min_shanten,
            );
            *tile_left.get_mut(&kvs[0].0).unwrap() += 3;
        }

        // continue with a sequence
        if kvs.len() >= 3 {
            if let Ok(current) = TileBlock::new_sequence([kvs[0].0, kvs[1].0, kvs[2].0]) {
                *tile_left.get_mut(&current.tiles()[0]).unwrap() -= 1;
                *tile_left.get_mut(&current.tiles()[1]).unwrap() -= 1;
                *tile_left.get_mut(&current.tiles()[2]).unwrap() -= 1;
                Self::find_common_patterns(
                    tile_left,
                    completed + 1,
                    incompleted,
                    have_pair,
                    min_shanten,
                );
                *tile_left.get_mut(&current.tiles()[0]).unwrap() += 1;
                *tile_left.get_mut(&current.tiles()[1]).unwrap() += 1;
                *tile_left.get_mut(&current.tiles()[2]).unwrap() += 1;
            }
        }

        // continue with a pair
        if *tile_left.get(&kvs[0].0).unwrap() >= 2 {
            *tile_left.get_mut(&kvs[0].0).unwrap() -= 2;
            Self::find_common_patterns(tile_left, completed, incompleted + 1, true, min_shanten);
            *tile_left.get_mut(&kvs[0].0).unwrap() += 2;
        }

        // continue with a incompleted AB
        if kvs.len() >= 2 {
            if let Ok(current) = TileBlock::new_incompleted([kvs[0].0, kvs[1].0]) {
                *tile_left.get_mut(&current.tiles()[0]).unwrap() -= 1;
                *tile_left.get_mut(&current.tiles()[1]).unwrap() -= 1;
                Self::find_common_patterns(
                    tile_left,
                    completed,
                    incompleted + 1,
                    have_pair,
                    min_shanten,
                );
                *tile_left.get_mut(&current.tiles()[0]).unwrap() += 1;
                *tile_left.get_mut(&current.tiles()[1]).unwrap() += 1;
            }
        }

        // continue with a incompleted AC
        if kvs.len() >= 3 {
            if let Ok(current) = TileBlock::new_incompleted([kvs[0].0, kvs[2].0]) {
                *tile_left.get_mut(&current.tiles()[0]).unwrap() -= 1;
                *tile_left.get_mut(&current.tiles()[1]).unwrap() -= 1;
                Self::find_common_patterns(
                    tile_left,
                    completed,
                    incompleted + 1,
                    have_pair,
                    min_shanten,
                );
                *tile_left.get_mut(&current.tiles()[0]).unwrap() += 1;
                *tile_left.get_mut(&current.tiles()[1]).unwrap() += 1;
            }
        }

        // continue without this tile
        *tile_left.get_mut(&kvs[0].0).unwrap() -= 1;
        Self::find_common_patterns(tile_left, completed, incompleted, have_pair, min_shanten);
        *tile_left.get_mut(&kvs[0].0).unwrap() += 1;
    }

    /// maybe forward shanten
    fn maybe_effective(&self, draw: Tile) -> bool {
        self.tiles[..13].iter().any(|&tile| draw.is_related(tile))
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
