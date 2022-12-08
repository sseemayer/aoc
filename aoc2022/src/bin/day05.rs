use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

lazy_static! {
    static ref RE_INSTRUCTION: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

#[derive(Debug, Clone)]
struct State {
    stacks: HashMap<usize, VecDeque<char>>,

    instructions: VecDeque<Instruction>,
}

#[derive(Debug, Clone)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Error, Debug)]
enum ParseInstructionError {
    #[error("Bad instruction: '{}'", _0)]
    BadInstruction(String),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

impl std::str::FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE_INSTRUCTION
            .captures(s)
            .ok_or_else(|| ParseInstructionError::BadInstruction(s.to_string()))?;

        let count: usize = captures.get(1).expect("count").as_str().parse()?;
        let from: usize = captures.get(2).expect("from").as_str().parse()?;
        let to: usize = captures.get(3).expect("to").as_str().parse()?;

        Ok(Self {
            count,
            from: from - 1,
            to: to - 1,
        })
    }
}

enum ParserState {
    Stacks,
    Instructions,
}
impl State {
    fn parse(path: &str) -> Result<State> {
        let mut state = ParserState::Stacks;
        let mut stacks: HashMap<usize, VecDeque<char>> = HashMap::new();
        let mut instructions = VecDeque::new();
        for line in BufReader::new(File::open(path)?).lines() {
            let line = line?;

            match state {
                ParserState::Stacks => {
                    if line.trim().is_empty() {
                        state = ParserState::Instructions;
                        continue;
                    }

                    let line = line.chars().collect::<Vec<char>>();
                    for (i, ofs) in (0..line.len()).step_by(4).enumerate() {
                        let tokens = &line[ofs..ofs + 3];

                        if tokens[0] != '[' || tokens[2] != ']' {
                            continue;
                        }

                        stacks.entry(i).or_default().push_front(tokens[1]);
                    }
                }
                ParserState::Instructions => {
                    let instruction: Instruction = line.parse()?;
                    instructions.push_back(instruction);
                }
            }
        }

        Ok(State {
            stacks,
            instructions,
        })
    }

    fn step1(&mut self) -> Result<bool> {
        if let Some(Instruction { count, from, to }) = self.instructions.pop_front() {
            for _ in 0..count {
                let items = self.pop_n(from, 1)?;

                let to_stack = self
                    .stacks
                    .get_mut(&to)
                    .ok_or_else(|| anyhow!("invalid to stack {}", to))?;

                to_stack.extend(items.into_iter());
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn step2(&mut self) -> Result<bool> {
        if let Some(Instruction { count, from, to }) = self.instructions.pop_front() {
            let items = self.pop_n(from, count)?;

            let to_stack = self
                .stacks
                .get_mut(&to)
                .ok_or_else(|| anyhow!("invalid to stack {}", to))?;

            to_stack.extend(items.into_iter());

            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn pop_n(&mut self, from: usize, n: usize) -> Result<Vec<char>> {
        let from_stack = self
            .stacks
            .get_mut(&from)
            .ok_or_else(|| anyhow!("invalid from stack {}", from))?;

        let mut out = Vec::new();
        for _ in 0..n {
            let item = from_stack
                .pop_back()
                .ok_or_else(|| anyhow!("popping from empty stack {}", from))?;

            out.push(item);
        }

        out.reverse();

        Ok(out)
    }

    fn get_sorted(&self) -> Vec<(&usize, &VecDeque<char>)> {
        let mut stacks = self.stacks.iter().collect::<Vec<_>>();
        stacks.sort_by_key(|v| v.0);
        stacks
    }

    fn get_code(&self) -> String {
        self.get_sorted()
            .into_iter()
            .filter_map(|(_i, s)| s.back())
            .collect()
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, s) in self.get_sorted() {
            write!(f, "{:2}: {}\n", i, s.into_iter().collect::<String>())?;
        }

        Ok(())
    }
}

fn part1(mut state: State) -> Result<()> {
    while state.step1()? {
        // do work
    }

    println!("Part 1: {}", state.get_code());
    Ok(())
}

fn part2(mut state: State) -> Result<()> {
    while state.step2()? {
        // do work
    }

    println!("Part 2: {}", state.get_code());
    Ok(())
}

fn main() -> Result<()> {
    let state = State::parse("data/day05/input")?;

    part1(state.clone())?;
    part2(state.clone())?;

    Ok(())
}
