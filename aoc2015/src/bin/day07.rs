use aoc2015::util::read_to_lines;
use aoc2015::{format_err, Error, Result};
use std::collections::HashMap;

enum Source {
    Static { v: u16 },
    Wire { id: String },
}

impl Source {
    /// Get the value of the current source, if it can be known, or None.
    fn get_value(&self, circuit: &Circuit) -> Option<u16> {
        match self {
            Source::Static { v } => Some(*v),
            Source::Wire { id } => {
                if let Some(Wiring::Assign {
                    src: Source::Static { v },
                }) = circuit.wires.get(id)
                {
                    Some(*v)
                } else {
                    None
                }
            }
        }
    }
}

impl std::fmt::Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Source::Static { v } => write!(f, "{}", v),
            Source::Wire { id } => write!(f, "{}", id),
        }
    }
}

impl std::str::FromStr for Source {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if let Ok(v) = s.parse() {
            Ok(Source::Static { v })
        } else {
            Ok(Source::Wire { id: s.to_owned() })
        }
    }
}

enum Wiring {
    Assign { src: Source },
    Not { src: Source },
    And { a: Source, b: Source },
    Or { a: Source, b: Source },
    LShift { src: Source, amount: Source },
    RShift { src: Source, amount: Source },
}

impl Wiring {
    fn get_value(&self, circuit: &Circuit) -> Option<u16> {
        match self {
            Wiring::Assign { src } => src.get_value(&circuit),
            Wiring::Not { src } => src.get_value(&circuit).map(|v| !v),
            Wiring::And { a, b } => {
                if let (Some(va), Some(vb)) = (a.get_value(&circuit), b.get_value(&circuit)) {
                    Some(va & vb)
                } else {
                    None
                }
            }
            Wiring::Or { a, b } => {
                if let (Some(va), Some(vb)) = (a.get_value(&circuit), b.get_value(&circuit)) {
                    Some(va | vb)
                } else {
                    None
                }
            }
            Wiring::LShift { src, amount } => {
                if let (Some(vs), Some(va)) = (src.get_value(&circuit), amount.get_value(&circuit))
                {
                    Some(vs << va)
                } else {
                    None
                }
            }
            Wiring::RShift { src, amount } => {
                if let (Some(vs), Some(va)) = (src.get_value(&circuit), amount.get_value(&circuit))
                {
                    Some(vs >> va)
                } else {
                    None
                }
            }
        }
    }
}

impl std::str::FromStr for Wiring {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<_> = s.trim().split(" ").collect();

        match &tokens[..] {
            &[src] => Ok(Wiring::Assign { src: src.parse()? }),
            &["NOT", src] => Ok(Wiring::Not { src: src.parse()? }),
            &[a, "AND", b] => Ok(Wiring::And {
                a: a.parse()?,
                b: b.parse()?,
            }),
            &[a, "OR", b] => Ok(Wiring::Or {
                a: a.parse()?,
                b: b.parse()?,
            }),
            &[src, "LSHIFT", amount] => Ok(Wiring::LShift {
                src: src.parse()?,
                amount: amount.parse()?,
            }),
            &[src, "RSHIFT", amount] => Ok(Wiring::RShift {
                src: src.parse()?,
                amount: amount.parse()?,
            }),
            _ => Err(format_err!("Unknown wiring spec: '{}'", s)),
        }
    }
}

impl std::fmt::Debug for Wiring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Wiring::Assign { src } => write!(f, "{:?}", src),
            Wiring::Not { src } => write!(f, "!{:?}", src),
            Wiring::And { a, b } => write!(f, "{:?} AND {:?}", a, b),
            Wiring::Or { a, b } => write!(f, "{:?} AND {:?}", a, b),
            Wiring::LShift { src, amount } => write!(f, "{:?} << {:?}", src, amount),
            Wiring::RShift { src, amount } => write!(f, "{:?} >> {:?}", src, amount),
        }
    }
}

#[derive(Debug)]
struct Circuit {
    wires: HashMap<String, Wiring>,
}

impl Circuit {
    fn solve(&mut self) {
        loop {
            let mut solutions = HashMap::new();

            {
                for (id, w) in self.wires.iter() {
                    if let Wiring::Assign {
                        src: Source::Static { .. },
                    } = w
                    // nothing to do here
                    {
                    } else {
                        if let Some(v) = w.get_value(&self) {
                            solutions.insert(
                                id.to_owned(),
                                Wiring::Assign {
                                    src: Source::Static { v },
                                },
                            );
                        }
                    }
                }
            }

            if solutions.len() == 0 {
                break;
            }

            for (k, v) in solutions.into_iter() {
                self.wires.insert(k, v);
            }
        }
    }
}

impl Circuit {
    fn from_instrs(s: &[String]) -> Result<Self> {
        let mut wires = HashMap::new();

        for l in s.iter() {
            let tokens: Vec<_> = l.split("->").collect();
            let wiring: Wiring = tokens[0].trim().parse()?;
            let dst = tokens[1].trim().to_owned();

            wires.insert(dst, wiring);
        }

        Ok(Circuit { wires })
    }
}

fn main() -> Result<()> {
    println!("PART ONE");
    let circuit: Vec<String> = read_to_lines("data/day07/input")?;
    let mut circuit = Circuit::from_instrs(&circuit)?;

    circuit.solve();

    let v_a = circuit.wires["a"].get_value(&circuit).unwrap();
    println!("Wire a has signal {}", v_a);

    println!("PART TWO");
    let circuit: Vec<String> = read_to_lines("data/day07/input")?;
    let mut circuit = Circuit::from_instrs(&circuit)?;

    circuit.wires.insert(
        "b".to_owned(),
        Wiring::Assign {
            src: Source::Static { v: v_a },
        },
    );

    circuit.solve();
    let v_a = circuit.wires["a"].get_value(&circuit).unwrap();
    println!("Wire a has signal {}", v_a);

    Ok(())
}
