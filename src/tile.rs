use anyhow::{anyhow, Error, Result};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

pub const ALL_TILES: [Tile; 35] = [
    Tile(0x01),
    Tile(0x02),
    Tile(0x03),
    Tile(0x04),
    Tile(0x05),
    Tile(0x06),
    Tile(0x07),
    Tile(0x08),
    Tile(0x09),
    Tile(0x11),
    Tile(0x12),
    Tile(0x13),
    Tile(0x14),
    Tile(0x15),
    Tile(0x16),
    Tile(0x17),
    Tile(0x18),
    Tile(0x19),
    Tile(0x21),
    Tile(0x22),
    Tile(0x23),
    Tile(0x24),
    Tile(0x25),
    Tile(0x26),
    Tile(0x27),
    Tile(0x28),
    Tile(0x29),
    Tile(0x30),
    Tile(0x40),
    Tile(0x50),
    Tile(0x60),
    Tile(0x70),
    Tile(0x80),
    Tile(0x90),
    Tile(0xff),
];

pub const T_1M: Tile = ALL_TILES[0];
pub const T_2M: Tile = ALL_TILES[1];
pub const T_3M: Tile = ALL_TILES[2];
pub const T_4M: Tile = ALL_TILES[3];
pub const T_5M: Tile = ALL_TILES[4];
pub const T_6M: Tile = ALL_TILES[5];
pub const T_7M: Tile = ALL_TILES[6];
pub const T_8M: Tile = ALL_TILES[7];
pub const T_9M: Tile = ALL_TILES[8];
pub const T_1P: Tile = ALL_TILES[9];
pub const T_2P: Tile = ALL_TILES[10];
pub const T_3P: Tile = ALL_TILES[11];
pub const T_4P: Tile = ALL_TILES[12];
pub const T_5P: Tile = ALL_TILES[13];
pub const T_6P: Tile = ALL_TILES[14];
pub const T_7P: Tile = ALL_TILES[15];
pub const T_8P: Tile = ALL_TILES[16];
pub const T_9P: Tile = ALL_TILES[17];
pub const T_1S: Tile = ALL_TILES[18];
pub const T_2S: Tile = ALL_TILES[19];
pub const T_3S: Tile = ALL_TILES[20];
pub const T_4S: Tile = ALL_TILES[21];
pub const T_5S: Tile = ALL_TILES[22];
pub const T_6S: Tile = ALL_TILES[23];
pub const T_7S: Tile = ALL_TILES[24];
pub const T_8S: Tile = ALL_TILES[25];
pub const T_9S: Tile = ALL_TILES[26];
pub const T_TON: Tile = ALL_TILES[27];
pub const T_NAN: Tile = ALL_TILES[28];
pub const T_SHAA: Tile = ALL_TILES[29];
pub const T_PEI: Tile = ALL_TILES[30];
pub const T_HAKU: Tile = ALL_TILES[31];
pub const T_HATSU: Tile = ALL_TILES[32];
pub const T_CHUN: Tile = ALL_TILES[33];
pub const T_INVALID: Tile = ALL_TILES[34];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile(u8);

impl FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() < 2 {
            return Err(anyhow!("\"{}\" is too short to be a tile", s));
        }

        let bytes = s.as_bytes();
        let res = if bytes[0] >= b'1' && bytes[0] <= b'9' {
            (match bytes[1] {
                b'm' => 0,
                b'p' => 1,
                b's' => 2,
                _ => return Err(anyhow!("\"{}\" is numbered but not m/s/p", s)),
            } << 4)
                + bytes[0] as u8
                - b'0'
        } else {
            (match s {
                "ton" => 3,
                "nan" => 4,
                "shaa" => 5,
                "pei" => 6,
                "haku" => 7,
                "hatsu" => 8,
                "chun" => 9,
                _ => return Err(anyhow!("\"{}\" is not a tile", s)),
            }) << 4
        };
        Ok(Tile(res))
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const HIGH_BITS: [&str; 10] = [
            "m", "p", "s", "ton", "nan", "shaa", "pei", "haku", "hatsu", "chun",
        ];

        if *self == T_INVALID {
            write!(f, "INVALID")
        } else if self.0 & 0xf == 0 {
            write!(f, "{}", HIGH_BITS[(self.0 >> 4) as usize])
        } else {
            write!(f, "{}{}", self.0 & 0xf, HIGH_BITS[(self.0 >> 4) as usize])
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Tile as Display>::fmt(self, f)
    }
}

impl Tile {
    pub fn is_honor(&self) -> bool {
        self.0 & 0xf == 0
    }

    pub fn is_terminal(&self) -> bool {
        self.0 & 0xf == 1 || self.0 & 0xf == 9
    }

    pub fn is_numbered(&self) -> bool {
        !self.is_honor()
    }

    pub fn is_dragon(&self) -> bool {
        *self == T_HAKU || *self == T_HATSU || *self == T_CHUN
    }

    pub fn is_wind(&self) -> bool {
        self.is_honor() && !self.is_dragon()
    }

    pub fn tile_type(&self) -> u8 {
        self.0 >> 4
    }

    pub fn number(&self) -> u8 {
        self.0 & 0xf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_TILE_STRS: [&str; 34] = [
        "1m", "2m", "3m", "4m", "5m", "6m", "7m", "8m", "9m", "1p", "2p", "3p", "4p", "5p", "6p",
        "7p", "8p", "9p", "1s", "2s", "3s", "4s", "5s", "6s", "7s", "8s", "9s", "ton", "nan",
        "shaa", "pei", "haku", "hatsu", "chun",
    ];

    #[test]
    fn from_str() -> Result<()> {
        for tile_str in ALL_TILE_STRS {
            let tile = Tile::from_str(tile_str)?;
            assert_eq!(tile.to_string(), tile_str);
        }
        let not_tiles = ["foo", "bar", "0s", "5q", "10"];
        for not_tile_str in not_tiles {
            assert!(Tile::from_str(not_tile_str).is_err());
        }
        Ok(())
    }

    #[test]
    fn tile_ord() {
        let mut v = ALL_TILE_STRS
            .iter()
            .rev()
            .map(|s| Tile::from_str(s).unwrap())
            .collect::<Vec<_>>();
        v.sort();
        assert_eq!(
            v.iter().map(|tile| tile.to_string()).collect::<Vec<_>>(),
            ALL_TILE_STRS
        );
    }
}
