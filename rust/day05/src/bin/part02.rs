use std::{collections::HashMap, ops::Range};

use anyhow::Result;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input02.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

#[derive(Debug)]
struct SeedRange {
    src: usize,
    dst: usize,
    len: usize,
}

fn find_location(seed: usize, ranges: &Vec<SeedRange>) -> usize {
    for range in ranges {
        let diff = range.dst as isize - range.src as isize;
        if seed >= range.src && seed <= (range.src + range.len) {
            return (seed as isize + diff) as usize;
        }
    }
    seed
}

fn process(input: &str) -> Result<String> {
    let (seeds_raw, maps_raw) = input.split_once("\n\n").unwrap();
    let mut maps = HashMap::new();

    maps_raw.split("\n\n").for_each(|map| {
        let (id, _) = map.lines().nth(0).unwrap().split_once(" ").unwrap();

        let ranges = map
            .lines()
            .skip(1)
            .map(|line| {
                let mut iter = line.split(" ");
                let dst = iter.next().unwrap().parse::<usize>().unwrap();
                let src = iter.next().unwrap().parse::<usize>().unwrap();
                let range = iter.next().unwrap().parse::<usize>().unwrap();

                SeedRange {
                    src,
                    dst,
                    len: range,
                }
            })
            .collect_vec();

        maps.insert(id, ranges);
    });

    let seeds = seeds_raw
        .split_once("seeds: ")
        .unwrap()
        .1
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    let closest = seeds
        .chunks(2)
        .flat_map(|c| (c[0]..c[0] + c[1]))
        .map(|seed| find_location(seed, maps.get("seed-to-soil").unwrap()))
        .map(|soil| find_location(soil, maps.get("soil-to-fertilizer").unwrap()))
        .map(|fertilizer| find_location(fertilizer, maps.get("fertilizer-to-water").unwrap()))
        .map(|water| find_location(water, maps.get("water-to-light").unwrap()))
        .map(|light| find_location(light, maps.get("light-to-temperature").unwrap()))
        .map(|temperature| find_location(temperature, maps.get("temperature-to-humidity").unwrap()))
        .map(|humidity| find_location(humidity, maps.get("humidity-to-location").unwrap()))
        .min()
        .unwrap();

    Ok(closest.to_string())
    // Ok("".to_string())
}

// seed: (79..93)
// soil: (98..100) => (50..52), (50..98) => (52..100)
// fert: (52..54) => (37..39)
// wate: (25..95) => (18..88)
// ligh: (77..100) => (45..68), (45..64) => (81..100), (64..77) => (68..81)
// temp: (69..70) => (0..1), (0..69) => (1..70)
// humi: (56..93) => (60..97)
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
    assert_eq!(process(input).unwrap(), "46".to_owned());
}
