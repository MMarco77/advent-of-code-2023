use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

use itertools::Itertools;
advent_of_code::solution!(11);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Glyph {
    Galaxy,
    EmptySpace(u32),
}

impl From<char> for Glyph {
    fn from(value: char) -> Self {
        match value {
            '#' => Glyph::Galaxy,
            '.' => Glyph::EmptySpace(1),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Universe {
    pub grid: Vec<Vec<Glyph>>,
}

impl Display for Glyph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Glyph::EmptySpace(v) => write!(f, "{}", ".".repeat(*v as usize)),
            Glyph::Galaxy => write!(f, "#"),
        }
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid {
            for tile in line {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn expand(universe: Universe, expansion: u32) -> Universe {
    // Expand Galaxy
    let width = universe.grid.first().expect("Empty universe").len();
    let height = universe.grid.len();

    // Column first
    let mut col_idx_expand: HashSet<usize> = HashSet::new();
    for x_pos in 0..width {
        let mut found_galaxy = false;
        for item in universe.grid.iter().take(height) {
            if let Glyph::Galaxy = item[x_pos] {
                found_galaxy = true;
                break;
            }
        }
        if !found_galaxy {
            col_idx_expand.insert(x_pos);
        }
    }

    let mut exp_universe: Vec<Vec<Glyph>> = Vec::new();
    let new_width = width + col_idx_expand.len() * expansion as usize;

    for line in universe.grid.iter() {
        let empty_line = line.iter().all(|elt| !matches!(elt, Glyph::Galaxy));

        if empty_line {
            for _ in 0..expansion {
                exp_universe.push(vec![Glyph::EmptySpace(new_width.try_into().unwrap())]);
            }
            continue;
        }

        exp_universe.push(
            line.iter()
                .enumerate()
                .flat_map(|(pos, g)| match g {
                    Glyph::Galaxy => vec![*g],
                    Glyph::EmptySpace(v) => {
                        if col_idx_expand.get(&pos).is_some() {
                            vec![Glyph::EmptySpace(expansion)]
                        } else {
                            vec![Glyph::EmptySpace(*v)]
                        }
                    }
                })
                .collect(),
        );
    }

    Universe { grid: exp_universe }
}

fn get_combination(universe: Universe) -> Vec<Vec<(i32, i32)>> {
    let mut galaxy = Vec::new();
    for (y_pos, line) in universe.grid.iter().enumerate() {
        let mut x_pos = 0u32;
        for c in line.iter() {
            match c {
                Glyph::Galaxy => {
                    galaxy.push((x_pos as i32, y_pos as i32));
                    x_pos += 1;
                }
                Glyph::EmptySpace(v) => {
                    x_pos += v;
                }
            }
        }
    }

    galaxy.into_iter().combinations(2).collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let universe = Universe {
        grid: input
            .lines()
            .map(|line| line.chars().map(From::from).collect::<Vec<_>>())
            .collect(),
    };
    // println!("Print universe \n{}", universe);

    let expended_universe = expand(universe, 2);
    // println!("Print expended of 2 of universe \n{}", expended_universe);

    let galaxies = get_combination(expended_universe);
    Some(
        galaxies
            .iter()
            .map(|p| {
                let l = p.first().expect("No first galaxy");
                let r = p.last().expect("No second galaxy");
                ((l.0 - r.0).abs() + (l.1 - r.1).abs()) as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let universe = Universe {
        grid: input
            .lines()
            .map(|line| line.chars().map(From::from).collect::<Vec<_>>())
            .collect(),
    };
    // println!("Print universe \n{}", universe);

    let expended_universe = expand(universe, 1000000);
    // println!("Print expended of 2 of universe \n{}", expended_universe);

    let galaxies = get_combination(expended_universe);
    Some(
        galaxies
            .iter()
            .map(|p| {
                let l = p.first().expect("No first galaxy");
                let r = p.last().expect("No second galaxy");
                ((l.0 - r.0).abs() + (l.1 - r.1).abs()) as u32
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
        // 9957702
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
        // 512240933238,
    }
}
