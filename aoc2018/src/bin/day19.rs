use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

use aoc2018::vm::{Instruction, InstructionPointer, Opcode, State, Vm};

fn parse(path: &str) -> Result<Vm> {
    let mut ip = InstructionPointer::Managed(0);
    let mut instructions = Vec::new();
    for line in BufReader::new(File::open(path)?).lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        if line.starts_with("#ip ") {
            let ip_register: i64 = line
                .trim_start_matches("#ip ")
                .parse()
                .context("Parse IP register")?;

            ip = InstructionPointer::Register(ip_register);

            continue;
        }

        let instruction: Instruction<Opcode> = line.trim().parse()?;
        instructions.push(instruction);
    }

    Ok(Vm::new(instructions, ip))
}

fn fast_run(r4: usize) -> usize {
    // the decompiled code is just a factorization of r4 and summing up of factors.
    let mut r0 = 0;
    for r3 in 1..r4 {
        if r4 % r3 == 0 {
            r0 += r4 / r3;
        }
    }
    r0 + 1
}

fn main() -> Result<()> {
    let vm = parse("data/day19/input")?;

    let mut vm_part1 = vm.clone();
    vm_part1.debug = false;
    vm_part1.run_to_end();
    println!("Part 1: {}", vm_part1.state.get(0));

    println!("Part 1, fast: {}", fast_run(958));

    println!("Part 2: {}", fast_run(10551358));

    Ok(())
}
