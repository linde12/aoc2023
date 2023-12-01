use anyhow::Result;

fn main() {
    let input = include_str!("./input01.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

fn process(input: &str) -> Result<String> {
    let sum: usize = input
        .lines()
        .map(|l| {
            let digit1 = l.chars().skip_while(|c| !c.is_digit(10)).next().unwrap();
            let digit2 = l
                .chars()
                .rev()
                .skip_while(|c| !c.is_digit(10))
                .next()
                .unwrap();

            let s = format!("{}{}", digit1, digit2);
            s.parse::<usize>().unwrap()
        })
        .sum();

    Ok(sum.to_string())
}

#[test]
fn it_works() {
    let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
    assert_eq!(process(input).unwrap(), "142".to_owned());
}
