use crate::FromStrFast;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../inputs/input_day5.txt");

pub fn part1() -> usize {
    let mut requirements = HashMap::<u8, HashSet<u8>>::new();
    let mut input = INPUT.lines();
    input
        .by_ref()
        .take_while(|l| l.len() == 5)
        .map(|l| (u8::from_str_fast(&l[..2]), u8::from_str_fast(&l[3..])))
        .for_each(|(l, r)| {
            requirements.entry(l).or_insert_with(HashSet::new).insert(r);
        });

    input
        .filter_map(|line| {
            if line
                .split(',')
                .map(u8::from_str_fast)
                .is_sorted_by(|&l, r| requirements[&l].contains(r))
            {
                let center = line.len() / 2;
                Some(usize::from_str_fast(&line[center - 1..center + 1]))
            } else {
                None
            }
        })
        .sum::<usize>()
}

pub fn part2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results() {
        assert_eq!(part1(), 4905);
        assert_eq!(part2(), 0);
    }
}
