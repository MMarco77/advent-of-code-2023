advent_of_code::solution!(10);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tile {
    Vertical,
    Horizontal,
    BottomLeft,
    BottomRight,
    TopRight,
    TopLeft,
    Ground,
    Start,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::BottomLeft,
            'J' => Self::BottomRight,
            '7' => Self::TopRight,
            'F' => Self::TopLeft,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

impl Tile {
    fn get_next_direction(self, direction: Direction) -> Option<Direction> {
        match (self, direction) {
            (Tile::Vertical, direction) => Some(direction),
            (Tile::Horizontal, direction) => Some(direction),
            (Tile::TopLeft, Direction::North) => Some(Direction::East),
            (Tile::TopLeft, Direction::West) => Some(Direction::South),
            (Tile::TopRight, Direction::North) => Some(Direction::West),
            (Tile::TopRight, Direction::East) => Some(Direction::South),
            (Tile::BottomRight, Direction::South) => Some(Direction::West),
            (Tile::BottomRight, Direction::East) => Some(Direction::North),
            (Tile::BottomLeft, Direction::South) => Some(Direction::East),
            (Tile::BottomLeft, Direction::West) => Some(Direction::North),
            (_, _) => None,
        }
    }
}

impl From<(Direction, Direction)> for Tile {
    fn from(value: (Direction, Direction)) -> Self {
        match value {
            (Direction::North, Direction::North) => Self::Vertical,
            (Direction::South, Direction::South) => Self::Vertical,
            (Direction::East, Direction::East) => Self::Horizontal,
            (Direction::West, Direction::West) => Self::Horizontal,
            (Direction::South, Direction::West) => Self::TopLeft,
            (Direction::East, Direction::North) => Self::TopLeft,
            (Direction::South, Direction::East) => Self::TopRight,
            (Direction::West, Direction::North) => Self::TopRight,
            (Direction::North, Direction::East) => Self::BottomRight,
            (Direction::West, Direction::South) => Self::BottomRight,
            (Direction::North, Direction::West) => Self::BottomLeft,
            (Direction::East, Direction::South) => Self::BottomLeft,
            (_, _) => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub y_pos: usize,
    pub x_pos: usize,
}

pub struct PathFinder {
    pub start_tile: Tile,
    // pub loop_elements: HashSet<Point>,
    pub size: usize,
}

impl PathFinder {
    pub fn new(tiles: Vec<Vec<Tile>>, starting_pos: Coordinate) -> Self {
        for starting_direction in [
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::North,
        ] {
            let mut size = 0;
            let mut direction = starting_direction;
            let mut position = starting_pos;
            loop {
                // Advance position depending on the direction
                position = match direction {
                    Direction::North => Coordinate {
                        y_pos: position.y_pos - 1,
                        x_pos: position.x_pos,
                    },
                    Direction::East => Coordinate {
                        y_pos: position.y_pos,
                        x_pos: position.x_pos + 1,
                    },
                    Direction::South => Coordinate {
                        y_pos: position.y_pos + 1,
                        x_pos: position.x_pos,
                    },
                    Direction::West => Coordinate {
                        y_pos: position.y_pos,
                        x_pos: position.x_pos - 1,
                    },
                };

                // Add the tile in the loop elements
                size += 1;

                // If we reached the start again, success
                if position == starting_pos {
                    return Self {
                        start_tile: Tile::from((starting_direction, direction)),
                        size,
                    };
                }

                // Get the output direction
                let cur_tile = tiles[position.y_pos][position.x_pos].clone();
                direction = match cur_tile.get_next_direction(direction) {
                    None => break,
                    Some(direction) => direction,
                };
            }
        }
        unreachable!()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    //Get all tiles
    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(From::from).collect())
        .collect();

    // Grid dim
    let width = tiles[0].len();
    // let height = tiles.len();

    // Add border
    let tiles: Vec<Vec<Tile>> = std::iter::once(vec![Tile::Ground; width + 2])
        .chain(tiles.into_iter().map(|line| {
            std::iter::once(Tile::Ground)
                .chain(line)
                .chain(std::iter::once(Tile::Ground))
                .collect()
        }))
        .chain(std::iter::once(vec![Tile::Ground; width + 2]))
        .collect();

    // looking for starting position
    let starting_pos = tiles
        .iter()
        .enumerate()
        .filter_map(|(y_pos, x_line)| {
            x_line
                .iter()
                .position(|t| *t == Tile::Start)
                .map(|x_pos| (x_pos, y_pos))
        })
        .next()
        .expect("Stqrting point is missing");
    let starting_pos = Coordinate {
        x_pos: starting_pos.0,
        y_pos: starting_pos.1,
    };

    let path_finder = PathFinder::new(tiles, starting_pos);
    Some((path_finder.size / 2).try_into().unwrap())
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
