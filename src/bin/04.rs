use std::collections::{HashSet, HashMap};

advent_of_code::solution!(4);

fn process_card(a: HashSet<&str>, b: HashSet<&str>) -> u32 {
    let common = a.intersection(&b).collect::<Vec<_>>();
    match common.len() {
        v if v > 0 => 2u32.pow((v - 1) as u32),
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .fold(0, |acc, line| match line.split_once(':') {
                Some((_, games)) => match games.split_once('|') {
                    Some((nbr_u_hv, win_nbr)) => {
                        let left = nbr_u_hv
                            .split(' ')
                            .filter(|s| s.parse::<u32>().is_ok())
                            .collect();
                        let right = win_nbr
                            .split(' ')
                            .filter(|s| s.parse::<u32>().is_ok())
                            .collect();
                        acc + process_card(left, right)
                    }
                    None => acc,
                },
                None => acc,
            }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut scratchcards: HashMap<u32, u32> = HashMap::new();
    for card_step in input.lines() {
        if let Some((card, games)) =card_step.split_once(':') {
            let (_, card_id_str) = card.split_once(" ").unwrap_or_default();
            let card_id =card_id_str.trim().parse::<u32>().expect("Invalid Card ID");
            let cur_play_count = scratchcards.entry(card_id).or_insert(0);
            *cur_play_count += 1;
            let count_current_card = *cur_play_count;
            let (nbr_u_hv, win_nbr) = games.split_once('|').unwrap_or_default();

            let left: HashSet<&str> = nbr_u_hv
                            .split(' ')
                            .filter(|s| s.parse::<u32>().is_ok())
                            .collect();
            let right: HashSet<&str> = win_nbr
                .split(' ')
                .filter(|s| s.parse::<u32>().is_ok())
                .collect();

            let nbr_copy_card = left.intersection(&right).collect::<Vec<_>>();
            if nbr_copy_card.len() == 0 {continue}
            for copy_card_id in 1..=nbr_copy_card.len() as u32 {
                let cur_count = scratchcards.entry(card_id + copy_card_id).or_insert(0);
                *cur_count += count_current_card;
            }
        }
    }

    // Compute card number
    Some(scratchcards.iter().fold(0, |acc, (id, count)| {
        acc + count
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
