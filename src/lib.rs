#![feature(custom_test_frameworks)]
#![feature(iter_map_windows)]
#![feature(array_windows)]
#![feature(allocator_api)]
extern crate core;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

macro_rules! fast_uint_parse_impl {
    ($($typ:ty),*) => {
        $(impl FromStrFast for $typ {
            fn from_str_fast(s: &str) -> Self {
                s.bytes()
                    .fold(0, |acc, c| acc * 10 + <$typ>::from(c - b'0'))
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
    use crate::FromStrFast;

    #[test]
    fn test_fast_uint_parse() {
        assert_eq!(u32::from_str_fast("123"), 123);
    }
}
