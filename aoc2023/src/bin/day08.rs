use std::{collections::HashMap, str::FromStr};

use strum::{EnumIter, IntoEnumIterator};

use anyhow::{anyhow, bail, Error, Result};

#[derive(Debug, Clone, Hash, PartialEq, Eq, EnumIter)]
enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        Ok(match value {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => bail!("Bad instruction: '{}'", value),
        })
    }
}

#[derive(Debug, Clone)]
struct Network {
    instructions: Vec<Instruction>,

    nodes: HashMap<String, Node>,
}

impl Network {
    fn parse(path: &str) -> Result<Self> {
        let mut instructions = Vec::new();
        let mut nodes = HashMap::new();

        for line in aoc::io::read_all(path)?.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if instructions.is_empty() {
                instructions.extend(
                    line.chars()
                        .map(|c| Instruction::try_from(c))
                        .collect::<Result<Vec<Instruction>>>()?,
                );
            } else {
                let (source, node) = line
                    .split_once(" = ")
                    .ok_or_else(|| anyhow!("Bad node line: '{}'", line))?;

                let source = source.to_string();
                let node: Node = node.parse()?;

                nodes.insert(source, node);
            }
        }

        Ok(Self {
            instructions,
            nodes,
        })
    }

    fn simulate_single(&self) -> Result<usize> {
        let mut steps = 0;
        let mut current = "AAA";

        while current != "ZZZ" {
            let current_instruction = &self.instructions[steps % self.instructions.len()];

            current = self
                .nodes
                .get(current)
                .ok_or_else(|| anyhow!("Got lost at {}", current))?
                .directions
                .get(current_instruction)
                .ok_or_else(|| {
                    anyhow!("No direction for {:?} at {}", current_instruction, current)
                })?;

            steps += 1;
        }

        Ok(steps)
    }

    fn simulate_multi(&self) -> Result<usize> {
        let starts: Vec<&str> = self
            .nodes
            .keys()
            .filter_map(|k| if k.ends_with("A") { Some(&k[..]) } else { None })
            .collect();

        let mut steps_to_first_goal: Vec<usize> = Vec::new();
        let mut steps_to_cycle_goal: Vec<usize> = Vec::new();

        for start in &starts {
            let mut steps = 0;
            let mut goal_count = 0;
            let mut current = *start;
            loop {
                if current.ends_with('Z') {
                    goal_count += 1;
                    if goal_count == 1 {
                        steps_to_first_goal.push(steps);
                    } else {
                        let stfg = steps_to_first_goal
                            .last()
                            .expect("Always had a first goal time");
                        steps_to_cycle_goal.push(steps - *stfg);
                        break;
                    }
                }

                let current_instruction = &self.instructions[steps % self.instructions.len()];

                current = self
                    .nodes
                    .get(current)
                    .ok_or_else(|| anyhow!("Got lost at {}", current))?
                    .directions
                    .get(current_instruction)
                    .ok_or_else(|| {
                        anyhow!("No direction for {:?} at {}", current_instruction, current)
                    })?;

                steps += 1;
            }
        }

        // this happened to be true for my input and will greatly simplify things
        assert_eq!(steps_to_first_goal, steps_to_cycle_goal);

        Ok(aoc::math::lcm_multiple(&steps_to_first_goal[..]))
    }
}

#[derive(Debug, Clone)]
struct Node {
    directions: HashMap<Instruction, String>,
}

impl FromStr for Node {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let directions: HashMap<Instruction, String> = Instruction::iter()
            .zip(
                s.trim_start_matches("(")
                    .trim_end_matches(")")
                    .trim()
                    .split(", "),
            )
            .map(|(i, d)| (i, d.to_string()))
            .collect();

        Ok(Self { directions })
    }
}

fn main() -> Result<()> {
    let network = Network::parse("data/day08/input")?;

    println!("Part 1: {}", network.simulate_single()?);

    println!("Part 2: {}", network.simulate_multi()?);

    Ok(())
}
