use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Source {
    Constant { value: i64 },
    Register { id: usize },
}

impl std::str::FromStr for Source {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(if let Ok(value) = s.parse::<i64>() {
            Source::Constant { value }
        } else {
            Source::Register {
                id: "abcdefghijklmnopqrstuvwxyz".find(s).unwrap(),
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    // Cpy -> Jnz
    Cpy { source: Source, register: Source },
    // Inc -> Dec
    Inc { register: Source },
    // Dec -> Inc
    Dec { register: Source },
    // Jnz -> Cpy
    Jnz { source: Source, offset: Source },
    // Tgl -> Inc
    Tgl { offset: Source },
    // Out -> Inc
    Out { source: Source },
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        Ok(match &tokens[..] {
            &["cpy", source, register] => {
                let source: Source = source.parse()?;
                let register = register.parse()?;
                Instruction::Cpy { source, register }
            }
            &["inc", register] => {
                let register = register.parse()?;
                Instruction::Inc { register }
            }
            &["dec", register] => {
                let register = register.parse()?;
                Instruction::Dec { register }
            }
            &["jnz", source, offset] => {
                let source = source.parse()?;
                let offset = offset.parse()?;
                Instruction::Jnz { source, offset }
            }
            &["tgl", offset] => {
                let offset = offset.parse()?;
                Instruction::Tgl { offset }
            }
            &["out", source] => {
                let source = source.parse()?;
                Instruction::Out { source }
            }
            _ => return Err(anyhow!("Bad instruction: '{}'", s)),
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct State {
    pub ic: i64,
    pub registers: Vec<i64>,
    pub instructions: Vec<Instruction>,
}

pub enum StepResult {
    OutOfProgram,
    OkNoOutput,
    OkOutput { out: i64 },
}

impl State {
    pub fn from_instructions(instructions: Vec<Instruction>) -> Self {
        let registers = (0..26).map(|_| 0).collect();
        State {
            instructions,
            registers,
            ..Default::default()
        }
    }

    pub fn get_instruction(&self, pos: i64) -> Option<Instruction> {
        if pos < 0 {
            return None;
        }
        if pos as usize >= self.instructions.len() {
            return None;
        }
        self.instructions.get(pos as usize).cloned()
    }

    pub fn get_value(&mut self, source: &Source) -> i64 {
        match source {
            Source::Constant { value } => *value,
            Source::Register { id } => self.registers[*id],
        }
    }

    pub fn set_value(&mut self, source: &Source, value: i64) {
        match source {
            Source::Constant { .. } => { /* ignore setting to a constant */ }
            Source::Register { id } => {
                self.registers[*id] = value;
            }
        }
    }

    pub fn step_turbo<F: Fn(&mut Self) -> Option<StepResult>>(
        &mut self,
        speed_patch: F,
    ) -> StepResult {
        if let Some(ret) = speed_patch(self) {
            ret
        } else {
            self.step()
        }
    }

    pub fn step(&mut self) -> StepResult {
        let inst = self.get_instruction(self.ic);
        if inst.is_none() {
            return StepResult::OutOfProgram;
        }

        match inst.unwrap() {
            Instruction::Cpy { source, register } => {
                let value = self.get_value(&source);
                self.set_value(&register, value);
            }
            Instruction::Inc { register } => {
                let value = self.get_value(&register);
                self.set_value(&register, value + 1);
            }
            Instruction::Dec { register } => {
                let value = self.get_value(&register);
                self.set_value(&register, value - 1);
            }
            Instruction::Jnz { source, offset } => {
                let value = self.get_value(&source);
                let ofs = self.get_value(&offset);
                if value != 0 {
                    self.ic += ofs;
                    return StepResult::OkNoOutput;
                }
            }
            Instruction::Tgl { offset } => {
                let ofs = self.get_value(&offset);

                if let Some(inst) = self.get_instruction(self.ic + ofs) {
                    let new_inst: Instruction = match inst {
                        Instruction::Cpy { source, register } => Instruction::Jnz {
                            source: source.clone(),
                            offset: register.clone(),
                        },
                        Instruction::Inc { register } => Instruction::Dec {
                            register: register.clone(),
                        },
                        Instruction::Dec { register } => Instruction::Inc {
                            register: register.clone(),
                        },
                        Instruction::Jnz { source, offset } => Instruction::Cpy {
                            source: source.clone(),
                            register: offset.clone(),
                        },
                        Instruction::Tgl { offset } => Instruction::Inc {
                            register: offset.clone(),
                        },
                        Instruction::Out { source } => Instruction::Inc {
                            register: source.clone(),
                        },
                    };
                    self.instructions[(self.ic + ofs) as usize] = new_inst;
                }
            }
            Instruction::Out { source } => {
                let out = self.get_value(&source);
                self.ic += 1;
                return StepResult::OkOutput { out };
            }
        }
        self.ic += 1;
        StepResult::OkNoOutput
    }
}
