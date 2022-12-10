use std::{
    collections::HashMap,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{bail, Context, Result};
use strum::EnumIter;

#[derive(Debug, Hash, PartialEq, Eq, EnumIter, Clone)]
pub enum Opcode {
    AddR,
    AddI,
    MulR,
    MulI,
    BAnR,
    BAnI,
    BOrR,
    BOrI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

impl std::str::FromStr for Opcode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "addr" => Ok(Opcode::AddR),
            "addi" => Ok(Opcode::AddI),
            "mulr" => Ok(Opcode::MulR),
            "muli" => Ok(Opcode::MulI),
            "banr" => Ok(Opcode::BAnR),
            "bani" => Ok(Opcode::BAnI),
            "borr" => Ok(Opcode::BOrR),
            "bori" => Ok(Opcode::BOrI),
            "setr" => Ok(Opcode::SetR),
            "seti" => Ok(Opcode::SetI),
            "gtir" => Ok(Opcode::GtIR),
            "gtri" => Ok(Opcode::GtRI),
            "gtrr" => Ok(Opcode::GtRR),
            "eqir" => Ok(Opcode::EqIR),
            "eqri" => Ok(Opcode::EqRI),
            "eqrr" => Ok(Opcode::EqRR),
            _ => bail!("Bad opcode: {}", s),
        }
    }
}

impl Opcode {
    pub fn execute(&self, params: &Params, state: &mut State) {
        let a = params.a;
        let b = params.b;
        let c = params.c;
        match self {
            // addr (add register) stores into register C the result of adding register A and register B.
            Opcode::AddR => state.set(c, state.get(a) + state.get(b)),

            // addi (add immediate) stores into register C the result of adding register A and value B.
            Opcode::AddI => state.set(c, state.get(a) + b),

            // mulr (multiply register) stores into register C the result of multiplying register A and register B.
            Opcode::MulR => state.set(c, state.get(a) * state.get(b)),

            // muli (multiply immediate) stores into register C the result of multiplying register A and value B.
            Opcode::MulI => state.set(c, state.get(a) * b),

            // banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
            Opcode::BAnR => state.set(c, state.get(a) & state.get(b)),

            // bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
            Opcode::BAnI => state.set(c, state.get(a) & b),

            // borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
            Opcode::BOrR => state.set(c, state.get(a) | state.get(b)),

            // bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
            Opcode::BOrI => state.set(c, state.get(a) | b),

            // setr (set register) copies the contents of register A into register C. (Input B is ignored.)
            Opcode::SetR => state.set(c, state.get(a)),

            // seti (set immediate) stores value A into register C. (Input B is ignored.)
            Opcode::SetI => state.set(c, a),

            // gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
            Opcode::GtIR => state.set(c, if a > state.get(b) { 1 } else { 0 }),

            // gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
            Opcode::GtRI => state.set(c, if state.get(a) > b { 1 } else { 0 }),

            // gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
            Opcode::GtRR => state.set(c, if state.get(a) > state.get(b) { 1 } else { 0 }),

            // eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
            Opcode::EqIR => state.set(c, if a == state.get(b) { 1 } else { 0 }),

            // eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
            Opcode::EqRI => state.set(c, if state.get(a) == b { 1 } else { 0 }),

            // eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
            Opcode::EqRR => state.set(c, if state.get(a) == state.get(b) { 1 } else { 0 }),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct State {
    registers: HashMap<i64, i64>,
}

impl State {
    pub fn set(&mut self, address: i64, value: i64) {
        self.registers.insert(address, value);
    }

    pub fn get(&self, address: i64) -> i64 {
        *self.registers.get(&address).unwrap_or(&0)
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut regs = self.registers.iter().collect::<Vec<_>>();
        regs.sort();

        let regs = regs
            .iter()
            .map(|(k, v)| format!("r{}={}", k, v))
            .collect::<Vec<String>>();

        write!(f, "[{}]", regs.join(", "))
    }
}

impl std::str::FromStr for State {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let registers = s
            .trim()
            .trim_start_matches("[")
            .trim_end_matches("]")
            .split(", ")
            .enumerate()
            .map(|(i, v)| Ok((i as i64, v.parse::<i64>()?)))
            .collect::<Result<HashMap<i64, i64>>>()?;

        Ok(State { registers })
    }
}

#[derive(Debug, Clone)]
pub struct Params {
    a: i64,
    b: i64,
    c: i64,
}

impl std::fmt::Display for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.a, self.b, self.c)
    }
}

