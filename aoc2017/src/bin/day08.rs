use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};

#[derive(Debug, Clone)]
struct Instruction {
    register: String,
    operation: Operation,
    amount: i64,
    condition: Condition,
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        if tokens.len() != 7 || tokens[3] != "if" {
            return Err(anyhow!("Bad instruction: '{}", s));
        }

        let register = tokens[0].to_string();
        let operation: Operation = tokens[1].parse()?;
        let amount: i64 = tokens[2].parse().context("Parse amount")?;

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
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "inc" => Operation::Inc,
            "dec" => Operation::Dec,
            _ => return Err(anyhow!("Bad operation: '{}'", s)),
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
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        if tokens.len() != 3 {
            return Err(anyhow!("Bad condition: '{}'", s));
        }

        let register = tokens[0].to_string();
        let relation: Relation = tokens[1].parse()?;
        let threshold: i64 = tokens[2].parse().context("Parse threshold")?;

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
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            ">" => Relation::Greater,
            ">=" => Relation::GreaterOrEqual,
            "<" => Relation::Less,
            "<=" => Relation::LessOrEqual,
            "==" => Relation::Equal,
            "!=" => Relation::UnEqual,
            _ => return Err(anyhow!("Bad relation: '{}'", s)),
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
    let instructions: Vec<Instruction> = aoc::io::read_lines("data/day08/input")?;

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
