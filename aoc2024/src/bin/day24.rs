use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

use anyhow::{anyhow, bail, Context, Error, Result};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;

lazy_static! {
    static ref RE_INPUT: Regex = Regex::new(r"^(\w+):\s(0|1)$").expect("valid regex");
    static ref RE_GATE: Regex =
        Regex::new(r"^(\w+)\s+(AND|OR|XOR)\s+(\w+)\s*->\s*(\w+)$").expect("valid regex");
}

#[derive(Debug, Clone)]
struct Circuit {
    output_names: Vec<String>,

    wires: HashMap<String, Gate>,
}

#[derive(Debug, Serialize)]
struct CircuitSerialize {
    nodes: Vec<NodeSerialize>,
    links: Vec<LinkSerialize>,
}

#[derive(Debug, Serialize)]
struct NodeSerialize {
    id: String,
    #[serde(rename = "type")]
    node_type: NodeType,

    value: Option<bool>,
}

#[derive(Debug, Serialize)]
struct LinkSerialize {
    source: String,
    target: String,
}

#[derive(Debug, Serialize)]
enum NodeType {
    Input,
    And,
    Or,
    Xor,
    Pruned,
}

impl std::str::FromStr for Circuit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut output_names: HashSet<String> = Default::default();
        let mut wires: HashMap<String, Gate> = Default::default();
        for line in s.lines() {
            if let Some(m) = RE_INPUT.captures(line) {
                let name = m.get(1).ok_or(anyhow!("get input name"))?.as_str();
                let value: bool = m.get(2).ok_or(anyhow!("get input value"))?.as_str() == "1";

                wires.insert(name.to_string(), Gate::Constant(value));
            } else if let Some(m) = RE_GATE.captures(line) {
                let a = m.get(1).ok_or(anyhow!("get first operand"))?.as_str();
                let op = m.get(2).ok_or(anyhow!("Get operand"))?.as_str();
                let b = m.get(3).ok_or(anyhow!("get second operand"))?.as_str();
                let name = m.get(4).ok_or(anyhow!("get gate destination"))?.as_str();

                let gate = match op {
                    "AND" => Gate::And(a.to_string(), b.to_string()),
                    "OR" => Gate::Or(a.to_string(), b.to_string()),
                    "XOR" => Gate::Xor(a.to_string(), b.to_string()),
                    _ => bail!("bad operand: {}", op),
                };

                wires.insert(name.to_string(), gate);

                if name.starts_with("z") {
                    output_names.insert(name.to_string());
                }
            }
        }

        let mut output_names: Vec<String> = output_names.into_iter().collect();
        output_names.sort();
        output_names.reverse();

        Ok(Self {
            output_names,
            wires,
        })
    }
}

impl Circuit {
    fn get_value(&self, name: &str) -> Result<bool> {
        let gate = self
            .wires
            .get(name)
            .ok_or(anyhow!("bad identifier: {}", name))?;
        gate.get_value(self)
    }

    fn get_number(&self) -> Result<usize> {
        let mut out = 0;

        for output in &self.output_names {
            out <<= 1;

            let val = self.get_value(output)?;

            // println!("{}: {}", output, val);
            if val {
                out += 1;
            }
        }

        Ok(out)
    }

    fn find_wrong(&self) -> Vec<String> {
        let mut out: HashSet<String> = Default::default();

        for (id, gate) in &self.wires {
            // check outputs
            if id.starts_with("z") {
                if let Gate::Xor(_, _) = gate {
                    // we could check if inputs are correct here
                } else if id == "z45" {
                    // exception for last output
                } else {
                    out.insert(id.to_string());
                }
            }

            // XOR gates must have either a direct input or a direct output
            if let Gate::Xor(a, b) = gate {
                if !(a.starts_with("x")
                    || a.starts_with("y")
                    || b.starts_with("x")
                    || b.starts_with("y")
                    || id.starts_with("z"))
                {
                    out.insert(id.to_string());
                }
            }

            // AND gates may only feed into OR gates
            if let Gate::And(a, b) = gate {
                if a == "x00" || b == "x00" {
                    continue;
                }

                for (_, gate2) in &self.wires {
                    match gate2 {
                        Gate::And(a, b) if a == id || b == id => {
                            out.insert(id.to_string());
                        }
                        Gate::Xor(a, b) if a == id || b == id => {
                            out.insert(id.to_string());
                        }
                        _ => {}
                    }
                }
            }

            // XOR gates may only feed into AND or XOR gates
            if let Gate::Xor(_, _) = gate {
                for (_, gate2) in &self.wires {
                    match gate2 {
                        Gate::Or(a, b) if a == id || b == id => {
                            out.insert(id.to_string());
                        }
                        _ => {}
                    }
                }
            }
        }

        let mut out: Vec<String> = out.into_iter().collect();
        out.sort();

        out
    }

