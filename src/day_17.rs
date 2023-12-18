use advent_of_code::{
    grid::{Grid, GridPoint, GridPointDelta, EAST, NORTH, SOUTH, WEST},
    parse::{parsers, Parser},
};

use std::collections::{BinaryHeap, HashSet};

fn parse(input: &str) -> Grid<u32> {
    parsers::chars(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .many()
        .map(|i| i.collect())
        .many_lines("\n")
        .map(|i| Grid::of_vec_of_vecs(i.collect()).unwrap())
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SearchElement {
    grid_point: GridPoint<usize>,
    next_dir: GridPointDelta<isize>,
    weight: u32,
    cost: u32,
}

impl SearchElement {
    fn new(
        grid_point: GridPoint<usize>,
        next_dir: GridPointDelta<isize>,
        target: GridPoint<usize>,
        cost: u32,
    ) -> Self {
        SearchElement {
            grid_point,
            next_dir,
            weight: cost + (target.sub::<isize>(grid_point).unwrap().l1_norm()) as u32,
            cost,
        }
    }

    fn unwrap(self) -> (GridPoint<usize>, GridPointDelta<isize>, u32) {
        (self.grid_point, self.next_dir, self.cost)
    }
}

impl PartialOrd for SearchElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for SearchElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}

fn next_dirs(dir: GridPointDelta<isize>) -> (GridPointDelta<isize>, GridPointDelta<isize>) {
    match dir {
        NORTH | SOUTH => (EAST, WEST),
        EAST | WEST => (NORTH, SOUTH),
        _ => panic!("unrecognized direction"),
    }
}

fn search(
    grid: &Grid<u32>,
    start: GridPoint<usize>,
    starting_dirs: Vec<GridPointDelta<isize>>,
    target: GridPoint<usize>,
    min_move: usize,
    max_move: usize,
) -> u32 {
    let mut to_search: BinaryHeap<SearchElement> = BinaryHeap::new();
    let mut seen: HashSet<(GridPoint<usize>, GridPointDelta<isize>)> = HashSet::new();
    starting_dirs.into_iter().for_each(|starting_dir| {
        to_search.push(SearchElement::new(start, starting_dir, target, 0));
    });
    let grid_dimensions = grid.dimensions();
    while let Some(next) = to_search.pop() {
        let (current, dir, mut cost) = next.unwrap();
        if seen.contains(&(current, dir)) {
            continue;
        }
        seen.insert((current, dir));
        if current == target {
            return cost;
        }
        let (left, right) = next_dirs(dir);
        for (idx, point) in current
            .traverse_by(dir, grid_dimensions)
            .skip(1)
            .take(max_move)
            .enumerate()
        {
            cost += grid.get(point).unwrap();
            if idx + 1 >= min_move {
                to_search.push(SearchElement::new(point, left, target, cost));
                to_search.push(SearchElement::new(point, right, target, cost));
            }
        }
    }
    panic!("Terminated without reaching target");
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let grid = parse(input);

    search(
        &grid,
        GridPoint::new(0, 0),
        vec![EAST, SOUTH],
        GridPoint::new(grid.rows() - 1, grid.cols() - 1),
        1,
        3,
    )
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let grid = parse(input);

    search(
        &grid,
        GridPoint::new(0, 0),
        vec![EAST, SOUTH],
        GridPoint::new(grid.rows() - 1, grid.cols() - 1),
        4,
        10,
    )
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
    const DAY: Day = Day::Day17;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 102);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 94);
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
            767
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
            904
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
