use std::{collections::HashSet, io::Write};

use aoc2016::asmbunny::{AsmError, Instruction, State, StepResult};
use snafu::{ResultExt, Snafu};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Asmbunny error: {}", source))]
    Asm { source: AsmError },
}

fn main() -> Result<()> {
    let instructions: Vec<Instruction> = std::fs::read_to_string("data/day25/input")
        .context(Io)?
        .lines()
        .map(|l| l.parse().context(Asm))
        .collect::<Result<_>>()?;

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
                aoc2016::asmbunny::StepResult::OutOfProgram => {
                    found = false;
                    println!("\nOut of program!");
                    break;
                }
                aoc2016::asmbunny::StepResult::OkNoOutput => {}
                aoc2016::asmbunny::StepResult::OkOutput { out } => {
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
