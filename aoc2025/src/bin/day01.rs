use colored::Colorize;
use std::str::FromStr;

use anyhow::{Context, Error, Result, bail};

/// A move consisting of a signed number of steps.
///
/// Negative steps indicate left turns, positive steps indicate right turns.
#[derive(Clone, Copy)]
struct Move(i32);

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (dir_char, dist_str) = s
            .chars()
            .next()
            .and_then(|c| Some((c, &s[1..])))
            .context("Empty move string")?;

        let distance: i32 = dist_str
            .parse()
            .with_context(|| format!("Invalid distance: {}", dist_str))?;

        let distance = match dir_char {
            'L' => -distance,
            'R' => distance,
            _ => bail!("Invalid direction character: {}", dir_char),
        };

        Ok(Move(distance))
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir_char = if self.0 < 0 { 'L' } else { 'R' };
        write!(f, "{}{}", dir_char, self.0.abs())
    }
}

fn main() -> Result<()> {
    //let moves: Vec<Move> = aoc::io::read_lines("data/day01/example")?;
    let moves: Vec<Move> = aoc::io::read_lines((2025, 1))?;

    let mut pos: i32 = 50;
    let mut n_zero_stop: usize = 0;
    let mut n_zero_pass: usize = 0;
    for Move(delta) in moves {
        // invariant: pos is always a positive number between 0 and 99

        // compute new position, possibly negative
        let new_pos = pos + delta;

        // count how many times we pass zero
        if delta > 0 {
            n_zero_pass += ((pos + delta) / 100).abs() as usize;
        } else {
            n_zero_pass += ((pos + delta) / 100).abs() as usize;

            if pos > 0 && new_pos <= 0 {
                // also count first time we cross or touch zero
                n_zero_pass += 1;
            }
        }

        pos = new_pos.rem_euclid(100);

        // count how many times we stop at zero
        if pos == 0 {
            n_zero_stop += 1;
        }

        // println!(
        //     "{:?}, pos: {}, p1={}, p2={}",
        //     Move(delta),
        //     pos,
        //     n_zero_stop,
        //     n_zero_pass
        // );
    }

    println!("{} {}", "Part 1:".bold().green(), n_zero_stop);
    println!("{} {}", "Part 2:".bold().green(), n_zero_pass);

    Ok(())
}
