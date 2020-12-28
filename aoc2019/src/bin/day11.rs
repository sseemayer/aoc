use failure::{format_err, Error};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct State {
    memory: Vec<i64>,
    ic: usize,
    inputs: Vec<i64>,
    relative_base: i64,
}

enum IntCodeResult {
    Input,
    Output,
    Halt,
}

impl State {
    fn get_memory(&mut self, address: usize) -> i64 {
        while self.memory.len() <= address {
            self.memory.push(0);
        }

        self.memory[address]
    }

    fn set_memory(&mut self, address: usize, value: i64) {
        while self.memory.len() <= address {
            self.memory.push(0);
        }

        self.memory[address] = value;
    }

    fn get_address(&mut self, parmodes: &Vec<u8>, ofs: usize) -> Result<usize> {
        let pm = parmodes.get(ofs - 1).unwrap_or(&0);
        let pv = self.memory[self.ic + ofs];

        let addr = match pm {
            0 => {
                // position mode
                pv as usize
            }
            2 => {
                // relative mode
                (self.relative_base + pv) as usize
            }
            _ => return Err(format_err!("Invalid parameter mode at {}: {}", ofs, pm)),
        };

        Ok(addr)
    }

    fn get_parameter(&mut self, parmodes: &Vec<u8>, ofs: usize) -> Result<i64> {
        let pm = parmodes.get(ofs - 1).unwrap_or(&0);
        let pv = self.memory[self.ic + ofs];

        let val = match pm {
            0 => {
                // position mode
                self.get_memory(pv as usize)
            }
            1 => {
                // immediate mode
                pv
            }
            2 => {
                // relative mode
                self.get_memory((self.relative_base + pv) as usize)
            }
            _ => return Err(format_err!("Invalid parameter mode at {}: {}", ofs, pm)),
        };

        Ok(val)
    }

    fn get_opcode_and_parmode(&self) -> (i64, Vec<u8>) {
        let mut v = self.memory[self.ic];

        let opcode = v % 100;
        v /= 100;

        let mut parmodes = Vec::new();

        while v > 0 {
            let pm = v % 10;
            parmodes.push(pm as u8);
            v /= 10;
        }

        return (opcode, parmodes);
    }

    fn run(&mut self, outputs: &mut Vec<i64>) -> Result<IntCodeResult> {
        loop {
            let (opcode, parmodes) = self.get_opcode_and_parmode();

            match opcode {
                1 => {
                    // add
                    let a = self.get_parameter(&parmodes, 1)?;
                    let b = self.get_parameter(&parmodes, 2)?;
                    let pos_store = self.get_address(&parmodes, 3)?;

                    self.set_memory(pos_store, a + b);
                    self.ic += 4;
                }
                2 => {
                    // mul
                    let a = self.get_parameter(&parmodes, 1)?;
                    let b = self.get_parameter(&parmodes, 2)?;
                    let pos_store = self.get_address(&parmodes, 3)?;

                    self.set_memory(pos_store, a * b);
                    self.ic += 4;
                }
                3 => {
                    // input
                    if self.inputs.len() > 0 {
                        let pos_store = self.get_address(&parmodes, 1)? as usize;
                        let input_val = self.inputs.remove(0);
                        self.set_memory(pos_store, input_val);
                        self.ic += 2;
                    } else {
                        return Ok(IntCodeResult::Input);
                    }
                }
                4 => {
                    // output
                    let v = self.get_parameter(&parmodes, 1)?;
                    outputs.push(v);
                    self.ic += 2;
                    return Ok(IntCodeResult::Output);
                }
                5 => {
                    // jump-if-true
                    let a = self.get_parameter(&parmodes, 1)?;
                    let b = self.get_parameter(&parmodes, 2)?;

                    self.ic = if a != 0 { b as usize } else { self.ic + 3 };
                }
                6 => {
                    // jump-if false
                    let a = self.get_parameter(&parmodes, 1)?;
                    let b = self.get_parameter(&parmodes, 2)?;

                    self.ic = if a == 0 { b as usize } else { self.ic + 3 };
                }
                7 => {
                    // less-than
                    let a = self.get_parameter(&parmodes, 1)?;
                    let b = self.get_parameter(&parmodes, 2)?;
                    let pos_store = self.get_address(&parmodes, 3)?;

                    self.set_memory(pos_store, if a < b { 1 } else { 0 });
                    self.ic += 4;
                }
                8 => {
                    // equals
                    let a = self.get_parameter(&parmodes, 1)?;
                    let b = self.get_parameter(&parmodes, 2)?;
                    let pos_store = self.get_address(&parmodes, 3)?;

                    self.set_memory(pos_store, if a == b { 1 } else { 0 });
                    self.ic += 4;
                }
                9 => {
                    // shift relative base
                    let a = self.get_parameter(&parmodes, 1)?;
                    self.relative_base += a;
                    self.ic += 2;
                }
                99 => {
                    // halt
                    return Ok(IntCodeResult::Halt);
                }
                _ => return Err(format_err!("Invalid opcode: {}", opcode)),
            }
        }
    }
}

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

