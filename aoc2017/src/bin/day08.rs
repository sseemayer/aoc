use std::collections::HashMap;

use snafu::{ResultExt, Snafu};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Int format error for '{}': {}", data, source))]
    ParseInt {
        data: String,
        source: std::num::ParseIntError,
    },

    #[snafu(display("Invalid relation '{}'", data))]
    ParseRelation { data: String },

    #[snafu(display("Invalid condition '{}'", data))]
    ParseCondition { data: String },

    #[snafu(display("Invalid operation '{}'", data))]
    ParseOperation { data: String },

    #[snafu(display("Invalid instruction '{}'", data))]
    ParseInstruction { data: String },
}

#[derive(Debug, Clone)]
struct Instruction {
    register: String,
    operation: Operation,
    amount: i64,
    condition: Condition,
}

impl std::str::FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        if tokens.len() != 7 || tokens[3] != "if" {
            return Err(Error::ParseInstruction {
                data: s.to_string(),
            });
        }

        let register = tokens[0].to_string();
        let operation: Operation = tokens[1].parse()?;
        let amount: i64 = tokens[2].parse().context(ParseInt {
            data: tokens[2].to_string(),
        })?;

        let condition: Condition = tokens[4..].join(" ").parse()?;

        Ok(Instruction {
            register,
            operation,
            amount,
            condition,
        })
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Inc,
    Dec,
}

impl std::str::FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "inc" => Operation::Inc,
            "dec" => Operation::Dec,
            _ => {
                return Err(Error::ParseOperation {
                    data: s.to_string(),
                })
            }
        })
    }
}

#[derive(Debug, Clone)]
struct Condition {
    register: String,
    relation: Relation,
    threshold: i64,
}

impl std::str::FromStr for Condition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        if tokens.len() != 3 {
            return Err(Error::ParseCondition {
                data: s.to_string(),
            });
        }

        let register = tokens[0].to_string();
        let relation: Relation = tokens[1].parse()?;
        let threshold: i64 = tokens[2].parse().context(ParseInt {
            data: tokens[2].to_string(),
        })?;

        Ok(Condition {
            register,
            relation,
            threshold,
        })
    }
}

#[derive(Debug, Clone)]
enum Relation {
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Equal,
    UnEqual,
}

impl std::str::FromStr for Relation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            ">" => Relation::Greater,
            ">=" => Relation::GreaterOrEqual,
            "<" => Relation::Less,
            "<=" => Relation::LessOrEqual,
            "==" => Relation::Equal,
            "!=" => Relation::UnEqual,
            _ => {
                return Err(Error::ParseRelation {
                    data: s.to_string(),
                })
            }
        })
    }
}

#[derive(Debug, Default)]
struct State {
    registers: HashMap<String, i64>,
}

impl State {
    fn new() -> Self {
        Default::default()
    }
}

impl Condition {
    fn check(&self, state: &State) -> bool {
        let reg_val = *state.registers.get(&self.register).unwrap_or(&0);
        match self.relation {
            Relation::Greater => reg_val > self.threshold,
            Relation::GreaterOrEqual => reg_val >= self.threshold,
            Relation::Less => reg_val < self.threshold,
            Relation::LessOrEqual => reg_val <= self.threshold,
            Relation::Equal => reg_val == self.threshold,
            Relation::UnEqual => reg_val != self.threshold,
        }
    }
}

impl Instruction {
    fn run(&self, state: &mut State) {
        if !self.condition.check(state) {
            return;
        }

        let reg = state
            .registers
            .entry(self.register.to_string())
            .or_insert(0);

        match self.operation {
            Operation::Inc => *reg += self.amount,
            Operation::Dec => *reg -= self.amount,
        }
    }
}

fn main() -> Result<()> {
    let instructions: Vec<Instruction> = std::fs::read_to_string("data/day08/input")
        .context(Io)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_>>()?;

    let mut state = State::new();

    let mut max_intermediate_val = 0;
    for inst in instructions {
        inst.run(&mut state);

        let miv = state.registers.values().max().unwrap();
        if *miv > max_intermediate_val {
            max_intermediate_val = *miv;
        }
    }

    let max_val = state.registers.values().max().unwrap();

    println!("Part 1: {}", max_val);
    println!("Part 2: {}", max_intermediate_val);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        Ok(())
    }
}
