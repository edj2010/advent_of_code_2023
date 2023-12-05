use advent_of_code::parse::{parsers, Parser};
use std::{cmp::min, collections::HashSet};

fn parse(input: &str) -> impl Iterator<Item = (u32, HashSet<u32>, Vec<u32>)> {
    parsers::tag("Card ")
        .ignore_and_then(parsers::many_chars(|c| c == ' ').ignore_and_then(parsers::number()))
        .skip_tag(": ")
        .and_then(
            parsers::char(' ')
                .maybe()
                .ignore_and_then(parsers::number())
                .list(" ")
                .map(|iter| iter.collect::<HashSet<u32>>()),
        )
        .skip_tag(" | ")
        .and_then(
            parsers::char(' ')
                .maybe()
                .ignore_and_then(parsers::number())
                .list(" ")
                .map(|iter| iter.collect::<Vec<u32>>()),
        )
        .map(|((a, b), c)| (a, b, c))
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    parse(input)
        .map(|(_, winning_numbers, card_numbers)| {
            card_numbers.iter().fold(0, |acc, num| {
                if !winning_numbers.contains(num) {
                    acc
                } else if acc == 0 {
                    1
                } else {
                    acc * 2
                }
            })
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let card_scores: Vec<usize> = parse(input)
        .map(|(_, winning_numbers, card_numbers)| {
            card_numbers
                .iter()
                .filter(|num| winning_numbers.contains(num))
                .count()
        })
        .collect();
    let mut instances = vec![1; card_scores.len()];
    for (idx, score) in card_scores.iter().enumerate() {
        for later_card in (idx + 1)..=(min(idx + score, instances.len() - 1)) {
            instances[later_card] += instances[idx];
        }
    }
    instances.iter().sum()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    const DAY: Day = Day::Day04;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 30);
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
            25004
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
            14427616
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
