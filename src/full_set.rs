use std::{collections::BTreeMap, vec};

use super::ready_set::ReadyTileSet;
use crate::{
    tile::Tile,
    tile_block::TileBlock,
    tile_pattern::TilePattern,
    yaku::{Han, Yaku},
};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy)]
pub struct FullTileSet {
    pub(crate) tiles: [Tile; 14],
    pub(crate) last_draw: Tile,
}

impl FullTileSet {
    pub fn yakus(&self) -> Option<(Vec<Yaku>, Han)> {
        let patterns = self.patterns();
        let mut possible_yakus = patterns
            .iter()
            .map(|pattern| pattern.yakus())
            .collect::<Vec<_>>();
        if !patterns.is_empty() {
            possible_yakus.push(vec![]);
        }
        possible_yakus.sort_by_key(|yakus| yakus.iter().map(|&yaku| yaku.into()).sum::<Han>());
        possible_yakus
            .last()
            .map(|v| (v.clone(), v.iter().map(|&yaku| yaku.into()).sum::<Han>()))
    }

    pub fn discard(self, tile: &Tile) -> Result<ReadyTileSet> {
        let mut tiles = self.tiles;
        if let Some(index) = tiles.iter().position(|t| t == tile) {
            tiles[index..].rotate_left(1);
            Ok(ReadyTileSet { tiles })
        } else {
            Err(anyhow!("no such tile"))
        }
    }

    /// Vec of all possible patterns
    fn patterns(&self) -> Vec<TilePattern> {
        let mut patterns = vec![];
        // check kokushi
        if self
            .tiles
            .iter()
            .all(|tile| tile.is_terminal() || tile.is_honor())
            && self
                .tiles
                .windows(2)
                .filter(|pair| pair[0] == pair[1])
                .count()
                == 1
        {
            patterns.push(TilePattern::new(
                self.tiles
                    .iter()
                    .map(|&tile| TileBlock::new(&[tile]))
                    .collect(),
                self.last_draw,
            ));
        }

        // check chiitoi
        if self.tiles.chunks(2).all(|pair| pair[0] == pair[1])
            && self.tiles.windows(3).all(|quad| quad[0] != quad[2])
        {
            let pattern = self.tiles.chunks(2).map(TileBlock::new).collect();
            patterns.push(TilePattern::new(pattern, self.last_draw));
        }

        // check common
        let mut tile_left = BTreeMap::new();
        for &tile in &self.tiles {
            *tile_left.entry(tile).or_default() += 1;
        }
        for mut pattern in Self::find_common_patterns(&mut tile_left, 4, 1, self.last_draw) {
            pattern.sort();
            patterns.push(TilePattern::new(pattern, self.last_draw));
        }
        patterns.sort();
        patterns.dedup();
        patterns
    }

