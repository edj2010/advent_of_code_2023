use std::ops::Add;

use advent_of_code::{
    grid::{Grid, GridPoint, GridPointDelta, EAST, NORTH, SOUTH, WEST},
    parse::{parsers, Parser},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Round,
    Square,
    Empty,
}

fn parse(input: &str) -> Grid<Cell> {
    parsers::tag_replace("#", Cell::Square)
        .or(parsers::tag_replace("O", Cell::Round))
        .or(parsers::tag_replace(".", Cell::Empty))
        .many()
        .map(|i| i.collect())
        .many_lines("\n")
        .map(|i| Grid::of_vec_of_vecs(i.collect()).unwrap())
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

fn evaluate_north_weight(grid: Grid<Cell>) -> u32 {
    grid.all_cols()
        .into_iter()
        .map(|col| {
            col.into_iter()
                .rev()
                .enumerate()
                .fold(0, |total_load, (current_distance, cell)| match cell {
                    Cell::Round => total_load + (current_distance as u32) + 1,
                    Cell::Empty => total_load,
                    Cell::Square => total_load,
                })
        })
        .sum()
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let mut grid = parse(input);
    move_rocks(&mut grid, NORTH);
    evaluate_north_weight(grid)
}

fn move_rocks(grid: &mut Grid<Cell>, direction: GridPointDelta<isize>) {
    let (initial_corner, secondary_direction) = match direction {
        NORTH => (GridPoint::new(0, 0), EAST),
        WEST => (GridPoint::new(0, 0), SOUTH),
        SOUTH => (GridPoint::new(grid.rows() - 1, 0), EAST),
        EAST => (GridPoint::new(0, grid.cols() - 1), SOUTH),
        _ => panic!("unrecognize direction {:?}", direction),
    };
    let grid_dimensions = grid.dimensions();
    for initial_point in initial_corner.traverse_by(secondary_direction, grid_dimensions) {
        let mut next_empty_space = initial_point;
        for point in initial_point.traverse_by(-direction, grid_dimensions) {
            match grid.get(point).unwrap() {
                Cell::Empty => (),
                Cell::Round => {
                    grid.set(point, Cell::Empty).unwrap();
                    grid.set(next_empty_space, Cell::Round).unwrap();
                    next_empty_space = next_empty_space.add(-direction).unwrap_or(point);
                }
                Cell::Square => {
                    next_empty_space = point.add(-direction).unwrap_or(point); // this only fails sometimes at the edge of the board and next empty space won't be used
                }
            }
        }
    }
}

fn cycle(grid: &mut Grid<Cell>) {
    move_rocks(grid, NORTH);
    move_rocks(grid, WEST);
    move_rocks(grid, SOUTH);
    move_rocks(grid, EAST);
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let mut seen = vec![];
    let mut grid = parse(input);
    seen.push(grid.clone());
    let full_iter_count = 1_000_000_000;
    for idx in 1..full_iter_count {
        cycle(&mut grid);
        if let Some(seen_idx) = seen.iter().position(|g| g == &grid) {
            let cycle_length = idx - seen_idx;
            let desired_idx_offset = (full_iter_count - idx) % cycle_length + seen_idx;
            grid = seen[desired_idx_offset].clone();
            break;
        } else {
            seen.push(grid.clone());
        }
    }
    evaluate_north_weight(grid)
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
    const DAY: Day = Day::Day14;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 136);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 64);
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
            108759
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
            89089
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
