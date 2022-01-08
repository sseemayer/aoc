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
    Snd { x: Value },
    Set { x: String, y: Value },
    Add { x: String, y: Value },
    Mul { x: String, y: Value },
    Mod { x: String, y: Value },
    Rcv { x: Value },
    Jgz { x: Value, y: Value },
}

impl std::str::FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.trim().split_whitespace().collect();

        match tokens[0] {
            "snd" => {
                let x: Value = tokens[1].parse()?;
                Ok(Command::Snd { x })
            }
            "set" => {
                let x = tokens[1].to_string();
                let y: Value = tokens[2].parse()?;
                Ok(Command::Set { x, y })
            }
            "add" => {
                let x = tokens[1].to_string();
                let y: Value = tokens[2].parse()?;
                Ok(Command::Add { x, y })
            }
            "mul" => {
                let x = tokens[1].to_string();
                let y: Value = tokens[2].parse()?;
                Ok(Command::Mul { x, y })
            }
            "mod" => {
                let x = tokens[1].to_string();
                let y: Value = tokens[2].parse()?;
                Ok(Command::Mod { x, y })
            }
            "rcv" => {
                let x: Value = tokens[1].parse()?;
                Ok(Command::Rcv { x })
            }
            "jgz" => {
                let x: Value = tokens[1].parse()?;
                let y: Value = tokens[2].parse()?;
                Ok(Command::Jgz { x, y })
            }
            _ => Err(ParseCommandError::BadCommand {
                data: s.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    is_part1: bool,

    ip: i64,
    n_sent: usize,
    signals: Vec<i64>,
    program: Vec<Command>,
    registers: HashMap<String, i64>,
}

impl State {
    fn from_program(program: &[Command], p_value: Option<i64>, is_part1: bool) -> Self {
        let signals = Vec::new();
        let program = program.to_vec();
        let mut registers = HashMap::new();

        if let Some(v) = p_value {
            registers.insert("p".to_string(), v);
        }

        State {
            is_part1,
            ip: 0,
            n_sent: 0,
            signals,
            program,
            registers,
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
            Command::Snd { x } => {
                let v = self.get_value(&x);
                self.n_sent += 1;
                self.ip += 1;
                return Some(v);
            }
            Command::Set { x, y } => {
                let vy = self.get_value(y);
                self.registers.insert(x.to_string(), vy);
                self.ip += 1;
            }
            Command::Add { x, y } => {
                let vx = *self.registers.get(x).unwrap_or(&0);
                let vy = self.get_value(y);
                self.registers.insert(x.to_string(), vx + vy);
                self.ip += 1;
            }
            Command::Mul { x, y } => {
                let vx = *self.registers.get(x).unwrap_or(&0);
                let vy = self.get_value(y);
                self.registers.insert(x.to_string(), vx * vy);
                self.ip += 1;
            }
            Command::Mod { x, y } => {
                let vx = *self.registers.get(x).unwrap_or(&0);
                let vy = self.get_value(y);
                self.registers.insert(x.to_string(), vx % vy);
                self.ip += 1;
            }
            Command::Rcv { x } => {
                if self.is_part1 {
                    let vx = self.get_value(x);
                    if vx != 0 {
                        let v = self.signals.pop().expect("Sent sound");
                        println!("Part 1: {}", v);

                        self.ip = -100;
                    }
                } else {
                    if !self.signals.is_empty() {
                        let v = self.signals.remove(0);

                        if let Value::Register(r) = x {
                            self.registers.insert(r.to_string(), v);
                        }
                        self.ip += 1;
                    }
                }
            }
            Command::Jgz { x, y } => {
                let vx = self.get_value(x);
                let vy = self.get_value(y);

                if vx > 0 {
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
            if let Some(v) = self.step() {
                self.signals.push(v);
            }
        }
    }
}

fn main() -> Result<()> {
    let program: Vec<Command> = read_lines("data/day18/input").context(ReadLines)?;

    let mut state = State::from_program(&program[..], None, true);
    state.run_single();

    let mut state_a = State::from_program(&program[..], Some(0), false);
    let mut state_b = State::from_program(&program[..], Some(1), false);

    loop {
        let live_a = state_a.ip >= 0 && (state_a.ip as usize) < program.len();
        let live_b = state_b.ip >= 0 && (state_b.ip as usize) < program.len();

        let last_a = state_a.ip;
        let last_b = state_b.ip;

        if live_a {
            if let Some(v) = state_a.step() {
                state_b.signals.push(v);
            }
        }

        if live_b {
            if let Some(v) = state_b.step() {
                state_a.signals.push(v);
            }
        }

        if last_a == state_a.ip && last_b == state_b.ip {
            // deadlock
            break;
        }

        if !live_a && !live_b {
            break;
        }
    }

    println!("Part 2: {}", state_b.n_sent);

    Ok(())
}
