use crate::tile::Tile;
use std::{
    fmt::{Debug, Display},
    iter::Sum,
    ops::Add,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Yaku {
    // /// 立直（门前清限定）
    // Riichi,
    /// 断幺九
    Tanyao,
    // /// 门前清自摸和（门前清限定）
    // MenzenchinTsumohou,
    // /// 自风牌
    // YakuhaiJikaze(Kazehai),
    // /// 场风牌
    // YakuhaiBakaze(Kazehai),
    /// 三元牌
    YakuhaiSangenpai(Tile),
    /// 平和（门前清限定）(不考虑自风场风，都算平胡)
    Pinfu,
    /// 一杯口（门前清限定）
    Iipeikou,
    // /// 抢杠
    // Chankan,
    // /// 岭上开花
    // Rinshankaihou,
    // /// 海底捞月
    // Haiteiraoyue,
    // /// 河底捞鱼
    // Houteiraoyui,
    // /// 一发
    // Ippatsu,
    // /// 宝牌（不是役）
    // Dora,
    // /// 赤宝牌（不是役）
    // Akadora,
    // /// 两立直（门前清限定）
    // DoubleRiichi,
    /// 三色同刻
    Sanshokudoukou,
    // /// 三杠子
    // Sankantsu,
    /// 对对和
    Toitoihou,
    /// 三暗刻
    Sanankou,
    /// 小三元
    Shousangen,
    /// 混老头
    Honroutou,
    /// 七对子（门前清限定）
    Chiitoitsu,
    /// 混全带幺九（副露减1番）
    Honchantaiyaochuu,
    /// 一气通贯（副露减1番）
    Ikkitsuukan,
    /// 三色同顺（副露减1番）
    Sanshokudoujun,
    /// 二杯口（门前清限定）
    Ryanpeikou,
    /// 纯全带幺九（副露减1番）
    Junchantaiyaochuu,
    /// 混一色（副露减1番）
    Honiisou,
    /// 清一色（副露减1番）
    Chiniisou,
    // /// 流局满贯
    // Nagashimangan,
    // /// 天和（庄家限定）
    // Tenhou,
    // /// 地和（子家限定）
    // Chiihou,
    /// 大三元
    Daisangen,
    /// 四暗刻（门前清限定）
    Suuankou,
    /// 字一色
    Tsuuiisou,
    /// 绿一色
    Ryuuiisou,
    /// 清老头
    Chinroutou,
    /// 国士无双（门前清限定）
    Kokushimusou,
    /// 小四喜
    Shousuushii,
    // /// 四杠子
    // Suukantsu,
    /// 九莲宝灯（门前清限定）
    Chuurenpoutou,
    /// 四暗刻单骑（门前清限定）
    Suuankoutanki,
    /// 国士无双十三面（门前清限定）
    Kokushimusou13,
    /// 纯正九莲宝灯（门前清限定）
    Junseichuurenpoutou,
    /// 大四喜
    Daisuushii,
}

impl Display for Yaku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Han {
    is_yakuman: bool,
    score: u8,
}

impl Han {
    pub fn new(score: u8) -> Self {
        Han {
            is_yakuman: false,
            score,
        }
    }

    pub fn yakuman() -> Self {
        Han {
            is_yakuman: true,
            score: 1,
        }
    }

    pub fn double_yakuman() -> Self {
        Han {
            is_yakuman: true,
            score: 2,
        }
    }
}

impl Ord for Han {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.is_yakuman.cmp(&other.is_yakuman) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Han {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Han {
    type Output = Han;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_yakuman || rhs.is_yakuman {
            let mut score = 0;
            if self.is_yakuman {
                score += self.score;
            }
            if rhs.is_yakuman {
                score += rhs.score;
            }
            Han {
                is_yakuman: true,
                score,
            }
        } else {
            Han {
                is_yakuman: false,
                score: 13.min(self.score + rhs.score),
            }
        }
    }
}

impl Sum for Han {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Han::new(0), Add::add)
    }
}

impl From<Yaku> for Han {
    fn from(yaku: Yaku) -> Self {
        match yaku {
            Yaku::Tanyao | Yaku::YakuhaiSangenpai(_) | Yaku::Pinfu | Yaku::Iipeikou => Han::new(1),
            Yaku::Sanshokudoukou
            | Yaku::Toitoihou
            | Yaku::Sanankou
            | Yaku::Shousangen
            | Yaku::Honroutou
            | Yaku::Chiitoitsu
            | Yaku::Honchantaiyaochuu
            | Yaku::Ikkitsuukan
            | Yaku::Sanshokudoujun => Han::new(2),
            Yaku::Ryanpeikou | Yaku::Junchantaiyaochuu | Yaku::Honiisou => Han::new(3),
            Yaku::Chiniisou => Han::new(6),
            Yaku::Daisangen
            | Yaku::Suuankou
            | Yaku::Tsuuiisou
            | Yaku::Ryuuiisou
            | Yaku::Chinroutou
            | Yaku::Kokushimusou
            | Yaku::Shousuushii
            | Yaku::Chuurenpoutou => Han::yakuman(),
            Yaku::Suuankoutanki
            | Yaku::Kokushimusou13
            | Yaku::Junseichuurenpoutou
            | Yaku::Daisuushii => Han::double_yakuman(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn han_ord() {
        assert!(Han::double_yakuman() > Han::yakuman());
        assert!(Han::double_yakuman() > Han::new(100));
        assert!(Han::yakuman() == Han::yakuman());
        assert!(Han::new(5) < Han::new(10));
        assert!(Han::new(100) < Han::yakuman())
    }

    #[test]
    fn han_add() {
        assert!(Han::double_yakuman() + Han::yakuman() == Han::yakuman() + Han::double_yakuman());
        assert!(Han::yakuman() + Han::yakuman() == Han::double_yakuman());
        assert!(Han::yakuman() + Han::new(13) == Han::yakuman());
        assert!(Han::new(10) + Han::new(10) == Han::new(13));
        assert!(Han::new(1) + Han::new(5) == Han::new(6));
    }
}
