use std::collections::BTreeMap;

use advent_of_code::parse::{parsers, Parser};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    Red,
    Blue,
    Green,
}

fn parse(
    input: &str,
) -> impl Iterator<
    Item = (
        u32,
        impl Iterator<Item = impl Iterator<Item = (u32, Color)>>,
    ),
> {
    parsers::tag("Game ")
        .ignore_and_then(parsers::number())
        .skip_tag(": ")
        .and_then(
            parsers::number()
                .skip_tag(" ")
                .and_then(
                    parsers::tag_replace("blue", Color::Blue)
                        .or(parsers::tag_replace("green", Color::Green))
                        .or(parsers::tag_replace("red", Color::Red)),
                )
                .list(", ")
                .list("; "),
        )
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let desired_maxima: BTreeMap<Color, u32> =
        BTreeMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

    parse(input)
        .filter_map(|(id, mut game)| {
            if game.all(|mut round| {
                round.all(|(count, color)| desired_maxima.get(&color).unwrap_or(&0) >= &count)
            }) {
                Some(id)
            } else {
                None
            }
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    parse(input)
        .map(|(_, game)| {
            let mut minima = BTreeMap::from([(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)]);
            for round in game {
                for (count, color) in round {
                    minima.entry(color).and_modify(|e| *e = u32::max(*e, count));
                }
            }
            minima.values().product::<u32>()
        })
        .sum()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    const DAY: Day = Day::Day02;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 2286);
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
            1931
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
            83105
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
