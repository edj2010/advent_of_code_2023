use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code::{
    grid::{Grid, GridPoint, GridPointDelta, EAST, NORTH, PLUS_ADJACENT, SOUTH, WEST},
    parse::{parsers, Parser},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Horizontal,
    Vertical,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
}

impl Cell {
    fn of_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Empty),
            '-' => Some(Self::Horizontal),
            '|' => Some(Self::Vertical),
            'L' => Some(Self::NorthEast),
            'J' => Some(Self::NorthWest),
            '7' => Some(Self::SouthWest),
            'F' => Some(Self::SouthEast),
            'S' => Some(Self::Start),
            _ => None,
        }
    }

    fn valid_connection(self, moved_dir: GridPointDelta<isize>) -> bool {
        match self {
            Self::Empty => false,
            Self::Horizontal => moved_dir == EAST || moved_dir == WEST,
            Self::Vertical => moved_dir == NORTH || moved_dir == SOUTH,
            Self::NorthEast => moved_dir == SOUTH || moved_dir == WEST,
            Self::NorthWest => moved_dir == SOUTH || moved_dir == EAST,
            Self::SouthEast => moved_dir == NORTH || moved_dir == WEST,
            Self::SouthWest => moved_dir == NORTH || moved_dir == EAST,
            Self::Start => true,
        }
    }

    fn valid_directions(self) -> Vec<GridPointDelta<isize>> {
        match self {
            Self::Empty => vec![],
            Self::Horizontal => vec![EAST, WEST],
            Self::Vertical => vec![NORTH, SOUTH],
            Self::NorthEast => vec![NORTH, EAST],
            Self::NorthWest => vec![NORTH, WEST],
            Self::SouthEast => vec![SOUTH, EAST],
            Self::SouthWest => vec![SOUTH, WEST],
            Self::Start => PLUS_ADJACENT.iter().copied().collect(),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Cell>> {
    parsers::many_chars(|c| c != '\n')
        .map(|c| c.chars().filter_map(Cell::of_char).collect::<Vec<Cell>>())
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
        .collect()
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let grid: Grid<Cell> = Grid::of_vec_of_vecs(parse(input)).unwrap();
    let start = grid.find(&Cell::Start).unwrap();
    let mut seen: HashMap<GridPoint<usize>, usize> = HashMap::new();
    seen.insert(start, 0);
    let mut to_search: VecDeque<(usize, GridPoint<usize>)> = VecDeque::new();

    for delta in PLUS_ADJACENT.iter() {
        if let Some(next_loc) = start + *delta {
            let next = grid.get(next_loc).unwrap();
            if next.valid_connection(*delta) {
                seen.insert(next_loc, 1);
                to_search.push_back((1, next_loc));
            }
        }
    }

    while let Some((distance, current_loc)) = to_search.pop_front() {
        let current = grid.get(current_loc).unwrap();
        for delta in current.valid_directions() {
            let next_loc = (current_loc + delta).unwrap();
            if !seen.contains_key(&next_loc) {
                seen.insert(next_loc, distance + 1);
                to_search.push_back((distance + 1, next_loc));
            }
        }
    }

    *(seen.values().max().unwrap())
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let mut grid: Grid<Cell> = Grid::of_vec_of_vecs(parse(input)).unwrap();
    let start = grid.find(&Cell::Start).unwrap();
    let mut seen: HashSet<GridPoint<usize>> = HashSet::new();
    seen.insert(start);
    let mut to_search: VecDeque<GridPoint<usize>> = VecDeque::new();

    for delta in PLUS_ADJACENT.iter() {
        if let Some(next_loc) = start + *delta {
            let next = grid.get(next_loc).unwrap();
            if next.valid_connection(*delta) {
                seen.insert(next_loc);
                to_search.push_back(next_loc);
            }
        }
    }

    while let Some(current_loc) = to_search.pop_front() {
        let current = grid.get(current_loc).unwrap();
        for delta in current.valid_directions() {
            let next_loc = (current_loc + delta).unwrap();
            if !seen.contains(&next_loc) {
                seen.insert(next_loc);
                to_search.push_back(next_loc);
            }
        }
    }

    let dimensions = grid.dimensions();

    for point in dimensions.all_contained_points() {
        if !seen.contains(&point) {
            grid.set(point, Cell::Empty).unwrap();
        }
    }

    let mut inside_count = 0;
    for row in GridPoint::new(0, 0).traverse_by(SOUTH, dimensions) {
        let mut inside = false;
        let mut last_l: Option<Cell> = None;
        for loc in row.traverse_by(EAST, dimensions) {
            match *(grid.get(loc).unwrap()) {
                Cell::Vertical => inside = !inside,
                Cell::Empty => {
                    if inside {
                        inside_count += 1
                    }
                }
                Cell::NorthEast => last_l = Some(Cell::NorthEast),
                Cell::SouthEast => last_l = Some(Cell::SouthEast),
                Cell::SouthWest => {
                    if last_l == Some(Cell::NorthEast) {
                        inside = !inside;
                    }
                    last_l = None;
                }
                Cell::NorthWest => {
                    if last_l == Some(Cell::SouthEast) {
                        inside = !inside;
                    }
                    last_l = None;
                }
                _ => (),
            }
        }
    }

    inside_count
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const DAY: Day = Day::Day10;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                ".....
.S-7.
.|.|.
.L-J.
.....
"
            ),
            4
        );
        assert_eq!(
            part1(
                "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"
            ),
            8
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"
            ),
            4
        );
        assert_eq!(
            part2(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"
            ),
            8
        );
        assert_eq!(
            part2(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"
            ),
            10
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
            6754
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
            0
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
