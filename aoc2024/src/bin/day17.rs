use std::collections::HashMap;

use anyhow::{anyhow, bail, Context, Error, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_REGISTER: Regex =
        Regex::new(r"^Register (\w): (-?\d+)\s*$").expect("Valid regex");
}

#[derive(Debug, Clone)]
enum Opcode {
    /// The adv instruction (opcode 0) performs division.
    /// The numerator is the value in the A register.
    /// The denominator is found by raising 2 to the power of the instruction's combo operand.
    /// (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
    /// The result of the division operation is truncated to an integer and then written to the A register.
    Adv(ComboOperand),

    /// The bxl instruction (opcode 1) calculates the bitwise XOR of register B
    /// and the instruction's literal operand, then stores the result in register B.
    Bxl(isize),

    /// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
    /// (thereby keeping only its lowest 3 bits), then writes that value to the B register.
    Bst(ComboOperand),

    /// The jnz instruction (opcode 3) does nothing if the A register is 0.
    /// However, if the A register is not zero, it jumps by setting the instruction pointer
    /// to the value of its literal operand; if this instruction jumps, the instruction pointer
    /// is not increased by 2 after this instruction.
    Jnz(usize),

    /// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
    /// then stores the result in register B. (For legacy reasons, this instruction reads
    /// an operand but ignores it.)
    Bxc(isize),

    /// The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
    /// then outputs that value. (If a program outputs multiple values, they are separated by commas.)
    Out(ComboOperand),

    /// The bdv instruction (opcode 6) works exactly like the adv instruction
    /// except that the result is stored in the B register. (The numerator is still read from the A register.)
    Bdv(ComboOperand),

    /// The cdv instruction (opcode 7) works exactly like the adv instruction
    /// except that the result is stored in the C register. (The numerator is still read from the A register.)
    Cdv(ComboOperand),
}

impl Opcode {
    fn from_tape(opcode: u8, operand: u8) -> Result<Self> {
        let x = match opcode {
            0 => Opcode::Adv(ComboOperand::from_tape(operand)),
            1 => Opcode::Bxl(operand as isize),
            2 => Opcode::Bst(ComboOperand::from_tape(operand)),
            3 => Opcode::Jnz(operand as usize),
            4 => Opcode::Bxc(operand as isize),
            5 => Opcode::Out(ComboOperand::from_tape(operand)),
            6 => Opcode::Bdv(ComboOperand::from_tape(operand)),
            7 => Opcode::Cdv(ComboOperand::from_tape(operand)),
            _ => {
                bail!("Bad opcode: {}", opcode);
            }
        };

        Ok(x)
    }
}

#[derive(Debug, Clone)]
enum ComboOperand {
    Literal(isize),
    Register(char),
}

impl ComboOperand {
    fn from_tape(operand: u8) -> Self {
        match operand {
            4 => ComboOperand::Register('A'),
            5 => ComboOperand::Register('B'),
            6 => ComboOperand::Register('C'),
            _ => ComboOperand::Literal(operand as isize),
        }
    }

    fn get_value(&self, registers: &HashMap<char, isize>) -> isize {
        match self {
            ComboOperand::Literal(v) => *v,
            ComboOperand::Register(r) => *registers.get(r).unwrap_or(&0),
        }
    }
}

#[derive(Debug, Clone)]
struct Bitcode {
    regs: HashMap<char, isize>,

    ip: usize,
    tape: Vec<u8>,
}

impl std::str::FromStr for Bitcode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (registers, program) = s
            .split_once("\n\n")
            .ok_or(anyhow!("Expect delimiter line"))?;

        let mut regs = HashMap::new();
        for line in registers.lines() {
            if let Some(m) = RE_REGISTER.captures(line) {
                let reg = m
                    .get(1)
                    .expect("group 1 exists")
                    .as_str()
                    .chars()
                    .next()
                    .expect("non-empty group 1");

                let val: isize = m
                    .get(2)
                    .expect("group 2 exiss")
                    .as_str()
                    .parse()
                    .context("Parse register value")?;

                regs.insert(reg, val);
            }
        }

        let mut tape = Vec::new();
        for line in program.strip_prefix("Program: ").unwrap_or(program).lines() {
            tape.extend(
                line.split(",")
                    .map(|v| v.parse::<u8>().context("Parse tape"))
                    .collect::<Result<Vec<_>>>()?,
            );
        }

        Ok(Self { regs, ip: 0, tape })
    }
}

#[derive(Debug, Clone)]
enum StepResult {
    Terminated,
    Working,
    Output(u8),
}

impl Bitcode {
    fn get_reg(&self, register: char) -> isize {
        *self.regs.get(&register).unwrap_or(&0)
    }

