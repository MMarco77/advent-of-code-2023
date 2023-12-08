use lazy_static::lazy_static;
use std::{
    cmp::{max, min},
    collections::HashMap,
};

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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
enum HandType {
    HighCard(u32),
    OnePair(u32),
    TwoPair(u32, u32),
    ThreeOfkind(u32),
    FullHouse(u32, u32),
    FourOfkind(u32, u32),
    FiveOfkind(u32),
}

fn compare_two_value(a1: u32, a2: u32, b1: u32, b2: u32) -> std::cmp::Ordering {
    if max(a1, a2).cmp(&max(b1, b2)) == std::cmp::Ordering::Equal {
        min(a1, a2).cmp(&min(b1, b2))
    } else {
        max(a1, a2).cmp(&max(b1, b2))
    }
}

fn compare_hands(hand_a: HandType, hand_b: HandType) -> std::cmp::Ordering {
    if hand_a.cmp(&hand_b) != std::cmp::Ordering::Equal {
        return hand_a.cmp(&hand_b);
    }

    match (hand_a, hand_b) {
        (HandType::HighCard(a), HandType::HighCard(b)) => a.cmp(&b),
        (HandType::OnePair(a), HandType::OnePair(b)) => a.cmp(&b),
        (HandType::TwoPair(a1, a2), HandType::TwoPair(b1, b2)) => compare_two_value(a1, a2, b1, b2),
        (HandType::ThreeOfkind(a), HandType::ThreeOfkind(b)) => a.cmp(&b),
        (HandType::FullHouse(a1, a2), HandType::FullHouse(b1, b2)) => {
            compare_two_value(a1, a2, b1, b2)
        }
        (HandType::FourOfkind(a1, a2), HandType::FourOfkind(b1, b2)) => {
            compare_two_value(a1, a2, b1, b2)
        }
        (HandType::FiveOfkind(a), HandType::FiveOfkind(b)) => a.cmp(&b),
        _ => panic!("Not really similar!!!!"),
    }
}

fn get_key_by_value(map: &HashMap<char, usize>, target_value: usize) -> Option<char> {
    for (&key, &value) in map.iter() {
        if value == target_value {
            return Some(key);
        }
    }
    None
}

fn solve_hand(hand: &str) -> HandType {
    let mut counter: HashMap<char, usize> = HashMap::new();

    for card in hand.chars() {
        *counter.entry(card).or_insert(0) += 1;
    }

    let mut iter = counter.iter();
    let c1 = iter.next();
    let c2 = iter.next();
    let c3 = iter.next();
    let c4 = iter.next();
    let c5 = iter.next();

    // Determinate the combination
    match counter.len() {
        1 => {
            let card = c1.expect("No value").0;
            let value = CARD_LABEL.get(card).expect("Unknown value");
            HandType::FiveOfkind(*value)
        }
        2 => {
            let full_value = get_key_by_value(&counter, 4);
            let three_value = get_key_by_value(&counter, 3);
            let pair_value = get_key_by_value(&counter, 2);

            match (full_value, three_value, pair_value) {
                (Some(a), None, None) => HandType::FourOfkind(
                    *CARD_LABEL.get(&a).unwrap(),
                    *CARD_LABEL
                        .get(&get_key_by_value(&counter, 1).unwrap())
                        .unwrap(),
                ),
                (None, Some(a), Some(b)) => {
                    let v1 = *CARD_LABEL.get(&a).unwrap();
                    let v2 = *CARD_LABEL.get(&b).unwrap();
                    HandType::FullHouse(max(v1, v2), min(v1, v2))
                }
                (None, None, Some(a)) => HandType::ThreeOfkind(*CARD_LABEL.get(&a).unwrap()),
                _ => panic!("Match Hand type Full, Three or Pair"),
            }
        }
        3 => {
            if let Some(key) = get_key_by_value(&counter, 3) {
                HandType::ThreeOfkind(*CARD_LABEL.get(&key).unwrap())
            } else {
                let (a, b) = match (
                    c1.expect("No value").1,
                    c2.expect("No value").1,
                    c3.expect("No value").1,
                ) {
                    (2, 2, 1) => (c1.expect("No value").0, c2.expect("No value").0),
                    (2, 1, 2) => (c1.expect("No value").0, c3.expect("No value").0),
                    (1, 2, 2) => (c3.expect("No value").0, c2.expect("No value").0),
                    _ => panic!("Match Hand type Double Pair"),
                };
                let v1 = *CARD_LABEL.get(a).unwrap();
                let v2 = *CARD_LABEL.get(b).unwrap();
                HandType::TwoPair(max(v1, v2), min(v1, v2))
            }
        }
        4 => {
            if let Some(key) = get_key_by_value(&counter, 2) {
                HandType::OnePair(*CARD_LABEL.get(&key).unwrap())
            } else {
                panic!("Match Hand type Pair")
            }
        }
        5 => {
            let greater = [
                *CARD_LABEL.get(c1.unwrap().0).unwrap(),
                *CARD_LABEL.get(c2.unwrap().0).unwrap(),
                *CARD_LABEL.get(c3.unwrap().0).unwrap(),
                *CARD_LABEL.get(c4.unwrap().0).unwrap(),
                *CARD_LABEL.get(c5.unwrap().0).unwrap(),
            ]
            .iter()
            .cloned()
            .max()
            .expect("Failed to find better hand");
            HandType::HighCard(greater)
        }
        _ => panic!("Match Hand type failed"),
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

    println!("{:#?}",hands);

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
        assert_eq!(result, Some(7830));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_hand() {
        assert_eq!(solve_hand("AAAAA"), HandType::FiveOfkind(14));
        assert_eq!(solve_hand("AAAA2"), HandType::FourOfkind(14, 2));
        assert_eq!(solve_hand("22223"), HandType::FourOfkind(2, 3));
        assert_eq!(solve_hand("AAAKK"), HandType::FullHouse(14, 13));
        assert_eq!(solve_hand("AAA32"), HandType::ThreeOfkind(14));
        assert_eq!(solve_hand("77752"), HandType::ThreeOfkind(7));
        assert_eq!(solve_hand("AA2KK"), HandType::TwoPair(14, 13));
        assert_eq!(solve_hand("AA52T"), HandType::OnePair(14));
        assert_eq!(solve_hand("23456"), HandType::HighCard(6));

        assert_eq!(
            solve_hand("AAAAA").cmp(&solve_hand("KKKKK")),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            solve_hand("AAAAA").cmp(&solve_hand("AAAAA")),
            std::cmp::Ordering::Equal
        );
        assert_eq!(
            solve_hand("22222").cmp(&solve_hand("AAAAA")),
            std::cmp::Ordering::Less
        );

        assert_eq!(
            solve_hand("AAA22").cmp(&solve_hand("AAA66")),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            solve_hand("AAK22").cmp(&solve_hand("AAK66")),
            std::cmp::Ordering::Less
        );

        assert_eq!(
            solve_hand("AA234").cmp(&solve_hand("TTKQ2")),
            std::cmp::Ordering::Greater
        );

        assert_eq!(
            solve_hand("KK677").cmp(&solve_hand("KTJJT")),
            std::cmp::Ordering::Greater
        );
    }
}
