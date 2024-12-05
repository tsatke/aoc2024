use crate::FromStrFast;
use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/input_day2.txt");

pub fn part1() -> usize {
    INPUT
        .lines()
        .filter(|line| {
            let mut elems = [0_u8; 8];
            let last = into_array(
                &mut elems,
                line.split_ascii_whitespace().map(u8::from_str_fast),
            );
            let elems = &elems[..=last];

            is_slice_valid(elems)
        })
        .count()
}

fn is_slice_valid(elems: &[u8]) -> bool {
    is_iter_valid(elems.iter())
}

fn is_iter_valid<'a>(iter: impl DoubleEndedIterator<Item = &'a u8> + Clone) -> bool {
    iter.clone()
        .tuple_windows()
        .all(|(&a, &b)| (1..=3).contains(&a.abs_diff(b)))
        && (iter.clone().is_sorted() || iter.clone().rev().is_sorted())
}

fn into_array(arr: &mut [u8; 8], items: impl Iterator<Item = u8>) -> usize {
    let mut last = 0;
    items.enumerate().for_each(|(i, n)| {
        arr[i] = n;
        last = i;
    });
    last
}

pub fn part2() -> usize {
    INPUT
        .lines()
        .filter(|line| {
            let mut full_line_elems = [0_u8; 8];
            let last = into_array(
                &mut full_line_elems,
                line.split_ascii_whitespace().map(u8::from_str_fast),
            );
            let full_line_elems = &full_line_elems[..=last];
            is_slice_valid(full_line_elems) || {
                (0..line.len())
                    .map(|skip| {
                        full_line_elems
                            .iter()
                            .enumerate()
                            .filter_map(move |(i, n)| if i == skip { None } else { Some(n) })
                    })
                    .any(is_iter_valid)
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results() {
        assert_eq!(part1(), 483);
        assert_eq!(part2(), 528);
    }
}
