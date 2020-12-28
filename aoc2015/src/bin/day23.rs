use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use aoc2015::Result;
use failure::{bail, Error};

#[derive(Debug, Default, Clone)]
struct State {
    reg_a: usize,
    reg_b: usize,
    ic: usize,
}

impl State {
    fn get_reg(&self, register: &Register) -> &usize {
        match register {
            Register::A => &self.reg_a,
            Register::B => &self.reg_b,
        }
    }

    fn get_reg_mut(&mut self, register: &Register) -> &mut usize {
        match register {
            Register::A => &mut self.reg_a,
            Register::B => &mut self.reg_b,
        }
    }

    fn jump_by_offset(&mut self, offset: i64) {
        let new_ic = (self.ic as i64) + offset;
        if new_ic < 0 {
            panic!("IC is {}", new_ic)
        }
        self.ic = new_ic as usize;
    }
}

#[derive(Debug, Clone)]
enum Register {
    A,
    B,
}

impl std::str::FromStr for Register {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            _ => bail!("Invalid register: {}", s),
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Hlf { r: Register },
    Tpl { r: Register },
    Inc { r: Register },
    Jmp { offset: i64 },
    Jie { r: Register, offset: i64 },
    Jio { r: Register, offset: i64 },
}

impl std::str::FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let op = &s[..3];
        let arg = &s[3..].trim();

        match op {
            "hlf" => {
                let r: Register = arg.parse()?;
                Ok(Instruction::Hlf { r })
            }
            "tpl" => {
                let r: Register = arg.parse()?;
                Ok(Instruction::Tpl { r })
            }
            "inc" => {
                let r: Register = arg.parse()?;
                Ok(Instruction::Inc { r })
            }
            "jmp" => {
                let offset: i64 = arg.parse()?;
                Ok(Instruction::Jmp { offset })
            }
            "jie" => {
                let args: Vec<_> = arg.split(",").collect();
                let r: Register = args[0].trim().parse()?;
                let offset: i64 = args[1].trim().parse()?;
                Ok(Instruction::Jie { r, offset })
            }
            "jio" => {
                let args: Vec<_> = arg.split(",").collect();
                let r: Register = args[0].trim().parse()?;
                let offset: i64 = args[1].trim().parse()?;
                Ok(Instruction::Jio { r, offset })
            }
            _ => bail!("Invalid instruction: {}", s),
        }
    }
}

impl Instruction {
    fn run(&self, state: &mut State) {
        match self {
            Instruction::Hlf { r } => *state.get_reg_mut(r) /= 2,
            Instruction::Tpl { r } => *state.get_reg_mut(r) *= 3,
            Instruction::Inc { r } => *state.get_reg_mut(r) += 1,
            Instruction::Jmp { offset } => {
                state.jump_by_offset(*offset);
                return;
            }
            Instruction::Jie { r, offset } => {
                if state.get_reg(r) % 2 == 0 {
                    state.jump_by_offset(*offset);
                    return;
                }
            }
            Instruction::Jio { r, offset } => {
                if *state.get_reg(r) == 1 {
                    state.jump_by_offset(*offset);
                    return;
                }
            }
        }
        state.ic += 1;
    }
}

fn main() -> Result<()> {
    let br = BufReader::new(File::open("data/day23/input")?);
    let instructions: Vec<Instruction> = br.lines().map(|l| l?.parse()).collect::<Result<_>>()?;

    let mut state: State = Default::default();
    while let Some(inst) = instructions.get(state.ic) {
        inst.run(&mut state);
    }

    println!("Part 1: {:?}", state);

    let mut state: State = Default::default();
    state.reg_a = 1;
    while let Some(inst) = instructions.get(state.ic) {
        inst.run(&mut state);
    }
    println!("Part 2: {:?}", state);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
}
