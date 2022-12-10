use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{bail, Result};
use strum::IntoEnumIterator;

use aoc2018::vm::{Instruction, InstructionPointer, Opcode, State, Vm};

#[derive(Debug)]
struct Input {
    examples: Vec<Example>,
    instructions: Vec<Instruction<i64>>,
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

    let mut vm = Vm::new(program, InstructionPointer::Managed(0));
    vm.run_to_end();

    println!("Part 2: {}", vm.state.get(0));

    Ok(())
}
