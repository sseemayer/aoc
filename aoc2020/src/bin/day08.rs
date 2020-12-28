use snafu::{ResultExt, Snafu};
use std::collections::HashSet;
use std::fs::File;

use aoc2020::code::{CodeError, Instruction, ParseError, Program, State};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Code execution error: {}", source))]
    Code { source: CodeError },

    #[snafu(display("Code parsing error: {}", source))]
    Parse { source: ParseError },
}

type Result<T> = std::result::Result<T, Error>;

fn run_until_loop(program: Program) -> (bool, State) {
    let mut state = State::with_program(program);
    let mut seen_ics = HashSet::new();

    loop {
        // println!(
        //     "@{:6}: {:?}",
        //     state.ic,
        //     state.program.instructions.get(state.ic)
        // );
        if seen_ics.contains(&state.ic) {
            return (true, state);
        }

        seen_ics.insert(state.ic);

        if let Err(CodeError::OutOfProgram { .. }) = state.step() {
            return (false, state);
        }
    }
}

fn main() -> Result<()> {
    let mut f = File::open("data/day08/input").context(Io)?;
    let program = State::parse_program(&mut f).context(Parse)?;

    // part 1
    let (_, state) = run_until_loop(program.clone());
    println!("Accumulator after first loop: {}\n\n", state.accumulator);

    // part 2
    for i in 0..program.instructions.len() {
        let mut patched_program = program.clone();

        let old_instruction = program.instructions[i].clone();

        patched_program.instructions[i] = match old_instruction {
            Instruction::Nop { delta } => Instruction::Jmp { delta },
            Instruction::Jmp { delta } => Instruction::Nop { delta },
            _ => continue,
        };

        println!(
            "@{:6}: {:?} -> {:?}",
            i, old_instruction, patched_program.instructions[i]
        );

        let (looped, state) = run_until_loop(patched_program);

        if !looped {
            println!("Terminated! Accumulator: {}", state.accumulator);
            break;
        }
    }

    Ok(())
}
