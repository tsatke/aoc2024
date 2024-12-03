#![feature(custom_test_frameworks)]
#![feature(test)]
#![allow(soft_unstable)]

const INPUT: &str = include_str!("../inputs/input_day1.txt");

pub fn solution_01_1() -> usize {
    let lines = INPUT.lines().count();

    let mut left = vec![0; lines];
    let mut right = vec![0; lines];
    INPUT
        .lines()
        .map(|line| (&line[0..5], &line[8..]))
        .for_each(|(a, b)| {
            left.push(a.parse::<usize>().unwrap());
            right.push(b.parse::<usize>().unwrap());
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

    extern crate test;

    #[test]
    fn test_part1() {
        assert_eq!(solution_01_1(), 2066446);
    }

    #[bench]
    fn bench_part1(b: &mut test::Bencher) {
        b.iter(|| solution_01_1());
    }
}
