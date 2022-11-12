use std::collections::HashMap;

use snafu::{ResultExt, Snafu};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Int format error for '{}': {}", data, source))]
    ParseInt {
        data: String,
        source: std::num::ParseIntError,
    },
}

struct Operation {
    new_value: u8,
    direction: i8,
    new_state: char,
}

struct State {
    operations: [Operation; 2],
}

struct Program {
    states: HashMap<char, State>,
}

struct TuringMachine {
    tape: HashMap<i64, u8>,
    cursor: i64,
    state: char,

    steps_left: usize,
    program: Program,
}

impl TuringMachine {
    fn step(&mut self) {
        let tape_value = self.tape.get(&self.cursor).unwrap_or(&0);

        let operation = &self
            .program
            .states
            .get(&self.state)
            .expect("valid state")
            .operations[*tape_value as usize];

        self.tape.insert(self.cursor, operation.new_value);
        self.cursor += operation.direction as i64;
        self.state = operation.new_state;

        self.steps_left -= 1;
    }

    fn simulate(&mut self) {
        while self.steps_left > 0 {
            self.step()
        }
    }

    fn checksum(&self) -> usize {
        self.tape.values().map(|v| *v as usize).sum::<usize>()
    }
}

fn make_machine() -> TuringMachine {
    let mut states = HashMap::new();
    states.insert(
        'A',
        State {
            operations: [
                Operation {
                    new_value: 1,
                    direction: 1,
                    new_state: 'B',
                },
                Operation {
                    new_value: 0,
                    direction: -1,
                    new_state: 'B',
                },
            ],
        },
    );

    states.insert(
        'B',
        State {
            operations: [
                Operation {
                    new_value: 1,
                    direction: -1,
                    new_state: 'C',
                },
                Operation {
                    new_value: 0,
                    direction: 1,
                    new_state: 'E',
                },
            ],
        },
    );

    states.insert(
        'C',
        State {
            operations: [
                Operation {
                    new_value: 1,
                    direction: 1,
                    new_state: 'E',
                },
                Operation {
                    new_value: 0,
                    direction: -1,
                    new_state: 'D',
                },
            ],
        },
    );

    states.insert(
        'D',
        State {
            operations: [
                Operation {
                    new_value: 1,
                    direction: -1,
                    new_state: 'A',
                },
                Operation {
                    new_value: 1,
                    direction: -1,
                    new_state: 'A',
                },
            ],
        },
    );

    states.insert(
        'E',
        State {
            operations: [
                Operation {
                    new_value: 0,
                    direction: 1,
                    new_state: 'A',
                },
                Operation {
                    new_value: 0,
                    direction: 1,
                    new_state: 'F',
                },
            ],
        },
    );

    states.insert(
        'F',
        State {
            operations: [
                Operation {
                    new_value: 1,
                    direction: 1,
                    new_state: 'E',
                },
                Operation {
                    new_value: 1,
                    direction: 1,
                    new_state: 'A',
                },
            ],
        },
    );

    let machine = TuringMachine {
        tape: HashMap::new(),
        cursor: 0,
        state: 'A',
        steps_left: 12861455,
        program: Program { states },
    };

    machine
}

fn main() -> Result<()> {
    let mut machine = make_machine();

    machine.simulate();

    println!("Part 1: {}", machine.checksum());

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
