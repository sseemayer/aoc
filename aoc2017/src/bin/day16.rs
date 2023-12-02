use anyhow::{anyhow, Context, Result};
use aoc::io::read_all;

#[derive(Debug, Clone)]
enum Move {
    Spin { n: usize },
    Exchange { a: usize, b: usize },
    Partner { a: String, b: String },
}

impl std::str::FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (cmd, rest) = s.trim().split_at(1);

        match cmd {
            "s" => {
                let n: usize = rest.parse().context("Parse s argument")?;
                Ok(Move::Spin { n })
            }
            "x" => {
                let (a, b) = rest.split_once("/").ok_or(anyhow!("Parse x command"))?;

                let a: usize = a.parse().context("Parse first x argument")?;
                let b: usize = b.parse().context("Parse second x argument")?;

                Ok(Move::Exchange { a, b })
            }
            "p" => {
                let (a, b) = rest.split_once("/").ok_or(anyhow!("Parse p commnad"))?;

                Ok(Move::Partner {
                    a: a.to_string(),
                    b: b.to_string(),
                })
            }
            _ => return Err(anyhow!("Bad command: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    idx_to_id: Vec<String>,
}

impl State {
    fn new() -> Self {
        let mut idx_to_id = Vec::new();
        for idx in 0..16 {
            let id = (('a' as u8 + idx as u8) as char).to_string();

            idx_to_id.push(id);
        }

        State { idx_to_id }
    }

    fn step(&mut self, m: &Move) {
        match m {
            Move::Spin { n } => {
                for _ in 0..*n {
                    let v = self.idx_to_id.pop().expect("non empty");
                    self.idx_to_id.insert(0, v);
                }
            }
            Move::Exchange { a, b } => {
                let va = self.idx_to_id[*a].clone();
                let vb = self.idx_to_id[*b].clone();

                self.idx_to_id[*a] = vb;
                self.idx_to_id[*b] = va;
            }
            Move::Partner { a, b } => {
                let ia = self
                    .idx_to_id
                    .iter()
                    .enumerate()
                    .find_map(|(i, v)| if v == a { Some(i) } else { None })
                    .expect("Element A");

                let ib = self
                    .idx_to_id
                    .iter()
                    .enumerate()
                    .find_map(|(i, v)| if v == b { Some(i) } else { None })
                    .expect("Element B");

                self.idx_to_id[ia] = b.to_string();
                self.idx_to_id[ib] = a.to_string();
            }
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.idx_to_id.join(""))
    }
}

fn main() -> Result<()> {
    let moves: Vec<Move> = read_all("data/day16/input")
        .context("Read input")?
        .split(",")
        .map(|s| s.parse())
        .collect::<Result<Vec<Move>>>()?;

    let initial = State::new();

    let mut state = State::new();
    let mut rep = 0;
    let target_reps = 1_000_000_000;
    while rep < target_reps {
        for m in moves.iter() {
            state.step(m);
        }

        if rep == 0 {
            println!("Part 1:    {}", state)
        };

        rep += 1;

        println!("{:>10} {}", rep, state);

        if state == initial {
            // we found the cycle
            println!("Found cycle after {} reps", rep);

            let loop_reps = rep;

            rep = target_reps - (target_reps % loop_reps);
        }
    }

    println!("Part 2:    {}", state);

    Ok(())
}
