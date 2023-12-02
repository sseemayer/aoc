use std::{collections::HashSet, io::Write};

use anyhow::Result;
use aoc2016::asmbunny::{Instruction, State, StepResult};

fn main() -> Result<()> {
    let instructions: Vec<Instruction> = aoc::io::read_lines("data/day25/input")?;

    let mut a = 1;
    loop {
        let mut state = State::from_instructions(instructions.clone());
        state.registers[0] = a;

        print!("\na = {:5}: ", a);

        let mut found = true;
        let mut seen_states = HashSet::new();
        let mut n_outputs = 0;
        while !seen_states.contains(&state) {
            seen_states.insert(state.clone());

            match state.step() {
                StepResult::OutOfProgram => {
                    found = false;
                    println!("\nOut of program!");
                    break;
                }
                StepResult::OkNoOutput => {}
                StepResult::OkOutput { out } => {
                    print!("{}", out);
                    if out != n_outputs % 2 {
                        found = false;
                        //println!("\nInvalid output #{}: {}", n_outputs, out);
                    }
                    n_outputs += 1;

                    std::io::stdout().flush().unwrap();
                }
            }
        }
        // println!("\nLooping after {} states!", seen_states.len());

        if found && n_outputs >= 2 {
            println!("\nPart 1: a={}", a);
            break;
        }

        a += 1;
    }

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
