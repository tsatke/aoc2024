use crate::FromStrFast;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

// 491 - too high
const INPUT: &str = include_str!("../inputs/input_day2.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum MonotonicDirection {
    Broken,
    Increasing(u8),
    Decreasing(u8),
    First(u8),
    None,
}

pub fn part1() -> usize {
    INPUT.lines().filter(is_line_valid_pt1).count()
}

fn is_line_valid_pt1(line: &&str) -> bool {
    line.split_whitespace()
        .map(u8::from_str_fast)
        .fold_while(MonotonicDirection::None, |acc, x| match acc {
            MonotonicDirection::Broken => unreachable!(),
            MonotonicDirection::None => Continue(MonotonicDirection::First(x)),
            MonotonicDirection::First(old) => {
                if x > old && x - old <= 3 {
                    Continue(MonotonicDirection::Increasing(x))
                } else if x < old && old - x <= 3 {
                    Continue(MonotonicDirection::Decreasing(x))
                } else {
                    Done(MonotonicDirection::Broken)
                }
            }
            MonotonicDirection::Increasing(old) => {
                if x > old && x - old <= 3 {
                    Continue(MonotonicDirection::Increasing(x))
                } else {
                    Done(MonotonicDirection::Broken)
                }
            }
            MonotonicDirection::Decreasing(old) => {
                if x < old && old - x <= 3 {
                    Continue(MonotonicDirection::Decreasing(x))
                } else {
                    Done(MonotonicDirection::Broken)
                }
            }
        })
        .into_inner()
        != MonotonicDirection::Broken
}

pub fn part2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::day01::part2;
    use crate::day02::{part1, part2};

    #[test]
    fn test_results() {
        assert_eq!(part1(), 483);
        assert_eq!(part2(), 0);
    }
}
