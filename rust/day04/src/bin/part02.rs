use std::collections::{HashMap, HashSet};

use anyhow::Result;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input02.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

type Card = (HashSet<usize>, HashSet<usize>);

fn get_matches(card: &Card) -> usize {
    let (winning, mine) = card;
    winning.intersection(&mine).count()
}

fn process(input: &str) -> Result<String> {
    let originals = input
        .lines()
        .filter_map(|l| {
            let (_, numbers) = l.split_once(": ").unwrap();
            let (winning_raw, mine_raw) = numbers.split_once(" | ").unwrap();

            let winning: HashSet<_> = winning_raw
                .split(" ")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect();

            let mine: HashSet<_> = mine_raw
                .split(" ")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect();

            Some((winning, mine))
        })
        .collect_vec();

    let mut hand: HashMap<usize, usize> = (0..originals.len()).map(|i| (i, 1)).collect();

    for card_idx in 0..hand.len() {
        let card = &originals[card_idx];
        let amount = hand[&card_idx];
        let n_matches = get_matches(&card);

        for i in 0..n_matches {
            let next_idx = card_idx + i + 1;
            if hand.contains_key(&next_idx) {
                hand.entry(next_idx).and_modify(|e| *e += 1 * amount);
            }
        }
    }

    let n_cards = hand.values().sum::<usize>();
    Ok(n_cards.to_string())
}

#[test]
fn it_works() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    assert_eq!(process(input).unwrap(), "30".to_owned());
}
