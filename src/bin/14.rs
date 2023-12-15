use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

static TARGET: usize = 1_000_000_000;

advent_of_code::solution!(14);

#[derive(Debug, Clone, Hash)]
enum RockForm {
    Rounded,
    Square,
    EmptySpace,
}

impl From<char> for RockForm {
    fn from(value: char) -> Self {
        match value {
            'O' => RockForm::Rounded,
            '#' => RockForm::Square,
            '.' => RockForm::EmptySpace,
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Tilt {
    West,
    East,
    North,
    South,
}

fn transpose(pattern: &Vec<Vec<RockForm>>) -> Vec<Vec<RockForm>> {
    if pattern.is_empty() {
        return Vec::new();
    }

    let height = pattern.len();
    let width = pattern[0].len();

    (0..width)
        .map(|x_pos| {
            (0..height)
                .map(|y_pos| pattern[y_pos][x_pos].clone())
                .collect()
        })
        .collect()
}

fn move_to_origin(line: Vec<RockForm>) -> Vec<RockForm> {
    let mut round_count: Vec<RockForm> = Vec::new();
    let mut empty_count: Vec<RockForm> = Vec::new();
    let mut new_line = Vec::new();

    for c in line.into_iter() {
        match c {
            RockForm::Square => {
                new_line.append(&mut round_count);
                new_line.append(&mut empty_count);
                new_line.push(RockForm::Square);
            }
            RockForm::EmptySpace => empty_count.push(c),
            RockForm::Rounded => round_count.push(c),
        }
    }

    // Last possible rest
    if !round_count.is_empty() {
        new_line.append(&mut round_count);
    }
    if !empty_count.is_empty() {
        new_line.append(&mut empty_count);
    }
    new_line
}

#[derive(Debug, Clone, Hash)]
struct Platform {
    grid: Vec<Vec<RockForm>>,
}

impl Platform {
    pub fn tilt(mut self, tilt: Tilt) -> Self {
        match tilt {
            Tilt::North => {
                self.grid = transpose(&self.grid);
                self.grid = self.grid.into_iter().map(move_to_origin).collect();
                self.grid = transpose(&self.grid);
            }
            Tilt::West => {
                self.grid = self.grid.into_iter().map(move_to_origin).collect();
            }
            Tilt::East => {
                self.grid.reverse();
                self.grid = self.grid.into_iter().map(move_to_origin).collect();
                self.grid.reverse();
            }
            Tilt::South => {
                self.grid = transpose(&self.grid);
                self.grid.reverse();
                self.grid = self.grid.into_iter().map(move_to_origin).collect();
                self.grid.reverse();
                self.grid = transpose(&self.grid);
            }
        }
        self
    }

    pub fn compute_load(&self) -> u128 {
        let width: u128 = self.grid.len().try_into().unwrap();
        self.grid
            .iter()
            .enumerate()
            .fold(0u128, |acc, (y_pos, line)| {
                let count: u128 = line
                    .iter()
                    .filter_map(|t| {
                        if let RockForm::Rounded = t {
                            Some(1u128)
                        } else {
                            None
                        }
                    })
                    .sum();
                acc + (width - y_pos as u128) * count
            })
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.grid.iter() {
            let mut d: String = line.iter().fold(String::new(), |mut acc, m| {
                acc.push(match m {
                    RockForm::Rounded => 'O',
                    RockForm::Square => '#',
                    RockForm::EmptySpace => '.',
                });
                acc
            });
            d.push('\n');
            write!(f, "{}", d).expect("Not written");
        }
        Ok(())
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut platform = Platform {
        grid: input
            .lines()
            .map(|line| line.chars().map(From::from).collect())
            .collect(),
    };
    platform = platform.tilt(Tilt::North);
    Some(platform.compute_load())
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut platform = Platform {
        grid: input
            .lines()
            .map(|line| line.chars().map(From::from).collect())
            .collect(),
    };

    // BF?
    // for counter in 1..TARGET {
    //     if counter == 0 || counter % 100000 == 0 {
    //         println!("Count {counter}/TARGET [{}%]", counter*100/TARGET);
    //     }
    //     platform = platform.tilt(Tilt::North).tilt(Tilt::West).tilt(Tilt::South).tilt(Tilt::East);
    // }

    // Store grid hash to increase loop speed
    let mut grid_results: HashMap<u64, usize> = HashMap::new();

    // Do cycles
    let mut cycle_count = 0;
    loop {
        // Compute platform hash
        let platform_hash = calculate_hash(&platform);

        // Check if we looped somehow
        if let Some(last_cycle_count) = grid_results.get(&platform_hash) {
            // Determinate how many steps to skip
            let cycle_skip = cycle_count - last_cycle_count;
            let skip_count = (TARGET - cycle_count) / cycle_skip;

            // Perform the skip
            cycle_count += skip_count * cycle_skip;

            // Break the loop in case we reached the end
            if cycle_count == TARGET {
                break;
            };
        }

        // Add the hash to the collection
        grid_results.insert(platform_hash, cycle_count);

        // Tilt the platform
        platform = platform
            .tilt(Tilt::North)
            .tilt(Tilt::West)
            .tilt(Tilt::South)
            .tilt(Tilt::East);

        // increase the cycle count
        cycle_count += 1;

        // termination check
        if cycle_count == TARGET {
            break;
        }
    }

    Some(platform.compute_load())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