impl Position {
    fn step(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Up => Position {
                i: self.i - 1,
                j: self.j,
            },
            Direction::Right => Position {
                i: self.i,
                j: self.j + 1,
            },
            Direction::Down => Position {
                i: self.i + 1,
                j: self.j,
            },
            Direction::Left => Position {
                i: self.i,
                j: self.j - 1,
            },
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rot_instr(&self, code: i64) -> Self {
        if code == 0 {
            self.rot_left()
        } else {
            self.rot_right()
        }
    }

    fn rot_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn rot_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug)]
struct Board {
    robot_pos: Position,
    robot_dir: Direction,
    drawing: HashMap<Position, i64>,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        // find maximum drawing coords
        let mut i_min = 0;
        let mut i_max = 0;
        let mut j_min = 0;
        let mut j_max = 0;
        for pos in self.drawing.keys() {
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
                let c = match self.drawing.get(&Position { i, j }).unwrap_or(&0) {
                    0 => ".",
                    1 => "#",
                    _ => "?",
                };

                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Board {
    fn new() -> Self {
        Board {
            robot_pos: Position { i: 0, j: 0 },
            robot_dir: Direction::Up,
            drawing: HashMap::new(),
        }
    }

    fn get(&self) -> i64 {
        *(self.drawing.get(&self.robot_pos).unwrap_or(&0))
    }

    fn set(&mut self, color: i64) {
        self.drawing.insert(self.robot_pos, color);
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
                    let v = self.get();
                    //println!("Get {}: {}", self.robot_pos, v);
                    state.inputs.push(v);
                }
                IntCodeResult::Output => {
                    if outputs.len() >= 2 {
                        let paint_color = outputs.remove(0);
                        let dir = outputs.remove(0);

                        //println!(
                        //    "Set {}: color={} direction={}",
                        //    self.robot_pos, paint_color, dir
                        //);

                        self.set(paint_color);

                        self.robot_dir = self.robot_dir.rot_instr(dir);
                        self.robot_pos = self.robot_pos.step(&self.robot_dir);

                        //println!("Mov {}, direction {:?}", self.robot_pos, self.robot_dir);
                    }
                }
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    // /*
    let mut f = File::open("data/day11/input")?;

    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    let buf = String::from_utf8(buf)?;
    // */
    let program: Vec<i64> = buf
        .trim()
        .split(",")
        .map(|v| v.parse().map_err(|e: std::num::ParseIntError| e.into()))
        .collect::<Result<Vec<i64>>>()?;

    println!("FIRST RUN");
    let mut board = Board::new();

    board.run(&program)?;

    println!("{}", board);

    println!("Drawn places: {}", board.drawing.len());

    println!("SECOND RUN");
    let mut board = Board::new();
    board.drawing.insert(Position { i: 0, j: 0 }, 1);

    board.run(&program)?;

    println!("{}", board);

    Ok(())
}
