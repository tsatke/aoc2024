use std::iter::FusedIterator;
use std::marker::PhantomData;

const INPUT: &str = include_str!("../inputs/input_day3.txt");

struct P1;
struct P2;

struct Multiplications<P> {
    input: &'static [u8],
    index: usize,
    _part: PhantomData<P>,
}

impl<P> Multiplications<P> {
    fn advance(&mut self) -> Option<()> {
        if self.index >= self.input.len() {
            return None;
        }
        self.index += 1;
        Some(())
    }

    fn scan_mul(&mut self) -> bool {
        self.index + 3 <= self.input.len() && &self.input[self.index..self.index + 3] == b"mul"
    }

    fn parse_num(&mut self) -> Option<u16> {
        let mut left = 0;
        for i in 0..3 {
            if !is_digit(self.input[self.index]) {
                if i == 0 {
                    // not even 1 digit
                    return None;
                }
                break;
            }
            let digit = self.input[self.index] - b'0';
            left = left * 10 + digit as u16;

            self.advance()?;
        }
        Some(left)
    }

    fn scan_dont(&mut self) -> bool {
        self.index + 7 <= self.input.len() && &self.input[self.index..self.index + 7] == b"don't()"
    }
}

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

impl Iterator for Multiplications<P1> {
    type Item = (u16, u16);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.input.len() {
            return None;
        }

        'outer: loop {
            if self.scan_mul() {
                // 'mul'
                self.index += 3;

                // opening parenthesis
                if self.input[self.index] != b'(' {
                    self.advance()?;
                    continue 'outer;
                }
                self.advance()?;

                let Some(left) = self.parse_num() else {
                    continue 'outer;
                };

                // comma
                if self.input[self.index] != b',' {
                    self.advance()?;
                    continue 'outer;
                }
                self.advance()?;

                let Some(right) = self.parse_num() else {
                    continue 'outer;
                };

                // closing parenthesis
                if self.input[self.index] != b')' {
                    self.advance()?;
                    continue 'outer;
                }
                self.advance()?;

                return Some((left, right));
            }
            self.advance()?;
        }
    }
}

impl Iterator for Multiplications<P2> {
    type Item = (u16, u16);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.input.len() {
            return None;
        }

        'outer: loop {
            if self.scan_mul() {
                // 'mul'
                self.index += 3;

                // opening parenthesis
                if self.input[self.index] != b'(' {
                    self.advance()?;
                    continue 'outer;
                }
                self.advance()?;

                let Some(left) = self.parse_num() else {
                    continue 'outer;
                };

                // comma
                if self.input[self.index] != b',' {
                    self.advance()?;
                    continue 'outer;
                }
                self.advance()?;

                let Some(right) = self.parse_num() else {
                    continue 'outer;
                };

                // closing parenthesis
                if self.input[self.index] != b')' {
                    self.advance()?;
                    continue 'outer;
                }
                self.advance()?;

                return Some((left, right));
            } else if self.scan_dont() {
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

impl FusedIterator for Multiplications<P1> {}
impl FusedIterator for Multiplications<P2> {}

pub fn part1() -> usize {
    Multiplications::<P1> {
        input: INPUT.as_bytes(),
        index: 0,
        _part: PhantomData,
    }
    .fold(0, |acc, (a, b)| acc + (a as usize * b as usize))
}

pub fn part2() -> usize {
    Multiplications::<P2> {
        input: INPUT.as_bytes(),
        index: 0,
        _part: PhantomData,
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
