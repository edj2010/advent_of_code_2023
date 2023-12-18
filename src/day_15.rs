use advent_of_code::parse::{parsers, Parser};

fn hash(s: &str) -> u32 {
    s.bytes()
        .into_iter()
        .filter(|c| *c != b'\n')
        .fold(0, |v, b| (17 * (v + (b as u32))) % 256)
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    parsers::many_chars(|c| c != ',')
        .list(",")
        .skip_tag("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
        .map(|s| hash(&s))
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add(u32),
    Remove,
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let mut boxes: Vec<Vec<(String, u32)>> = (0..256).map(|_| Vec::new()).collect();

    parsers::many_chars(|c| c != '=' && c != '-')
        .and_then(
            parsers::tag_replace("-", Operation::Remove).or(parsers::tag("=")
                .ignore_and_then(parsers::number())
                .map(|n| Operation::Add(n))),
        )
        .list(",")
        .skip_tag("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
        .for_each(|(label, op)| {
            let idx = hash(&label) as usize;
            match op {
                Operation::Remove => boxes[idx]
                    .iter()
                    .position(|(s, _)| s == &label)
                    .into_iter()
                    .for_each(|to_remove| {
                        boxes[idx].remove(to_remove);
                    }),
                Operation::Add(n) => {
                    if let Some(jdx) = boxes[idx].iter().position(|(s, _)| s == &label) {
                        boxes[idx][jdx] = (label.clone(), n)
                    } else {
                        boxes[idx].push((label.clone(), n))
                    }
                }
            }
        });

    boxes
        .into_iter()
        .enumerate()
        .map(|(idx, lenses)| {
            (idx + 1) as u32
                * lenses
                    .into_iter()
                    .enumerate()
                    .map(|(jdx, (_, focal_length))| (jdx + 1) as u32 * focal_length)
                    .sum::<u32>()
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

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";
    const DAY: Day = Day::Day15;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 1320);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 145);
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
            510792
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
            269410
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
