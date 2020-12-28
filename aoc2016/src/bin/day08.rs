use snafu::{ResultExt, Snafu};

use aoc2016::map::Map;

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

    #[snafu(display("Invalid instruction: '{}'", data))]
    ParseInstruction { data: String },
}

#[derive(Debug, Clone)]
struct Tile;

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#")
    }
}

#[derive(Debug)]
enum Instruction {
    Rect { width: usize, height: usize },
    RotateColumn { x: usize, by: usize },
    RotateRow { y: usize, by: usize },
}

impl std::str::FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        match &tokens[..] {
            &["rect", rectdef] => {
                let tokens: Vec<&str> = rectdef.split("x").collect();
                if tokens.len() != 2 {
                    return Err(Error::ParseInstruction {
                        data: rectdef.to_string(),
                    });
                }

                let width: usize = tokens[0].parse().context(ParseInt {
                    data: tokens[0].to_string(),
                })?;
                let height: usize = tokens[1].parse().context(ParseInt {
                    data: tokens[1].to_string(),
                })?;
                Ok(Instruction::Rect { width, height })
            }
            &["rotate", "column", xdef, "by", bydef] => {
                if &xdef[..2] != "x=" {
                    return Err(Error::ParseInstruction {
                        data: xdef.to_string(),
                    });
                }

                let x = xdef[2..].parse().context(ParseInt {
                    data: xdef[2..].to_string(),
                })?;
                let by = bydef.parse().context(ParseInt {
                    data: bydef.to_string(),
                })?;

                Ok(Instruction::RotateColumn { x, by })
            }
            &["rotate", "row", ydef, "by", bydef] => {
                if &ydef[..2] != "y=" {
                    return Err(Error::ParseInstruction {
                        data: ydef.to_string(),
                    });
                }

                let y = ydef[2..].parse().context(ParseInt {
                    data: ydef[2..].to_string(),
                })?;
                let by = bydef.parse().context(ParseInt {
                    data: bydef.to_string(),
                })?;

                Ok(Instruction::RotateRow { y, by })
            }
            _ => {
                return Err(Error::ParseInstruction {
                    data: s.to_string(),
                })
            }
        }
    }
}

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

impl Instruction {
    fn apply_to(&self, map: &mut Map<[usize; 2], Tile>) {
        match self {
            Instruction::Rect { width, height } => {
                for i in 0..*height {
                    for j in 0..*width {
                        map.set([i, j], Tile);
                    }
                }
            }
            Instruction::RotateColumn { x, by } => {
                for _ in 0..*by {
                    // save the rightmost pixel
                    let save = map.get(&[HEIGHT - 1, *x]).is_some();

                    for k in 1..HEIGHT {
                        let i = HEIGHT - k;
                        if map.get(&[i - 1, *x]).is_some() {
                            map.set([i, *x], Tile);
                        } else {
                            map.remove(&[i, *x]);
                        }
                    }

                    if save {
                        map.set([0, *x], Tile);
                    } else {
                        map.remove(&[0, *x]);
                    }
                }
            }
            Instruction::RotateRow { y, by } => {
                for _ in 0..*by {
                    // save the rightmost pixel
                    let save = map.get(&[*y, WIDTH - 1]).is_some();

                    for k in 1..WIDTH {
                        let j = WIDTH - k;
                        if map.get(&[*y, j - 1]).is_some() {
                            map.set([*y, j], Tile);
                        } else {
                            map.remove(&[*y, j]);
                        }
                    }

                    if save {
                        map.set([*y, 0], Tile);
                    } else {
                        map.remove(&[*y, 0]);
                    }
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let instructions: Vec<Instruction> = std::fs::read_to_string("data/day08/input")
        .context(Io)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_>>()?;

    let mut state: Map<[usize; 2], Tile> = Map::new();
    state.fixed_extent = Some(([0, 0], [HEIGHT, WIDTH]));

    for inst in instructions {
        inst.apply_to(&mut state);

        println!("{:?}\n{}", inst, state);
    }

    let n_on = state.data.len();

    println!("Part 1: got {} pixels that are lit", n_on);
    println!("Part 2:\n{}", state);

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
