use advent_of_code::{
    difference_sequence::DifferenceSequence,
    parse::{parsers, Parser},
};

fn parse(input: &str) -> impl Iterator<Item = impl Iterator<Item = i64>> {
    parsers::signed_number::<i64>()
        .list(" ")
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    parse(input)
        .map(|iter| {
            let initial_list: Vec<i64> = iter.collect();
            let initial_length = initial_list.len();
            DifferenceSequence::from_iter(initial_list)
                .nth(initial_length)
                .unwrap()
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    parse(input)
        .map(|iter| DifferenceSequence::from_iter(iter).step_back())
        .sum()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
    const DAY: Day = Day::Day09;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 2);
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
            2175229206
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
            942
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
