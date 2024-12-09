use crate::FromStrFast;

const INPUT: &str = include_str!("../inputs/input_day7.txt");

type Num = u64;

fn solve<const PART2: bool>() -> u64 {
    INPUT
        .lines()
        .filter_map(|line| {
            let mut nums = line
                .split_ascii_whitespace()
                .map(|word| Num::from_str_fast(word.trim_end_matches(':')));
            let target = nums.next().unwrap();

            let mut operands = [Num::default(); 12];
            let last = nums
                .enumerate()
                .map(|(i, n)| {
                    operands[i] = n;
                    i
                })
                .last()
                .unwrap();
            let operands = &operands[..=last];

            // if the target is smaller than the smallest number we can produce, skip
            if operands.iter().sum::<u64>() - (operands.len() as Num) > target {
                return None;
            }

            // if the target is larger than the largest number we can produce, skip
            if operands.iter().fold(1, |l, &r| l * (r + 1)) < target {
                return None;
            }

            if is_valid::<PART2>(target, &operands) {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

#[inline(always)]
fn is_valid<const CONCAT: bool>(target: u64, operands: &[u64]) -> bool {
    if operands.len() == 1 {
        return operands[0] == target;
    }

    let last_index = operands.len() - 1;
    let last = operands[last_index];
    let (res, rem) = (target / last, target % last);
    if rem == 0 && is_valid::<CONCAT>(res, &operands[..last_index]) {
        true
    } else if target > last && is_valid::<CONCAT>(target - last, &operands[..last_index]) {
        return true;
    } else if CONCAT {
        todo!()
    } else {
        false
    }
}

#[must_use]
pub fn part1() -> u64 {
    solve::<false>()
}

#[must_use]
pub fn part2() -> u64 {
    solve::<true>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results() {
        assert_eq!(part1(), 3351424677624);
        assert_eq!(part2(), 0);
    }
}
