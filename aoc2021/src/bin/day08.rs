use aoc2021::io::{read_lines, ReadLinesError};
use thiserror::Error;

use std::{collections::HashMap, str::FromStr};

//  aaaa
// b    c
// b    c
//  dddd
// e    f
// e    f
//  gggg
//
//  digit | abcdefg
//      0 | 1110111
//      1 | 0010010
//      2 | 1011101
//      3 | 1011011
//      4 | 0111010
//      5 | 1101011
//      6 | 1101111
//      7 | 1010010
//      8 | 1111111
//      9 | 1111011

#[derive(Debug)]
struct Display {
    digits: Vec<Digit>,
    output: Vec<Digit>,
}

#[derive(Error, Debug)]
enum ParseDisplayError {
    #[error("Bad line: '{}'", .0)]
    BadLine(String),

    #[error(transparent)]
    Digit(#[from] ParseDigitError),
}

impl FromStr for Display {
    type Err = ParseDisplayError;
    fn from_str(s: &str) -> Result<Display, ParseDisplayError> {
        let (digits, output) = s
            .split_once(" | ")
            .ok_or(ParseDisplayError::BadLine(s.to_string()))?;

        let digits = digits
            .trim()
            .split_whitespace()
            .map(|d| Ok(d.parse()?))
            .collect::<Result<Vec<Digit>, ParseDisplayError>>()?;

        let output = output
            .trim()
            .split_whitespace()
            .map(|d| Ok(d.parse()?))
            .collect::<Result<Vec<Digit>, ParseDisplayError>>()?;

        Ok(Display { digits, output })
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Digit(u8);

impl std::fmt::Debug for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:07b}", self.0)
    }
}

impl Digit {
    fn get_bit(&self, n: usize) -> u8 {
        (self.0 >> (6 - n)) % 2
    }

    fn count_ones(&self) -> usize {
        let mut sum = 0;
        for n in 0..7 {
            sum += self.get_bit(n) as usize;
        }
        sum
    }
}

#[derive(Error, Debug)]
enum ParseDigitError {}

impl FromStr for Digit {
    type Err = ParseDigitError;
    fn from_str(s: &str) -> Result<Digit, ParseDigitError> {
        let bits: u8 = s
            .chars()
            .map(|c| 1 << (6 - ((c as u8) - ('a' as u8))))
            .sum();

        Ok(Digit(bits))
    }
}

impl std::ops::BitAnd for Digit {
    type Output = Digit;

    fn bitand(self, rhs: Digit) -> Digit {
        Digit(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for Digit {
    type Output = Digit;

    fn bitor(self, rhs: Digit) -> Digit {
        Digit(self.0 | rhs.0)
    }
}

impl Display {
    fn solve(&self) -> HashMap<Digit, u8> {
        let mut solutions: HashMap<u8, Digit> = HashMap::new();

        // first pass: only solve the numbers identifiable by unique number of ones
        for d in self.digits.iter() {
            match d.count_ones() {
                2 => {
                    solutions.insert(1, *d);
                }
                3 => {
                    solutions.insert(7, *d);
                }
                4 => {
                    solutions.insert(4, *d);
                }
                7 => {
                    solutions.insert(8, *d);
                }
                _ => {}
            }
        }

        // second pass: use bit logic to solve ambiguous cases by creating numbers from already-solved numbers
        let one = solutions[&1];
        let four = solutions[&4];
        let eight = solutions[&8];
        for d in self.digits.iter() {
            match d.count_ones() {
                5 => {
                    // we have a 2, 3, or 5
                    if *d & one == one {
                        //  ++++
                        //      X
                        //      X
                        //  ++++
                        //      X
                        //      X
                        //  ++++
                        solutions.insert(3, *d);
                    } else if *d | four == eight {
                        //  ++++
                        // x    X
                        // x    X
                        //  XXXX
                        // +    x
                        // +    x
                        //  ++++
                        solutions.insert(2, *d);
                    } else {
                        solutions.insert(5, *d);
                    }
                }
                6 => {
                    // we have a 0, 6, or 9
                    if *d & four == four {
                        //  ++++
                        // X    X
                        // X    X
                        //  XXXX
                        //      X
                        //      X
                        //  ++++
                        solutions.insert(9, *d);
                    } else if *d & one == one {
                        //  ++++
                        // +    X
                        // +    X
                        //
                        // +    X
                        // +    X
                        //  ++++
                        solutions.insert(0, *d);
                    } else {
                        solutions.insert(6, *d);
                    }
                }
                _ => {}
            }
        }

        let mut out = HashMap::new();
        for (k, v) in solutions.into_iter() {
            out.insert(v, k);
        }

        out
    }
}

fn main() -> Result<(), ReadLinesError<Display>> {
    let data = read_lines("data/day08/input")?;

    let mut counts: HashMap<u8, usize> = HashMap::new();
    let mut total_value = 0;
    for display in data {
        let solution = display.solve();

        let mut value = 0;
        for digit in display.output {
            let n = solution[&digit];
            value *= 10;
            value += n as usize;
            *counts.entry(n).or_default() += 1;
        }

        total_value += value;
    }

    println!(
        "Part 1: {}",
        counts[&1] + counts[&4] + counts[&7] + counts[&8]
    );
    println!("Part 2: {}", total_value);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_digit() {
        let dig1: Digit = "dab".parse().unwrap();
        let dig2: Digit = "bad".parse().unwrap();
        assert_eq!(dig1, dig2);
    }
}
