advent_of_code::solution!(12);
#[derive(Debug)]
enum Record {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Record {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => unreachable!(),
        }
    }
}

fn split_input(input: &str) -> Vec<(Vec<Record>, Vec<u32>)> {
    input
        .lines()
        .map(|c| {
            let (logs, crc) = c.split_once(' ').expect("Invalid record");
            (
                logs.chars().map(From::from).collect(),
                crc.split(',').filter_map(|c| c.parse().ok()).collect(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let records = split_input(input);
    println!("{:#?}", records);

    unimplemented!()
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
