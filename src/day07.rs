use crate::FromStrFast;

const INPUT: &str = include_str!("../inputs/input_day7.txt");

type Num = u64;

#[must_use]
pub fn part1() -> u64 {
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

            let mut mul_cache = [0_u64; 12];
            let mut acc = 1;
            for (i, &op) in operands.iter().enumerate() {
                acc *= op;
                mul_cache[i] = acc;
            }
            if acc == target {
                return Some(target);
            }
            // println!("cache: {:?} (operands: {:?})", mul_cache, operands);

            for op_bits in 1_u16..=(1 << operands.len()) - 1 {
                let idx = (op_bits.leading_zeros() as usize) - (16 - operands.len());
                // println!("op_bits: {:05b} (idx: {})", op_bits, idx);
                let mut acc = if idx == 0 { 0 } else { mul_cache[idx - 1] };
                // print!("{} ", acc);

                let mut mask = (1_u16 << (operands.len() - 1)) >> idx;
                for op in &operands[idx..] {
                    if op_bits & mask == mask {
                        // print!("+ {} (0b{:b}) ", op, mask);
                        acc += op;
                    } else {
                        // print!("* {} (0b{:b}) ", op, mask);
                        acc *= op;
                    }
                    mask >>= 1;
                }
                // println!();

                if acc == target {
                    return Some(target);
                }
            }

            None
        })
        .sum()
}

#[must_use]
pub fn part2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results() {
        assert_eq!(part1(), 3351424677624);
        // assert_eq!(part1(), 3749);
        assert_eq!(part2(), 0);
    }
}
