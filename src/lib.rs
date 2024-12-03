#![feature(custom_test_frameworks)]

const INPUT: &str = include_str!("../inputs/input_day1.txt");

pub fn solution_01_1() -> u32 {
    const LINES: usize = 1000;

    let mut left = [0_i32; LINES];
    let mut right = [0_i32; LINES];
    INPUT
        .lines()
        .map(|line| {
            (
                u32::from_str_fast(&line[0..5]),
                u32::from_str_fast(&line[8..]),
            )
        })
        .enumerate()
        .for_each(|(i, (a, b))| {
            left[i] = a as i32;
            right[i] = b as i32;
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

macro_rules! fast_uint_parse_impl {
    ($($typ:ty),*) => {
        $(impl FromStrFast for $typ {
            fn from_str_fast(s: &str) -> Self {
                s.bytes()
                    .fold(0, |acc, c| acc * 10 + (c - b'0') as $typ)
            }
        })*
    };
}

fast_uint_parse_impl!(u8, u16, u32, u64, u128, usize);

pub trait FromStrFast {
    fn from_str_fast(s: &str) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_uint_parse() {
        assert_eq!(u32::from_str_fast("123"), 123);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution_01_1(), 2066446);
    }
}
