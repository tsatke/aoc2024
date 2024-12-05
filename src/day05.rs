use crate::FromStrFast;
use std::collections::HashSet;
use std::hash::{BuildHasher, Hasher};
use std::ops::BitXor;

const INPUT: &str = include_str!("../inputs/input_day5.txt");

#[derive(Copy, Clone)]
struct FnvBuildHasher;

impl BuildHasher for FnvBuildHasher {
    type Hasher = FnvHasher;

    fn build_hasher(&self) -> Self::Hasher {
        FnvHasher::new()
    }
}

struct FnvHasher(u64);

impl FnvHasher {
    const fn new() -> Self {
        Self(0xcbf29ce484222325)
    }
}

impl Hasher for FnvHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        const MAGIC_PRIME: u64 = 0x00000100000001b3;

        for &b in bytes {
            self.0 = self.0.bitxor(b as u64).wrapping_mul(MAGIC_PRIME);
        }
    }
}

pub fn part1() -> usize {
    let mut requirements = [const { HashSet::with_hasher(FnvBuildHasher) }; u8::MAX as usize];
    let mut input = INPUT.lines();
    input
        .by_ref()
        .take_while(|l| l.len() == 5)
        .map(|l| (u8::from_str_fast(&l[..2]), u8::from_str_fast(&l[3..])))
        .for_each(|(l, r)| {
            requirements[l as usize].insert(r);
        });

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
