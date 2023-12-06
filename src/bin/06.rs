use std::iter::zip;

advent_of_code::solution!(6);

fn round_up(num: u64, denum: u64) -> u64 {
    let quotient = num / denum;
    let rest = num % denum;

    if rest > 0 {
        quotient + 1
    } else {
        quotient
    }
}

fn get_record_count(time: u64, distance: u64) -> Option<u32> {
    let mut counter = 0u32;
    let mut cur_hold_speed = 1u64;
    let mut start_range = false;
    loop {
        let elapsed_time: u64 = round_up(distance, cur_hold_speed);
        let total = cur_hold_speed + elapsed_time;
        if total < time || (total == time && (elapsed_time * cur_hold_speed) > distance) {
            counter += 1;
            start_range = true;
        } else if start_range {
            break;
        }
        cur_hold_speed += 1;
    }
    Some(counter)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut iter = input.lines();

    let time_str = iter.next().expect("Missing Time line");
    let distance_str = iter.next().expect("Missing Distance line");

    let time: Vec<u64> = time_str
        .split_whitespace()
        .map(|s| s.parse::<u64>())
        .filter_map(Result::ok)
        .collect::<Vec<u64>>();
    let distance: Vec<u64> = distance_str
        .split_whitespace()
        .map(|s| s.parse::<u64>())
        .filter_map(Result::ok)
        .collect::<Vec<u64>>();

    // Compute racing
    zip(time, distance).fold(None, |acc, (t, d)| {
        let rec_count: Option<u32> = get_record_count(t, d);
        match (acc, rec_count) {
            (None, None) => None,
            (None, Some(a)) => Some(a),
            (Some(a), None) => Some(a),
            (Some(a), Some(b)) => Some(a * b),
        }
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut iter = input.lines();

    let (_, time_str) = iter
        .next()
        .expect("Missing Time line")
        .split_once(':')
        .unwrap_or_default();
    let (_, distance_str) = iter
        .next()
        .expect("Missing Time line")
        .split_once(':')
        .unwrap_or_default();

    let time_str: String = time_str.chars().filter(|c| !c.is_whitespace()).collect();
    let time = time_str
        .parse::<u64>()
        .unwrap_or_else(|_| panic!("Invalid u64 nbr for {}", time_str));

    let distance_str: String = distance_str
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let distance = distance_str
        .parse::<u64>()
        .unwrap_or_else(|_| panic!("Invalid u64 nbr for {}", time_str));
    get_record_count(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
