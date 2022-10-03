use anyhow::{anyhow, Error, Result};
use once_cell::sync::Lazy;
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

const ALL_TILES: [&str; 34] = [
    "1m", "2m", "3m", "4m", "5m", "6m", "7m", "8m", "9m", "1s", "2s", "3s", "4s", "5s", "6s", "7s",
    "8s", "9s", "1p", "2p", "3p", "4p", "5p", "6p", "7p", "8p", "9p", "ton", "nan", "shaa", "pei",
    "haku", "hatsu", "chun",
];

pub static T_1M: Lazy<Tile> = Lazy::new(|| "1m".parse().unwrap());
pub static T_2M: Lazy<Tile> = Lazy::new(|| "2m".parse().unwrap());
pub static T_3M: Lazy<Tile> = Lazy::new(|| "3m".parse().unwrap());
pub static T_4M: Lazy<Tile> = Lazy::new(|| "4m".parse().unwrap());
pub static T_5M: Lazy<Tile> = Lazy::new(|| "5m".parse().unwrap());
pub static T_6M: Lazy<Tile> = Lazy::new(|| "6m".parse().unwrap());
pub static T_7M: Lazy<Tile> = Lazy::new(|| "7m".parse().unwrap());
pub static T_8M: Lazy<Tile> = Lazy::new(|| "8m".parse().unwrap());
pub static T_9M: Lazy<Tile> = Lazy::new(|| "9m".parse().unwrap());
pub static T_1S: Lazy<Tile> = Lazy::new(|| "1s".parse().unwrap());
pub static T_2S: Lazy<Tile> = Lazy::new(|| "2s".parse().unwrap());
pub static T_3S: Lazy<Tile> = Lazy::new(|| "3s".parse().unwrap());
pub static T_4S: Lazy<Tile> = Lazy::new(|| "4s".parse().unwrap());
pub static T_5S: Lazy<Tile> = Lazy::new(|| "5s".parse().unwrap());
pub static T_6S: Lazy<Tile> = Lazy::new(|| "6s".parse().unwrap());
pub static T_7S: Lazy<Tile> = Lazy::new(|| "7s".parse().unwrap());
pub static T_8S: Lazy<Tile> = Lazy::new(|| "8s".parse().unwrap());
pub static T_9S: Lazy<Tile> = Lazy::new(|| "9s".parse().unwrap());
pub static T_1P: Lazy<Tile> = Lazy::new(|| "1p".parse().unwrap());
pub static T_2P: Lazy<Tile> = Lazy::new(|| "2p".parse().unwrap());
pub static T_3P: Lazy<Tile> = Lazy::new(|| "3p".parse().unwrap());
pub static T_4P: Lazy<Tile> = Lazy::new(|| "4p".parse().unwrap());
pub static T_5P: Lazy<Tile> = Lazy::new(|| "5p".parse().unwrap());
pub static T_6P: Lazy<Tile> = Lazy::new(|| "6p".parse().unwrap());
pub static T_7P: Lazy<Tile> = Lazy::new(|| "7p".parse().unwrap());
pub static T_8P: Lazy<Tile> = Lazy::new(|| "8p".parse().unwrap());
pub static T_9P: Lazy<Tile> = Lazy::new(|| "9p".parse().unwrap());
pub static T_TON: Lazy<Tile> = Lazy::new(|| "ton".parse().unwrap());
pub static T_NAN: Lazy<Tile> = Lazy::new(|| "nan".parse().unwrap());
pub static T_SHAA: Lazy<Tile> = Lazy::new(|| "shaa".parse().unwrap());
pub static T_PEI: Lazy<Tile> = Lazy::new(|| "pei".parse().unwrap());
pub static T_HAKU: Lazy<Tile> = Lazy::new(|| "haku".parse().unwrap());
pub static T_HATSU: Lazy<Tile> = Lazy::new(|| "hatsu".parse().unwrap());
pub static T_CHUN: Lazy<Tile> = Lazy::new(|| "chun".parse().unwrap());

#[derive(Clone, PartialEq, Eq)]
pub struct Tile(String, u8);

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_index = ALL_TILES
            .iter()
            .position(|&s| s == self.to_string())
            .unwrap();
        let other_index = ALL_TILES
            .iter()
            .position(|&s| s == other.to_string())
            .unwrap();
        self_index.cmp(&other_index)
    }
}

impl FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "ton" | "nan" | "shaa" | "pei" | "haku" | "hatsu" | "chun" => {
                Ok(Tile(s.to_string(), 0))
            }
            _ => {
                let chars = s.chars().collect::<Vec<_>>();
                if chars.len() == 2 && chars[0] >= '1' && chars[0] <= '9' {
                    match chars[1] {
                        'p' | 's' | 'm' => Ok(Tile(
                            chars[1].to_string(),
                            chars[0].to_digit(10).unwrap() as u8,
                        )),
                        _ => Err(anyhow!("not in any numbered suit")),
                    }
                } else {
                    Err(anyhow!("not a tile"))
                }
            }
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.1 == 0 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}{}", self.1, self.0)
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
        self.1 == 0
    }

    pub fn is_terminal(&self) -> bool {
        self.1 == 1 || self.1 == 9
    }

    pub fn is_numbered(&self) -> bool {
        !self.is_honor()
    }

    pub fn is_dragon(&self) -> bool {
        *self == *T_HAKU || *self == *T_HATSU || *self == *T_CHUN
    }

    pub fn is_wind(&self) -> bool {
        self.is_honor() && !self.is_dragon()
    }

    pub fn tile_type(&self) -> &str {
        &self.0
    }

    pub fn number(&self) -> u8 {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() -> Result<()> {
        for tile_str in ALL_TILES {
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
        let mut v = ALL_TILES
            .iter()
            .rev()
            .map(|s| Tile::from_str(s).unwrap())
            .collect::<Vec<_>>();
        v.sort();
        assert_eq!(
            v.iter().map(|tile| tile.to_string()).collect::<Vec<_>>(),
            ALL_TILES
        );
    }
}
