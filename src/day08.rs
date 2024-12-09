use crate::slab::Slab;
use std::collections::HashSet;

const LINES: usize = 50;

const INPUT: &str = include_str!("../inputs/input_day8.txt");

fn populate_antennas(antennas: &mut [Slab<LINES, usize>; u8::MAX as usize]) {
    INPUT
        .bytes()
        .filter(|&c| c != b'\n')
        .enumerate()
        .filter(|(_, c)| c.is_ascii_alphanumeric())
        .for_each(|(offset, c)| {
            antennas[c as usize].push_back(offset);
        });
}

fn within_bounds(pt: (i8, i8)) -> bool {
    let range = 0..LINES as i8;
    range.contains(&pt.0) && range.contains(&pt.1)
}

fn to_point(offset: usize) -> (i8, i8) {
    ((offset / LINES) as i8, (offset % LINES) as i8)
}

#[must_use]
pub fn part1() -> usize {
    let mut antennas = [const { Slab::<LINES, usize>::new() }; u8::MAX as usize];
    populate_antennas(&mut antennas);

    let mut antinodes = HashSet::new();

    for frequency in (b'0'..=b'9').chain(b'a'..=b'z').chain(b'A'..=b'Z') {
        let slab = &antennas[frequency as usize];
        if slab.len() < 2 {
            continue;
        }

        for i in 0..slab.len() {
            let left = slab[i];

            for &right in &slab[i + 1..] {
                let left = to_point(left);
                let right = to_point(right);

                let dx = i8::from(right.0) - i8::from(left.0);
                let dy = i8::from(right.1) - i8::from(left.1);

                let pt1 = (right.0 + dx, right.1 + dy);
                if within_bounds(pt1) {
                    antinodes.insert(pt1);
                }
                let pt2 = (left.0 - dx, left.1 - dy);
                if within_bounds(pt2) {
                    antinodes.insert(pt2);
                }
            }
        }
    }

    antinodes.len()
}

#[must_use]
pub fn part2() -> usize {
    let mut antennas = [const { Slab::<LINES, usize>::new() }; u8::MAX as usize];
    populate_antennas(&mut antennas);

    let mut antinodes = HashSet::new();

    for frequency in (b'0'..=b'9').chain(b'a'..=b'z').chain(b'A'..=b'Z') {
        let slab = &antennas[frequency as usize];
        if slab.len() < 2 {
            continue;
        }

        for i in 0..slab.len() {
            let left = slab[i];

            for &right in &slab[i + 1..] {
                let left = to_point(left);
                let right = to_point(right);

                let dx = i8::from(right.0) - i8::from(left.0);
                let dy = i8::from(right.1) - i8::from(left.1);

                let mut current = left;
                while within_bounds(current) {
                    antinodes.insert(current);
                    current = (current.0 - dx, current.1 - dy);
                }
                current = right;
                while within_bounds(current) {
                    antinodes.insert(current);
                    current = (current.0 + dx, current.1 + dy);
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results() {
        assert_eq!(part1(), 398);
        assert_eq!(part2(), 1333);
    }
}
