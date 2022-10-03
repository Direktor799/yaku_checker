use crate::{tile::Tile, yaku::Yaku, T_CHUN, T_HAKU, T_HATSU};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TilePattern {
    /// could be
    /// [1] * 14            kokushi
    /// [2] * 7             chiitoi
    /// [3] * 4 + [2] * 1   common
    pub(crate) pattern: Vec<TileBlock>,
    pub(crate) last_draw: Tile,
}

impl TilePattern {
    pub fn new(pattern: Vec<TileBlock>, last_draw: Tile) -> Self {
        assert_eq!(pattern.iter().map(|block| block.0.len()).sum::<usize>(), 14);
        assert!(pattern.len() == 5 || pattern.len() == 7 || pattern.len() == 14);
        Self { pattern, last_draw }
    }

    pub fn yaku(&self) -> Vec<Yaku> {
        let mut ret = vec![];

        if self.is_kokushi13() {
            return vec![Yaku::Kokushimusou13];
        } else if self.is_kokushi() {
            return vec![Yaku::Kokushimusou];
        }

        if self.is_daisangen() {
            ret.push(Yaku::Daisangen);
        }

        if self.is_suuankoutanki() {
            ret.push(Yaku::Suuankoutanki);
        } else if self.is_suuankou() {
            ret.push(Yaku::Suuankou);
        }

        ret
    }

    fn is_tanyao(&self) -> bool {
        self.pattern.len() == 5
            && self.pattern.iter().all(|block| {
                if let Some(tile) = block.triplet() {
                    tile.is_numbered() && !tile.is_terminal()
                } else if let Some(tile) = block.sequence() {
                    tile.is_numbered() && tile.number() > 1 && tile.number() < 7
                } else if let Some(tile) = block.pair() {
                    tile.is_numbered() && !tile.is_terminal()
                } else {
                    unreachable!()
                }
            })
    }

    fn have_yakuhai_sangenpai(&self) -> Vec<Tile> {
        let mut ans = vec![];
        if self.pattern.len() == 5 {
            ans = self
                .pattern
                .iter()
                .filter_map(|block| match block.triplet() {
                    Some(tile) if tile.is_dragon() => Some(tile),
                    _ => None,
                })
                .collect();
            ans.sort();
        }
        ans
    }

