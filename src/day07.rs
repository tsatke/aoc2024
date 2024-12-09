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

            if !PART2 {
                // if the target is smaller than the smallest number we can produce, skip
                if operands.iter().sum::<u64>() - (operands.len() as Num) > target {
                    return None;
                }

                // if the target is larger than the largest number we can produce, skip
                if operands.iter().fold(1, |l, &r| l * (r + 1)) < target {
                    return None;
                }
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
        true
    } else if CONCAT {
        let log10 = 10_u32.pow(fast_log10(last as u32) + 1) as u64;

        // if we can divide by log10, then divide by [last-1] and sub [last], we're good and technically
        // did the concatenation

        // simply put: 5014: 5 10 14 = 5 * 10 || 14 = 5 * 10 * 100 + 14, and we can boil down
        // *100+14 in a single step without mutating the operands

        let (res, rem) = (target / log10, target % log10);
        rem == last && is_valid::<CONCAT>(res, &operands[..last_index])
    } else {
        false
    }
}

// thanks https://da-data.blogspot.com/2023/02/integer-log10-in-rust-and-c.html
fn fast_log10(n: u32) -> u32 {
    const TEN_THRESHOLDS: [u32; 10] = [
        9,
        99,
        999,
        9999,
        99999,
        999999,
        9999999,
        99999999,
        999_999_999,
        u32::MAX,
    ];

    let guess = (n.ilog2() * 9) >> 5;
    let ttg = TEN_THRESHOLDS[guess as usize];
    guess + (n > ttg) as u32
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
        assert_eq!(part2(), 204976636995111);
    }

    #[test]
    fn foo() {
        for i in 1_u32..100000 {
            assert_eq!(fast_log10(i), i.ilog10(), "incorrect for {i}");
        }
    }
}