    fn to_serialize(&self) -> CircuitSerialize {
        let mut nodes = Vec::new();
        let mut links = Vec::new();

        let check_id = |id: &str| {
            if id.starts_with("x") || id.starts_with("y") || id.starts_with("z") {
                let n: usize = id
                    .replace("x", "")
                    .replace("y", "")
                    .replace("z", "")
                    .parse()
                    .unwrap();

                n <= 5
            } else if id == "vdc" {
                false
            } else {
                true
            }
        };

        for (id, gate) in &self.wires {
            if !check_id(id) {
                continue;
            }

            match gate {
                Gate::Constant(v) => nodes.push(NodeSerialize {
                    id: id.to_string(),
                    node_type: NodeType::Input,
                    value: Some(*v),
                }),
                Gate::And(a, b) => {
                    if !check_id(a) || !check_id(b) {
                        nodes.push(NodeSerialize {
                            id: id.to_string(),
                            node_type: NodeType::Pruned,
                            value: None,
                        });
                        continue;
                    }

                    nodes.push(NodeSerialize {
                        id: id.to_string(),
                        node_type: NodeType::And,
                        value: None,
                    });

                    links.push(LinkSerialize {
                        source: a.to_string(),
                        target: id.to_string(),
                    });

                    links.push(LinkSerialize {
                        source: b.to_string(),
                        target: id.to_string(),
                    });
                }
                Gate::Xor(a, b) => {
                    if !check_id(a) || !check_id(b) {
                        nodes.push(NodeSerialize {
                            id: id.to_string(),
                            node_type: NodeType::Pruned,
                            value: None,
                        });
                        continue;
                    }

                    nodes.push(NodeSerialize {
                        id: id.to_string(),
                        node_type: NodeType::Xor,
                        value: None,
                    });

                    links.push(LinkSerialize {
                        source: a.to_string(),
                        target: id.to_string(),
                    });

                    links.push(LinkSerialize {
                        source: b.to_string(),
                        target: id.to_string(),
                    });
                }
                Gate::Or(a, b) => {
                    if !check_id(a) || !check_id(b) {
                        nodes.push(NodeSerialize {
                            id: id.to_string(),
                            node_type: NodeType::Pruned,
                            value: None,
                        });
                        continue;
                    }

                    nodes.push(NodeSerialize {
                        id: id.to_string(),
                        node_type: NodeType::Or,
                        value: None,
                    });

                    links.push(LinkSerialize {
                        source: a.to_string(),
                        target: id.to_string(),
                    });

                    links.push(LinkSerialize {
                        source: b.to_string(),
                        target: id.to_string(),
                    });
                }
            }
        }

        CircuitSerialize { nodes, links }
    }
}

#[derive(Debug, Clone)]
enum Gate {
    Constant(bool),
    And(String, String),
    Xor(String, String),
    Or(String, String),
}

impl Gate {
    fn get_value(&self, circuit: &Circuit) -> Result<bool> {
        match self {
            Gate::Constant(v) => Ok(*v),
            Gate::And(a, b) => Ok(circuit.get_value(a)? & circuit.get_value(b)?),
            Gate::Xor(a, b) => Ok(circuit.get_value(a)? ^ circuit.get_value(b)?),
            Gate::Or(a, b) => Ok(circuit.get_value(a)? | circuit.get_value(b)?),
        }
    }
}

fn main() -> Result<()> {
    //let circuit: Circuit = aoc::io::read_all("data/day24/example")?.parse()?;
    let circuit: Circuit = aoc::io::read_all((2024, 24))?.parse()?;

    let cs = circuit.to_serialize();
    serde_json::to_writer(
        &mut File::create("utils/day24/data.json").context("create json")?,
        &cs,
    )
    .context("write JSON")?;

    println!("Part 1: {}", circuit.get_number()?);

    let wrong = circuit.find_wrong();
    println!("Part 2: {}", wrong.into_iter().join(","));

    Ok(())
}
