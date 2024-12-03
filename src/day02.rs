use crate::FromStrFast;
use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/input_day2.txt");

pub fn part1() -> usize {
    INPUT.lines().filter(is_line_valid).count()
}

fn is_line_valid(line: &&str) -> bool {
    let mut elems = [0_u8; 8];
    let last = into_array(&mut elems, line.split_whitespace().map(u8::from_str_fast));
    let elems = &elems[..=last];

    is_slice_valid(elems)
}

fn is_slice_valid(elems: &[u8]) -> bool {
    elems
        .iter()
        .tuple_windows()
        .all(|(&a, &b)| (1..=3).contains(&a.abs_diff(b)))
        && (elems.is_sorted() || elems.iter().rev().is_sorted())
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
            is_line_valid(line) || {
                (0..line.len()).any(|skip| {
                    let mut elems = [0_u8; 8];
                    let last = into_array(
                        &mut elems,
                        line.split_whitespace()
                            .map(u8::from_str_fast)
                            .enumerate()
                            .filter_map(|(i, n)| if i == skip { None } else { Some(n) }),
                    );
                    let elems = &elems[..=last];
                    is_slice_valid(elems)
                })
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day02::{part1, part2};

    #[test]
    fn test_results() {
        assert_eq!(part1(), 483);
        assert_eq!(part2(), 528);
    }
}
