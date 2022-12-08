use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{bail, Context, Result};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Hash, PartialEq, Eq, EnumIter, Clone)]
enum Opcode {
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

impl Opcode {
    fn execute(&self, params: &Params, state: &mut State) {
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
struct State {
    registers: HashMap<i64, i64>,
}

#[derive(Debug, Clone)]
struct Params {
    a: i64,
    b: i64,
    c: i64,
}

impl State {
    fn set(&mut self, address: i64, value: i64) {
        self.registers.insert(address, value);
    }

    fn get(&self, address: i64) -> i64 {
        *self.registers.get(&address).unwrap_or(&0)
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

#[derive(Debug)]
struct Input {
    examples: Vec<Example>,
    instructions: Vec<Instruction<i64>>,
}

#[derive(Debug, Clone)]
struct Instruction<T> {
    opcode: T,
    params: Params,
}

impl Instruction<i64> {
    fn to_executable(&self, mapping: &HashMap<i64, Opcode>) -> Instruction<Opcode> {
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

enum ParserState {
    Examples,
    Instructions,
}

impl Input {
    fn parse(path: &str) -> Result<Self> {
        let mut state = ParserState::Examples;

        let mut examples = Vec::new();
        let mut instructions = Vec::new();

        let mut state_from: Option<State> = None;
        let mut instruction: Option<Instruction<i64>> = None;
        let mut state_to: Option<State> = None;

        for line in BufReader::new(File::open(path)?).lines() {
            let line = line?;

            match state {
                ParserState::Examples => match (&state_from, &instruction, &state_to) {
                    (None, _, _) => {
                        if line.starts_with("Before: ") {
                            state_from =
                                Some(line.trim_start_matches("Before: ").parse::<State>()?);
                        } else if line.trim().is_empty() {
                            state = ParserState::Instructions;
                        } else {
                            bail!("Unexpected line: '{}'", line);
                        }
                    }
                    (Some(_), None, _) => {
                        instruction = Some(line.parse::<Instruction<i64>>()?);
                    }
                    (Some(_), Some(_), None) => {
                        state_to = Some(line.trim_start_matches("After: ").parse::<State>()?);
                    }
                    (Some(b), Some(i), Some(a)) => {
                        if !line.trim().is_empty() {
                            bail!("Expected empty line: {}", line);
                        }

                        examples.push(Example {
                            before: b.clone(),
                            instruction: i.clone(),
                            after: a.clone(),
                        });

                        state_from = None;
                        instruction = None;
                        state_to = None;
                    }
                },
                ParserState::Instructions => {
                    if line.trim().is_empty() {
                        continue;
                    }
                    instructions.push(line.parse::<Instruction<i64>>()?);
                }
            }
        }

        if let (Some(b), Some(i), Some(a)) = (state_from, instruction, state_to) {
            examples.push(Example {
                before: b.clone(),
                instruction: i.clone(),
                after: a.clone(),
            });
        }

        Ok(Self {
            examples,
            instructions,
        })
    }
}

#[derive(Debug)]
struct Example {
    before: State,
    after: State,
    instruction: Instruction<i64>,
}

impl Example {
    fn infer_opcode(&self) -> HashSet<Opcode> {
        let mut out = HashSet::new();

        for inst in Opcode::iter() {
            let mut new_state = self.before.clone();
            inst.execute(&self.instruction.params, &mut new_state);

            if &new_state == &self.after {
                out.insert(inst);
            }
        }

        out
    }
}

fn simplify(opcode_mapping: &mut HashMap<i64, HashSet<Opcode>>) -> HashMap<i64, Opcode> {
    let mut out = HashMap::new();
    loop {
        let mut simplified = None;

        for (oc, cands) in opcode_mapping.iter() {
            if cands.len() == 1 {
                let value = cands.iter().next().expect("Candidate").clone();
                // println!("{} is {:?}", oc, value);
                simplified = Some((*oc, value));
                break;
            }
        }

        if let Some((oc, value)) = simplified {
            for cands in opcode_mapping.values_mut() {
                cands.remove(&value);
            }

            out.insert(oc, value);
        } else {
            break;
        }
    }

    out
}

fn main() -> Result<()> {
    let input = Input::parse("data/day16/input")?;

    let mut three_or_mores = 0;

    let mut opcode_candidates: HashMap<i64, HashSet<Opcode>> = HashMap::new();
    for ex in &input.examples {
        let inferred = ex.infer_opcode();
        // println!("{} could be {:?}", ex.instruction.opcode, inferred);

        if inferred.len() >= 3 {
            three_or_mores += 1;
        }

        let candidates = opcode_candidates
            .entry(ex.instruction.opcode)
            .or_insert_with(|| inferred.clone());

        *candidates = &candidates.clone() & &inferred;
    }

    println!("Part 1: {}", three_or_mores);

    let opcode_solved = simplify(&mut opcode_candidates);

    let program = input
        .instructions
        .iter()
        .map(|i| i.to_executable(&opcode_solved))
        .collect::<Vec<_>>();

    let mut state = State::default();
    for instruction in program {
        instruction.opcode.execute(&instruction.params, &mut state);
    }

    println!("Part 2: {}", state.get(0));

    Ok(())
}
