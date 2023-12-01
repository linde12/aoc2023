use std::collections::HashMap;

use anyhow::Result;

fn main() {
    let input = include_str!("./input02.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_first_digit(bytes: &[u8], iterable: impl IntoIterator<Item = usize>) -> usize {
    iterable
        .into_iter()
        .find_map(|i| {
            // char at index i is a digit (1,2,3,4) - return the digit
            if bytes[i].is_ascii_digit() {
                return Some((bytes[i] as char).to_digit(10).unwrap() as usize);
            }

            // see if the remaining characters starts with one of the string digits
            for (value, digit) in DIGITS.into_iter().enumerate() {
                if bytes[i..].starts_with(digit.as_bytes()) {
                    return Some(value + 1);
                }
            }
            None
        })
        .unwrap()
}

fn process(input: &str) -> Result<String> {
    let sum: usize = input
        .lines()
        .map(|l| {
            let bytes = l.as_bytes();
            let digit1 = 10 * find_first_digit(bytes, 0..bytes.len());
            let digit2 = find_first_digit(bytes, (0..bytes.len()).rev());

            digit1 + digit2
        })
        .sum();

    Ok(sum.to_string())
}

#[test]
fn it_works() {
    let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
    assert_eq!(process(input).unwrap(), "281".to_owned());
}
