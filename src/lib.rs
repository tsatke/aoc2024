#![feature(custom_test_frameworks)]
#![feature(iter_map_windows)]

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

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
    use crate::FromStrFast;

    #[test]
    fn test_fast_uint_parse() {
        assert_eq!(u32::from_str_fast("123"), 123);
    }
}
