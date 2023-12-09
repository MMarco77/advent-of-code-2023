use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(2);

fn get_play(play: &str) -> Option<(u32, &str)> {
    let mut parts = play.split_whitespace();

    let number_str = parts.next().unwrap_or_default();
    let number: Result<u32, _> = number_str.parse();

    let word = parts.next().unwrap_or_default();

    match number {
        Ok(parsed_number) => Some((parsed_number, word)),
        Err(_) => None,
    }
}

fn update_ref(ref_value: Option<u32>, new_value: u32) -> Option<u32> {
    match ref_value {
        Some(v) if v < new_value => Some(new_value),
        None => Some(new_value),
        _ => ref_value,
    }
}

fn power_of_game(phase: &str, re_parsing_color: &Regex) -> Option<u32> {
    let mut min_green: Option<u32> = None;
    let mut min_blue: Option<u32> = None;
    let mut min_red: Option<u32> = None;
    for capture in re_parsing_color.captures_iter(phase) {
        if let Some(play) = capture.name("color") {
            if let Some((count, color)) = get_play(play.as_str()) {
                if color == "green" {
                    min_green = update_ref(min_green, count);
                } else if color == "red" {
                    min_red = update_ref(min_red, count);
                } else if color == "blue" {
                    min_blue = update_ref(min_blue, count);
                } else {
                    panic!("Color missing {color}")
                }
            } else {
                return None;
            }
        }
    }

    // Compute minimum
    match (min_green, min_red, min_blue) {
        (Some(g), Some(r), Some(b)) => Some(g * r * b),
        (_, _, _) => None,
    }
}

fn is_game_possible(
    phase: &str,
    available_colors: &HashMap<&str, u32>,
    re_parsing_color: &Regex,
    re_parsing_id: &Regex,
) -> Option<u32> {
    let id: u32 = re_parsing_id
        .captures(phase)
        .and_then(|cap| {
            cap.name("id")
                .map(|id| id.as_str().parse::<u32>().expect("Invalid digit"))
        })
        .expect("Wrong ID");
    for capture in re_parsing_color.captures_iter(phase) {
        if let Some(play) = capture.name("color") {
            if let Some((count, color)) = get_play(play.as_str()) {
                if available_colors.get(color).expect("Invalid color") < &count {
                    return None;
                }
            } else {
                return None;
            }
        }
    }
    Some(id)
}

pub fn part_one(input: &str) -> Option<u32> {
    let available_colors = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let re_parser_color: Regex = Regex::new(r"(?P<color>\d+ (?:blue|red|green))*").unwrap();
    let re_parser_id: Regex = Regex::new(r"Game (?P<id>\d+):").unwrap();

    let sum_id: u32 = input.lines().fold(0, |acc, line| {
        match is_game_possible(line, &available_colors, &re_parser_color, &re_parser_id) {
            Some(id) => acc + id,
            None => acc,
        }
    });
    Some(sum_id)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re_parser_color: Regex = Regex::new(r"(?P<color>\d+ (?:blue|red|green))*").unwrap();

    let sum_power: u32 = input
        .lines()
        .filter_map(|line| power_of_game(line, &re_parser_color))
        .sum();
    Some(sum_power)
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
        assert_eq!(result, Some(2286));
    }
}
