use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};
use aoc::direction::Direction;
use itertools::Itertools;

trait Instruction {
    fn get_data(&self) -> (Direction, usize);
}

#[derive(Debug, Clone)]
struct Part1Instruction {
    direction: Direction,
    amount: usize,
}

impl FromStr for Part1Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        if tokens.len() != 3 {
            bail!("Bad instruction line: '{}' - wrong token count", s);
        }

        let direction = match tokens[0] {
            "U" => Direction::North,
            "D" => Direction::South,
            "L" => Direction::West,
            "R" => Direction::East,
            _ => bail!("Bad direction: '{}'", tokens[0]),
        };

        let amount: usize = tokens[1].parse().context("Parse amount")?;

        Ok(Self { direction, amount })
    }
}

impl Instruction for Part1Instruction {
    fn get_data(&self) -> (Direction, usize) {
        (self.direction, self.amount)
    }
}

#[derive(Debug, Clone)]
struct Part2Instruction {
    direction: Direction,
    amount: usize,
}

impl FromStr for Part2Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        if tokens.len() != 3 {
            bail!("Bad instruction line: '{}' - wrong token count", s);
        }

        let mut code = tokens[2]
            .trim_start_matches("(#")
            .trim_end_matches(")")
            .to_string();
        if code.len() != 6 {
            bail!("Bad hex code: '{:?}'", code);
        }

        let dir_code = usize::from_str_radix(&code.pop().expect("non-empty").to_string(), 16)?;
        let direction = match dir_code {
            0 => Direction::East,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::North,
            _ => bail!("Bad direction code: {}", dir_code),
        };

        let amount = usize::from_str_radix(&code, 16).context("Parse amount")?;

        Ok(Self { direction, amount })
    }
}

impl Instruction for Part2Instruction {
    fn get_data(&self) -> (Direction, usize) {
        (self.direction, self.amount)
    }
}

fn dig_instructions<I: Instruction>(instructions: &[I]) -> isize {
    let mut i = 0;
    let mut j = 0;

    let mut distance = 0;
    let mut vertices = Vec::new();
    vertices.push([i, j]);

    for inst in instructions {
        let (direction, amount) = inst.get_data();
        let [di, dj] = direction.dpos();

        for _ in 0..amount {
            i += di as isize;
            j += dj as isize;
        }

        vertices.push([i, j]);

        distance += amount as isize;
    }

    let area = vertices
        .iter()
        .tuple_windows()
        .fold(0, |acc, ([i0, j0], [i1, j1])| acc + (j0 * i1) - (i0 * j1));

    let area = (distance + area) / 2 + 1;

    area
}

fn main() -> Result<()> {
    let path = "data/day18/input";

    let instructions: Vec<Part1Instruction> = aoc::io::read_lines(path)?;
    let area = dig_instructions(&instructions[..]);
    println!("Part 1: {}", area);

    let instructions: Vec<Part2Instruction> = aoc::io::read_lines(path)?;
    let area = dig_instructions(&instructions[..]);
    println!("Part 2: {}", area);

    Ok(())
}
