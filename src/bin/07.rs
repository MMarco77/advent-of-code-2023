// Wrong Vqlue
// 249057120
// 248409953
// 245065331
// 248179786

use lazy_static::lazy_static;
use std::{collections::HashMap, fmt, iter::zip};

advent_of_code::solution!(7);

lazy_static! {
    static ref CARD_LABEL: HashMap<char, u32> = HashMap::from([
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
        ('2', 2u32)
    ]);
}

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

fn compare_hands(hand_a: HandType, hand_b: HandType) -> std::cmp::Ordering {
    let (left, right) = match (hand_a, hand_b) {
        (HandType::HighCard(a), HandType::HighCard(b)) => (a, b),
        (HandType::OnePair(a), HandType::OnePair(b)) => (a, b),
        (HandType::TwoPair(a), HandType::TwoPair(b)) => (a, b),
        (HandType::ThreeOfkind(a), HandType::ThreeOfkind(b)) => (a, b),
        (HandType::FullHouse(a), HandType::FullHouse(b)) => (a, b),
        (HandType::FourOfkind(a), HandType::FourOfkind(b)) => (a, b),
        (HandType::FiveOfkind(a), HandType::FiveOfkind(b)) => (a, b),
        // (HandType::FiveOfkind(_), _ ) => return std::cmp::Ordering::Greater,
        // (HandType::FourOfkind(_), _ ) => return std::cmp::Ordering::Greater,
        // (HandType::FullHouse(_), _ ) => return std::cmp::Ordering::Greater,
        // (HandType::ThreeOfkind(_), _ ) => return std::cmp::Ordering::Greater,
        // (HandType::TwoPair(_), _ ) => return std::cmp::Ordering::Greater,
        // (HandType::OnePair(_), _ ) => return std::cmp::Ordering::Greater,
        // (HandType::HighCard(_), _ ) => return std::cmp::Ordering::Greater,
        _ => return hand_a.cmp(&hand_b),
    };

    let left = left
        .chars()
        .map(|c| CARD_LABEL.get(&c).expect("Invalid card"))
        .collect::<Vec<_>>();
    let right = right
        .chars()
        .map(|c| CARD_LABEL.get(&c).expect("Invalid card"))
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

fn solve_hand(hand: &str) -> HandType {
    let mut counter: HashMap<char, usize> = HashMap::new();

    for card in hand.chars() {
        *counter.entry(card).or_insert(0) += 1;
    }

    // Determinate the combination
    match counter.len() {
        1 => HandType::FiveOfkind(hand),
        2 => {
            let full_value = is_nth_same_card(&counter, 4);
            let three_value = is_nth_same_card(&counter, 3);
            let pair_value = is_nth_same_card(&counter, 2);

            match (full_value, three_value, pair_value) {
                (true, false, false) => HandType::FourOfkind(hand),
                (false, true, true) => HandType::FullHouse(hand),
                (false, false, true) => HandType::ThreeOfkind(hand),
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
    hands.sort_by(|a, b| compare_hands(a.0, b.0));

    Some(hands.iter().enumerate().fold(0, |acc, (range, (_, bid))| {
        acc + (range as u32 + 1) * (*bid)
    }))
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
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_hand() {
        assert_eq!(
            compare_hands(solve_hand("AAAAA"), solve_hand("KKKKK")),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            compare_hands(solve_hand("AAAAA"), solve_hand("AAAAA")),
            std::cmp::Ordering::Equal
        );
        assert_eq!(
            compare_hands(solve_hand("2AAAA"), solve_hand("3AAAA")),
            std::cmp::Ordering::Less
        );

        assert_eq!(
            compare_hands(solve_hand("AAA22"), solve_hand("AAA66")),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            compare_hands(solve_hand("AAK22"), solve_hand("AAK66")),
            std::cmp::Ordering::Less
        );

        assert_eq!(
            compare_hands(solve_hand("AA234"), solve_hand("TTKQ2")),
            std::cmp::Ordering::Greater
        );

        assert_eq!(
            compare_hands(solve_hand("KK677"), solve_hand("KTJJT")),
            std::cmp::Ordering::Greater
        );
    }
}
