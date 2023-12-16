use advent_of_code::{
    grid::{Grid, GridPoint, GridPointDelta, EAST, SOUTH},
    parse::{parsers, Parser},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Galaxy,
}

fn parse(input: &str) -> Vec<Vec<Cell>> {
    parsers::tag_replace(".", Cell::Empty)
        .or(parsers::tag_replace("#", Cell::Galaxy))
        .many()
        .map(|many| many.collect::<Vec<Cell>>())
        .many_lines("\n")
        .map(|lines| lines.collect())
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

pub fn galaxy_dist(input: &str, expansion: u64) -> u64 {
    let map = Grid::of_vec_of_vecs(parse(input)).unwrap();
    let galaxies: Vec<GridPoint<usize>> = map
        .iter_points()
        .filter(|point| map.get(*point) == Ok(&Cell::Galaxy))
        .collect();
    let mut total_distance: u64 = 0;
    for &point_a in galaxies.iter() {
        for &point_b in galaxies.iter() {
            let distance: GridPointDelta<isize> = point_a.sub(point_b).unwrap();
            total_distance += (distance.row_delta().abs() + distance.col_delta().abs()) as u64;
        }
    }

    total_distance /= 2;

    let grid_dimensions = map.dimensions();

    for row in GridPoint::new(0, 0).traverse_by(SOUTH, grid_dimensions) {
        if row
            .traverse_by(EAST, grid_dimensions)
            .all(|point| map.get(point) == Ok(&Cell::Empty))
        {
            let above_count = galaxies
                .iter()
                .filter(|galaxy| galaxy.row() < row.row())
                .count();
            let below_count = galaxies.len() - above_count;
            total_distance += expansion * (above_count * below_count) as u64;
        }
    }

    for col in GridPoint::new(0, 0).traverse_by(EAST, grid_dimensions) {
        if col
            .traverse_by(SOUTH, grid_dimensions)
            .all(|point| map.get(point) == Ok(&Cell::Empty))
        {
            let left_count = galaxies
                .iter()
                .filter(|galaxy| galaxy.col() < col.col())
                .count();
            let right_count = galaxies.len() - left_count;
            total_distance += expansion * (left_count * right_count) as u64;
        }
    }
    total_distance
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u64 {
    galaxy_dist(input, 1)
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u64 {
    galaxy_dist(input, 999999)
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
    const DAY: Day = Day::Day11;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 374);
    }

    #[test]
    fn part2_example() {
        assert_eq!(galaxy_dist(EXAMPLE, 9), 1030);
        assert_eq!(galaxy_dist(EXAMPLE, 99), 8410);
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
            10885634
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
            707505470642
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