#[derive(Debug, Clone)]
pub struct Instruction<T> {
    pub opcode: T,
    pub params: Params,
}

impl<T> std::fmt::Display for Instruction<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.opcode, self.params)
    }
}

impl Instruction<i64> {
    pub fn to_executable(&self, mapping: &HashMap<i64, Opcode>) -> Instruction<Opcode> {
        let opcode = mapping
            .get(&self.opcode)
            .expect("Mappable instruction")
            .clone();

        Instruction {
            opcode,
            params: self.params.clone(),
        }
    }
}

impl std::str::FromStr for Instruction<i64> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<i64> = s
            .split_whitespace()
            .map(|v| v.parse::<i64>().context("Parse instruction tokens"))
            .collect::<Result<Vec<i64>>>()?;

        if tokens.len() != 4 {
            bail!("Expected 4 tokens: '{}'", s)
        }

        Ok(Self {
            opcode: tokens[0],
            params: Params {
                a: tokens[1],
                b: tokens[2],
                c: tokens[3],
            },
        })
    }
}

impl std::str::FromStr for Instruction<Opcode> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        if tokens.len() != 4 {
            bail!("Expected 4 tokens: '{}'", s)
        }

        let opcode: Opcode = tokens[0].parse()?;
        let a: i64 = tokens[1].parse()?;
        let b: i64 = tokens[2].parse()?;
        let c: i64 = tokens[3].parse()?;

        Ok(Self {
            opcode,
            params: Params { a, b, c },
        })
    }
}

#[derive(Debug, Clone)]
pub enum InstructionPointer {
    Managed(i64),
    Register(i64),
}

impl InstructionPointer {
    pub fn get(&self, state: &State) -> i64 {
        match self {
            InstructionPointer::Managed(m) => *m,
            InstructionPointer::Register(r) => state.get(*r),
        }
    }

    pub fn inc(&mut self, state: &mut State) {
        match self {
            InstructionPointer::Managed(m) => *m += 1,
            InstructionPointer::Register(r) => {
                let r = *r as i64;
                state.set(r, state.get(r) + 1)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vm {
    pub state: State,
    pub instructions: Vec<Instruction<Opcode>>,

    pub ip: InstructionPointer,

    pub debug: bool,
}

impl Vm {
    pub fn parse(path: &str) -> Result<Vm> {
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

    pub fn new(instructions: Vec<Instruction<Opcode>>, ip: InstructionPointer) -> Self {
        Self {
            state: State::default(),
            instructions,
            ip,
            debug: false,
        }
    }

    fn get_instruction(&self) -> Option<&Instruction<Opcode>> {
        let ip = self.ip.get(&self.state);

        if ip < 0 || ip as usize >= self.instructions.len() {
            return None;
        }

        let ip = ip as usize;

        let instruction = &self.instructions[ip];

        Some(instruction)
    }

    pub fn step(&mut self) -> bool {
        let instruction = if let Some(inst) = self.get_instruction() {
            inst.clone()
        } else {
            return false;
        };

        if self.debug {
            println!(
                "{} {} {}",
                self.ip.get(&self.state),
                instruction,
                self.state
            );
        }

        instruction
            .opcode
            .execute(&instruction.params, &mut self.state);

        self.ip.inc(&mut self.state);

        true
    }

    pub fn run_to_end(&mut self) {
        while self.step() {
            // keep going
        }
    }

    pub fn run_with_interrupt<F>(&mut self, mut interrupt: F)
    where
        F: FnMut(&mut Self) -> bool,
    {
        while interrupt(self) && self.step() {
            // keep going
        }
    }
}
