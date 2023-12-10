use std::{collections::HashMap, fmt, iter::zip};

advent_of_code::solution!(7);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Copy)]
enum HandType<'a> {
    HighCard(&'a str),
    OnePair(&'a str),
    TwoPair(&'a str),
    ThreeOfkind(&'a str),
    FullHouse(&'a str),
    FourOfkind(&'a str),
    FiveOfkind(&'a str),
}

impl fmt::Display for HandType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HandType::HighCard(v) => write!(f, "HighCard {}", *v),
            HandType::OnePair(v) => write!(f, "OnePair {}", *v),
            HandType::TwoPair(v) => write!(f, "TwoPair {}", *v),
            HandType::ThreeOfkind(v) => write!(f, "ThreeOfkind {}", *v),
            HandType::FullHouse(v) => write!(f, "FullHouse {}", *v),
            HandType::FourOfkind(v) => write!(f, "FourOfkind {}", *v),
            HandType::FiveOfkind(v) => write!(f, "FiveOfkind {}", *v),
        }
    }
}

fn compare_hands(
    hand_a: HandType,
    hand_b: HandType,
    card_weight: &HashMap<char, u32>,
) -> std::cmp::Ordering {
    let (left, right) = match (hand_a, hand_b) {
        (HandType::HighCard(a), HandType::HighCard(b)) => (a, b),
        (HandType::OnePair(a), HandType::OnePair(b)) => (a, b),
        (HandType::TwoPair(a), HandType::TwoPair(b)) => (a, b),
        (HandType::ThreeOfkind(a), HandType::ThreeOfkind(b)) => (a, b),
        (HandType::FullHouse(a), HandType::FullHouse(b)) => (a, b),
        (HandType::FourOfkind(a), HandType::FourOfkind(b)) => (a, b),
        (HandType::FiveOfkind(a), HandType::FiveOfkind(b)) => (a, b),
        _ => return hand_a.cmp(&hand_b),
    };

    let left = left
        .chars()
        .map(|c| card_weight.get(&c).expect("Invalid card"))
        .collect::<Vec<_>>();
    let right = right
        .chars()
        .map(|c| card_weight.get(&c).expect("Invalid card"))
        .collect::<Vec<_>>();

    for (l, r) in zip(left, right) {
        if l == r {
            continue;
        }
        return l.cmp(r);
    }
    std::cmp::Ordering::Equal
}

fn is_nth_same_card(map: &HashMap<char, usize>, target_value: usize) -> bool {
    for (&_key, &value) in map.iter() {
        if value == target_value {
            return true;
        }
    }
    false
}

fn solve_hand_with_jockers(hand: &str) -> HandType {
    let mut counter: HashMap<char, usize> = HashMap::new();
    let mut jockers = 0u32;

    for card in hand.chars() {
        if card == 'J' {
            jockers += 1;
        } else {
            *counter.entry(card).or_insert(0) += 1;
        }
    }

    match jockers {
        0 => return solve_hand(hand),
        5 => return HandType::FiveOfkind(hand),
        _ => {}
    }

    // Determinate the combination
    match counter.len() {
        1 => HandType::FiveOfkind(hand),
        2 => {
            // J XXXX   => AABB(2|2) AAAB(3|1)
            // JJ XXX   => AAB(2|1)
            // JJJ XX   => AB(1|1)
            // JJJJ X   => X
            let three_value = is_nth_same_card(&counter, 3);
            let double_value = is_nth_same_card(&counter, 2);
            let uniq_value = is_nth_same_card(&counter, 1);

            match (three_value, double_value, uniq_value) {
                (false, true, false) => HandType::FullHouse(hand), // 2|2
                (false, true, true) => HandType::FourOfkind(hand), // 2|1
                (true, false, true) => HandType::FourOfkind(hand), // 3|1
                (false, false, true) => HandType::FourOfkind(hand), // 1|1
                _ => unreachable!("Match 2 groups with jocker"),
            }
        }
        3 => {
            // J XXXX   => AABC(2|1|1)
            // JJ XXX   => ABC(1|1|1)
            // JJJ XX   => X
            // JJJJ X   => X
            HandType::ThreeOfkind(hand)
        }
        4 => HandType::OnePair(hand),
        _ => unreachable!("Match Hand type failed"),
    }
}

fn solve_hand(hand: &str) -> HandType {
    let mut counter: HashMap<char, usize> = HashMap::new();
    for card in hand.chars() {
        *counter.entry(card).or_insert(0) += 1;
    }

    // Determinate the combination
    match counter.len() {
        1 => HandType::FiveOfkind(hand),
        2 => {
            let four_value = is_nth_same_card(&counter, 4);
            let three_value = is_nth_same_card(&counter, 3);
            let pair_value = is_nth_same_card(&counter, 2);

            match (four_value, three_value, pair_value) {
                (true, false, false) => HandType::FourOfkind(hand),
                (false, true, true) => HandType::FullHouse(hand),
                _ => unreachable!("Match Hand type Full, Three or Pair"),
            }
        }
        3 => {
            if is_nth_same_card(&counter, 3) {
                HandType::ThreeOfkind(hand)
            } else {
                HandType::TwoPair(hand)
            }
        }
        4 => HandType::OnePair(hand),
        5 => HandType::HighCard(hand),
        _ => unreachable!("Match Hand type failed"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let card_weight: HashMap<char, u32> = HashMap::from([
        ('A', 14u32),
        ('K', 13u32),
        ('Q', 12u32),
        ('J', 11u32),
        ('T', 10u32),
        ('9', 9u32),
        ('8', 8u32),
        ('7', 7u32),
        ('6', 6u32),
        ('5', 5u32),
        ('4', 4u32),
        ('3', 3u32),
        ('2', 2u32),
    ]);
    let mut hands: Vec<(HandType, u32)> = input
        .lines()
        .map(|x| {
            let (hand, bid) = x.trim().split_once(' ').expect("Wrong hand");
            (
                solve_hand(hand.trim()),
                bid.trim().parse::<u32>().expect("Wrong bid"),
            )
        })
        .collect();
    hands.sort_by(|a, b| compare_hands(a.0, b.0, &card_weight));

    Some(hands.iter().enumerate().fold(0, |acc, (range, (_, bid))| {
        acc + (range as u32 + 1) * (*bid)
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let card_weight: HashMap<char, u32> = HashMap::from([
        ('A', 14u32),
        ('K', 13u32),
        ('Q', 12u32),
        ('T', 10u32),
        ('9', 9u32),
        ('8', 8u32),
        ('7', 7u32),
        ('6', 6u32),
        ('5', 5u32),
        ('4', 4u32),
        ('3', 3u32),
        ('2', 2u32),
        ('J', 1u32),
    ]);
    let mut hands: Vec<(HandType, u32)> = input
        .lines()
        .map(|x| {
            let (hand, bid) = x.trim().split_once(' ').expect("Wrong hand");
            (
                solve_hand_with_jockers(hand.trim()),
                bid.trim().parse::<u32>().expect("Wrong bid"),
            )
        })
        .collect();
    hands.sort_by(|a, b| compare_hands(a.0, b.0, &card_weight));

    Some(hands.iter().enumerate().fold(0, |acc, (range, (_, bid))| {
        acc + (range as u32 + 1) * (*bid)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_sub("examples", DAY, 1));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_sub("examples", DAY, 2));
        assert_eq!(result, Some(5905));
    }
}
