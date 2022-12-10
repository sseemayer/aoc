use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
enum Operation {
    NoOp,
    AddX(i32),
}

impl Operation {
    fn begin_execution(&self) -> usize {
        match self {
            Operation::NoOp => 0,
            Operation::AddX(_) => 1,
        }
    }

    fn finish_execution(&self, state: &mut State) {
        match self {
            Operation::NoOp => {}
            Operation::AddX(v) => state.x += v,
        }
    }
}

impl std::str::FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.trim().split_whitespace().collect::<Vec<_>>();

        match tokens[0] {
            "addx" => {
                let dx: i32 = tokens[1].parse()?;
                Ok(Operation::AddX(dx))
            }
            "noop" => Ok(Operation::NoOp),
            _ => Err(anyhow!("Bad op: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
enum Tile {
    On,
    Off,
}

type Map = aoc::map::Map<[i32; 2], Tile>;

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::On => write!(f, "█"),
            Tile::Off => write!(f, " "),
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    program: Vec<Operation>,
    ip: usize,

    x: i32,
    signal_strength: i32,

    clock: usize,
    delay: usize,

    screen: Map,
}

impl State {
    fn new(program: Vec<Operation>) -> Self {
        Self {
            program,
            ip: 0,

            x: 1,
            signal_strength: 0,

            clock: 0,
            delay: 0,

            screen: Map::new(),
        }
    }

    fn step(&mut self) -> bool {
        if self.delay == 0 {
            if self.clock > 0 {
                let finished_op = self.program[self.ip].clone();
                finished_op.finish_execution(self);
                self.ip += 1;
            }

            if self.ip >= self.program.len() {
                return false;
            }

            let starting_op = &self.program[self.ip];
            self.delay = starting_op.begin_execution()
        } else {
            self.delay -= 1;
        }

        self.clock += 1;

        let i = ((self.clock - 1) / 40) as i32;
        let j = ((self.clock - 1) % 40) as i32;

        let on_sprite = j >= self.x - 1 && j <= self.x + 1;

        self.screen
            .set([i, j], if on_sprite { Tile::On } else { Tile::Off });

        if self.clock >= 20 && (self.clock - 20) % 40 == 0 {
            // println!("TICK!");
            self.signal_strength += self.x * self.clock as i32;
        }

        // println!(
        //     "t={} d={} x={} s={} ({} {}) o={}",
        //     self.clock, self.delay, self.x, self.signal_strength, i, j, on_sprite
        // );

        true
    }

    fn run(&mut self) {
        while self.step() {
            // working hard
        }
    }
}

fn parse(path: &str) -> Result<Vec<Operation>> {
    BufReader::new(File::open(path)?)
        .lines()
        .map(|l| l?.parse())
        .collect()
}

fn main() -> Result<()> {
    let program = parse("data/day10/input")?;

    let mut state = State::new(program);
    state.run();

    println!("Part 1: {}", state.signal_strength);
    println!("Part 2:\n{}", state.screen);

    Ok(())
}
