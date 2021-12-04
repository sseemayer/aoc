use aoc2021::io::{read_lines, ReadLinesError};
use thiserror::Error;

use std::str::FromStr;

#[derive(Clone)]
struct BitString(Vec<u8>);

impl std::fmt::Debug for BitString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.iter() {
            write!(f, "{}", b)?;
        }

        Ok(())
    }
}

impl BitString {
    fn not(&self) -> Self {
        let bits = self.iter().map(|d| if *d == 1 { 0 } else { 1 }).collect();
        Self(bits)
    }
    fn to_number(&self) -> usize {
        let mut out = 0usize;
        for d in self.iter() {
            out <<= 1;
            out += *d as usize;
        }
        out
    }
}

impl std::ops::Deref for BitString {
    type Target = Vec<u8>;
    fn deref(&self) -> &Vec<u8> {
        &self.0
    }
}

impl FromStr for BitString {
    type Err = ParseBitStringError;
    fn from_str(s: &str) -> Result<BitString, ParseBitStringError> {
        let bits: Vec<u8> = s
            .chars()
            .filter_map(|c| c.to_digit(2))
            .map(|d| d as u8)
            .collect();

        if bits.len() != s.len() {
            return Err(ParseBitStringError::BadBits(s.to_string()));
        }

        Ok(BitString(bits))
    }
}

#[derive(Error, Debug)]
enum ParseBitStringError {
    #[error("Bad bit string: {}", .0)]
    BadBits(String),
}

fn majority_vote_pos(strings: &[BitString], i: usize) -> u8 {
    let mut n = 0;
    for s in strings.iter() {
        if s[i] == 1 {
            n += 1;
        }
    }

    if n * 2 >= strings.len() {
        1
    } else {
        0
    }
}

fn majority_vote(strings: &[BitString]) -> BitString {
    let mut out = Vec::new();
    for i in 0..strings[0].len() {
        out.push(majority_vote_pos(strings, i));
    }

    BitString(out)
}

fn eliminate(input: &[BitString], invert: bool) -> BitString {
    let mut strings = input.to_vec();
    let mut idx = 0;

    while strings.len() > 1 {
        let bit = majority_vote_pos(&strings[..], idx);
        let bit = if invert { 1 - bit } else { bit };

        strings.retain(|s| s[idx] == bit);

        idx += 1;
    }

    strings.pop().expect("Found a single code")
}

fn main() -> Result<(), ReadLinesError<BitString>> {
    let data = read_lines("data/day03/input")?;

    let part1 = majority_vote(&data[..]);

    println!(
        "Part 1: {} [{:?}]",
        part1.to_number() * part1.not().to_number(),
        part1
    );

    let o2_generator_rating = eliminate(&data[..], false);
    let co2_scrubber_rating = eliminate(&data[..], true);
    println!(
        "Part 2: {} [{:?} * {:?}]",
        o2_generator_rating.to_number() * co2_scrubber_rating.to_number(),
        o2_generator_rating,
        co2_scrubber_rating
    );

    Ok(())
}
