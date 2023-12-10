use advent_of_code::{
    itertools::Itertools,
    number_theory::chinese_remainder::chinese_remainder_many_with_modulus,
    parse::{parsers, Parser},
};
use std::collections::HashMap;

fn parse(input: &str) -> (String, HashMap<String, (String, String)>) {
    parsers::many_chars(|c| c.is_alphabetic())
        .skip_tag("\n\n")
        .and_then(
            parsers::many_chars(|c| c.is_alphanumeric())
                .skip_tag(" = (")
                .and_then(
                    parsers::many_chars(|c| c.is_alphanumeric())
                        .skip_tag(", ")
                        .and_then(parsers::many_chars(|c| c.is_alphanumeric())),
                )
                .skip_tag(")")
                .many_lines("\n")
                .map(|iter| iter.collect::<HashMap<String, (String, String)>>()),
        )
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let (instructions, map) = parse(input);
    let mut current = "AAA";
    let mut steps = 0;
    for instruction in instructions.chars().cycle() {
        if current == "ZZZ" {
            break;
        }
        let (l, r) = map.get(current).unwrap();
        current = if instruction == 'R' { r } else { l };
        steps += 1;
    }
    steps
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let (instructions, map) = parse(input);
    let cycles = map
        .keys()
        .filter(|s| s.chars().last() == Some('A'))
        .map(|start| {
            let path = instructions.chars().enumerate().cycle().scan(
                start.clone(),
                |current, (instruction_idx, instruction)| {
                    let (l, r) = map.get(current).unwrap();
                    *current = if instruction == 'R' {
                        r.clone()
                    } else {
                        l.clone()
                    };
                    Some((instruction_idx, current.clone()))
                },
            );
            let distance_to_cycle = path.clone().distance_to_cycle().unwrap();
            let cycle_length = path.clone().cycle_length().unwrap();
            let possible_terminals: Vec<(i64, i64, i64)> = path
                .skip(distance_to_cycle)
                .take(cycle_length)
                .enumerate()
                .filter_map(|(idx, (_, node))| {
                    if node.chars().last() == Some('Z') {
                        Some((
                            distance_to_cycle as i64,
                            cycle_length as i64,
                            ((idx + 1) % cycle_length) as i64,
                        ))
                    } else {
                        None
                    }
                })
                .collect();
            possible_terminals
        })
        .fold(vec![vec![]], |possible_terminal_sets, next_terminals| {
            possible_terminal_sets
                .into_iter()
                .flat_map(|terminal_set: Vec<(i64, i64, i64)>| {
                    next_terminals
                        .iter()
                        .map(|terminal| {
                            let mut ret = terminal_set.clone();
                            ret.push(*terminal);
                            ret
                        })
                        .collect::<Vec<Vec<(i64, i64, i64)>>>()
                })
                .collect()
        });
    cycles
        .into_iter()
        .filter_map(|i| {
            chinese_remainder_many_with_modulus(i.into_iter().map(
                |(distance_to_cycle, cycle_length, cycle_idx)| {
                    ((cycle_idx + distance_to_cycle) % cycle_length, cycle_length)
                },
            ))
        })
        .map(|(mut remainder, modulus)| {
            while remainder <= 0 {
                remainder += modulus;
            }
            remainder
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

    const DAY: Day = Day::Day08;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"
            ),
            2
        );
        assert_eq!(
            part1(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"
            ),
            6
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"
            ),
            6
        );
        assert_eq!(
            part2(
                "LR

11A = (11D, XXX)
11D = (XXX, 11E)
11E = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"
            ),
            6
        );
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
            11309
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
            13740108158591
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
