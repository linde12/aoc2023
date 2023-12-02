use std::collections::HashMap;

use anyhow::Result;

fn main() {
    let input = include_str!("./input01.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Set {
    fn possible(&self) -> bool {
        return self.red <= 12 && self.green <= 13 && self.blue <= 14;
    }
}

impl From<HashMap<&str, usize>> for Set {
    fn from(mut value: HashMap<&str, usize>) -> Self {
        Set {
            red: value.remove("red").unwrap_or(0),
            green: value.remove("green").unwrap_or(0),
            blue: value.remove("blue").unwrap_or(0),
        }
    }
}

// 12 red cubes, 13 green cubes, and 14 blue cubes
fn process(input: &str) -> Result<String> {
    let sum: usize = input
        .lines()
        .filter_map(|l| {
            let (id, sets) = l.split_once(": ").unwrap();
            let ok = sets.split("; ").all(|set| {
                // populate hashmap with quantity for each color for the set
                let colors: HashMap<&str, usize> = set
                    .split(", ")
                    .flat_map(|gem| gem.split_once(" "))
                    .map(|(quantity, color)| (color, quantity.parse().unwrap()))
                    .collect();

                Set::from(colors).possible()
            });

            // if all sets are ok, return the game id
            ok.then(|| id.split_once(" ").unwrap().1.parse::<usize>().unwrap())
        })
        .sum();

    Ok(sum.to_string())
}

#[test]
fn it_works() {
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    assert_eq!(process(input).unwrap(), "8".to_owned());
}
