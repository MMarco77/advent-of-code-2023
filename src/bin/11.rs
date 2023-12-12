use std::collections::HashSet;

use itertools::Itertools;
advent_of_code::solution!(11);

#[derive(Copy, Clone, Debug)]
pub enum Glyph {
    Galaxy,
    EmptySpace,
}

fn interpret(value: char) -> Glyph {
    match value {
        '#' => Glyph::Galaxy,
        '.' => Glyph::EmptySpace,
        _ => unreachable!(),
    }
}

fn expand(universe: Vec<Vec<Glyph>>) -> Vec<Vec<(i32, i32)>> {
    // Expand Galaxy
    let width = universe[0].len();
    let height = universe.len();

    // Column first
    let mut col_idx_expand: HashSet<usize> = HashSet::new();
    for x_pos in 0..width {
        let mut found_galaxy = false;
        for item in universe.iter().take(height) {
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
    let new_width = universe[0].len() + col_idx_expand.len();
    for line in universe.iter() {
        let empty_line = line.iter().all(|elem| matches!(elem, Glyph::EmptySpace));
        if empty_line {
            exp_universe.push(vec![Glyph::EmptySpace; new_width]);
            exp_universe.push(vec![Glyph::EmptySpace; new_width]);
            continue;
        }
        if col_idx_expand.is_empty() {
            exp_universe.push(line.clone());
        }

        // Add column

        exp_universe.push(
            line.iter()
                .enumerate()
                .flat_map(|(pos, g)| match g {
                    Glyph::Galaxy => vec![*g],
                    Glyph::EmptySpace => {
                        if col_idx_expand.get(&(pos + 1)).is_some() {
                            vec![Glyph::EmptySpace; 2]
                        } else {
                            vec![Glyph::EmptySpace]
                        }
                    }
                })
                .collect(),
        );
    }

    let galaxy = exp_universe
        .iter()
        .enumerate()
        .flat_map(|(y, c)| {
            c.iter().enumerate().filter_map(move |(x, c)| match c {
                Glyph::Galaxy => Some((x as i32, y as i32)),
                _ => None,
            })
        })
        .collect::<Vec<_>>();

    galaxy.into_iter().combinations(2).collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let universe: Vec<Vec<Glyph>> = input
        .lines()
        .map(|line| line.chars().map(interpret).collect::<Vec<_>>())
        .collect();

    let galaxies = expand(universe);

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

pub fn part_two(_: &str) -> Option<u32> {
    None
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
    }
}
