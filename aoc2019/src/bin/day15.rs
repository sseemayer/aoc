use aoc2019::board::{Board, Direction, Position};
use aoc2019::intcode::{parse_program, State};
use aoc2019::result::{format_err, Result};
use aoc2019::util::read_to_string;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Tile {
    Unknown,
    Empty,
    Wall,
    OxygenSystem,
}

impl Tile {
    fn from_output(o: i64) -> Result<Self> {
        match o {
            0 => Ok(Tile::Wall),
            1 => Ok(Tile::Empty),
            2 => Ok(Tile::OxygenSystem),

            _ => Err(format_err!("Output is not a tile: {}", o)),
        }
    }
}

impl std::default::Default for Tile {
    fn default() -> Self {
        Tile::Unknown
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let c = match self {
            Tile::Unknown => "░",
            Tile::Empty => " ",
            Tile::Wall => "█",
            Tile::OxygenSystem => "O",
        };
        write!(f, "{}", c)
    }
}

enum BfsResult {
    Found { pos: Position, path: Vec<Direction> },
    NotFound { max_path_len: usize },
}

trait Bfs {
    fn search_bfs(&self, stat_pos: &Position, target: &Tile) -> BfsResult;
    fn run_discovery(&mut self, program: &Vec<i64>) -> Result<()>;
}

impl Bfs for Board<Tile> {
    fn search_bfs(&self, start_pos: &Position, target: &Tile) -> BfsResult {
        // this is breadth first search for a tile of interest, given the current knowledge
        let mut queue = vec![(start_pos.to_owned(), Vec::new())];
        let mut seen = HashSet::new();

        let mut max_path_len = 0;

        seen.insert(start_pos.clone());

        while !queue.is_empty() {
            let (cur, history) = queue.remove(0);
            seen.insert(cur.clone());
            let tile = self.get(&cur);

            if &tile == target {
                return BfsResult::Found {
                    pos: cur,
                    path: history,
                };
            } else if tile == Tile::Wall {
                // don't extend search from this tile
                continue;
            }

            for dir in Direction::ALL.iter() {
                let (i_ofs, j_ofs) = dir.to_ofs();

                let cand_pos = Position {
                    i: cur.i + i_ofs,
                    j: cur.j + j_ofs,
                };

                if !seen.contains(&cand_pos) {
                    let mut new_history: Vec<Direction> = history.iter().map(|h| *h).collect();
                    new_history.push(*dir);

                    if new_history.len() > max_path_len {
                        max_path_len = new_history.len();
                    }

                    queue.push((cand_pos, new_history));
                }
            }
        }

        BfsResult::NotFound { max_path_len }
    }

    fn run_discovery(&mut self, program: &Vec<i64>) -> Result<()> {
        // fill up knowledge of the playfield
        while let BfsResult::Found { pos, path } = self.search_bfs(&Position::ZERO, &Tile::Unknown)
        {
            let n_steps = path.len();
            let mut state = State {
                memory: program.clone(),
                ic: 0,
                inputs: path.iter().map(|d| d.to_input()).collect(),
                relative_base: 0,
            };

            let mut outputs = Vec::new();
            for _ in 0..n_steps {
                state.run(&mut outputs)?;
            }

            //println!(
            //    "Target {}: Path {:?} gives outputs {:?}",
            //    pos, path, outputs
            //);

            let last_output = outputs.pop().unwrap();
            let tile = Tile::from_output(last_output)?;

            self.set(&pos, tile);

            println!("\n\n{}", self);
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let program = parse_program(&read_to_string("data/day15/input")?)?;

    println!("Program: {:?}", program);

    let mut board = Board::new();
    board.set(&Position::ZERO, Tile::Empty);
    board.run_discovery(&program)?;

    println!("BOARD:\n{}", board);

    if let BfsResult::Found { pos: o2s_pos, path } =
        board.search_bfs(&Position::ZERO, &Tile::OxygenSystem)
    {
        println!("Path: {:?} ({} steps)", path, path.len());

        if let BfsResult::NotFound { max_path_len } = board.search_bfs(&o2s_pos, &Tile::Unknown) {
            println!("Flooding time: {} minutes", max_path_len - 1);
        }
    }

    Ok(())
}
