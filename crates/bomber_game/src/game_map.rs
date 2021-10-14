use std::{array::IntoIter, convert::TryFrom, ops::{Add, Sub}, str::FromStr};

use anyhow::{anyhow, Result};
use bomber_lib::world::{Direction, Distance, Tile};

use crate::Wrapper;

pub const INITIAL_LOCATION: Location = Location(4, 0);

#[allow(unused)]
#[rustfmt::skip]
pub const EASY: &str =
    "###...###\n\
     ##.....##\n\
     #..###..#\n\
     #..###..#\n\
     #...#...#\n\
     #.......#\n\
     ####.####";

#[allow(unused)]
#[rustfmt::skip]
pub const DANGEROUS: &str =
    "####.####\n\
     #.......#\n\
     #.#####.#\n\
     #.XXXXX.#\n\
     #.......#\n\
     #.......#\n\
     ####.####";

pub struct GameMap {
    tiles: Vec<Vec<Tile>>,
}

impl GameMap {
    pub fn tiles_surrounding_location(&self, location: Location) -> Vec<(Tile, Distance)> {
        // TODO do more than the adjacent orthogonals, and do it programatically
        IntoIter::new([(-1i32, 0i32), (1, 0), (0, 1), (0, -1)])
            .filter_map(|(x, y)| {
                self.tile(location + (x, y)).and_then(|t| Some((t, Distance(x, y))))
            })
            .collect()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Location(pub usize, pub usize);

impl Add<Direction> for Location {
    type Output = Option<Location>;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::West if self.0 == 0 => None,
            Direction::South if self.1 == 0 => None,
            Direction::West => Some(Location(self.0 - 1, self.1)),
            Direction::North => Some(Location(self.0, self.1 + 1)),
            Direction::East => Some(Location(self.0 + 1, self.1)),
            Direction::South => Some(Location(self.0, self.1 - 1)),
        }
    }
}

impl Add<(i32, i32)> for Location {
    type Output = Location;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self((self.0 as i32 + rhs.0).max(0) as usize, (self.1 as i32 + rhs.1).max(0) as usize)
    }
}

impl Sub<Location> for Location {
    type Output = Distance;

    fn sub(self, rhs: Location) -> Self::Output {
        Distance(self.0 as i32 - rhs.0 as i32, self.1 as i32 - rhs.1 as i32)
    }
}

impl GameMap {
    pub fn size(&self) -> (usize, usize) {
        (self.tiles[0].len(), self.tiles.len())
    }

    pub fn tile(&self, location: Location) -> Option<Tile> {
        self.tiles.get(location.1).and_then(|v| v.get(location.0)).cloned()
    }
}

impl TryFrom<char> for Wrapper<Tile> {
    type Error = anyhow::Error;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        match character {
            '.' => Ok(Wrapper(Tile::EmptyFloor)),
            '#' => Ok(Wrapper(Tile::Wall)),
            'X' => Ok(Wrapper(Tile::Lava)),
            's' => Ok(Wrapper(Tile::Switch)),
            _ => Err(anyhow!("Invalid character for tile: {}", character)),
        }
    }
}

impl FromStr for GameMap {
    type Err = anyhow::Error;

    fn from_str(text: &str) -> Result<Self> {
        let lines: Vec<&str> = text.lines().rev().collect();
        if lines.windows(2).any(|w| w[0].len() != w[1].len()) {
            Err(anyhow!("Mismatched row sizes in the game map"))
        } else if lines.is_empty() || lines[0].is_empty() {
            Err(anyhow!("Game map must have at least a row and a column"))
        } else {
            let convert_line = |l: &str| -> Result<Vec<Tile>> {
                l.chars().map(|c| Wrapper::<Tile>::try_from(c).map(|w| w.0)).collect()
            };
            let tiles: Result<Vec<Vec<Tile>>> = lines.into_iter().map(convert_line).collect();
            Ok(Self { tiles: tiles? })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_game_maps() {
        #[rustfmt::skip]
        let game_map_text =
            "####.###\n\
             #......#\n\
             #.####.#\n\
             #..##..#\n\
             #X.##..#\n\
             #......#\n\
             ####.###";
        let game_map = GameMap::from_str(game_map_text).unwrap();
        assert_eq!(game_map.size(), (8, 7));
        assert_eq!(game_map.tile(Location(0, 0)).unwrap(), Tile::Wall);
        assert_eq!(game_map.tile(Location(4, 0)).unwrap(), Tile::EmptyFloor);
        assert_eq!(game_map.tile(Location(1, 1)).unwrap(), Tile::EmptyFloor);
        assert_eq!(game_map.tile(Location(1, 2)).unwrap(), Tile::Lava);
        assert_eq!(game_map.tile(Location(8, 8)), None);
    }
}
