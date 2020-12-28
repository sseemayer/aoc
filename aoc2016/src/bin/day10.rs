use std::collections::HashMap;

use snafu::{ResultExt, Snafu};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Default)]
struct Bot {
    id: usize,
    inputs: Vec<usize>,
    destination_low: Option<Destination>,
    destination_high: Option<Destination>,
}

#[derive(Debug)]
struct Message {
    destination: Destination,
    value: usize,
}

impl Bot {
    fn new(id: usize) -> Self {
        Bot {
            id,
            ..Default::default()
        }
    }

    fn wake(&mut self) -> Vec<Message> {
        if self.inputs.len() != 2 {
            return Vec::new();
        }

        if let (Some(d_low), Some(d_high)) = (
            self.destination_low.as_ref(),
            self.destination_high.as_ref(),
        ) {
            self.inputs.sort();

            let v_low = self.inputs[0];
            let v_high = self.inputs[1];

            // println!(
            //     "Bot {} woke up! {} -> {:?}, {} -> {:?}",
            //     self.id, v_low, d_low, v_high, d_high
            // );

            if v_low == 17 && v_high == 61 {
                println!("Part 1: processed by bot {}", self.id);
            }

            self.inputs.clear();
            vec![
                Message {
                    destination: d_low.clone(),
                    value: v_low,
                },
                Message {
                    destination: d_high.clone(),
                    value: v_high,
                },
            ]
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug, Clone)]
enum Destination {
    Bot { id: usize },
    Output { id: usize },
}

impl Destination {
    fn from_type_and_id(dest_type: &str, id: &str) -> Result<Self> {
        let id: usize = id.parse().context(ParseInt {
            data: id.to_string(),
        })?;
        match dest_type {
            "bot" => Ok(Destination::Bot { id }),
            "output" => Ok(Destination::Output { id }),
            _ => Err(Error::ParseDestination {
                data: dest_type.to_string(),
            }),
        }
    }
}

#[derive(Debug, Default)]
struct State {
    bots: HashMap<usize, Bot>,
    outputs: HashMap<usize, Vec<usize>>,
}

impl State {
    fn absorb_instruction(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Value { data, destination } => self.absorb_message(&Message {
                destination: destination.clone(),
                value: *data,
            }),
            Instruction::BotConnect {
                bot_id,
                destination_low,
                destination_high,
            } => {
                let bot = self.bots.entry(*bot_id).or_insert(Bot::new(*bot_id));
                bot.destination_low = Some(destination_low.clone());
                bot.destination_high = Some(destination_high.clone());

                let messages = bot.wake();
                for msg in messages {
                    self.absorb_message(&msg);
                }
            }
        }
    }

    fn absorb_message(&mut self, msg: &Message) {
        match msg.destination {
            Destination::Output { id } => {
                self.outputs.entry(id).or_insert(Vec::new()).push(msg.value);
                // println!("Got output for #{}: {}", id, msg.value);
            }
            Destination::Bot { id } => {
                let bot = self.bots.entry(id).or_insert(Bot::new(id));
                bot.inputs.push(msg.value);

                let messages = bot.wake();
                for msg in messages {
                    self.absorb_message(&msg);
                }
            }
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Value {
        data: usize,
        destination: Destination,
    },
    BotConnect {
        bot_id: usize,
        destination_low: Destination,
        destination_high: Destination,
    },
}

impl std::str::FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        match &tokens[..] {
            &["value", data, "goes", "to", dest_type, id] => {
                let data: usize = data.parse().context(ParseInt {
                    data: data.to_string(),
                })?;
                let destination = Destination::from_type_and_id(dest_type, id)?;
                Ok(Instruction::Value { data, destination })
            }
            &["bot", bot_id, "gives", "low", "to", low_type, low_id, "and", "high", "to", high_type, high_id] =>
            {
                let bot_id: usize = bot_id.parse().context(ParseInt {
                    data: bot_id.to_string(),
                })?;
                let destination_low = Destination::from_type_and_id(low_type, low_id)?;
                let destination_high = Destination::from_type_and_id(high_type, high_id)?;

                Ok(Instruction::BotConnect {
                    bot_id,
                    destination_low,
                    destination_high,
                })
            }
            _ => Err(Error::ParseInstruction {
                data: s.to_string(),
            }),
        }
    }
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Int format error for '{}': {}", data, source))]
    ParseInt {
        data: String,
        source: std::num::ParseIntError,
    },

    #[snafu(display("Invalid instruction: '{}'", data))]
    ParseInstruction { data: String },

    #[snafu(display("Invalid destination: '{}'", data))]
    ParseDestination { data: String },
}

fn main() -> Result<()> {
    let instructions: Vec<Instruction> = std::fs::read_to_string("data/day10/input")
        .context(Io)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_>>()?;

    let mut state: State = Default::default();

    for inst in instructions {
        state.absorb_instruction(&inst);
    }

    println!(
        "Part 2: product is {}",
        state.outputs[&0][0] * state.outputs[&1][0] * state.outputs[&2][0]
    );

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
