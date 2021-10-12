use std::ops::Add;

use bomber_lib::world::{Direction, Tile};

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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Location(pub usize, pub usize);

impl Add<Direction> for Location {
    type Output = Option<Location>;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::West | Direction::NorthWest | Direction::SouthWest if self.0 == 0 => None,
            Direction::South | Direction::SouthWest | Direction::SouthEast if self.1 == 0 => None,
            Direction::West => Some(Location(self.0 - 1, self.1)),
            Direction::NorthWest => Some(Location(self.0 - 1, self.1 + 1)),
            Direction::North => Some(Location(self.0, self.1 + 1)),
            Direction::NorthEast => Some(Location(self.0 + 1, self.1 + 1)),
            Direction::East => Some(Location(self.0 + 1, self.1)),
            Direction::SouthEast => Some(Location(self.0 + 1, self.1 - 1)),
            Direction::South => Some(Location(self.0, self.1 - 1)),
            Direction::SouthWest => Some(Location(self.0 - 1, self.1 - 1)),
        }
    }
}

impl GameMap {
    pub fn size(&self) -> (usize, usize) {
        (self.tiles[0].len(), self.tiles.len())
    }

    pub fn tile(&self, location: Location) -> Option<Tile> {
        self.tiles
            .get(location.1)
            .and_then(|v| v.get(location.0))
            .cloned()
    }

    pub fn inspect_from(&self, location: Location, direction: Direction) -> Tile {
        (location + direction)
            .and_then(|p| self.tile(p))
            .unwrap_or(Tile::Wall)
    }
}

impl From<char> for Wrapper<Tile> {
    fn from(character: char) -> Self {
        Wrapper(match character {
            '.' => Tile::EmptyFloor,
            '#' => Tile::Wall,
            'X' => Tile::Lava,
            's' => Tile::Switch,
            _ => panic!("Character has no associated tile"),
        })
    }
}

impl<T: AsRef<str>> From<T> for GameMap {
    fn from(text: T) -> Self {
        let lines: Vec<&str> = text.as_ref().lines().rev().collect();
        // Very panicky (this should be a TryFrom) but good for a quick test
        assert!(lines.windows(2).all(|w| w[0].len() == w[1].len()));
        assert!(lines.len() > 0 && lines[0].len() > 0);
        let convert_line = |l: &str| -> Vec<Tile> { l.chars().map(|c| Wrapper::<Tile>::from(c).0).collect() };

        Self {
            tiles: lines.into_iter().map(convert_line).collect(),
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
        let game_map = GameMap::from(game_map_text);
        assert_eq!(game_map.size(), (8, 7));
        assert_eq!(game_map.tile(Location(0, 0)).unwrap(), Tile::Wall);
        assert_eq!(game_map.tile(Location(4, 0)).unwrap(), Tile::EmptyFloor);
        assert_eq!(game_map.tile(Location(1, 1)).unwrap(), Tile::EmptyFloor);
        assert_eq!(game_map.tile(Location(1, 2)).unwrap(), Tile::Lava);
        assert_eq!(game_map.tile(Location(8, 8)), None);
    }
}
