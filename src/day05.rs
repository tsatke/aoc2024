use crate::FromStrFast;
use ahash::AHasher;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{BuildHasher, Hasher};
use std::ops::BitXor;

const INPUT: &str = include_str!("../inputs/input_day5.txt");

#[derive(Copy, Clone)]
struct AHashBuilder;

impl BuildHasher for AHashBuilder {
    type Hasher = AHasher;

    fn build_hasher(&self) -> Self::Hasher {
        AHasher::default()
    }
}

fn init() -> (
    [HashSet<u8, AHashBuilder>; u8::MAX as usize],
    impl Iterator<Item = &'static str>,
) {
    let mut requirements = [const { HashSet::with_hasher(AHashBuilder) }; u8::MAX as usize];
    let mut input = INPUT.lines();
    input
        .by_ref()
        .take_while(|l| l.len() == 5)
        .map(|l| (u8::from_str_fast(&l[..2]), u8::from_str_fast(&l[3..])))
        .for_each(|(l, r)| {
            requirements[l as usize].insert(r);
        });
    (requirements, input)
}

pub fn part1() -> usize {
    let (requirements, input) = init();

    input
        .filter_map(|line| {
            if line
                .split(',')
                .map(u8::from_str_fast)
                .is_sorted_by(|&l, r| requirements[l as usize].contains(r))
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
    let (requirements, input) = init();

    input
        .filter_map(|line| {
            let mut elems = [0_u8; 23];
            let mut last = 0;
            line.split(',')
                .map(u8::from_str_fast)
                .zip(elems.iter_mut())
                .enumerate()
                .for_each(|(i, (elem, slot))| {
                    *slot = elem;
                    last = i;
                });
            let elems = &mut elems[..=last];
            if elems.is_sorted_by(|&l, r| requirements[l as usize].contains(r)) {
                None
            } else {
                elems.sort_by(|&l, r| {
                    if requirements[l as usize].contains(r) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                Some(elems[elems.len() / 2] as usize)
            }
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results() {
        assert_eq!(part1(), 4905);
        assert_eq!(part2(), 6204);
    }
}
