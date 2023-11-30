use anyhow::Result;

fn main() {
    let input = include_str!("./input01.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

fn process(input: &str) -> Result<String> {
    Ok(input.into())
}

#[test]
fn it_works() {
    assert!(1 == 1);
}
