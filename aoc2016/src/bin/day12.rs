use anyhow::Result;
use aoc2016::asmbunny::{Instruction, State};

fn main() -> Result<()> {
    let instructions: Vec<Instruction> = aoc::io::read_lines("data/day12/input")?;

    let mut state = State::from_instructions(instructions.clone());
    while (state.ic >= 0) && (state.ic < instructions.len() as i64) {
        state.step();
    }

    println!("Part 1: {:#?}", state.registers[0]);

    let mut state: State = State::from_instructions(instructions.clone());
    state.registers[2] = 1;
    while (state.ic >= 0) && (state.ic < instructions.len() as i64) {
        state.step();
    }

    println!("Part 2: {:#?}", state.registers[0]);

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
