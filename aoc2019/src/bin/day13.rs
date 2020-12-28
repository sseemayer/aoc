use std::collections::HashMap;
use std::convert::TryFrom;

use aoc2019::intcode::{parse_program, IntCodeResult, State};
use aoc2019::result::{format_err, Error, Result};
use aoc2019::util::read_to_string;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Position {
    i: i64,
    j: i64,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.j, self.i)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl std::convert::TryFrom<i64> for Tile {
    type Error = Error;
    fn try_from(v: i64) -> Result<Self> {
        match v {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::Paddle),
            4 => Ok(Tile::Ball),
            _ => Err(format_err!("Invalid tile: {}", v)),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let c = match self {
            Tile::Empty => ".",
            Tile::Wall => "#",
            Tile::Block => "X",
            Tile::Paddle => "=",
            Tile::Ball => "o",
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
struct Board {
    tiles: HashMap<Position, Tile>,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        // find maximum drawing coords
        let mut i_min = 0;
        let mut i_max = 0;
        let mut j_min = 0;
        let mut j_max = 0;
        for pos in self.tiles.keys() {
            if pos.i < i_min {
                i_min = pos.i;
            }
            if pos.i > i_max {
                i_max = pos.i;
            }
            if pos.j < j_min {
                j_min = pos.j;
            }
            if pos.j > j_max {
                j_max = pos.j;
            }
        }

        for i in i_min..=i_max {
            for j in j_min..=j_max {
                let t = self.tiles.get(&Position { i, j }).unwrap_or(&Tile::Empty);

                write!(f, "{}", t)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Board {
    fn new() -> Self {
        Board {
            tiles: HashMap::new(),
        }
    }

    fn get(&self, pos: &Position) -> &Tile {
        self.tiles.get(pos).unwrap_or(&Tile::Empty)
    }

    fn set(&mut self, pos: &Position, tile: Tile) {
        self.tiles.insert(pos.clone(), tile);
    }

    fn run(&mut self, program: &Vec<i64>) -> Result<()> {
        let inputs = Vec::new();
        let mut state = State {
            memory: program.clone(),
            ic: 0,
            inputs,
            relative_base: 0,
        };

        // run to completion
        let mut outputs = Vec::new();
        loop {
            match state.run(&mut outputs)? {
                IntCodeResult::Halt => break,
                IntCodeResult::Input => {
                    let ball_pos = self.get_pos_of(Tile::Ball).unwrap();
                    let paddle_pos = self.get_pos_of(Tile::Paddle).unwrap();

                    let delta_j = ball_pos.j - paddle_pos.j;

                    let v = if delta_j < 0 {
                        -1
                    } else if delta_j == 0 {
                        0
                    } else {
                        1
                    };
                    state.inputs.push(v);
                    println!("{}\nINPUT: {}", self, v);
                }
                IntCodeResult::Output => {
                    if outputs.len() >= 3 {
                        let j = outputs.remove(0);
                        let i = outputs.remove(0);
                        let t = outputs.remove(0);

                        if i == 0 && j == -1 {
                            println!("NEW SCORE: {}", t);
                        } else {
                            let pos = Position { i, j };
                            let tile = Tile::try_from(t)?;

                            self.set(&pos, tile);
                        }

                        //println!("Mov {}, direction {:?}", self.robot_pos, self.robot_dir);
                    }
                }
            }
        }

        Ok(())
    }

    fn count(&self) -> HashMap<&Tile, usize> {
        let mut out = HashMap::new();

        for tile in self.tiles.values() {
            out.entry(tile).and_modify(|c| *c += 1).or_insert(1);
        }

        out
    }

    fn get_pos_of(&self, v: Tile) -> Option<Position> {
        for (pos, tile) in self.tiles.iter() {
            if *tile == v {
                return Some(*pos);
            }
        }
        None
    }
}

fn main() -> Result<()> {
    let mut board = Board::new();
    let mut program = parse_program(&read_to_string("data/day13/input")?)?;

    println!("FIRST RUN");
    board.run(&program)?;

    println!("{}", board);

    println!("Counts: {:#?}", board.count());

    println!("SECOND RUN");
    program[0] = 2;

    board.run(&program)?;

    Ok(())
}
