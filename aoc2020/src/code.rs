use std::io::{BufRead, BufReader, Read};

use snafu::{OptionExt, ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum CodeError {
    #[snafu(display("Jumped to nonexistant instruction at {}", ic))]
    OutOfProgram { ic: usize },
}

pub type CodeResult<T> = std::result::Result<T, CodeError>;

#[derive(Clone)]
pub enum Instruction {
    Nop { delta: i64 },
    Acc { delta: i64 },
    Jmp { delta: i64 },
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Nop { delta } => write!(f, "nop {:+}", delta),
            Instruction::Acc { delta } => write!(f, "acc {:+}", delta),
            Instruction::Jmp { delta } => write!(f, "jmp {:+}", delta),
        }
    }
}

#[derive(Clone, Default)]
pub struct Program {
    pub offset: usize,
    pub instructions: Vec<Instruction>,
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, inst) in self.instructions.iter().enumerate() {
            write!(f, "{:8} {:?}\n", i + self.offset, inst)?;
        }

        Ok(())
    }
}

impl Program {
    pub fn slice(&self, start: usize, end: usize) -> Program {
        Program {
            offset: self.offset + start,
            instructions: self.instructions[start..end].iter().cloned().collect(),
        }
    }

    pub fn slice_from(&self, start: usize) -> Program {
        Program {
            offset: self.offset + start,
            instructions: self.instructions[start..].iter().cloned().collect(),
        }
    }

    pub fn slice_to(&self, end: usize) -> Program {
        Program {
            offset: self.offset,
            instructions: self.instructions[..end].iter().cloned().collect(),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum ParseError {
    #[snafu(display("Invalid instruction: \"{}\"", line))]
    InvalidInstruction { line: String },

    #[snafu(display("Invalid numeric argument: {}", source))]
    IntFormat { source: std::num::ParseIntError },

    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },
}

pub type ParseResult<T> = std::result::Result<T, ParseError>;

impl std::str::FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 5 {
            return Err(ParseError::InvalidInstruction {
                line: s.to_string(),
            });
        }

        if &s[3..4] != " " {
            return Err(ParseError::InvalidInstruction {
                line: s.to_string(),
            });
        }

        let instr = &s[..3];
        let argument: i64 = s[4..].parse().context(IntFormat)?;

        match instr {
            "nop" => Ok(Instruction::Nop { delta: argument }),
            "acc" => Ok(Instruction::Acc { delta: argument }),
            "jmp" => Ok(Instruction::Jmp { delta: argument }),
            _ => Err(ParseError::InvalidInstruction {
                line: s.to_string(),
            }),
        }
    }
}

#[derive(Debug, Default)]
pub struct State {
    pub program: Program,
    pub accumulator: i64,
    pub ic: usize,
}

impl State {
    pub fn with_program(program: Program) -> Self {
        Self {
            program,
            ..Default::default()
        }
    }

    /// Execute one step of the current machine state
    pub fn step(&mut self) -> CodeResult<()> {
        let instruction = self
            .program
            .instructions
            .get(self.ic)
            .context(OutOfProgram { ic: self.ic })?;

        match instruction {
            Instruction::Nop { .. } => {
                self.ic += 1;
            }
            Instruction::Acc { delta } => {
                self.accumulator += delta;
                self.ic += 1;
            }
            Instruction::Jmp { delta } => {
                self.ic = ((self.ic as i64) + delta) as usize;
            }
        }

        Ok(())
    }

    /// Parse a program from a file.
    pub fn parse_program<F: Read>(f: &mut F) -> ParseResult<Program> {
        let br = BufReader::new(f);
        let mut out = Vec::new();
        for line in br.lines() {
            let line = line.context(Io)?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            out.push(line.parse()?);
        }

        Ok(Program {
            offset: 0,
            instructions: out,
        })
    }
}
