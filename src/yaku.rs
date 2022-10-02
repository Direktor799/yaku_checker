use crate::tile::Tile;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    /// 平和（门前清限定）
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
