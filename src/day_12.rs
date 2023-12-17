use advent_of_code::parse::{parsers, Parser};
use std::{collections::HashMap, iter::zip, mem::take};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Working,
    Broken,
    Unknown,
}

impl Spring {
    fn matches(self, other: Self) -> bool {
        match (self, other) {
            (Self::Broken, Self::Broken)
            | (Self::Working, Self::Working)
            | (Self::Unknown, _)
            | (_, Self::Unknown) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Island(Vec<Spring>);

impl Island {
    fn empty() -> Self {
        Island(vec![])
    }

    fn matches(&self, other: &Self) -> bool {
        zip(self.0.iter(), other.0.iter()).all(|(a, b)| a.matches(*b))
    }

    fn all_working(len: usize) -> Self {
        Island(vec![Spring::Working; len as usize])
    }

    fn append_spring_type(&mut self, spring: Spring, len: usize) {
        self.0.extend(vec![spring; len]);
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn tail(&self, to_skip: usize) -> Self {
        Island(self.0.iter().skip(to_skip).copied().collect())
    }

    fn count_spring_arrangements(&self, validation: Vec<usize>) -> u64 {
        if validation.is_empty() {
            let option = Island::all_working(self.len());
            if option.matches(self) {
                return 1;
            } else {
                return 0;
            }
        }
        let mut spring_arrangements_cache: HashMap<(usize, usize), u64> = HashMap::new();
        for validation_idx in (0..validation.len()).rev() {
            for spring_idx in (0..self.0.len()).rev() {
                let key = (validation_idx, spring_idx);
                let next_key = (validation_idx, spring_idx + 1);
                let if_skip = if self.0[spring_idx].matches(Spring::Working) {
                    spring_arrangements_cache
                        .get(&next_key)
                        .copied()
                        .unwrap_or(0)
                } else {
                    0
                };
                let group_len = validation[validation_idx];
                let tail = self.tail(spring_idx);

                if validation_idx == validation.len() - 1 {
                    let to_insert = if tail.len() < group_len {
                        0
                    } else {
                        let mut desired = Island::empty();
                        desired.append_spring_type(Spring::Broken, group_len);
                        desired.append_spring_type(Spring::Working, tail.len() - group_len);
                        if tail.matches(&desired) {
                            1
                        } else {
                            0
                        }
                    } + if_skip;
                    spring_arrangements_cache.insert(key, to_insert);
                } else {
                    let to_insert = if tail.len() < group_len + 2 {
                        0
                    } else {
                        let mut desired = Island::empty();
                        desired.append_spring_type(Spring::Broken, group_len);
                        desired.append_spring_type(Spring::Working, 1);

                        let after = spring_arrangements_cache
                            .get(&(validation_idx + 1, spring_idx + group_len + 1))
                            .unwrap();
                        if tail.matches(&desired) {
                            *after
                        } else {
                            0
                        }
                    } + if_skip;
                    spring_arrangements_cache.insert(key, to_insert);
                }
            }
        }
        *(spring_arrangements_cache.get(&(0, 0)).unwrap())
    }
}

#[derive(Debug)]
struct SpringField(Vec<Island>);

impl FromIterator<Spring> for SpringField {
    fn from_iter<T: IntoIterator<Item = Spring>>(iter: T) -> Self {
        let mut islands: Vec<Island> = Vec::new();
        let mut island: Vec<Spring> = Vec::new();
        for spring in iter {
            if spring != Spring::Working {
                island.push(spring);
            } else if !island.is_empty() {
                islands.push(Island(take(&mut island)));
            }
        }
        if !island.is_empty() {
            islands.push(Island(island));
        }
        SpringField(islands)
    }
}

impl SpringField {
    fn count_spring_arrangements(&self, validation: Vec<usize>) -> u64 {
        let mut spring_arrangements_cache: HashMap<(usize, usize), u64> = HashMap::new();
        for validation_idx in (0..=validation.len()).rev() {
            spring_arrangements_cache.insert(
                (validation_idx, self.0.len()),
                if validation_idx == validation.len() {
                    1
                } else {
                    0
                },
            );
            for island_idx in (0..self.0.len()).rev() {
                let key = (validation_idx, island_idx);
                let to_insert = (validation_idx..=(validation.len()))
                    .map(|next_validation_idx| {
                        let island_count = self.0[island_idx].count_spring_arrangements(
                            validation[validation_idx..next_validation_idx]
                                .iter()
                                .copied()
                                .collect(),
                        );
                        let next_count = spring_arrangements_cache
                            .get(&(next_validation_idx, island_idx + 1))
                            .copied()
                            .unwrap_or(0);
                        island_count * next_count
                    })
                    .sum();
                spring_arrangements_cache.insert(key, to_insert);
            }
        }
        *(spring_arrangements_cache.get(&(0, 0)).unwrap())
    }

    // fn count_spring_arrangements_old(&self, validation: Vec<usize>) -> u64 {
    //     println!("{:?} {:?}", self, validation);
    //     if self.0.is_empty() {
    //         if validation.is_empty() {
    //             return 1;
    //         } else {
    //             return 0;
    //         }
    //     } else if self.0.len() == 1 {
    //         return self.0[0].count_spring_arrangements(validation);
    //     }
    //     let mut total_count = 0;
    //     let rest_self = SpringField(self.0.iter().skip(1).cloned().collect());
    //     for count in 0..=validation.len() {
    //         let first: Vec<usize> = validation[0..count].iter().copied().collect();
    //         let first_island_count = self.0[0].count_spring_arrangements(first);
    //         if first_island_count > 0 {
    //             let rest_count = rest_self
    //                 .count_spring_arrangements(validation[count..].iter().copied().collect());
    //             total_count += first_island_count * rest_count;
    //         }
    //     }
    //     total_count
    // }
}

fn parse(input: &str, duplicate_count: usize) -> impl Iterator<Item = (SpringField, Vec<usize>)> {
    parsers::tag_replace(".", Spring::Working)
        .or(parsers::tag_replace("#", Spring::Broken))
        .or(parsers::tag_replace("?", Spring::Unknown))
        .many()
        .map(|i| {
            let springs: Vec<Spring> = i.collect();
            let total_count = springs.len() * duplicate_count + duplicate_count - 1;
            springs
                .into_iter()
                .chain(vec![Spring::Unknown])
                .cycle()
                .take(total_count)
                .collect()
        })
        .skip_tag(" ")
        .and_then(parsers::number().list(",").map(|l| {
            let validation: Vec<usize> = l.collect();
            let total_count = validation.len() * duplicate_count;
            validation.into_iter().cycle().take(total_count).collect()
        }))
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u64 {
    parse(input, 1)
        .map(|(springs, validation)| springs.count_spring_arrangements(validation))
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u64 {
    parse(input, 5)
        .map(|(springs, validation)| springs.count_spring_arrangements(validation))
        .enumerate()
        .map(|(idx, v)| {
            println!("{} {}", idx, v);
            v
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

    const EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
    const DAY: Day = Day::Day12;

    #[test]
    fn simple_example() {
        assert_eq!(
            part1(
                ".?.????.?..??#???# 2,1,1,1,1,2
"
            ),
            1
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 525152);
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
            7843
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
            10153896718999
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
