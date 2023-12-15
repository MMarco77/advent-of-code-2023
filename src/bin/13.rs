// 27646
// Convert line to value ('.':0, '#': 1)
// Windows slice + reinjection du hashset.
// par ligne, inversion, ligne.
//

use std::{
    cmp::min,
    collections::HashSet,
    fmt::{Display, Formatter},
};

advent_of_code::solution!(13);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Mirror {
    Ash,
    Rocks,
}

impl From<char> for Mirror {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rocks,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Pattern {
    pattern: Vec<Vec<Mirror>>,
}

fn transpose(pattern: &Vec<Vec<Mirror>>) -> Vec<Vec<Mirror>> {
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

fn find_mirror_pos(pattern: &[Mirror]) -> HashSet<usize> {
    let width = pattern.len();
    let mut axis_pos: HashSet<usize> = HashSet::new();

    for pos in 1..(width - 1) {
        let min_ref_size = min(pos + 1, width - 1 - pos);
        let mut reflec_left = pattern[(pos + 1 - min_ref_size)..=pos].to_vec();
        reflec_left.reverse();
        let reflec_rigth: Vec<Mirror> = pattern[(pos + 1)..(pos + 1 + min_ref_size)].to_vec();

        let mut sym = true;
        for (elem1, elem2) in reflec_left.iter().zip(reflec_rigth.iter()) {
            if *elem1 != *elem2 {
                sym = false;
                break;
            }
        }
        if sym {
            axis_pos.insert(pos);
        }
    }

    axis_pos
}

fn find_vertical_mirror_pos(pattern: &[Vec<Mirror>]) -> Option<usize> {
    let mut common_set: HashSet<usize> = find_mirror_pos(pattern.first().unwrap());
    for p in pattern.iter().skip(1) {
        let current_set = find_mirror_pos(p);
        common_set.retain(|&x| current_set.contains(&x));
    }

    common_set.into_iter().next()
}

impl Pattern {
    pub fn new() -> Self {
        Self {
            pattern: Vec::new(),
        }
    }

    pub fn compute_mirror_width(&self) -> u32 {
        let mut res = 0usize;
        if let Some(v) = find_vertical_mirror_pos(&self.pattern) {
            res += v + 1;
        }

        // Revert pattern
        let revert_pattern = transpose(&self.pattern);
        if let Some(v) = find_vertical_mirror_pos(&revert_pattern) {
            res += 100 * (v + 1);
        }

        res.try_into().unwrap()
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.pattern.iter() {
            let mut d: String = line.iter().fold(String::new(), |mut acc, m| {
                acc.push(match m {
                    Mirror::Ash => '.',
                    Mirror::Rocks => '#',
                });
                acc
            });
            d.push('\n');
            write!(f, "{}", d).expect("Not written");
        }
        Ok(())
    }
}

impl Display for Mirror {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mirror::Ash => write!(f, "."),
            Mirror::Rocks => write!(f, "#"),
        }
    }
}

fn split_input(input: &str) -> Vec<Pattern> {
    let mut patterns: Vec<Pattern> = Vec::new();
    let mut current_pattern = Pattern::new();

    for line in input.lines() {
        if line.is_empty() {
            patterns.push(current_pattern);
            current_pattern = Pattern::new();
            continue;
        }
        current_pattern
            .pattern
            .push(line.chars().map(From::from).collect());
    }

    // Last grid never register
    if !current_pattern.pattern.is_empty() {
        patterns.push(current_pattern);
    }

    patterns
}

pub fn part_one(input: &str) -> Option<u32> {
    let patterns_map = split_input(input);

    // for p in patterns_map.iter() {
    //     println!("{}\n", *p);
    // }

    Some(
        patterns_map
            .iter()
            .map(|c| c.clone().compute_mirror_width())
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
