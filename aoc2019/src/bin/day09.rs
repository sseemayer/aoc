use failure::{format_err, Error};
use std::fs::File;
use std::io::Read;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct State {
    memory: Vec<i64>,
    ic: usize,
    inputs: Vec<i64>,
    relative_base: i64,
}

enum IntCodeResult {
    Output,
    Halt,
}

impl State {
    fn get_memory(&mut self, address: usize) -> i64 {
        while self.memory.len() <= address {
            self.memory.push(0);
        }

        self.memory[address]
    }

    fn set_memory(&mut self, address: usize, value: i64) {
        while self.memory.len() <= address {
            self.memory.push(0);
        }

        self.memory[address] = value;
    }

    fn get_address(&mut self, parmodes: &Vec<u8>, ofs: usize) -> Result<usize> {
        let pm = parmodes.get(ofs - 1).unwrap_or(&0);
        let pv = self.memory[self.ic + ofs];

        let addr = match pm {
            0 => {
                // position mode
                pv as usize
            }
            2 => {
                // relative mode
                (self.relative_base + pv) as usize
            }
            _ => return Err(format_err!("Invalid parameter mode at {}: {}", ofs, pm)),
        };

        Ok(addr)
    }

    fn get_parameter(&mut self, parmodes: &Vec<u8>, ofs: usize) -> Result<i64> {
        let pm = parmodes.get(ofs - 1).unwrap_or(&0);
        let pv = self.memory[self.ic + ofs];

        let val = match pm {
            0 => {
                // position mode
                self.get_memory(pv as usize)
            }
            1 => {
                // immediate mode
                pv
            }
            2 => {
                // relative mode
                self.get_memory((self.relative_base + pv) as usize)
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
                    let pos_store = self.get_address(&parmodes, 3)?;

                    self.set_memory(pos_store, a + b);
                    self.ic += 4;
                }
                2 => {
                    // mul
                    let a = self.get_parameter(&parmodes, 1)?;
                    let b = self.get_parameter(&parmodes, 2)?;
                    let pos_store = self.get_address(&parmodes, 3)?;

                    self.set_memory(pos_store, a * b);
                    self.ic += 4;
                }
                3 => {
                    // input
                    let pos_store = self.get_address(&parmodes, 1)? as usize;
                    let input_val = self.inputs.remove(0);
                    self.set_memory(pos_store, input_val);
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
                    let pos_store = self.get_address(&parmodes, 3)?;

                    self.set_memory(pos_store, if a < b { 1 } else { 0 });
                    self.ic += 4;
                }
                8 => {
                    // equals
                    let a = self.get_parameter(&parmodes, 1)?;
                    let b = self.get_parameter(&parmodes, 2)?;
                    let pos_store = self.get_address(&parmodes, 3)?;

                    self.set_memory(pos_store, if a == b { 1 } else { 0 });
                    self.ic += 4;
                }
                9 => {
                    // shift relative base
                    let a = self.get_parameter(&parmodes, 1)?;
                    self.relative_base += a;
                    self.ic += 2;
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

fn run_simple(program: &Vec<i64>, inputs: Vec<i64>) -> Result<Vec<i64>> {
    let mut state = State {
        memory: program.clone(),
        ic: 0,
        inputs,
        relative_base: 0,
    };

    let mut outputs = Vec::new();

    // run to completion
    loop {
        match state.run(&mut outputs)? {
            IntCodeResult::Halt => break,
            _ => {}
        }
    }

    Ok(outputs)
}

fn main() -> Result<()> {
    // /*
    let mut f = File::open("data/day09/input")?;

    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    let buf = String::from_utf8(buf)?;
    // */
    let program: Vec<i64> = buf
        .trim()
        .split(",")
        .map(|v| v.parse().map_err(|e: std::num::ParseIntError| e.into()))
        .collect::<Result<Vec<i64>>>()?;

    println!("FIRST RUN");

    println!("Outputs: {:?}", run_simple(&program, vec![1])?);

    println!("SECOND RUN");

    println!("Outputs: {:?}", run_simple(&program, vec![2])?);

    Ok(())
}
