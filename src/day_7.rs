use std::{cmp::Ordering, collections::BTreeMap, iter::zip};

use advent_of_code::{
    itertools::Itertools,
    parse::{parsers, Parser},
};

const CARDRANK: &'static str = "0123456789TJQKA";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    High,
    Pair,
    TwoPair,
    ThreeSet,
    FullHouse,
    FourSet,
    FiveSet,
}

impl HandType {
    pub fn classify_hand(hand: &str) -> Self {
        let counts = hand.chars().value_counts();
        match counts.keys().count() {
            1 => Self::FiveSet,
            2 => {
                if counts.values().contains(&&4) {
                    Self::FourSet
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                if counts.values().contains(&&3) {
                    Self::ThreeSet
                } else {
                    Self::TwoPair
                }
            }
            4 => Self::Pair,
            _ => Self::High,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: String,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: &str) -> Self {
        Hand {
            cards: cards.to_string(),
            hand_type: HandType::classify_hand(cards),
        }
    }

    fn new_with_wild(cards: &str, wild_card: char) -> Self {
        Hand {
            hand_type: CARDRANK
                .chars()
                .filter(|&c| c != wild_card)
                .map(|wild_target| {
                    Hand::new(
                        &cards
                            .chars()
                            .map(|c| if c == wild_card { wild_target } else { c })
                            .collect::<String>(),
                    )
                })
                .max()
                .map(|hand| hand.hand_type)
                .unwrap(),
            cards: cards
                .chars()
                .map(|c| if c == wild_card { '0' } else { c })
                .collect(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => zip(self.cards.chars(), other.cards.chars())
                .find_map(|(a, b)| {
                    if a == b {
                        None
                    } else {
                        CARDRANK
                            .chars()
                            .position(|c| c == a)
                            .partial_cmp(&CARDRANK.chars().position(|c| c == b))
                    }
                })
                .unwrap_or(Ordering::Equal),
            o => o,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn parse(input: &str) -> impl Iterator<Item = (String, u32)> {
    parsers::many_chars(|c| c.is_alphanumeric())
        .skip_tag(" ")
        .and_then(parsers::number::<u32>())
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    parse(input)
        .map(|(s, i)| (Hand::new(&s), i))
        .collect::<BTreeMap<Hand, u32>>()
        .into_iter()
        .enumerate()
        .map(|(idx, (_, value))| (idx as u32 + 1) * value)
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    parse(input)
        .map(|(s, i)| (Hand::new_with_wild(&s, 'J'), i))
        .collect::<BTreeMap<Hand, u32>>()
        .into_iter()
        .enumerate()
        .map(|(idx, (_, value))| (idx as u32 + 1) * value)
        .sum()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
    const DAY: Day = Day::Day07;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 6440);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 5905);
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
            253603890
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
            253630098
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
