use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use lazy_static::lazy_static;
use regex::Regex;
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Number parsing error: {}", source))]
    ParseNumber { source: std::num::ParseIntError },

    #[snafu(display("Instruction parsing error"))]
    ParseInstruction,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
struct Mask {
    data: Vec<Option<u8>>,
}

impl Mask {
    fn apply_to_value(&self, mut v: usize) -> usize {
        for (i, m) in self.data.iter().enumerate() {
            let i = self.data.len() - i - 1;
            let n = v & (1 << i);

            // value:  000000000000000000000000000000001011  (decimal 11)
            // mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            // result: 000000000000000000000000000001001001  (decimal 73)

            if let Some(m) = m {
                // override bit i with m
                v = v & !n | ((*m as usize) << i);
            }
        }

        v
    }

    fn apply_to_memory(&self, mut addr: usize, v: usize, mem: &mut HashMap<usize, usize>) {
        let mut floating_bits = Vec::new();
        for (i, m) in self.data.iter().enumerate() {
            let i = self.data.len() - i - 1;
            match m {
                // If the bitmask bit is 0, the corresponding memory address bit is unchanged.
                Some(0) => {}
                // If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
                Some(1) => addr |= 1 << i,
                // If the bitmask bit is X, the corresponding memory address bit is floating.
                None => {
                    floating_bits.push(i);
                }
                _ => panic!("Non-binary values in bitmask"),
            }
        }

        /// recursively choose values for all floating bits
        /// and set memory addressess accordingly
        fn floating_set(
            addr: usize,
            floating: &[usize],
            v: usize,
            mem: &mut HashMap<usize, usize>,
        ) {
            if floating.is_empty() {
                mem.insert(addr, v);
                return;
            }

            // grab a single bit in addr
            let i = floating[0];
            let n = addr & (1 << i);

            // set that bit to either 0 or 1
            let a0 = addr & !n;
            let a1 = a0 | (1 << i);

            // apply recursively with partially applied bits
            floating_set(a0, &floating[1..], v, mem);
            floating_set(a1, &floating[1..], v, mem);
        }

        floating_set(addr, &floating_bits[..], v, mem);
    }
}

impl std::str::FromStr for Mask {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let data: Vec<_> = s
            .chars()
            .map(|c| match c {
                '0' => Ok(Some(0)),
                '1' => Ok(Some(1)),
                'X' => Ok(None),
                _ => Err(Error::ParseInstruction),
            })
            .collect::<Result<_>>()?;

        Ok(Mask { data })
    }
}

impl std::fmt::Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|m| m.map(|c| ('0' as u8 + c) as char).unwrap_or('X'))
                .collect::<String>()
        )
    }
}

enum Instruction {
    SetMask { mask: Mask },
    SetMemory { address: usize, value: usize },
}

impl Instruction {
    fn run_part1(&self, state: &mut State) {
        match self {
            Instruction::SetMask { mask } => state.mask = mask.clone(),
            Instruction::SetMemory { address, value } => {
                let corrected_value = state.mask.apply_to_value(*value);
                state.mem.insert(*address, corrected_value);
            }
        }
    }

    fn run_part2(&self, state: &mut State) {
        match self {
            Instruction::SetMask { mask } => state.mask = mask.clone(),
            Instruction::SetMemory { address, value } => {
                state.mask.apply_to_memory(*address, *value, &mut state.mem);
            }
        }
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::SetMask { mask } => write!(f, "mask = {:?}", mask),
            Instruction::SetMemory { address, value } => write!(f, "mem[{}] = {}", address, value),
        }
    }
}

impl std::str::FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE_MEM: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
            static ref RE_MASK: Regex = Regex::new(r"^mask = ([01X]+)$").unwrap();
        }

        if let Some(m) = RE_MEM.captures(s) {
            let address = m.get(1).unwrap().as_str().parse().context(ParseNumber)?;
            let value = m.get(2).unwrap().as_str().parse().context(ParseNumber)?;
            Ok(Instruction::SetMemory { address, value })
        } else if let Some(m) = RE_MASK.captures(s) {
            let mask = m.get(1).unwrap().as_str().parse()?;
            Ok(Instruction::SetMask { mask })
        } else {
            Err(Error::ParseInstruction)
        }
    }
}

#[derive(Debug)]
struct State {
    mask: Mask,
    mem: HashMap<usize, usize>,
}

impl State {
    fn new() -> Self {
        let data = (0..36).map(|_| None).collect();
        let mask = Mask { data };
        let mem = HashMap::new();
        State { mask, mem }
    }
}

fn main() -> Result<()> {
    let filename = "data/day14/input";
    let f = File::open(filename).context(Io)?;

    let instructions: Vec<Instruction> = BufReader::new(f)
        .lines()
        .map(|l| l.context(Io)?.parse())
        .collect::<Result<_>>()?;

    let mut state = State::new();
    for inst in &instructions {
        inst.run_part1(&mut state);
    }

    let mut memsum = 0;
    for v in state.mem.values() {
        memsum += v;
    }

    println!("Part 1: memory sum = {}", memsum);

    let mut state = State::new();
    for inst in &instructions {
        inst.run_part2(&mut state);
    }

    let mut memsum = 0;
    for v in state.mem.values() {
        memsum += v;
    }

    println!("Part 2: memory sum = {}", memsum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_to_value() {
        let mask: Mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse().unwrap();

        // value:  000000000000000000000000000000001011  (decimal 11)
        // mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        // result: 000000000000000000000000000001001001  (decimal 73)
        assert_eq!(mask.apply_to_value(0b1011), 0b1001001);

        // value:  000000000000000000000000000001100101  (decimal 101)
        // mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        // result: 000000000000000000000000000001100101  (decimal 101)
        assert_eq!(mask.apply_to_value(0b1100101), 0b1100101);

        // value:  000000000000000000000000000000000000  (decimal 0)
        // mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        // result: 000000000000000000000000000001000000  (decimal 64)
        assert_eq!(mask.apply_to_value(0b0), 0b1000000);
    }
}
