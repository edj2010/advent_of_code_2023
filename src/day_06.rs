use advent_of_code::parse::{parsers, Parser};

use std::iter::zip;

fn parse1(input: &str) -> (impl Iterator<Item = u64>, impl Iterator<Item = u64>) {
    parsers::tag("Time:")
        .ignore_and_then(
            parsers::many_chars(|c| c == ' ')
                .ignore_and_then(parsers::number())
                .many(),
        )
        .skip_tag("\n")
        .and_then(
            parsers::tag("Distance:").ignore_and_then(
                parsers::many_chars(|c| c == ' ')
                    .ignore_and_then(parsers::number())
                    .many(),
            ),
        )
        .skip_tag("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

fn parse2(input: &str) -> (u64, u64) {
    parsers::tag("Time:")
        .ignore_and_then(parsers::number_with_seps(" "))
        .skip_tag("\n")
        .and_then(parsers::tag("Distance:").ignore_and_then(parsers::number_with_seps(" ")))
        .skip_tag("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let (times, distances) = parse1(input);
    zip(times, distances)
        .map(|(time, distance)| {
            (1..time)
                .filter(|hold| (time - hold) * hold > distance)
                .count() as u32
        })
        .product()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u64 {
    let (time, distance) = parse2(input);
    let mut min = 1;
    let mut max = time / 2;
    while min + 1 < max {
        let mid = (min + max) / 2;
        if (mid + 1) * (time - mid - 1) < distance {
            min = mid;
        } else if mid * (time - mid) > distance {
            max = mid;
        } else {
            return time - 2 * mid - 1;
        }
    }
    0
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200
";
    const DAY: Day = Day::Day06;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 288);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 71503);
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
            1731600
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
            40087680
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
