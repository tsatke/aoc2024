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

    fn scan(&mut self, s: &str) -> bool {
        self.index + s.len() <= self.input.len()
            && &self.input[self.index..self.index + s.len()] == s.as_bytes()
    }

    fn parse_num(&mut self) -> Option<u16> {
        let mut left = 0;
        for i in 0..3 {
            let c = self.input[self.index];
            if !c.is_ascii_digit() {
                if i == 0 {
                    // not even 1 digit
                    return None;
                }
                break;
            }
            let digit = self.input[self.index] - b'0';
            left = left * 10 + u16::from(digit);

            self.advance()?;
        }
        Some(left)
    }
}

impl Iterator for Multiplications<P1> {
    type Item = (u16, u16);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.input.len() {
            return None;
        }

        'outer: loop {
            if self.scan("mul(") {
                self.index += 4;

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
            if self.scan("mul(") {
                // 'mul'
                self.index += 4;

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
            } else if self.scan("don't()") {
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

#[must_use]
pub fn part1() -> usize {
    eval_muls(Multiplications::<P1> {
        input: INPUT.as_bytes(),
        index: 0,
        _part: PhantomData,
    })
}

#[must_use]
pub fn part2() -> usize {
    eval_muls(Multiplications::<P2> {
        input: INPUT.as_bytes(),
        index: 0,
        _part: PhantomData,
    })
}

fn eval_muls(muls: impl Iterator<Item = (u16, u16)>) -> usize {
    muls.fold(0, |acc, (a, b)| acc + (a as usize * b as usize))
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
