use failure::{format_err, Error};
use permutohedron::LexicalPermutation;
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct State {
    memory: Vec<i64>,
    ic: usize,
    inputs: Vec<i64>,
}

enum IntCodeResult {
    Output,
    Halt,
}

impl State {
    fn get_parameter(&self, parmodes: &Vec<u8>, ofs: usize) -> Result<i64> {
        let pm = parmodes.get(ofs - 1).unwrap_or(&0);
        let pv = self.memory[self.ic + ofs];

        let val = match pm {
            0 => {
                // position mode
                self.memory[pv as usize]
            }
            1 => {
                // immediate mode
                pv
            }
            _ => return Err(format_err!("Invalid parameter mode at {}: {}", ofs, pm)),
        };

        Ok(val)
    }

    fn get_opcode_and_parmode(&self) -> (i64, Vec<u8>) {
        let mut v = self.memory[self.ic];

        let opcode = v % 100;
        v /= 100;

        let mut parmodes = Vec::new();

        while v > 0 {
            let pm = v % 10;
            parmodes.push(pm as u8);
            v /= 10;
        }

        return (opcode, parmodes);
    }

    fn run(&mut self, outputs: &mut Vec<i64>) -> Result<IntCodeResult> {
        loop {
            let (opcode, parmodes) = self.get_opcode_and_parmode();

            match opcode {
                1 => {
                    // add
                    let a = self.get_parameter(&parmodes, 1)?;
                    let b = self.get_parameter(&parmodes, 2)?;
                    let pos_store = self.memory[self.ic + 3] as usize;

                    self.memory[pos_store] = a + b;
                    self.ic += 4;
                }
                2 => {
                    // mul
                    let a = self.get_parameter(&parmodes, 1)?;
                    let b = self.get_parameter(&parmodes, 2)?;
                    let pos_store = self.memory[self.ic + 3] as usize;

                    self.memory[pos_store] = a * b;
                    self.ic += 4;
                }
                3 => {
                    // input
                    let pos_store = self.memory[self.ic + 1] as usize;
                    let input_val = self.inputs.remove(0);
                    self.memory[pos_store] = input_val;
                    self.ic += 2;
                }
                4 => {
                    // output
                    let v = self.get_parameter(&parmodes, 1)?;
                    outputs.push(v);
                    self.ic += 2;
                    return Ok(IntCodeResult::Output);
                }
                5 => {
                    // jump-if-true
                    let a = self.get_parameter(&parmodes, 1)?;
                    let b = self.get_parameter(&parmodes, 2)?;

                    self.ic = if a != 0 { b as usize } else { self.ic + 3 };
                }
                6 => {
                    // jump-if false
                    let a = self.get_parameter(&parmodes, 1)?;
                    let b = self.get_parameter(&parmodes, 2)?;

                    self.ic = if a == 0 { b as usize } else { self.ic + 3 };
                }
                7 => {
                    // less-than
                    let a = self.get_parameter(&parmodes, 1)?;
                    let b = self.get_parameter(&parmodes, 2)?;
                    let pos_store = self.memory[self.ic + 3] as usize;

                    self.memory[pos_store] = if a < b { 1 } else { 0 };
                    self.ic += 4;
                }
                8 => {
                    // equals
                    let a = self.get_parameter(&parmodes, 1)?;
                    let b = self.get_parameter(&parmodes, 2)?;
                    let pos_store = self.memory[self.ic + 3] as usize;

                    self.memory[pos_store] = if a == b { 1 } else { 0 };
                    self.ic += 4;
                }
                99 => {
                    // halt
                    return Ok(IntCodeResult::Halt);
                }
                _ => return Err(format_err!("Invalid opcode: {}", opcode)),
            }
        }
    }
}

/*
fn run_chain(program: &Vec<i64>, phasing: &Vec<i64>) -> Result<Vec<i64>> {
    let mut input = 0;

    for p in phasing {
        let mut state = State {
            memory: program.clone(),
            ic: 0,
            inputs: vec![*p, input],
        };

        let mut outputs = Vec::new();

        loop {
            match state.run(&mut outputs)? {
                IntCodeResult::Output => {}
                IntCodeResult::Halt => break,
            }
        }

        input = outputs[0];
    }

    Ok(vec![input])
}
*/

fn run_in_loop(program: &Vec<i64>, phasing: &Vec<i64>) -> Result<Vec<i64>> {
    let mut state: Vec<RefCell<State>> = phasing
        .into_iter()
        .map(|p| {
            RefCell::new(State {
                memory: program.clone(),
                ic: 0,
                inputs: vec![*p],
            })
        })
        .collect();

    state[0].get_mut().inputs.push(0);

    let n_states = state.len();

    let mut current = 0;
    let mut running = n_states;

    let mut last_output = Vec::new();

    while running > 0 {
        let next = (current + 1) % n_states;
        let mut state_current = state[current].borrow_mut();
        let mut state_next = state[next].borrow_mut();

        match state_current.run(&mut state_next.inputs)? {
            IntCodeResult::Output => {
                last_output = state_next.inputs.clone();
            }
            IntCodeResult::Halt => {
                running -= 1;
            }
        }

        current = next;
    }

    Ok(last_output)
}

fn main() -> Result<()> {
    let mut f = File::open("data/day07/input")?;

    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    let buf = String::from_utf8(buf)?;

    let data: Vec<i64> = buf
        .trim()
        .split(",")
        .map(|v| v.parse().map_err(|e: std::num::ParseIntError| e.into()))
        .collect::<Result<Vec<i64>>>()?;

    println!("FIRST RUN");

    let mut phasing = [0, 1, 2, 3, 4];

    let mut max_signal = 0;
    loop {
        let out = run_in_loop(&data, &phasing.to_vec())?[0];

        if out > max_signal {
            println!("Got new max signal {} using phasing {:?}", out, phasing);
            max_signal = out;
        }

        if !phasing.next_permutation() {
            break;
        }
    }

    println!("SECOND RUN");

    let mut phasing = [5, 6, 7, 8, 9];

    let mut max_signal = 0;
    loop {
        let out = run_in_loop(&data, &phasing.to_vec())?[0];

        if out > max_signal {
            println!("Got new max signal {} using phasing {:?}", out, phasing);
            max_signal = out;
        }

        if !phasing.next_permutation() {
            break;
        }
    }

    Ok(())
}
