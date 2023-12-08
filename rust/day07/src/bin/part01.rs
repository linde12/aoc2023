use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input01.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

const CARD_VALUES: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

struct Hand {
    hand_type: HandType,
    values: (u8, u8, u8, u8, u8),
    bet: usize,
}

impl Hand {
    fn new(cards: &str, bet: &str) -> Self {
        let bet: usize = bet.parse().unwrap();
        let v = cards
            .chars()
            .map(|c| CARD_VALUES.iter().position(|c2| c2 == &c).unwrap() as u8)
            .collect_vec();
        assert_eq!(v.len(), 5, "must have exactly 5 cards");
        let hand_type = Self::get_hand_type(&v);

        Self {
            bet,
            values: (v[0], v[1], v[2], v[3], v[4]),
            hand_type,
        }
    }

    fn get_hand_type(cards: &Vec<u8>) -> HandType {
        let mut occurrences = HashMap::new();
        cards.iter().for_each(|card| {
            occurrences.entry(card).and_modify(|e| *e += 1).or_insert(1);
        });
        let occurrences = occurrences.into_values().sorted().rev().collect_vec();
        let first = *occurrences.get(0).unwrap_or(&0);
        let second = *occurrences.get(1).unwrap_or(&0);

        match first {
            5 => HandType::FiveKind,
            4 => HandType::FourKind,
            3 => {
                if second == 2 {
                    // second most common card is 2, triplets + pair = full house
                    HandType::FullHouse
                } else {
                    HandType::ThreeKind
                }
            }
            2 => {
                if second == 2 {
                    // second most common card is 2, pair + pair = two pairs
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        }
    }
}

fn process(input: &str) -> Result<String> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|l| {
            let (cards, bet) = l.split_once(" ").unwrap();
            Hand::new(cards, bet)
        })
        .collect();

    // sort by hand_type then values
    hands.sort_unstable_by_key(|hand| (hand.hand_type, hand.values));

    let winnings: usize = hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| hand.bet * (rank + 1))
        .sum();

    Ok(winnings.to_string())
}

#[test]
fn it_works() {
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    assert_eq!(process(input).unwrap(), "6440".to_owned());
}
