use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let final_count = input.lines().fold(0, |acc, line| -> u32 {
        let mut tens: Option<u32> = None;
        let mut unit: u32 = 0;
        line.chars().for_each(|c| {
            if c.is_ascii_digit() {
                let cur_digit = c.to_digit(10).expect("Invalid digit");
                if tens.is_none() {
                    tens = Some(cur_digit * 10);
                }
                unit = cur_digit;
            }
        });

        match tens {
            Some(tens_value) => acc + tens_value + unit,
            None => acc,
        }
    });

    Some(final_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let final_count = input.lines().fold(0, |acc, line| -> u32 {
        let mut tens: Option<u32> = None;
        let mut unit: u32 = 0;

        let digit_string = HashMap::from([
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ]);

        for (i, mut c) in line.chars().enumerate() {
            digit_string.iter().for_each(|(digit_str, digit_nbr)| {
                if line.get(i..).expect("oups").starts_with(digit_str) {
                    c = digit_nbr.chars().next().expect("No Way");
                }
            });

            if c.is_ascii_digit() {
                let cur_digit = c.to_digit(10).expect("Invalid digit");
                if tens.is_none() {
                    tens = Some(cur_digit * 10);
                }
                unit = cur_digit;
            }
        }

        match tens {
            Some(tens_value) => acc + tens_value + unit,
            None => acc,
        }
    });

    Some(final_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
