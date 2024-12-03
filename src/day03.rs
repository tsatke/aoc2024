use std::iter::FusedIterator;

const INPUT: &str = include_str!("../inputs/input_day3.txt");

struct Multiplications {
    input: &'static [u8],
    index: usize,
    check_for_dont: bool,
}

impl Multiplications {
    fn advance(&mut self) -> Option<()> {
        if self.index >= self.input.len() {
            return None;
        }
        self.index += 1;
        Some(())
    }
}

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

impl Iterator for Multiplications {
    type Item = (u16, u16);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.input.len() {
            return None;
        }

        'outer: loop {
            if self.index + 3 <= self.input.len()
                && &self.input[self.index..self.index + 3] == b"mul"
            {
                // 'mul'
                self.index += 3;

                // opening parenthesis
                if self.input[self.index] != b'(' {
                    self.advance()?;
                    continue 'outer;
                }
                self.advance()?;

                // up to three digits for left operator
                let mut left = 0;
                'lhs: for i in 0..3 {
                    if !is_digit(self.input[self.index]) {
                        if i == 0 {
                            // not even 1 digit
                            continue 'outer;
                        }
                        break 'lhs;
                    }
                    let digit = self.input[self.index] - b'0';
                    left = left * 10 + digit as u16;

                    self.advance()?;
                }

                // comma
                if self.input[self.index] != b',' {
                    self.advance()?;
                    continue 'outer;
                }
                self.advance()?;

                // up to three digits for right operator
                let mut right = 0;
                'rhs: for i in 0..3 {
                    if !is_digit(self.input[self.index]) {
                        if i == 0 {
                            // not even 1 digit
                            continue 'outer;
                        }
                        break 'rhs;
                    }
                    let digit = self.input[self.index] - b'0';
                    right = right * 10 + digit as u16;

                    self.advance()?;
                }

                // closing parenthesis
                if self.input[self.index] != b')' {
                    self.advance()?;
                    continue 'outer;
                }
                self.advance()?;

                return Some((left, right));
            } else if self.check_for_dont
                && self.index + 7 <= self.input.len()
                && &self.input[self.index..self.index + 7] == b"don't()"
            {
                // 'don't()'
                self.index += 7;

                // search for the next 'do()'
                while self.index + 4 < self.input.len() {
                    if &self.input[self.index..self.index + 4] == b"do()" {
                        self.index += 4;
                        continue 'outer;
                    }
                    self.advance()?;
                }
            }
            self.advance()?;
        }
    }
}

impl FusedIterator for Multiplications {}

pub fn part1() -> usize {
    Multiplications {
        input: INPUT.as_bytes(),
        index: 0,
        check_for_dont: false,
    }
    .fold(0, |acc, (a, b)| acc + (a as usize * b as usize))
}

pub fn part2() -> usize {
    Multiplications {
        input: INPUT.as_bytes(),
        index: 0,
        check_for_dont: true,
    }
    .fold(0, |acc, (a, b)| acc + (a as usize * b as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results() {
        assert_eq!(part1(), 189527826);
        assert_eq!(part2(), 63013756);
    }
}
