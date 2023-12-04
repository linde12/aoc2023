use std::collections::HashSet;

use anyhow::Result;

fn main() {
    let input = include_str!("./input01.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

fn double(mut v: usize, times: usize) -> usize {
    for _ in 0..times {
        v *= 2;
    }
    v
}

fn process(input: &str) -> Result<String> {
    let sum: usize = input
        .lines()
        .map(|l| {
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

            let matches = winning.intersection(&mine).count();
            if matches > 0 {
                double(1, matches - 1)
            } else {
                0
            }
        })
        .sum();

    Ok(sum.to_string())
}

#[test]
fn it_works() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    assert_eq!(process(input).unwrap(), "13".to_owned());
}
