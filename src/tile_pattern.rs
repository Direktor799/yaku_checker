use crate::{tile::Tile, tile_block::TileBlock, yaku::Yaku, T_2S, T_3S, T_4S, T_6S, T_8S, T_HATSU};
use std::collections::HashMap;

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
        assert_eq!(pattern.iter().map(|block| block.len()).sum::<u8>(), 14);
        assert!(pattern.len() == 5 || pattern.len() == 7 || pattern.len() == 14);
        Self { pattern, last_draw }
    }

    pub fn yakus(&self) -> Vec<Yaku> {
        let mut ret = vec![];

        if self.is_suuankoutanki() {
            ret.push(Yaku::Suuankoutanki);
        } else if self.is_suuankou() {
            ret.push(Yaku::Suuankou);
        }

        if self.is_kokushimusou13() {
            ret.push(Yaku::Kokushimusou13);
        } else if self.is_kokushimusou() {
            ret.push(Yaku::Kokushimusou);
        }

        if self.is_junseichuurenpoutou() {
            ret.push(Yaku::Junseichuurenpoutou);
        } else if self.is_chuurenpoutou() {
            ret.push(Yaku::Chuurenpoutou);
        }

        if self.is_daisuushii() {
            ret.push(Yaku::Daisuushii);
        } else if self.is_shousuushii() {
            ret.push(Yaku::Shousuushii);
        }

        if self.is_daisangen() {
            ret.push(Yaku::Daisangen);
        }

        if self.is_tsuuiisou() {
            ret.push(Yaku::Tsuuiisou);
        }

        if self.is_ryuuiisou() {
            ret.push(Yaku::Ryuuiisou);
        }

        if self.is_chinroutou() {
            ret.push(Yaku::Chinroutou);
        }

        if !ret.is_empty() {
            return ret;
        }

        if self.is_chiniisou() {
            ret.push(Yaku::Chiniisou);
        } else if self.is_honiisou() {
            ret.push(Yaku::Honiisou);
        }

        if self.is_ryanpeikou() {
            ret.push(Yaku::Ryanpeikou);
        } else if self.is_iipeikou() {
            ret.push(Yaku::Iipeikou);
        }

        if self.is_junchantaiyaochuu() {
            ret.push(Yaku::Junchantaiyaochuu);
        } else if self.is_honroutou() {
            ret.push(Yaku::Honroutou)
        } else if self.is_honchantaiyaochuu() {
            ret.push(Yaku::Honchantaiyaochuu);
        }

        if self.is_tanyao() {
            ret.push(Yaku::Tanyao)
        }

        ret.append(
            &mut self
                .have_yakuhai_sangenpai()
                .into_iter()
                .map(Yaku::YakuhaiSangenpai)
                .collect(),
        );

        if self.is_pinfu() {
            ret.push(Yaku::Pinfu);
        }

        if self.is_sanshokudoukou() {
            ret.push(Yaku::Sanshokudoukou);
        }

        if self.is_toitoihou() {
            ret.push(Yaku::Toitoihou);
        }

        if self.is_sanankou() {
            ret.push(Yaku::Sanankou);
        }

        if self.is_shousangen() {
            ret.push(Yaku::Shousangen);
        }

        if self.is_chiitoitsu() {
            ret.push(Yaku::Chiitoitsu);
        }

        if self.is_ikkitsuukan() {
            ret.push(Yaku::Ikkitsuukan);
        }

        if self.is_sanshokudoujun() {
            ret.push(Yaku::Sanshokudoujun);
        }

        ret
    }

    fn is_tanyao(&self) -> bool {
        self.pattern.iter().all(|block| {
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
                    seq_starts.push(tile);
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
            let triplets = self
                .pattern
                .iter()
                .filter_map(|block| block.triplet())
                .collect::<Vec<_>>();

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
        self.pattern.len() == 5
            && self
                .pattern
                .iter()
                .filter_map(|block| match block.triplet().or_else(|| block.pair()) {
                    Some(tile) if tile.is_dragon() => Some(tile),
                    _ => None,
                })
                .count()
                == 3
    }

    fn is_honroutou(&self) -> bool {
        self.pattern
            .iter()
            .flat_map(|block| block.tiles())
            .all(|tile| tile.is_honor() || tile.is_terminal())
    }

    fn is_chiitoitsu(&self) -> bool {
        self.pattern.len() == 7
    }

    fn is_honchantaiyaochuu(&self) -> bool {
        self.pattern.len() == 5
            && self.pattern.iter().all(|block| {
                if let Some(tile) = block.triplet().or_else(|| block.pair()) {
                    tile.is_honor() || tile.is_terminal()
                } else if let Some(tile) = block.sequence() {
                    tile.number() == 1 || tile.number() == 7
                } else {
                    unreachable!()
                }
            })
    }

    fn is_ikkitsuukan(&self) -> bool {
        if self.pattern.len() == 5 {
            let seq_starts = self
                .pattern
                .iter()
                .filter_map(|block| block.sequence())
                .collect::<Vec<_>>();
            let check_three_sequence = |target: &[Tile]| -> bool {
                target.iter().all(|tile| tile.is_numbered())
                    && target[0].number() + 3 == target[1].number()
                    && target[0].number() + 6 == target[2].number()
                    && target[0].tile_type() == target[1].tile_type()
                    && target[0].tile_type() == target[2].tile_type()
            };
            match seq_starts.len() {
                0 | 1 | 2 => false,
                3 => check_three_sequence(&seq_starts),
                4 => (0..4).any(|index| {
                    let mut three_seq_starts = seq_starts.clone();
                    three_seq_starts.remove(index);
                    check_three_sequence(&three_seq_starts)
                }),
                _ => {
                    unreachable!()
                }
            }
        } else {
            false
        }
    }

    fn is_sanshokudoujun(&self) -> bool {
        if self.pattern.len() == 5 {
            let seq_starts = self
                .pattern
                .iter()
                .filter_map(|block| block.sequence())
                .collect::<Vec<_>>();
            let check_three_sequence = |target: &[Tile]| -> bool {
                target.iter().all(|tile| tile.is_numbered())
                    && target[0].number() == target[1].number()
                    && target[0].number() == target[2].number()
                    && target[0].tile_type() != target[1].tile_type()
                    && target[0].tile_type() != target[2].tile_type()
                    && target[1].tile_type() != target[2].tile_type()
            };
            match seq_starts.len() {
                0 | 1 | 2 => false,
                3 => check_three_sequence(&seq_starts),
                4 => (0..4).any(|index| {
                    let mut three_seq_starts = seq_starts.clone();
                    three_seq_starts.remove(index);
                    check_three_sequence(&three_seq_starts)
                }),
                _ => {
                    unreachable!()
                }
            }
        } else {
            false
        }
    }

    fn is_ryanpeikou(&self) -> bool {
        if self.pattern.len() == 5 {
            let mut seq_starts = vec![];
            for block in &self.pattern {
                if let Some(tile) = block.sequence() {
                    seq_starts.push(tile);
                }
            }
            seq_starts.sort();
            seq_starts.len() == 4
                && seq_starts[0] == seq_starts[1]
                && seq_starts[2] == seq_starts[3]
        } else {
            false
        }
    }

    fn is_junchantaiyaochuu(&self) -> bool {
        self.pattern.len() == 5
            && self.pattern.iter().all(|block| {
                if let Some(tile) = block.triplet().or_else(|| block.pair()) {
                    tile.is_terminal()
                } else if let Some(tile) = block.sequence() {
                    tile.number() == 1 || tile.number() == 7
                } else {
                    unreachable!()
                }
            })
    }

    fn is_honiisou(&self) -> bool {
        let numbered_tiles = self
            .pattern
            .iter()
            .flat_map(|block| block.tiles())
            .filter(|tile| tile.is_numbered())
            .collect::<Vec<_>>();
        numbered_tiles
            .iter()
            .all(|tile| tile.tile_type() == numbered_tiles[0].tile_type())
    }

    fn is_chiniisou(&self) -> bool {
        let all_tiles = self
            .pattern
            .iter()
            .flat_map(|block| block.tiles())
            .collect::<Vec<_>>();
        all_tiles
            .iter()
            .all(|tile| tile.tile_type() == all_tiles[0].tile_type())
    }

    fn is_daisangen(&self) -> bool {
        self.pattern.len() == 5
            && self
                .pattern
                .iter()
                .filter_map(|block| match block.triplet() {
                    Some(tile) if tile.is_dragon() => Some(tile),
                    _ => None,
                })
                .count()
                == 3
    }

    fn is_suuankou(&self) -> bool {
        self.pattern.len() == 5
            && self
                .pattern
                .iter()
                .all(|block| block.triplet().is_some() || block.pair().is_some())
    }

    fn is_tsuuiisou(&self) -> bool {
        self.pattern
            .iter()
            .flat_map(|block| block.tiles())
            .all(|tile| tile.is_honor())
    }

    fn is_ryuuiisou(&self) -> bool {
        self.pattern
            .iter()
            .flat_map(|block| block.tiles())
            .all(|tile| {
                [T_2S, T_3S, T_4S, T_6S, T_8S, T_HATSU]
                    .into_iter()
                    .any(|t| t == *tile)
            })
    }

    fn is_chinroutou(&self) -> bool {
        self.pattern.len() == 5
            && self.pattern.iter().all(|block| {
                if let Some(tile) = block.triplet().or_else(|| block.pair()) {
                    tile.is_terminal()
                } else {
                    false
                }
            })
    }

    fn is_kokushimusou(&self) -> bool {
        self.pattern.len() == 14
    }

    fn is_shousuushii(&self) -> bool {
        self.pattern.len() == 5
            && self
                .pattern
                .iter()
                .filter_map(|block| match block.triplet().or_else(|| block.pair()) {
                    Some(tile) if tile.is_wind() => Some(tile),
                    _ => None,
                })
                .count()
                >= 4
    }

    fn is_chuurenpoutou(&self) -> bool {
        if self.pattern.len() == 5 {
            let all_tiles = self
                .pattern
                .iter()
                .flat_map(|block| block.tiles())
                .collect::<Vec<_>>();
            if all_tiles
                .iter()
                .any(|tile| tile.tile_type() != all_tiles[0].tile_type())
            {
                return false;
            }
            let mut map: HashMap<u8, u8> = HashMap::new();
            all_tiles
                .iter()
                .for_each(|tile| *map.entry(tile.number()).or_default() += 1);
            (1..=9).all(|num| {
                map.get(&num).is_some()
                    && if num == 1 || num == 9 {
                        *map.get(&num).unwrap() >= 3
                    } else {
                        true
                    }
            })
        } else {
            false
        }
    }

    fn is_suuankoutanki(&self) -> bool {
        self.is_suuankou()
            && self.pattern.iter().all(|block| {
                if let Some(tile) = block.pair() {
                    tile == self.last_draw
                } else {
                    true
                }
            })
    }

    fn is_kokushimusou13(&self) -> bool {
        self.is_kokushimusou()
            && self.pattern.windows(2).any(|pair| {
                pair[0].tiles()[0] == pair[1].tiles()[0] && pair[0].tiles()[0] == self.last_draw
            })
    }

    fn is_junseichuurenpoutou(&self) -> bool {
        if self.is_chuurenpoutou() {
            let mut map: HashMap<u8, u8> = HashMap::new();
            self.pattern
                .iter()
                .flat_map(|block| block.tiles())
                .for_each(|tile| *map.entry(tile.number()).or_default() += 1);
            if let Some(&num) = map.get(&self.last_draw.number()) {
                return if self.last_draw.number() == 1 || self.last_draw.number() == 9 {
                    num == 4
                } else {
                    num == 2
                };
            }
        }
        false
    }

    fn is_daisuushii(&self) -> bool {
        self.is_shousuushii()
            && self.pattern.iter().all(|block| {
                if let Some(tile) = block.triplet() {
                    tile.is_wind()
                } else {
                    block.pair().is_some()
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::tile_block::TileBlock;

    use super::*;

    fn build_pattern(pattern: Vec<Vec<&str>>, last_draw: &str) -> TilePattern {
        let pattern = pattern
            .iter()
            .map(|block| {
                TileBlock::new(&block.iter().map(|s| s.parse().unwrap()).collect::<Vec<_>>())
            })
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
            vec!["3p", "3p", "3p"],
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["6s", "6s", "6s"],
            vec!["chun", "chun"],
        ];
        let pattern = build_pattern(tileset, "3p");
        assert_eq!(pattern.have_yakuhai_sangenpai(), vec![T_HATSU]);

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
    fn shousangen() {
        let tileset = vec![
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["4p", "5p", "6p"],
            vec!["chun", "chun", "chun"],
            vec!["1p", "1p", "1p"],
            vec!["haku", "haku"],
        ];
        let pattern = build_pattern(tileset, "chun");
        assert!(pattern.is_shousangen());

        let tileset = vec![
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["4p", "5p", "6p"],
            vec!["chun", "chun", "chun"],
            vec!["haku", "haku", "haku"],
            vec!["1p", "1p"],
        ];
        let pattern = build_pattern(tileset, "chun");
        assert!(pattern.is_shousangen());

        let tileset = vec![
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["4p", "5p", "6p"],
            vec!["chun", "chun", "chun"],
            vec!["3p", "3p", "3p"],
            vec!["1p", "1p"],
        ];
        let pattern = build_pattern(tileset, "chun");
        assert!(!pattern.is_shousangen());
    }

    #[test]
    fn honroutou() {
        let tileset = vec![
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["9m", "9m", "9m"],
            vec!["chun", "chun", "chun"],
            vec!["1s", "1s", "1s"],
            vec!["1p", "1p"],
        ];
        let pattern = build_pattern(tileset, "chun");
        assert!(pattern.is_honroutou());

        let tileset = vec![
            vec!["9s", "9s", "9s"],
            vec!["9m", "9m", "9m"],
            vec!["1m", "1m", "1m"],
            vec!["1s", "1s", "1s"],
            vec!["1p", "1p"],
        ];
        let pattern = build_pattern(tileset, "1m");
        assert!(pattern.is_honroutou());

        let tileset = vec![
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["9m", "9m", "9m"],
            vec!["chun", "chun", "chun"],
            vec!["1s", "2s", "3s"],
            vec!["1p", "1p"],
        ];
        let pattern = build_pattern(tileset, "chun");
        assert!(!pattern.is_honroutou());
    }

    #[test]
    fn chiitoitsu() {
        let tileset = vec![
            vec!["hatsu", "hatsu"],
            vec!["9m", "9m"],
            vec!["chun", "chun"],
            vec!["1s", "1s"],
            vec!["1p", "1p"],
            vec!["5m", "5m"],
            vec!["ton", "ton"],
        ];
        let pattern = build_pattern(tileset, "chun");
        assert!(pattern.is_chiitoitsu());
    }

    #[test]
    fn honchantaiyaochuu() {
        let tileset = vec![
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["7m", "8m", "9m"],
            vec!["chun", "chun", "chun"],
            vec!["1s", "2s", "3s"],
            vec!["1p", "1p"],
        ];
        let pattern = build_pattern(tileset, "chun");
        assert!(pattern.is_honchantaiyaochuu());

        let tileset = vec![
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["9m", "9m", "9m"],
            vec!["chun", "chun", "chun"],
            vec!["1s", "1s", "1s"],
            vec!["1p", "1p"],
        ];
        let pattern = build_pattern(tileset, "chun");
        assert!(pattern.is_honchantaiyaochuu());

        let tileset = vec![
            vec!["1m", "2m", "3m"],
            vec!["9m", "9m", "9m"],
            vec!["9s", "9s", "9s"],
            vec!["1s", "1s", "1s"],
            vec!["1p", "1p"],
        ];
        let pattern = build_pattern(tileset, "9s");
        assert!(pattern.is_honchantaiyaochuu());

        let tileset = vec![
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["6m", "7m", "8m"],
            vec!["chun", "chun", "chun"],
            vec!["1s", "1s", "1s"],
            vec!["1p", "1p"],
        ];
        let pattern = build_pattern(tileset, "chun");
        assert!(!pattern.is_honchantaiyaochuu());
    }

    #[test]
    fn ikkitsuukan() {
        let tileset = vec![
            vec!["1s", "2s", "3s"],
            vec!["4s", "5s", "6s"],
            vec!["7s", "8s", "9s"],
            vec!["chun", "chun", "chun"],
            vec!["1p", "1p"],
        ];
        let pattern = build_pattern(tileset, "chun");
        assert!(pattern.is_ikkitsuukan());

        let tileset = vec![
            vec!["2s", "3s", "4s"],
            vec!["4s", "5s", "6s"],
            vec!["7s", "8s", "9s"],
            vec!["chun", "chun", "chun"],
            vec!["1s", "1s"],
        ];
        let pattern = build_pattern(tileset, "chun");
        assert!(!pattern.is_ikkitsuukan());
    }

    #[test]
    fn sanshokudoujun() {
        let tileset = vec![
            vec!["1s", "2s", "3s"],
            vec!["1m", "2m", "3m"],
            vec!["1p", "2p", "3p"],
            vec!["chun", "chun", "chun"],
            vec!["1p", "1p"],
        ];
        let pattern = build_pattern(tileset, "chun");
        assert!(pattern.is_sanshokudoujun());

        let tileset = vec![
            vec!["1s", "2s", "3s"],
            vec!["1m", "2m", "3m"],
            vec!["1p", "1p", "1p"],
            vec!["2p", "2p", "2p"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset, "1p");
        assert!(!pattern.is_sanshokudoujun());
    }

    #[test]
    fn ryanpeikou() {
        let tileset = vec![
            vec!["1s", "2s", "3s"],
            vec!["1s", "2s", "3s"],
            vec!["1p", "2p", "3p"],
            vec!["1p", "2p", "3p"],
            vec!["1p", "1p"],
        ];
        let pattern = build_pattern(tileset, "1p");
        assert!(pattern.is_ryanpeikou());

        let tileset = vec![
            vec!["1s", "2s", "3s"],
            vec!["1s", "2s", "3s"],
            vec!["1p", "2p", "3p"],
            vec!["2p", "3p", "4p"],
            vec!["1p", "1p"],
        ];
        let pattern = build_pattern(tileset, "1p");
        assert!(!pattern.is_ryanpeikou());
    }

    #[test]
    fn junchantaiyaochuu() {
        let tileset = vec![
            vec!["1m", "2m", "3m"],
            vec!["9m", "9m", "9m"],
            vec!["9s", "9s", "9s"],
            vec!["1s", "1s", "1s"],
            vec!["1p", "1p"],
        ];
        let pattern = build_pattern(tileset, "9s");
        assert!(pattern.is_junchantaiyaochuu());

        let tileset = vec![
            vec!["2m", "3m", "4m"],
            vec!["9m", "9m", "9m"],
            vec!["9s", "9s", "9s"],
            vec!["1s", "1s", "1s"],
            vec!["1p", "1p"],
        ];
        let pattern = build_pattern(tileset, "9s");
        assert!(!pattern.is_junchantaiyaochuu());

        let tileset = vec![
            vec!["1m", "2m", "3m"],
            vec!["9m", "9m", "9m"],
            vec!["9s", "9s", "9s"],
            vec!["haku", "haku", "haku"],
            vec!["1p", "1p"],
        ];
        let pattern = build_pattern(tileset, "9s");
        assert!(!pattern.is_junchantaiyaochuu());
    }

    #[test]
    fn honiisou() {
        let tileset = vec![
            vec!["3p", "3p", "3p"],
            vec!["4p", "5p", "6p"],
            vec!["chun", "chun", "chun"],
            vec!["1p", "1p", "1p"],
            vec!["haku", "haku"],
        ];
        let pattern = build_pattern(tileset, "chun");
        assert!(pattern.is_honiisou());

        let tileset = vec![
            vec!["3p", "3p", "3p"],
            vec!["4p", "5p", "6p"],
            vec!["9p", "9p", "9p"],
            vec!["1p", "1p", "1p"],
            vec!["5p", "5p"],
        ];
        let pattern = build_pattern(tileset, "chun");
        assert!(pattern.is_honiisou());

        let tileset = vec![
            vec!["3p", "3p", "3p"],
            vec!["4p", "5p", "6p"],
            vec!["chun", "chun", "chun"],
            vec!["1p", "1p", "1p"],
            vec!["2m", "2m"],
        ];
        let pattern = build_pattern(tileset, "chun");
        assert!(!pattern.is_honiisou());
    }

    #[test]
    fn chiniisou() {
        let tileset = vec![
            vec!["3p", "3p", "3p"],
            vec!["4p", "5p", "6p"],
            vec!["9p", "9p", "9p"],
            vec!["1p", "1p", "1p"],
            vec!["2p", "2p"],
        ];
        let pattern = build_pattern(tileset, "2p");
        assert!(pattern.is_chiniisou());

        let tileset = vec![
            vec!["3p", "3p", "3p"],
            vec!["4p", "5p", "6p"],
            vec!["9p", "9p", "9p"],
            vec!["1p", "1p", "1p"],
            vec!["2m", "2m"],
        ];
        let pattern = build_pattern(tileset, "2p");
        assert!(!pattern.is_chiniisou());
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

        let tileset = vec![
            vec!["haku", "haku", "haku"],
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["2p", "2p", "2p"],
            vec!["1p", "1p", "1p"],
            vec!["chun", "chun"],
        ];
        let pattern = build_pattern(tileset, "2p");
        assert!(!pattern.is_daisangen());
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

        let tileset = vec![
            vec!["haku", "haku", "haku"],
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["chun", "chun", "chun"],
            vec!["1p", "2p", "3p"],
            vec!["2p", "2p"],
        ];
        let pattern = build_pattern(tileset, "haku");
        assert!(!pattern.is_suuankou());
    }

    #[test]
    fn tsuuiisou() {
        let tileset = vec![
            vec!["haku", "haku", "haku"],
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["chun", "chun", "chun"],
            vec!["ton", "ton", "ton"],
            vec!["shaa", "shaa"],
        ];
        let pattern = build_pattern(tileset, "haku");
        assert!(pattern.is_tsuuiisou());

        let tileset = vec![
            vec!["haku", "haku", "haku"],
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["chun", "chun", "chun"],
            vec!["ton", "ton", "ton"],
            vec!["8p", "8p"],
        ];
        let pattern = build_pattern(tileset, "haku");
        assert!(!pattern.is_tsuuiisou());
    }

    #[test]
    fn ryuuiisou() {
        let tileset = vec![
            vec!["2s", "3s", "4s"],
            vec!["2s", "3s", "4s"],
            vec!["2s", "3s", "4s"],
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["8s", "8s"],
        ];
        let pattern = build_pattern(tileset, "8s");
        assert!(pattern.is_ryuuiisou());

        let tileset = vec![
            vec!["2s", "3s", "4s"],
            vec!["2s", "3s", "4s"],
            vec!["2s", "3s", "4s"],
            vec!["haku", "haku", "haku"],
            vec!["8s", "8s"],
        ];
        let pattern = build_pattern(tileset, "8s");
        assert!(!pattern.is_ryuuiisou());

        let tileset = vec![
            vec!["2s", "3s", "4s"],
            vec!["2s", "3s", "4s"],
            vec!["2s", "3s", "4s"],
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["8m", "8m"],
        ];
        let pattern = build_pattern(tileset, "8m");
        assert!(!pattern.is_ryuuiisou());
    }

    #[test]
    fn chinroutou() {
        let tileset = vec![
            vec!["1p", "1p", "1p"],
            vec!["9p", "9p", "9p"],
            vec!["1s", "1s", "1s"],
            vec!["9s", "9s", "9s"],
            vec!["1m", "1m"],
        ];
        let pattern = build_pattern(tileset, "1m");
        assert!(pattern.is_chinroutou());

        let tileset = vec![
            vec!["1p", "2p", "3p"],
            vec!["9p", "9p", "9p"],
            vec!["1s", "1s", "1s"],
            vec!["9s", "9s", "9s"],
            vec!["1m", "1m"],
        ];
        let pattern = build_pattern(tileset, "1m");
        assert!(!pattern.is_chinroutou());

        let tileset = vec![
            vec!["haku", "haku", "haku"],
            vec!["9p", "9p", "9p"],
            vec!["1s", "1s", "1s"],
            vec!["9s", "9s", "9s"],
            vec!["1m", "1m"],
        ];
        let pattern = build_pattern(tileset, "1m");
        assert!(!pattern.is_chinroutou());
    }

    #[test]
    fn kokushimusou() {
        let tileset = [
            "1p", "9p", "1s", "9s", "1m", "9m", "ton", "nan", "shaa", "pei", "haku", "hatsu",
            "hatsu", "chun",
        ];
        let pattern = build_pattern(tileset.iter().map(|&s| vec![s]).collect(), "shaa");
        assert!(pattern.is_kokushimusou());
    }

    #[test]
    fn shousuushii() {
        let tileset = vec![
            vec!["ton", "ton", "ton"],
            vec!["nan", "nan", "nan"],
            vec!["shaa", "shaa", "shaa"],
            vec!["3p", "3p", "3p"],
            vec!["pei", "pei"],
        ];
        let pattern = build_pattern(tileset.clone(), "pei");
        assert!(pattern.is_shousuushii());

        let tileset = vec![
            vec!["ton", "ton", "ton"],
            vec!["nan", "nan", "nan"],
            vec!["shaa", "shaa", "shaa"],
            vec!["pei", "pei", "pei"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset.clone(), "pei");
        assert!(pattern.is_shousuushii());

        let tileset = vec![
            vec!["ton", "ton", "ton"],
            vec!["nan", "nan", "nan"],
            vec!["shaa", "shaa", "shaa"],
            vec!["2p", "2p", "2p"],
            vec!["3p", "3p"],
        ];
        let pattern = build_pattern(tileset.clone(), "3p");
        assert!(!pattern.is_shousuushii());
    }

    #[test]
    fn chuurenpoutou() {
        let tileset = vec![
            vec!["1p", "1p", "1p"],
            vec!["2p", "3p", "4p"],
            vec!["5p", "6p", "7p"],
            vec!["9p", "9p", "9p"],
            vec!["8p", "8p"],
        ];
        let pattern = build_pattern(tileset.clone(), "9p");
        assert!(pattern.is_chuurenpoutou());

        let tileset = vec![
            vec!["1p", "1p", "1p"],
            vec!["1p", "2p", "3p"],
            vec!["5p", "6p", "7p"],
            vec!["7p", "8p", "9p"],
            vec!["9p", "9p"],
        ];
        let pattern = build_pattern(tileset.clone(), "9p");
        assert!(!pattern.is_chuurenpoutou());
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

        let tileset = vec![
            vec!["haku", "haku", "haku"],
            vec!["hatsu", "hatsu", "hatsu"],
            vec!["chun", "chun", "chun"],
            vec!["1p", "1p", "1p"],
            vec!["2p", "2p"],
        ];
        let pattern = build_pattern(tileset, "1p");
        assert!(!pattern.is_suuankoutanki());
    }

    #[test]
    fn kokushimusou13() {
        let tileset = [
            "1p", "9p", "1s", "9s", "1m", "9m", "ton", "nan", "shaa", "pei", "haku", "hatsu",
            "hatsu", "chun",
        ];
        let pattern = build_pattern(tileset.iter().map(|&s| vec![s]).collect(), "hatsu");
        assert!(pattern.is_kokushimusou13());

        let tileset = [
            "1p", "9p", "1s", "9s", "1m", "9m", "ton", "nan", "shaa", "pei", "haku", "hatsu",
            "hatsu", "chun",
        ];
        let pattern = build_pattern(tileset.iter().map(|&s| vec![s]).collect(), "haku");
        assert!(!pattern.is_kokushimusou13());
    }

    #[test]
    fn junseichuurenpoutou() {
        let tileset = vec![
            vec!["1p", "1p", "1p"],
            vec!["2p", "3p", "4p"],
            vec!["5p", "6p", "7p"],
            vec!["9p", "9p", "9p"],
            vec!["8p", "8p"],
        ];
        let pattern = build_pattern(tileset.clone(), "8p");
        assert!(pattern.is_junseichuurenpoutou());

        let tileset = vec![
            vec!["1p", "1p", "1p"],
            vec!["1p", "2p", "3p"],
            vec!["4p", "5p", "6p"],
            vec!["7p", "8p", "9p"],
            vec!["9p", "9p"],
        ];
        let pattern = build_pattern(tileset.clone(), "1p");
        assert!(pattern.is_junseichuurenpoutou());

        let tileset = vec![
            vec!["1p", "1p", "1p"],
            vec!["2p", "3p", "4p"],
            vec!["5p", "6p", "7p"],
            vec!["9p", "9p", "9p"],
            vec!["8p", "8p"],
        ];
        let pattern = build_pattern(tileset.clone(), "9p");
        assert!(!pattern.is_junseichuurenpoutou());

        let tileset = vec![
            vec!["1p", "1p", "1p"],
            vec!["1p", "2p", "3p"],
            vec!["4p", "5p", "6p"],
            vec!["7p", "8p", "9p"],
            vec!["9p", "9p"],
        ];
        let pattern = build_pattern(tileset.clone(), "9p");
        assert!(!pattern.is_junseichuurenpoutou());
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