    fn is_pinfu(&self) -> bool {
        if self.pattern.len() == 5 {
            let mut is_sidewait = false;
            for block in &self.pattern {
                if let Some(tile) = block.sequence() {
                    if tile.tile_type() == self.last_draw.tile_type()
                        && (tile.number() == self.last_draw.number()
                            || tile.number() + 2 == self.last_draw.number())
                    {
                        is_sidewait = true;
                    }
                } else if let Some(tile) = block.pair() {
                    if tile.is_dragon() {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            is_sidewait
        } else {
            false
        }
    }

    fn is_iipeikou(&self) -> bool {
        if self.pattern.len() == 5 {
            let mut seq_starts = vec![];
            for block in &self.pattern {
                if let Some(tile) = block.sequence() {
                    seq_starts.push(tile.clone());
                }
            }
            seq_starts.sort();
            seq_starts.windows(2).any(|seq| seq[0] == seq[1])
        } else {
            false
        }
    }

    fn is_sanshokudoukou(&self) -> bool {
        if self.pattern.len() == 5 {
            let mut triplets = vec![];
            for block in &self.pattern {
                if let Some(tile) = block.triplet() {
                    triplets.push(tile.clone());
                }
            }
            let check_three_triplet = |target: &[Tile]| -> bool {
                target.iter().all(|tile| tile.is_numbered())
                    && target[0].number() == target[1].number()
                    && target[0].number() == target[2].number()
                    && target[0].tile_type() != target[1].tile_type()
                    && target[0].tile_type() != target[2].tile_type()
                    && target[1].tile_type() != target[2].tile_type()
            };
            match triplets.len() {
                0 | 1 | 2 => false,
                3 => check_three_triplet(&triplets),
                4 => (0..4).any(|index| {
                    let mut three_triplets = triplets.clone();
                    three_triplets.remove(index);
                    check_three_triplet(&three_triplets)
                }),
                _ => {
                    unreachable!()
                }
            }
        } else {
            false
        }
    }

    fn is_toitoihou(&self) -> bool {
        self.pattern.len() == 5
            && self
                .pattern
                .iter()
                .all(|block| block.triplet().is_some() || block.pair().is_some())
    }

    fn is_sanankou(&self) -> bool {
        self.pattern.len() == 5
            && self
                .pattern
                .iter()
                .filter_map(|block| block.triplet())
                .count()
                >= 3
    }

    fn is_shousangen(&self) -> bool {
        // self.pattern.len() == 5 && self.pattern.iter()
        todo!()
    }

    fn is_kokushi13(&self) -> bool {
        self.pattern.len() == 14
            && self
                .pattern
                .windows(2)
                .any(|pair| pair[0].0[0] == pair[1].0[0] && pair[0].0[0] == self.last_draw)
    }

    fn is_kokushi(&self) -> bool {
        self.pattern.len() == 14
            && !self
                .pattern
                .windows(2)
                .any(|pair| pair[0].0[0] == pair[1].0[0] && pair[0].0[0] == self.last_draw)
    }

    fn is_daisangen(&self) -> bool {
        self.pattern.len() == 5
            && [&T_HAKU, &T_HATSU, &T_CHUN].into_iter().all(|tile| {
                self.pattern.iter().any(|block| {
                    if let Some(t) = block.triplet() {
                        t == **tile
                    } else {
                        false
                    }
                })
            })
    }

    fn is_suuankou(&self) -> bool {
        self.pattern.len() == 5
            && self
                .pattern
                .iter()
                .all(|block| block.triplet().is_some() || block.pair().is_some())
    }

    fn is_suuankoutanki(&self) -> bool {
        self.pattern.len() == 5
            && self.pattern.iter().all(|block| {
                if let Some(tile) = block.pair() {
                    tile == self.last_draw
                } else {
                    block.triplet().is_some()
                }
            })
    }

    fn is_daisuushii(&self) -> bool {
        self.pattern.len() == 5
            && self.pattern.iter().all(|block| {
                if let Some(tile) = block.triplet() {
                    tile.is_wind()
                } else {
                    block.pair().is_some()
                }
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TileBlock(Vec<Tile>);

impl TileBlock {
    pub fn new(tiles: Vec<Tile>) -> Self {
        TileBlock(tiles)
    }

    pub fn triplet(&self) -> Option<Tile> {
        if self.0.len() == 3 && self.0[0] == self.0[1] && self.0[0] == self.0[2] {
            Some(self.0[0].clone())
        } else {
            None
        }
    }

    pub fn sequence(&self) -> Option<Tile> {
        if self.0.len() == 3
            && self.0[0].tile_type() == self.0[1].tile_type()
            && self.0[0].tile_type() == self.0[2].tile_type()
            && self.0[0].number() + 1 == self.0[1].number()
            && self.0[0].number() + 2 == self.0[2].number()
        {
            Some(self.0[0].clone())
        } else {
            None
        }
    }

    pub fn pair(&self) -> Option<Tile> {
        if self.0.len() == 2 && self.0[0] == self.0[1] {
            Some(self.0[0].clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_pattern(pattern: Vec<Vec<&str>>, last_draw: &str) -> TilePattern {
        let pattern = pattern
            .iter()
            .map(|block| TileBlock::new(block.iter().map(|s| s.parse().unwrap()).collect()))
            .collect();
        let last_draw = last_draw.parse().unwrap();
        TilePattern { pattern, last_draw }
    }

    #[test]
    fn tanyao() {
        let tileset = vec![
            vec!["2p", "2p", "2p"],
            vec!["6p", "7p", "8p"],
            vec!["4m", "5m", "6m"],
            vec!["6s", "6s", "6s"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset, "3p");
        assert!(pattern.is_tanyao());

        let tileset = vec![
            vec!["2p", "2p", "2p"],
            vec!["7p", "8p", "9p"],
            vec!["4m", "5m", "6m"],
            vec!["6s", "6s", "6s"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset, "3p");
        assert!(!pattern.is_tanyao());
    }

    #[test]
    fn sangenpai() {
        let tileset = vec![
            vec!["2p", "2p", "2p"],
            vec!["chun", "chun", "chun"],
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["6s", "6s", "6s"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset, "3p");
        assert_eq!(
            pattern.have_yakuhai_sangenpai(),
            vec![T_HATSU.clone(), T_CHUN.clone()]
        );

        let tileset = vec![
            vec!["2p", "2p", "2p"],
            vec!["7p", "8p", "9p"],
            vec!["4m", "5m", "6m"],
            vec!["6s", "6s", "6s"],
            vec!["haku", "haku"],
        ];
        let pattern = build_pattern(tileset, "3p");
        assert!(pattern.have_yakuhai_sangenpai().is_empty());
    }

    #[test]
    fn pinfu() {
        let tileset = vec![
            vec!["2p", "3p", "4p"],
            vec!["5m", "6m", "7m"],
            vec!["3s", "4s", "5s"],
            vec!["7s", "8s", "9s"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset.clone(), "2p");
        assert!(pattern.is_pinfu());
        let pattern = build_pattern(tileset.clone(), "3p");
        assert!(!pattern.is_pinfu());
        let pattern = build_pattern(tileset, "4s");
        assert!(!pattern.is_pinfu());

        let tileset = vec![
            vec!["2p", "3p", "4p"],
            vec!["5m", "6m", "7m"],
            vec!["3s", "4s", "5s"],
            vec!["7s", "8s", "9s"],
            vec!["haku", "haku"],
        ];
        let pattern = build_pattern(tileset, "9s");
        assert!(!pattern.is_pinfu());

        let tileset = vec![
            vec!["2p", "3p", "4p"],
            vec!["5m", "5m", "5m"],
            vec!["3s", "4s", "5s"],
            vec!["7s", "8s", "9s"],
            vec!["haku", "haku"],
        ];
        let pattern = build_pattern(tileset, "9s");
        assert!(!pattern.is_pinfu());
    }

    #[test]
    fn iipeikou() {
        let tileset = vec![
            vec!["2p", "3p", "4p"],
            vec!["2p", "3p", "4p"],
            vec!["3s", "4s", "5s"],
            vec!["7s", "8s", "9s"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset.clone(), "2p");
        assert!(pattern.is_iipeikou());

        let tileset = vec![
            vec!["2p", "3p", "4p"],
            vec!["2p", "3p", "4p"],
            vec!["2p", "3p", "4p"],
            vec!["7s", "8s", "9s"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset.clone(), "2p");
        assert!(pattern.is_iipeikou());

        let tileset = vec![
            vec!["2p", "2p", "2p"],
            vec!["3p", "3p", "3p"],
            vec!["4p", "4p", "4p"],
            vec!["7s", "8s", "9s"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset.clone(), "2p");
        assert!(!pattern.is_iipeikou());
    }

    #[test]
    fn sanshokudoukou() {
        let tileset = vec![
            vec!["2p", "2p", "2p"],
            vec!["2s", "2s", "2s"],
            vec!["2m", "2m", "2m"],
            vec!["7s", "8s", "9s"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset.clone(), "2p");
        assert!(pattern.is_sanshokudoukou());

        let tileset = vec![
            vec!["2p", "2p", "2p"],
            vec!["3s", "3s", "3s"],
            vec!["2s", "2s", "2s"],
            vec!["2m", "2m", "2m"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset.clone(), "2p");
        assert!(pattern.is_sanshokudoukou());

        let tileset = vec![
            vec!["haku", "haku", "haku"],
            vec!["3s", "3s", "3s"],
            vec!["2s", "2s", "2s"],
            vec!["2m", "2m", "2m"],
            vec!["2p", "2p"],
        ];
        let pattern = build_pattern(tileset.clone(), "2p");
        assert!(!pattern.is_sanshokudoukou());
    }

    #[test]
    fn toitoihou() {
        let tileset = vec![
            vec!["2p", "2p", "2p"],
            vec!["3s", "3s", "3s"],
            vec!["2s", "2s", "2s"],
            vec!["2m", "2m", "2m"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset.clone(), "2p");
        assert!(pattern.is_toitoihou());

        let tileset = vec![
            vec!["2p", "2p", "2p"],
            vec!["2s", "2s", "2s"],
            vec!["2m", "2m", "2m"],
            vec!["7s", "8s", "9s"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset.clone(), "2p");
        assert!(!pattern.is_toitoihou());
    }

    #[test]
    fn sanankou() {
        let tileset = vec![
            vec!["2p", "2p", "2p"],
            vec!["3s", "3s", "3s"],
            vec!["2s", "2s", "2s"],
            vec!["2m", "3m", "4m"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset.clone(), "2p");
        assert!(pattern.is_sanankou());

        let tileset = vec![
            vec!["2p", "2p", "2p"],
            vec!["3s", "3s", "3s"],
            vec!["2s", "2s", "2s"],
            vec!["2m", "2m", "2m"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset.clone(), "2p");
        assert!(pattern.is_sanankou());

        let tileset = vec![
            vec!["2p", "2p", "2p"],
            vec!["3s", "3s", "3s"],
            vec!["2s", "3s", "4s"],
            vec!["2m", "3m", "4m"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset.clone(), "2p");
        assert!(!pattern.is_sanankou());
    }

    #[test]
    fn koukushi13() {
        let tileset = [
            "1p", "9p", "1s", "9s", "1m", "9m", "ton", "nan", "shaa", "pei", "haku", "hatsu",
            "hatsu", "chun",
        ];
        let pattern = build_pattern(tileset.iter().map(|&s| vec![s]).collect(), "hatsu");
        assert!(pattern.is_kokushi13());
    }

    #[test]
    fn koukushi() {
        let tileset = [
            "1p", "9p", "1s", "9s", "1m", "9m", "ton", "nan", "shaa", "pei", "haku", "hatsu",
            "hatsu", "chun",
        ];
        let pattern = build_pattern(tileset.iter().map(|&s| vec![s]).collect(), "shaa");
        assert!(pattern.is_kokushi());
    }

    #[test]
    fn daisangen() {
        let tileset = vec![
            vec!["haku", "haku", "haku"],
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["chun", "chun", "chun"],
            vec!["1p", "1p", "1p"],
            vec!["2p", "2p"],
        ];
        let pattern = build_pattern(tileset, "2p");
        assert!(pattern.is_daisangen());
    }

    #[test]
    fn suuankou() {
        let tileset = vec![
            vec!["haku", "haku", "haku"],
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["chun", "chun", "chun"],
            vec!["1p", "1p", "1p"],
            vec!["2p", "2p"],
        ];
        let pattern = build_pattern(tileset, "haku");
        assert!(pattern.is_suuankou());
    }

    #[test]
    fn suuankoutanki() {
        let tileset = vec![
            vec!["haku", "haku", "haku"],
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["chun", "chun", "chun"],
            vec!["1p", "1p", "1p"],
            vec!["2p", "2p"],
        ];
        let pattern = build_pattern(tileset, "2p");
        assert!(pattern.is_suuankoutanki());
    }

    #[test]
    fn daisuushii() {
        let tileset = vec![
            vec!["ton", "ton", "ton"],
            vec!["nan", "nan", "nan"],
            vec!["shaa", "shaa", "shaa"],
            vec!["pei", "pei", "pei"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset.clone(), "3p");
        assert!(pattern.is_daisuushii());

        let tileset = vec![
            vec!["ton", "ton", "ton"],
            vec!["nan", "nan", "nan"],
            vec!["shaa", "shaa", "shaa"],
            vec!["2p", "2p", "2p"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset.clone(), "3p");
        assert!(!pattern.is_daisuushii());
    }
}
