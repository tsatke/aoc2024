#![feature(custom_test_frameworks)]

const INPUT: &str = include_str!("../inputs/input_day1.txt");

pub fn solution_01_1() -> u64 {
    const LINES: usize = 1000;

    let mut left = [0_i64; LINES];
    let mut right = [0_i64; LINES];
    INPUT
        .lines()
        .map(|line| (&line[0..5], &line[8..]))
        .enumerate()
        .for_each(|(i, (a, b))| {
            left[i] = a.parse::<i64>().unwrap();
            right[i] = b.parse::<i64>().unwrap();
        });

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right.into_iter())
        .fold(0, |acc, (a, b)| acc + a.abs_diff(b))
}

pub fn solution_01_2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solution_01_1(), 2066446);
    }
}
