use aoc2021::io::{read_lines, ReadLinesError};
use thiserror::Error;

use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

#[derive(Error, Debug)]
enum ParseCommandError {
    #[error("Invalid command: {}", .0)]
    InvalidCommand(String),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

impl FromStr for Command {
    type Err = ParseCommandError;
    fn from_str(s: &str) -> Result<Command, ParseCommandError> {
        if let Some((cmd, amount)) = s.split_once(" ") {
            let amount: i64 = amount.parse()?;
            let cmd = match cmd {
                "forward" => Command::Forward(amount),
                "down" => Command::Down(amount),
                "up" => Command::Up(amount),
                _ => return Err(ParseCommandError::InvalidCommand(s.to_string())),
            };

            Ok(cmd)
        } else {
            Err(ParseCommandError::InvalidCommand(s.to_string()))
        }
    }
}

#[derive(Debug, Default)]
struct State1 {
    x: i64,
    y: i64,
}

impl State1 {
    fn step(&mut self, cmd: &Command) {
        match cmd {
            Command::Forward(n) => {
                self.x += n;
            }
            Command::Down(n) => {
                self.y += n;
            }
            Command::Up(n) => {
                self.y -= n;
            }
        }
    }
}

#[derive(Debug, Default)]
struct State2 {
    x: i64,
    y: i64,
    aim: i64,
}

impl State2 {
    fn step(&mut self, cmd: &Command) {
        match cmd {
            Command::Forward(n) => {
                self.x += n;
                self.y += self.aim * n;
            }
            Command::Down(n) => {
                self.aim += n;
            }
            Command::Up(n) => {
                self.aim -= n;
            }
        }
    }
}

fn main() -> Result<(), ReadLinesError<Command>> {
    let data = read_lines("data/day02/input")?;

    let mut state1 = State1::default();
    let mut state2 = State2::default();
    for cmd in data {
        state1.step(&cmd);
        state2.step(&cmd);
    }

    println!("Part 1: {}", state1.x * state1.y);
    println!("Part 2: {}", state2.x * state2.y);

    Ok(())
}
