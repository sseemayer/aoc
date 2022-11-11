use std::collections::HashMap;

use aoc::io::{read_lines, ReadLinesError};
use snafu::{ResultExt, Snafu};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Error reading commands: '{}'", source))]
    ReadLines { source: ReadLinesError<Command> },
}

#[derive(Debug, Snafu)]
enum ParseCommandError {
    #[snafu(display("Int format error for '{}': {}", data, source))]
    ParseInt {
        data: String,
        source: std::num::ParseIntError,
    },

    #[snafu(display("Bad command: '{}", data))]
    BadCommand { data: String },
}

#[derive(Debug, Clone)]
enum Value {
    Literal(i64),
    Register(String),
}

impl std::str::FromStr for Value {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Ok(v) = s.parse::<i64>() {
            Ok(Value::Literal(v))
        } else {
            Ok(Value::Register(s.to_string()))
        }
    }
}

#[derive(Debug, Clone)]
enum Command {
    Set { x: String, y: Value },
    Sub { x: String, y: Value },
    Mul { x: String, y: Value },
    Jnz { x: Value, y: Value },
}

impl std::str::FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.trim().split_whitespace().collect();

        match tokens[0] {
            "set" => {
                let x = tokens[1].to_string();
                let y: Value = tokens[2].parse()?;
                Ok(Command::Set { x, y })
            }
            "sub" => {
                let x = tokens[1].to_string();
                let y: Value = tokens[2].parse()?;
                Ok(Command::Sub { x, y })
            }
            "mul" => {
                let x = tokens[1].to_string();
                let y: Value = tokens[2].parse()?;
                Ok(Command::Mul { x, y })
            }
            "jnz" => {
                let x: Value = tokens[1].parse()?;
                let y: Value = tokens[2].parse()?;
                Ok(Command::Jnz { x, y })
            }
            _ => Err(ParseCommandError::BadCommand {
                data: s.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    ip: i64,
    program: Vec<Command>,
    registers: HashMap<String, i64>,

    n_mul: usize,
}

impl State {
    fn from_program(program: &[Command]) -> Self {
        let program = program.to_vec();
        let mut registers = HashMap::new();

        State {
            ip: 0,
            program,
            registers,

            n_mul: 0,
        }
    }

    fn get_value(&self, value: &Value) -> i64 {
        match value {
            Value::Literal(v) => *v,
            Value::Register(r) => *self.registers.get(r).unwrap_or(&0),
        }
    }

    fn step(&mut self) -> Option<i64> {
        let cmd = &self.program[self.ip as usize];

        // println!(
        //     "ip={} sigs={:?} regs={:?} cmd={:?}",
        //     self.ip, self.signals, self.registers, cmd
        // );

        match cmd {
            Command::Set { x, y } => {
                let vy = self.get_value(y);
                self.registers.insert(x.to_string(), vy);
                self.ip += 1;
            }
            Command::Sub { x, y } => {
                let vx = *self.registers.get(x).unwrap_or(&0);
                let vy = self.get_value(y);
                self.registers.insert(x.to_string(), vx - vy);
                self.ip += 1;
            }
            Command::Mul { x, y } => {
                let vx = *self.registers.get(x).unwrap_or(&0);
                let vy = self.get_value(y);
                self.registers.insert(x.to_string(), vx * vy);
                self.ip += 1;

                self.n_mul += 1;
            }
            Command::Jnz { x, y } => {
                let vx = self.get_value(x);
                let vy = self.get_value(y);

                if vx != 0 {
                    self.ip += vy;
                } else {
                    self.ip += 1;
                }
            }
        };

        None
    }

    fn run_single(&mut self) {
        while self.ip >= 0 && self.ip < self.program.len() as i64 {
            self.step();
        }
    }
}

fn decompiled(b0: i64, c: i64) -> i64 {
    let mut h = 0;
    for b in (b0..=c).step_by(17) {
        if !is_prime(b) {
            h += 1;
        }
    }
    h
}

fn is_prime(b: i64) -> bool {
    for d in 2..b {
        if b % d == 0 {
            return false;
        }
    }
    true
}

fn main() -> Result<()> {
    let program: Vec<Command> = read_lines("data/day23/input").context(ReadLines)?;

    let mut state = State::from_program(&program[..]);
    state.run_single();

    println!("Part 1: {}", state.n_mul);

    println!("Part 2: {}", decompiled(106500, 123500));

    Ok(())
}
