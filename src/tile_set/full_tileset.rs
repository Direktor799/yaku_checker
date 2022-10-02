use std::collections::BTreeMap;

use super::ready_tileset::ReadyTileSet;
use crate::{tile::Tile, tile_pattern::TilePattern, yaku::Yaku};
use anyhow::{anyhow, Result};

pub struct FullTileSet {
    pub(super) tiles: Vec<Tile>,
    pub(super) drawed_index: usize,
}

impl FullTileSet {
    pub fn yaku(&self) -> Option<Vec<Yaku>> {
        let _patterns = self.patterns();
        todo!()
    }

    pub fn discard(self, tile: &Tile) -> Result<ReadyTileSet> {
        let mut tiles = self.tiles;
        if let Some(index) = tiles.iter().position(|t| t == tile) {
            tiles.remove(index);
            Ok(ReadyTileSet { tiles })
        } else {
            Err(anyhow!("no such tile"))
        }
    }
    /// Vec of all possible patterns
    fn patterns(&self) -> Vec<TilePattern> {
        let last_draw = self.tiles[self.drawed_index].clone();
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
                self.tiles.iter().map(|tile| vec![tile.clone()]).collect(),
                last_draw.clone(),
            ));
        }

        // check chiitoi
        if self.tiles.chunks(2).all(|pair| pair[0] == pair[1])
            && self.tiles.windows(3).all(|quad| quad[0] != quad[2])
        {
            let pattern = self.tiles.chunks(2).map(|pair| pair.to_vec()).collect();
            patterns.push(TilePattern::new(pattern, last_draw.clone()));
        }

        // check common
        let mut tile_left = BTreeMap::new();
        for tile in &self.tiles {
            *tile_left.entry(tile.clone()).or_default() += 1;
        }
        for mut pattern in Self::find_common_patterns(&mut tile_left, 4, 1, &last_draw) {
            pattern.sort();
            patterns.push(TilePattern::new(pattern, last_draw.clone()));
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
        last_draw: &Tile,
    ) -> Vec<Vec<Vec<Tile>>> {
        if group_left == 0 && pair_left == 0 {
            return vec![vec![]];
        }
        let mut ret = vec![];

        // continue with a triplet
        if group_left > 0 {
            let kv = tile_left.iter().find(|(_, &v)| v != 0).unwrap();
            let (k, &v) = (kv.0.clone(), kv.1);
            if v >= 3 {
                let current = vec![k.clone(); 3];
                *tile_left.get_mut(&k).unwrap() -= 3;
                Self::find_common_patterns(tile_left, group_left - 1, pair_left, last_draw)
                    .into_iter()
                    .map(|mut v| {
                        let mut ans = vec![current.clone()];
                        ans.append(&mut v);
                        ans
                    })
                    .for_each(|v| ret.push(v));
                *tile_left.get_mut(&k).unwrap() += 3;
            }
        }

        // continue with a sequence
        if group_left > 0 && tile_left.iter().filter(|(_, &v)| v > 0).count() >= 3 {
            let iter = tile_left.iter().filter(|(_, &v)| v > 0).take(3);
            let ks = iter.clone().map(|(k, _)| k.clone()).collect::<Vec<_>>();
            let vs = iter.map(|(_, &v)| v).collect::<Vec<_>>();

            if ks[0].tile_type() == ks[1].tile_type()
                && ks[0].tile_type() == ks[2].tile_type()
                && vs[0] > 0
                && vs[1] > 0
                && vs[2] > 0
            {
                let current = ks.clone();
                *tile_left.get_mut(&ks[0]).unwrap() -= 1;
                *tile_left.get_mut(&ks[1]).unwrap() -= 1;
                *tile_left.get_mut(&ks[2]).unwrap() -= 1;
                Self::find_common_patterns(tile_left, group_left - 1, pair_left, last_draw)
                    .into_iter()
                    .map(|mut v| {
                        let mut ans = vec![current.clone()];
                        ans.append(&mut v);
                        ans
                    })
                    .for_each(|v| ret.push(v));
                *tile_left.get_mut(&ks[0]).unwrap() += 1;
                *tile_left.get_mut(&ks[1]).unwrap() += 1;
                *tile_left.get_mut(&ks[2]).unwrap() += 1;
            }
        }

        // continue with a pair
        if pair_left > 0 {
            let kv = tile_left.iter().find(|(_, &v)| v != 0).unwrap();
            let (k, &v) = (kv.0.clone(), kv.1);
            if v >= 2 {
                let current = vec![k.clone(); 2];
                *tile_left.get_mut(&k).unwrap() -= 2;
                Self::find_common_patterns(tile_left, group_left, pair_left - 1, last_draw)
                    .into_iter()
                    .map(|mut v| {
                        let mut ans = vec![current.clone()];
                        ans.append(&mut v);
                        ans
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
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_full_tileset_kukoshi_pattern() {
        let tileset = ReadyTileSet::from_str("19p 19s 19m haku hatsu chun tonn nan shaa pei")
            .unwrap()
            .draw(Tile::from_str("chun").unwrap());
        let patterns = tileset.patterns();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].pattern.len(), 14);
        assert_eq!(patterns[0].last_draw, Tile::from_str("chun").unwrap());
    }

    #[test]
    fn test_full_tileset_chiitoi_pattern() {
        let tileset = ReadyTileSet::from_str("1s2 2s2 3s2 4s2 5s2 6s2 7s")
            .unwrap()
            .draw(Tile::from_str("7s").unwrap());
        let patterns = tileset.patterns();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].pattern.len(), 7);
        assert_eq!(patterns[0].last_draw, Tile::from_str("7s").unwrap());
        let tileset = ReadyTileSet::from_str("1s2 2s2 3s2 4s2 5s2 6s3")
            .unwrap()
            .draw(Tile::from_str("6s").unwrap());
        let patterns = tileset.patterns();
        assert_eq!(patterns.len(), 0);
    }

    #[test]
    fn test_full_tileset_pattern() {
        let tileset = ReadyTileSet::from_str("123456789p 1234m")
            .unwrap()
            .draw(Tile::from_str("4m").unwrap());
        let patterns = tileset.patterns();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].pattern.len(), 5);
        assert_eq!(patterns[0].last_draw, Tile::from_str("4m").unwrap());

        let tileset = ReadyTileSet::from_str("1p3 2p3 3p 4p3 1m3")
            .unwrap()
            .draw(Tile::from_str("2p").unwrap());
        let patterns = tileset.patterns();
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0].pattern.len(), 5);
        assert_eq!(patterns[1].pattern.len(), 5);
        assert_eq!(patterns[0].last_draw, Tile::from_str("2p").unwrap());

        let tileset = ReadyTileSet::from_str("1p3 2p3 3p3 4p3 haku")
            .unwrap()
            .draw(Tile::from_str("haku").unwrap());
        let patterns = tileset.patterns();
        println!("{:?}", patterns);
        assert_eq!(patterns.len(), 3);
        assert_eq!(patterns[0].pattern.len(), 5);
        assert_eq!(patterns[1].pattern.len(), 5);
        assert_eq!(patterns[2].pattern.len(), 5);
        assert_eq!(patterns[0].last_draw, Tile::from_str("haku").unwrap());
    }
}
