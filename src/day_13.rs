use advent_of_code::{
    grid::Grid,
    parse::{parsers, Parser},
};

use std::iter::zip;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Ash,
    Rock,
}

fn parse(input: &str) -> impl Iterator<Item = Grid<Cell>> {
    parsers::tag_replace(".", Cell::Ash)
        .or(parsers::tag_replace("#", Cell::Rock))
        .many_at_least_one()
        .map(|i| i.collect::<Vec<Cell>>())
        .many_lines("\n")
        .map(|i| Grid::of_vec_of_vecs(i.collect()).unwrap())
        .list("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

// 0 - (4 + 0) / 2 = 2
// 1
// 2 - (4 + 2) / 2 = 3
// 3

fn symmetry_idx(rows: Vec<Vec<Cell>>) -> u32 {
    // equals first
    for (idx, row) in rows.iter().enumerate().skip(1).step_by(2) {
        let candidate_middle = (idx + 1) / 2;
        if row == &rows[0] && (1..candidate_middle).all(|jdx| rows[jdx] == rows[idx - jdx]) {
            return candidate_middle as u32;
        }
    }
    // equals last
    for (idx, row) in rows.iter().enumerate().rev().skip(1).step_by(2) {
        let candidate_middle = (idx + rows.len()) / 2;
        if row == &rows[rows.len() - 1]
            && (idx..candidate_middle).all(|jdx| rows[jdx] == rows[rows.len() + idx - jdx - 1])
        {
            return candidate_middle as u32;
        }
    }
    return 0;
}

fn smudge_symmetry_idx(rows: Vec<Vec<Cell>>) -> u32 {
    for candidate_middle in 1..rows.len() {
        let start_idx = if candidate_middle * 2 > rows.len() {
            candidate_middle * 2 - rows.len()
        } else {
            0
        };
        let distance: u32 = (start_idx..candidate_middle)
            .map(|idx| {
                zip(
                    rows[idx].iter(),
                    rows[2 * candidate_middle - idx - 1].iter(),
                )
                .filter(|(a, b)| a != b)
                .count() as u32
            })
            .sum();
        if distance == 1 {
            return candidate_middle as u32;
        }
    }
    return 0;
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    parse(input)
        .map(|grid| 100 * symmetry_idx(grid.all_rows()) + symmetry_idx(grid.all_cols()))
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    parse(input)
        .map(|grid| {
            100 * smudge_symmetry_idx(grid.all_rows()) + smudge_symmetry_idx(grid.all_cols())
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

    const EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
    const DAY: Day = Day::Day13;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 405);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 400);
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
            37381
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
            28210
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
