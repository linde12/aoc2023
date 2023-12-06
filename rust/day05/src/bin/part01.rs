use anyhow::Result;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input01.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

#[derive(Debug)]
struct Range {
    src: usize,
    dst: usize,
    range: usize,
}

fn find_location(seed: usize, ranges: &Vec<Range>) -> usize {
    for range in ranges {
        let diff = range.dst as isize - range.src as isize;
        if seed >= range.src && seed <= (range.src + range.range) {
            return (seed as isize + diff) as usize;
        }
    }
    seed
}

fn process(input: &str) -> Result<String> {
    let (seeds_raw, maps_raw) = input.split_once("\n\n").unwrap();
    let maps = maps_raw
        .split("\n\n")
        .map(|map| {
            let ranges = map
                .lines()
                .skip(1)
                .map(|line| {
                    let mut iter = line.split(" ");
                    let dst = iter.next().unwrap().parse::<usize>().unwrap();
                    let src = iter.next().unwrap().parse::<usize>().unwrap();
                    let range = iter.next().unwrap().parse::<usize>().unwrap();

                    Range { src, dst, range }
                })
                .collect_vec();

            ranges
        })
        .collect_vec();

    let seeds = seeds_raw
        .split_once("seeds: ")
        .unwrap()
        .1
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    let min = seeds
        .into_iter()
        .map(|seed| {
            maps.iter()
                .fold(seed, |prev, values| find_location(prev, values))
        })
        .min()
        .unwrap();

    Ok(min.to_string())
}

#[test]
fn it_works() {
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
    assert_eq!(process(input).unwrap(), "35".to_owned());
}
