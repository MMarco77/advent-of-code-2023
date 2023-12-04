use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

struct Game {
    id: u32,
    nbr_u_hv: HashSet<u32>,
    win_nbr: HashSet<u32>,
}

impl Game {
    pub fn new(game_str: &str) -> Game {
        // Gard vs game
        let (card, games) = game_str.split_once(':').unwrap_or_default();

        // Card ID
        let (_, card_id_str) = card.split_once(' ').unwrap_or_default();
        let card_id = card_id_str.trim().parse::<u32>().expect("Invalid Card ID");

        let (nbr_u_hv, win_nbr) = games.split_once('|').unwrap_or_default();

        let left = nbr_u_hv
            .split(' ')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        let right = win_nbr
            .split(' ')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        Game {
            id: card_id,
            nbr_u_hv: left,
            win_nbr: right,
        }
    }

    pub fn compute_score(&self) -> u32 {
        match self.get_score() {
            v if v > 0 => 2u32.pow((v - 1) as u32),
            _ => 0,
        }
    }

    pub fn get_score(&self) -> usize {
        self.nbr_u_hv
            .intersection(&self.win_nbr)
            .collect::<Vec<_>>()
            .len()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .fold(0, |acc, line| acc + Game::new(line).compute_score()),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut scratchcards: HashMap<u32, u32> = HashMap::new();
    for card_step in input.lines() {
        let game = Game::new(card_step);

        let cur_play_count = scratchcards.entry(game.id).or_insert(0);
        *cur_play_count += 1;
        let count_current_card = *cur_play_count;

        let nbr_copy_card = game.get_score();
        if nbr_copy_card == 0 {
            continue;
        }

        for copy_card_id in 1..=nbr_copy_card as u32 {
            let cur_count = scratchcards.entry(game.id + copy_card_id).or_insert(0);
            *cur_count += count_current_card;
        }
    }

    // Compute card number
    Some(scratchcards.iter().fold(0, |acc, (_, count)| acc + count))
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