    /// return vec of possible unfinished patterns
    fn find_common_patterns(
        tile_left: &mut BTreeMap<Tile, u8>,
        group_left: u8,
        pair_left: u8,
        last_draw: Tile,
    ) -> Vec<Vec<TileBlock>> {
        if group_left == 0 && pair_left == 0 {
            return vec![vec![]];
        }
        let mut ret = vec![];

        // continue with a triplet
        if group_left > 0 {
            let (&k, &v) = tile_left.iter().find(|(_, &v)| v != 0).unwrap();
            if v >= 3 {
                let current = TileBlock::new(&[k; 3]);
                *tile_left.get_mut(&k).unwrap() -= 3;
                Self::find_common_patterns(tile_left, group_left - 1, pair_left, last_draw)
                    .into_iter()
                    .map(|mut v| {
                        v.push(current);
                        v
                    })
                    .for_each(|v| ret.push(v));
                *tile_left.get_mut(&k).unwrap() += 3;
            }
        }

        // continue with a sequence
        if group_left > 0 && tile_left.iter().filter(|(_, &v)| v > 0).count() >= 3 {
            let ks = tile_left
                .iter()
                .filter_map(|(&k, &v)| if v > 0 { Some(k) } else { None })
                .take(3)
                .collect::<Vec<_>>();
            let current = TileBlock::new(&ks);

            if current.sequence().is_some() {
                *tile_left.get_mut(&current.tiles()[0]).unwrap() -= 1;
                *tile_left.get_mut(&current.tiles()[1]).unwrap() -= 1;
                *tile_left.get_mut(&current.tiles()[2]).unwrap() -= 1;
                Self::find_common_patterns(tile_left, group_left - 1, pair_left, last_draw)
                    .into_iter()
                    .map(|mut v| {
                        v.push(current);
                        v
                    })
                    .for_each(|v| ret.push(v));
                *tile_left.get_mut(&current.tiles()[0]).unwrap() += 1;
                *tile_left.get_mut(&current.tiles()[1]).unwrap() += 1;
                *tile_left.get_mut(&current.tiles()[2]).unwrap() += 1;
            }
        }

        // continue with a pair
        if pair_left > 0 {
            let (&k, &v) = tile_left.iter().find(|(_, &v)| v != 0).unwrap();
            if v >= 2 {
                let current = TileBlock::new(&[k; 2]);
                *tile_left.get_mut(&k).unwrap() -= 2;
                Self::find_common_patterns(tile_left, group_left, pair_left - 1, last_draw)
                    .into_iter()
                    .map(|mut v| {
                        v.push(current);
                        v
                    })
                    .for_each(|v| ret.push(v));
                *tile_left.get_mut(&k).unwrap() += 2;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use std::str::FromStr;

    #[test]
    fn kukoshi_pattern() {
        let tileset = ReadyTileSet::from_str("19p 19s 19m haku hatsu chun tonn nan shaa pei")
            .unwrap()
            .draw(T_CHUN);
        let patterns = tileset.patterns();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].pattern.len(), 14);
        assert_eq!(patterns[0].last_draw, T_CHUN);
    }

    #[test]
    fn chiitoi_pattern() {
        let tileset = ReadyTileSet::from_str("1p2 2s2 3m2 4p2 5s2 6m2 7p")
            .unwrap()
            .draw(T_7P);
        let patterns = tileset.patterns();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].pattern.len(), 7);
        assert_eq!(patterns[0].last_draw, T_7P);
        let tileset = ReadyTileSet::from_str("1p2 2s2 3m2 4p2 5s2 6m3")
            .unwrap()
            .draw(T_6S);
        let patterns = tileset.patterns();
        assert_eq!(patterns.len(), 0);
    }

    #[test]
    fn common_pattern() {
        let tileset = ReadyTileSet::from_str("123456789p 1234m")
            .unwrap()
            .draw(T_4M);
        let patterns = tileset.patterns();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].pattern.len(), 5);
        assert_eq!(patterns[0].last_draw, T_4M);

        let tileset = ReadyTileSet::from_str("1p3 2p3 3p 4p3 1m3")
            .unwrap()
            .draw(T_2P);
        let patterns = tileset.patterns();
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0].pattern.len(), 5);
        assert_eq!(patterns[1].pattern.len(), 5);
        assert_eq!(patterns[0].last_draw, T_2P);

        let tileset = ReadyTileSet::from_str("1p3 2p3 3p3 4p3 haku")
            .unwrap()
            .draw(T_HAKU);
        let patterns = tileset.patterns();
        assert_eq!(patterns.len(), 3);
        assert_eq!(patterns[0].pattern.len(), 5);
        assert_eq!(patterns[1].pattern.len(), 5);
        assert_eq!(patterns[2].pattern.len(), 5);
        assert_eq!(patterns[0].last_draw, T_HAKU.clone());
    }

    #[test]
    fn non_pattern() {
        let tileset = ReadyTileSet::from_str("124578p 124578m 1s")
            .unwrap()
            .draw(T_1S);
        let patterns = tileset.patterns();
        assert!(patterns.is_empty());
    }
}
