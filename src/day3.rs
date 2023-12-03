use advent_of_code::{
    grid::{Grid, GridPoint, ADJACENT, EAST, SOUTH},
    parse::{parsers, Parser},
};

use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> Vec<Vec<char>> {
    parsers::many_chars(|c| c != '\n')
        .map(|s| s.bytes().map(|b| b as char).collect::<Vec<char>>())
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
        .collect()
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let diagram: Grid<char> = Grid::of_vec_of_vecs(parse(input)).unwrap();
    let dimensions = diagram.dimensions();
    let mut sum = 0;
    for row_start in GridPoint::new(0_usize, 0).traverse_by(SOUTH, dimensions) {
        let mut current_number = 0;
        let mut important_number = false;
        for char_idx in row_start.traverse_by(EAST, dimensions) {
            let digit = diagram.get(char_idx).unwrap();
            if digit.is_numeric() {
                current_number = 10 * current_number + digit.to_digit(10).unwrap();
                for adjacent_offset in ADJACENT {
                    if let Some(offset_idx) = char_idx.add_checked(adjacent_offset, &dimensions) {
                        let adjacent_digit = diagram.get(offset_idx).unwrap();
                        important_number |= *adjacent_digit != '.' && !adjacent_digit.is_digit(10);
                    }
                }
            } else {
                if important_number {
                    sum += current_number;
                }
                current_number = 0;
                important_number = false;
            }
        }
        if important_number {
            sum += current_number;
        }
    }
    sum
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let diagram: Grid<char> = Grid::of_vec_of_vecs(parse(input)).unwrap();
    let mut gears: HashMap<GridPoint<usize>, Vec<u32>> = HashMap::new();
    let dimensions = diagram.dimensions();
    for row_start in GridPoint::new(0_usize, 0).traverse_by(SOUTH, dimensions) {
        let mut current_number = 0;
        let mut adjacent_gears: HashSet<GridPoint<usize>> = HashSet::new();
        for char_idx in row_start.traverse_by(EAST, dimensions) {
            let digit = diagram.get(char_idx).unwrap();
            if digit.is_numeric() {
                current_number = 10 * current_number + digit.to_digit(10).unwrap();
                for adjacent_offset in ADJACENT {
                    if let Some(offset_idx) = char_idx.add_checked(adjacent_offset, &dimensions) {
                        if *diagram.get(offset_idx).unwrap() == '*' {
                            adjacent_gears.insert(offset_idx);
                        }
                    }
                }
            } else {
                for gear in adjacent_gears {
                    gears.entry(gear).or_default().push(current_number);
                }
                adjacent_gears = HashSet::new();
                current_number = 0;
            }
        }
        for gear in adjacent_gears {
            gears.entry(gear).or_default().push(current_number);
        }
    }
    gears
        .values()
        .into_iter()
        .map(|part_numbers| {
            if part_numbers.len() == 2 {
                part_numbers[0] * part_numbers[1]
            } else {
                0
            }
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

    const EXAMPLE: &str = "...467..114..
......*......
.....35..633.
.........#...
...617*......
........+.58.
.....592.....
.........755.
......$.*....
....664.598..
";
    const DAY: Day = Day::Day03;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 4361);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 467835);
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
            520019
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
            75519888
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
