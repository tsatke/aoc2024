use crate::FromStrFast;

const LINES: usize = 1000;
const INPUT: &str = include_str!("../inputs/input_day1.txt");

fn populate_columns(left: &mut [i32; 1000], right: &mut [i32; 1000]) {
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
}

pub fn part1() -> u32 {
    let mut left = [0_i32; LINES];
    let mut right = [0_i32; LINES];
    populate_columns(&mut left, &mut right);

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .fold(0, |acc, (a, b)| acc + a.abs_diff(b))
}

pub fn part2() -> i64 {
    let mut left = [0_i32; LINES];
    let mut right = [0_i32; LINES];
    populate_columns(&mut left, &mut right);

    let mut lut = [0_u16; 100000];
    right.iter().for_each(|&r| lut[r as usize] += 1);
    left.iter()
        .fold(0, |acc, &l| acc + ((l as i64) * lut[l as usize] as i64))
}

#[cfg(test)]
mod tests {
    use crate::day01::{part1, part2};

    #[test]
    fn test_results() {
        assert_eq!(part1(), 2066446);
        assert_eq!(part2(), 24931009);
    }
}
