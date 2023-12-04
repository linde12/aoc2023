use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input01.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

fn resolve_digit_at(line: &Vec<char>, mut x: usize) -> usize {
    // fugly; am i being stupid?
    loop {
        if let Some(c) = line.get(x.wrapping_sub(1)) {
            if c.is_ascii_digit() {
                x -= 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    let digit: String = line
        .iter()
        .skip(x)
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digit.parse().unwrap()
}

fn process(input: &str) -> Result<String> {
    let matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut total = 0;

    for (y, line) in matrix.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if !c.is_ascii_digit() && *c != '.' {
                #[rustfmt::skip]
                let checks: [(isize, isize); 8] = [
                    (-1, -1), (-1, 0), (-1, 1),
                    (0, -1), /*(y, x),*/ (0, 1),
                    (1, -1), (1, 0), (1, 1),
                ];

                let parts: HashSet<usize> = checks
                    .into_iter()
                    .map(|(dy, dx)| (y.wrapping_add(dy as usize), x.wrapping_add(dx as usize)))
                    .filter_map(|(y, x)| {
                        if let Some(c) = matrix.get(y).and_then(|line| line.get(x)) {
                            if c.is_ascii_digit() {
                                return Some(resolve_digit_at(&matrix[y], x));
                            }
                            return None;
                        }
                        None
                    })
                    .collect();
                total += parts.iter().sum::<usize>();
            }
        }
    }

    Ok(total.to_string())
}

#[test]
fn it_works() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    assert_eq!(process(input).unwrap(), "4361".to_owned());
}
