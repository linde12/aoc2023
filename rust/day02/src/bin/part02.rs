use std::collections::HashMap;

use anyhow::Result;

fn main() {
    let input = include_str!("./input02.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Set {
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl From<HashMap<&str, usize>> for Set {
    fn from(value: HashMap<&str, usize>) -> Self {
        Set {
            red: value["red"],
            green: value["green"],
            blue: value["blue"],
        }
    }
}

// 12 red cubes, 13 green cubes, and 14 blue cubes
fn process(input: &str) -> Result<String> {
    let sum: usize = input
        .lines()
        .map(|l| {
            let (_, sets) = l.split_once(": ").unwrap();
            let mut colors: HashMap<&str, usize> =
                HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

            sets.split("; ")
                .flat_map(|set| set.split(", "))
                .flat_map(|gem| gem.split_once(" "))
                .for_each(|(quantity, color)| {
                    colors.entry(color).and_modify(|e| {
                        let qty = quantity.parse::<usize>().unwrap();
                        *e = std::cmp::max(qty, *e);
                    });
                });
            Set::from(colors).power()
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
    assert_eq!(process(input).unwrap(), "2286".to_owned());
}
