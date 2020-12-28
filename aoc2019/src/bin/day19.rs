use aoc2019::board::{Board, Direction, Position};
use aoc2019::intcode::{parse_program, IntCodeResult, State};
use aoc2019::result::{format_err, Error, Result};
use aoc2019::util::read_to_string;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Unknown,
    Stationary,
    Pulled,
}

impl Tile {
    fn from_output(o: i64) -> Result<Self> {
        match o {
            0 => Ok(Tile::Stationary),
            1 => Ok(Tile::Pulled),
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
            Tile::Unknown => "?".to_owned(),
            Tile::Stationary => "░".to_owned(),
            Tile::Pulled => "█".to_owned(),
        };
        write!(f, "{}", c)
    }
}

fn read_drone(prog: &Vec<i64>, i: i64, j: i64) -> Result<Tile> {
    let mut state = State::new(prog.clone());
    state.inputs.push(j);
    state.inputs.push(i);

    let mut out = Vec::new();
    state.run(&mut out)?;

    Tile::from_output(out[0])
}

fn read_drones(prog: &Vec<i64>, size: i64) -> Result<Board<Tile>> {
    let mut board = Board::new();

    for i in 0..size {
        for j in 0..size {
            board.set(&Position { i, j }, read_drone(&prog, i, j)?);
        }
    }

    Ok(board)
}

fn check_pos(prog: &Vec<i64>, i0: i64, j0: i64, dim: i64) -> Result<bool> {
    //
    // xxxx
    // x
    // x
    // x
    //
    //
    //

    for d in 0..dim {
        // left edge
        if read_drone(prog, i0 + d, j0)? != Tile::Pulled {
            return Ok(false);
        }

        // top edge
        if read_drone(prog, i0, j0 + d)? != Tile::Pulled {
            return Ok(false);
        }

        // right edge
        if read_drone(prog, i0 + d, j0 + dim - 1)? != Tile::Pulled {
            return Ok(false);
        }

        // bottom edge
        if read_drone(prog, i0 + dim - 1, j0 + d)? != Tile::Pulled {
            return Ok(false);
        }
    }

    for i in i0..(i0 + dim) {
        for j in j0..(j0 + dim) {}
    }

    return Ok(true);
}

fn main() -> Result<()> {
    let program = parse_program(&read_to_string("data/day19/input")?)?;

    println!("PART ONE");
    let board = read_drones(&program, 50)?;
    println!("BOARD:\n{}", board);

    let counts = board.count();

    println!("Counts: {:#?}", counts);

    println!("PART TWO");

    let mut jmin = 30;
    let mut jmax = 38;
    for i in 50..10000 {
        let tmin = read_drone(&program, i, jmin)?;
        let tmax = read_drone(&program, i, jmax)?;

        if tmin == Tile::Stationary {
            jmin += 1;
        }

        if tmax == Tile::Pulled {
            jmax += 1;
        }

        if read_drone(&program, i, jmin - 1)? != Tile::Stationary {
            println!("ERR i={} jmin={}", i, jmin);
        }
        if read_drone(&program, i, jmax + 1)? != Tile::Stationary {
            println!("ERR i={} jmin={}", i, jmin);
        }

        println!("i={:5}, {:5} <= j <= {:5}", i, jmin, jmax);

        for j in jmin..=jmax {
            if check_pos(&program, i, j, 100)? {
                println!("Solution {} @ i={} j={}", i + j * 10000, i, j);
                return Ok(());
            }
        }
    }

    Ok(())
}
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░████████░░░░░░░░░░░░
// 0         1         2         3         4
// 01234567890123456789012345678901234567890123456789
