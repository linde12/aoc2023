use anyhow::Result;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input01.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

fn process(input: &str) -> Result<String> {
    let lines = input
        .lines()
        .map(|l| {
            // FIXME: ugly parsing... just use regex already...
            let s: String = l.chars().skip_while(|c| !c.is_ascii_digit()).collect();
            s.split(" ")
                .filter_map(|s| {
                    let n: String = s
                        .chars()
                        .skip_while(|c| !c.is_ascii_digit())
                        .take_while(|c| c.is_ascii_digit())
                        .collect();
                    if !n.is_empty() {
                        return Some(n.parse::<usize>().unwrap());
                    }
                    None
                })
                .collect_vec()
        })
        .collect_vec();
    let races = lines[0].iter().zip(lines[1].iter()).collect_vec();

    let sum: usize = races
        .into_iter()
        .map(|(time, distance)| ways2win(*time, *distance))
        .product();

    Ok(sum.to_string())
}

fn ways2win(time: usize, distance: usize) -> usize {
    let mut ways_found = 0;
    for i in 1..=time {
        let distance_covered = i * (time - i);
        if distance_covered > distance {
            ways_found += 1;
        }
    }
    ways_found
}

#[test]
fn it_works() {
    let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
    assert_eq!(process(input).unwrap(), "288".to_owned());
}
