use std::cmp;
use std::ops::Range;

use anyhow::Result;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input02.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

#[derive(Debug)]
struct Transform {
    src: Range<isize>,
    dst: Range<isize>,
}

impl Transform {
    fn intersection(&self, seed: &Range<isize>) -> Range<isize> {
        cmp::max(seed.start, self.src.start)..cmp::min(seed.end, self.src.end)
    }
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
                    let dst = iter.next().unwrap().parse::<isize>().unwrap();
                    let src = iter.next().unwrap().parse::<isize>().unwrap();
                    let len = iter.next().unwrap().parse::<isize>().unwrap();

                    Transform {
                        src: (src..src + len),
                        dst: (dst..dst + len),
                    }
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
        .map(|s| s.parse::<isize>().unwrap())
        .collect_vec();

    let mut ranges = seeds
        .chunks(2)
        .map(|chunk| (chunk[0]..chunk[0] + chunk[1]))
        .collect_vec();

    for section in maps {
        ranges = ranges
            .into_iter()
            .flat_map(|seed| transform_seed(seed, &section))
            .collect();
    }
    let min = ranges.iter().map(|range| range.start).min().unwrap();
    Ok(min.to_string())
}

fn transform_seed(seed: Range<isize>, section: &Vec<Transform>) -> Vec<Range<isize>> {
    let mut processed = vec![];
    let mut to_process = vec![seed];

    // Idea is to check if the seed range intersects with any of the transformations of this
    // section. If no intersection is found for any range of this section, we don't translate and
    // simply add the seed to "processed" which will later be processed by the next layer/section.
    //
    // If a intersection is found, the intersecting range is translated and added to "processed".
    //
    // Then we also check if the seed started before and/or after the intersection, and try to find
    // intersecting transformations for those ranges too.
    //
    // E.g.
    // ......|-------------| (seed)
    // ........|---------| (transform1)
    // ..|-----| (transform2)
    // ....................|-----| (transform3)
    //
    // Entire intersection is translated and added to processed. Start of seed to start of
    // transform1.src is translated and later processed by transform2. End of transform1 to end of
    // seed is translated and later processed by transform3.
    //
    // The intresection in transform2 has no more matches in this section and is put in "processed"
    // The intresection in transform3 has no more matches in this section and is put in "processed"
    //
    // We end up with three ranges that match three transforms. Later we check these ranges in the
    // next layer until we've gone through all intersecting ranges in all layers. Then it is just a
    // matter of finding the lowest range.start and that is the lowest/closest location.
    while let Some(seed) = to_process.pop() {
        let maybe_transformation = section.iter().find(|transformation| {
            let intersection = transformation.intersection(&seed);
            !intersection.is_empty()
        });

        if maybe_transformation.is_none() {
            // no intersecting transformation found for this section, so not
            // transformation/translation to do - pass on to next layer
            processed.push(seed);
            continue;
        }

        let transformation = maybe_transformation.unwrap();
        let offset = transformation.dst.start - transformation.src.start;

        // Translate the parts of the ranges that overlapped (the intersection)
        // transform src -> dst
        let intersection = transformation.intersection(&seed);
        processed.push(intersection.start + offset..intersection.end + offset);

        // Handle case where seed starts before intesection. Make a range from seed start to
        // transformation start and handle as a new seed to process through the current section.
        // This could potentially match more transforms in the current section.
        // ....|-----| (transform src)
        // ..|----| (seed)
        if seed.start < transformation.src.start {
            to_process.push(seed.start..transformation.src.start - 1)
        }

        // Handle case where seed ends after intesection. Make a range from transformation end to
        // seed end and handle as a new seed to process through the current section.
        // This could potentially match more transforms in the current section.
        // ..|----| (transform src)
        // ....|-----| (seed)
        if seed.end > transformation.src.end {
            to_process.push(transformation.src.end + 1..seed.end)
        }
    }

    processed
}

// 98..100 => 50..52
// 50..98 => 52..100
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