    fn step(&mut self) -> Result<StepResult> {
        let Some(&opcode) = self.tape.get(self.ip) else {
            return Ok(StepResult::Terminated);
        };

        let Some(&operand) = self.tape.get(self.ip + 1) else {
            return Ok(StepResult::Terminated);
        };

        let opcode = Opcode::from_tape(opcode, operand)?;

        // println!("{:?}", opcode);

        match opcode {
            Opcode::Adv(op) => {
                // The adv instruction (opcode 0) performs division.

                // The numerator is the value in the A register.
                let num = self.get_reg('A');

                // The denominator is found by raising 2 to the power of the instruction's combo operand.
                let shift = op.get_value(&self.regs);

                // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
                let val = num >> shift;

                // The result of the division operation is truncated to an integer and then written to the A register.
                self.regs.insert('A', val);
            }
            Opcode::Bxl(op) => {
                // The bxl instruction (opcode 1) calculates the bitwise XOR of register B
                // and the instruction's literal operand, then stores the result in register B.

                let num = self.get_reg('B');
                let val = num ^ op;
                self.regs.insert('B', val);
            }
            Opcode::Bst(op) => {
                // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
                // (thereby keeping only its lowest 3 bits), then writes that value to the B register.

                let val = op.get_value(&self.regs) % 8;
                self.regs.insert('B', val);
            }
            Opcode::Jnz(op) => {
                // The jnz instruction (opcode 3) does nothing if the A register is 0.

                let num = self.get_reg('A');

                if num != 0 {
                    // However, if the A register is not zero, it jumps by setting the instruction pointer
                    // to the value of its literal operand; if this instruction jumps, the instruction pointer
                    // is not increased by 2 after this instruction.

                    self.ip = op as usize;
                    return Ok(StepResult::Working);
                }
            }
            Opcode::Bxc(_op) => {
                // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
                // then stores the result in register B. (For legacy reasons, this instruction reads
                // an operand but ignores it.)

                let b = self.get_reg('B');
                let c = self.get_reg('C');

                let val = b ^ c;
                self.regs.insert('B', val);
            }
            Opcode::Out(op) => {
                // The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
                // then outputs that value. (If a program outputs multiple values, they are separated by commas.)

                let val = (op.get_value(&self.regs) % 8) as u8;
                self.ip += 2;

                return Ok(StepResult::Output(val));
            }
            Opcode::Bdv(op) => {
                // The bdv instruction (opcode 6) works exactly like the adv instruction
                // except that the result is stored in the B register. (The numerator is still read from the A register.)

                // The numerator is the value in the A register.
                let num = self.get_reg('A');

                // The denominator is found by raising 2 to the power of the instruction's combo operand.
                let shift = op.get_value(&self.regs);

                // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
                let val = num >> shift;

                // The result of the division operation is truncated to an integer and then written to the B register.
                self.regs.insert('B', val);
            }
            Opcode::Cdv(op) => {
                // The cdv instruction (opcode 7) works exactly like the adv instruction
                // except that the result is stored in the C register. (The numerator is still read from the A register.)

                // The numerator is the value in the A register.
                let num = self.get_reg('A');

                // The denominator is found by raising 2 to the power of the instruction's combo operand.
                let shift = op.get_value(&self.regs);

                // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
                let val = num >> shift;

                // The result of the division operation is truncated to an integer and then written to the C register.
                self.regs.insert('C', val);
            }
        }

        self.ip += 2;
        Ok(StepResult::Working)
    }

    fn run(&mut self) -> Result<Vec<u8>> {
        let mut outputs = Vec::new();

        loop {
            match self.step()? {
                StepResult::Terminated => break,
                StepResult::Working => {}
                StepResult::Output(v) => outputs.push(v),
            }
        }

        Ok(outputs)
    }

    fn run_with_a(&self, a: isize) -> Result<Vec<u8>> {
        let mut candidate = self.clone();
        candidate.regs.insert('A', a);
        candidate.run()
    }

    fn find_quine(&self) -> Result<isize> {
        let mut valid = vec![0];

        for len in 1..=self.tape.len() {
            let mut new_valid = Vec::new();
            for n in valid {
                for i in 0..=8 {
                    let m = 8 * n + i;
                    let output = self.run_with_a(m)?;
                    if output == self.tape[self.tape.len() - len..] {
                        new_valid.push(m);
                    }
                }
            }

            // println!(
            //     "{:?} will produce {:?}",
            //     new_valid,
            //     &self.tape[self.tape.len() - len..]
            // );

            valid = new_valid;
        }

        valid.into_iter().min().ok_or(anyhow!("No solution found"))
    }
}

fn main() -> Result<()> {
    let prog: Bitcode = aoc::io::read_all((2024, 17))?.parse()?;
    //let prog: Bitcode = aoc::io::read_all("data/day17/example")?.parse()?;

    let outputs = prog.clone().run()?;

    println!(
        "Part 1: {}",
        outputs.iter().map(|v| format!("{}", v)).join(",")
    );

    let quine_input = prog.find_quine()?;

    println!("Part 2: {}", quine_input);

    Ok(())
}
