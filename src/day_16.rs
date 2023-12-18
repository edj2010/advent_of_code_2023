use std::collections::{HashSet, VecDeque};

use advent_of_code::{
    grid::{Grid, GridDimensions, GridPoint, GridPointDelta, EAST, NORTH, SOUTH, WEST},
    parse::{parsers, Parser},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    SplitterVertical,
    SplitterHorizontal,
    MirrorNE,
    MirrorNW,
}

fn parse(input: &str) -> Grid<Cell> {
    parsers::tag_replace(".", Cell::Empty)
        .or(parsers::tag_replace("|", Cell::SplitterVertical))
        .or(parsers::tag_replace("-", Cell::SplitterHorizontal))
        .or(parsers::tag_replace("/", Cell::MirrorNE))
        .or(parsers::tag_replace("\\", Cell::MirrorNW))
        .many()
        .map(|i| i.collect())
        .many_lines("\n")
        .map(|i| Grid::of_vec_of_vecs(i.collect()).unwrap())
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

fn queue_next(
    seen: &mut HashSet<(GridPoint<usize>, GridPointDelta<isize>)>,
    to_search: &mut VecDeque<(GridPoint<usize>, GridPointDelta<isize>)>,
    current: GridPoint<usize>,
    dir: GridPointDelta<isize>,
    dimensions: &GridDimensions<usize>,
) {
    if let Some(next) = current.add_checked(dir, dimensions) {
        if seen.insert((next, dir)) {
            to_search.push_back((next, dir));
        }
    }
}

fn simulate(grid: &Grid<Cell>, start: GridPoint<usize>, start_dir: GridPointDelta<isize>) -> usize {
    let mut seen: HashSet<(GridPoint<usize>, GridPointDelta<isize>)> = HashSet::new();
    let mut to_search: VecDeque<(GridPoint<usize>, GridPointDelta<isize>)> = VecDeque::new();
    seen.insert((start, start_dir));
    to_search.push_back((start, start_dir));
    while let Some((current, dir)) = to_search.pop_front() {
        let grid_dimensions = grid.dimensions();
        match grid.get(current).unwrap() {
            Cell::Empty => {
                queue_next(&mut seen, &mut to_search, current, dir, &grid_dimensions);
            }
            Cell::SplitterHorizontal => {
                if dir == EAST || dir == WEST {
                    queue_next(&mut seen, &mut to_search, current, dir, &grid_dimensions);
                } else {
                    queue_next(&mut seen, &mut to_search, current, EAST, &grid_dimensions);
                    queue_next(&mut seen, &mut to_search, current, WEST, &grid_dimensions);
                }
            }
            Cell::SplitterVertical => {
                if dir == NORTH || dir == SOUTH {
                    queue_next(&mut seen, &mut to_search, current, dir, &grid_dimensions);
                } else {
                    queue_next(&mut seen, &mut to_search, current, NORTH, &grid_dimensions);
                    queue_next(&mut seen, &mut to_search, current, SOUTH, &grid_dimensions);
                }
            }
            Cell::MirrorNE => {
                let new_dir = match dir {
                    NORTH => EAST,
                    EAST => NORTH,
                    SOUTH => WEST,
                    WEST => SOUTH,
                    _ => panic!("unrecognized direction"),
                };
                queue_next(
                    &mut seen,
                    &mut to_search,
                    current,
                    new_dir,
                    &grid_dimensions,
                );
            }
            Cell::MirrorNW => {
                let new_dir = match dir {
                    NORTH => WEST,
                    WEST => NORTH,
                    SOUTH => EAST,
                    EAST => SOUTH,
                    _ => panic!("unrecognized direction"),
                };
                queue_next(
                    &mut seen,
                    &mut to_search,
                    current,
                    new_dir,
                    &grid_dimensions,
                );
            }
        }
    }
    seen.into_iter()
        .map(|(a, _)| a)
        .collect::<HashSet<GridPoint<usize>>>()
        .len()
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let grid = parse(input);
    simulate(&grid, GridPoint::new(0, 0), EAST)
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let grid = parse(input);
    let grid_dimensions = grid.dimensions();
    GridPoint::new(0, 0)
        .traverse_by(EAST, grid_dimensions)
        .map(|start| (start, SOUTH))
        .chain(
            GridPoint::new(0, 0)
                .traverse_by(SOUTH, grid_dimensions)
                .map(|start| (start, EAST)),
        )
        .chain(
            GridPoint::new(grid.rows() - 1, 0)
                .traverse_by(EAST, grid_dimensions)
                .map(|start| (start, NORTH)),
        )
        .chain(
            GridPoint::new(0, grid.cols() - 1)
                .traverse_by(SOUTH, grid_dimensions)
                .map(|start| (start, WEST)),
        )
        .map(|(start, start_dir)| simulate(&grid, start, start_dir))
        .max()
        .unwrap()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
";
    const DAY: Day = Day::Day16;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 46);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 51);
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
            7242
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
            7572
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
