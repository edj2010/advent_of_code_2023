use advent_of_code::{
    interval::{DisjointIntervalUnion, Interval, IntervalBound},
    parse::{parsers, Parser},
};

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RangeMapElement {
    from: Interval<i64>,
    adjustment: i64,
}

impl RangeMapElement {
    fn contains_from(self, n: i64) -> bool {
        self.from.contains(&n)
    }

    fn convert(self, n: i64) -> i64 {
        if self.contains_from(n) {
            n + self.adjustment
        } else {
            n
        }
    }
}

#[derive(Debug)]
struct RangeMap {
    ranges: BTreeMap<i64, RangeMapElement>,
}

impl RangeMap {
    fn convert(&self, n: i64) -> i64 {
        self.ranges
            .range(..=n)
            .last()
            .map(|(_, element)| element.convert(n))
            .unwrap_or(n)
    }

    fn convert_range(&self, range: Interval<i64>) -> DisjointIntervalUnion<i64> {
        let unchanged = self.ranges.iter().fold(
            DisjointIntervalUnion::singleton(range),
            |unchanged, (_, range)| unchanged - range.from,
        );

        self.ranges
            .iter()
            .filter_map(|(_, element)| {
                Some(range.intersection(&element.from)? + element.adjustment)
            })
            .collect::<DisjointIntervalUnion<i64>>()
            | unchanged
    }
}

fn parse(
    input: &str,
) -> (
    impl Iterator<Item = i64>,
    impl Iterator<Item = (String, RangeMap)>,
) {
    parsers::tag("seeds: ")
        .ignore_and_then(parsers::number::<i64>().list(" "))
        .skip_tag("\n\n")
        .and_then(
            parsers::many_chars(|c| c != ' ')
                .skip_tag(" map:\n")
                .and_then(
                    parsers::number::<i64>()
                        .skip_tag(" ")
                        .and_then(parsers::number::<i64>().skip_tag(" "))
                        .and_then(parsers::number::<i64>())
                        .map(|((a, b), c)| {
                            (
                                b,
                                RangeMapElement {
                                    from: Interval::new(
                                        IntervalBound::Inclusive(b),
                                        IntervalBound::Inclusive(b + c - 1),
                                    ),
                                    adjustment: a - b,
                                },
                            )
                        })
                        .many_lines("\n")
                        .map(|elements| RangeMap {
                            ranges: elements.collect(),
                        }),
                )
                .list("\n"),
        )
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let (seeds, maps) = parse(input);
    let maps: Vec<RangeMap> = maps.map(|(_, range_map)| range_map).collect();
    seeds
        .map(|seed| maps.iter().fold(seed, |value, map| map.convert(value)))
        .min()
        .unwrap()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let (seeds, maps) = parse(input);
    let maps: Vec<RangeMap> = maps.map(|(_, range_map)| range_map).collect();
    let seeds: Vec<i64> = seeds.collect();
    let mut seed_ranges: Vec<Interval<i64>> = Vec::new();
    for idx in (0..seeds.len()).step_by(2) {
        seed_ranges.push(Interval::new(
            IntervalBound::Inclusive(seeds[idx]),
            IntervalBound::Inclusive(seeds[idx] + seeds[idx + 1] - 1),
        ));
    }

    seed_ranges
        .iter()
        .map(|&seed_range| {
            maps.iter()
                .fold(
                    DisjointIntervalUnion::singleton(seed_range),
                    |ranges: DisjointIntervalUnion<i64>, map| {
                        ranges
                            .into_iter()
                            .map(|range| map.convert_range(range))
                            .collect()
                    },
                )
                .lower_bound()
                .copied()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "seeds: 79 14 55 13

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
56 93 4
";
    const DAY: Day = Day::Day05;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 46);
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY
            )),
            175622908
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY
            )),
            5200543
        );
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        b.iter(|| {
            part1(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY,
            ))
        });
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        b.iter(|| {
            part2(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY,
            ))
        });
    }
}
