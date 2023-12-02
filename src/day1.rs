use advent_of_code::parse::{parsers, Parser};

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    parsers::chars(|c| c.is_alphanumeric())
        .many()
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
        .map(|line| {
            let digits: Vec<u32> = line.filter_map(|digit| digit.to_digit(10)).collect();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    parsers::many_chars(|c| c.is_alphanumeric())
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
        .map(|s| {
            let mut digits_at: Vec<Option<u32>> = vec![None; s.len()];
            s.bytes().enumerate().for_each(|(idx, d)| {
                (d as char)
                    .to_digit(10)
                    .iter()
                    .for_each(|&d| digits_at[idx] = Some(d))
            });
            [
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ]
            .iter()
            .for_each(|(word, digit)| {
                s.find(word)
                    .iter()
                    .for_each(|&idx| digits_at[idx] = Some(*digit));
                s.rfind(word)
                    .iter()
                    .for_each(|&idx| digits_at[idx] = Some(*digit));
            });
            let digits: Vec<u32> = digits_at.iter().filter_map(|&d| d).collect();
            digits.first().unwrap() * 10 + digits.last().unwrap()
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

    const DAY: Day = Day::Day01;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"
            ),
            142
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"
            ),
            281
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
            56397
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
            55701
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
