use aoc2015::board::{Board, Direction, Position};
use aoc2015::util::read_to_parsed_lines;
use aoc2015::{format_err, Error, Result};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum TileV1 {
    Unknown,
    On,
    Off,
}

impl std::default::Default for TileV1 {
    fn default() -> Self {
        TileV1::Unknown
    }
}

impl std::fmt::Display for TileV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let c = match self {
            TileV1::Unknown => "?",
            TileV1::On => "o",
            TileV1::Off => ".",
        };

        write!(f, "{}", c)
    }
}

trait Cmd {
    type TileType: std::cmp::PartialEq
        + std::fmt::Display
        + std::marker::Copy
        + std::default::Default;
    fn apply_to(&self, t: &Self::TileType) -> Self::TileType;
}

#[derive(Debug)]
enum CommandV1 {
    On,
    Off,
    Toggle,
}

impl std::str::FromStr for CommandV1 {
    type Err = Error;
    fn from_str(s: &str) -> Result<CommandV1> {
        match s {
            "turn_on" => Ok(CommandV1::On),
            "turn_off" => Ok(CommandV1::Off),
            "toggle" => Ok(CommandV1::Toggle),
            _ => Err(format_err!("invalid command: {}", s)),
        }
    }
}

impl Cmd for CommandV1 {
    type TileType = TileV1;
    fn apply_to(&self, t: &TileV1) -> TileV1 {
        match self {
            CommandV1::On => TileV1::On,
            CommandV1::Off => TileV1::Off,
            CommandV1::Toggle => match t {
                TileV1::On => TileV1::Off,
                _ => TileV1::On,
            },
        }
    }
}

#[derive(Debug)]
struct Instruction<C> {
    cmd: C,
    from: Position,
    to: Position,
}

impl<C: Cmd> Instruction<C> {
    fn apply_to(&self, board: &mut Board<<C as Cmd>::TileType>) {
        for i in self.from.i..=self.to.i {
            for j in self.from.j..=self.to.j {
                let t = board.get(&Position { i, j });
                let u = self.cmd.apply_to(&t);
                board.set(&Position { i, j }, u);
            }
        }
    }
}

struct Pos(Position);

impl std::str::FromStr for Pos {
    type Err = Error;
    fn from_str(s: &str) -> Result<Pos> {
        let tokens: Vec<i64> = s
            .split(",")
            .map(|v| v.parse().map_err(|e: std::num::ParseIntError| e.into()))
            .collect::<Result<Vec<i64>>>()?;
        if tokens.len() != 2 {
            return Err(format_err!("Position {} has invalid number of tokens", s));
        }

        Ok(Pos(Position {
            i: tokens[0],
            j: tokens[1],
        }))
    }
}

impl<C: std::str::FromStr<Err = Error>> std::str::FromStr for Instruction<C> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Instruction<C>> {
        let s = s.replace("turn ", "turn_");
        let tokens: Vec<&str> = s.split(" ").collect();
        if tokens.len() != 4 {
            return Err(format_err!("Line {} has invalid number of tokens", s));
        }

        let cmd = tokens[0].parse()?;
        let from: Pos = tokens[1].parse()?;
        let to: Pos = tokens[3].parse()?;

        Ok(Instruction {
            cmd,
            from: from.0,
            to: to.0,
        })
    }
}

#[derive(Debug)]
struct CommandV2 {
    amount: i64,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct TileV2(i64);

impl std::fmt::Display for TileV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:3}", self.0)
    }
}

impl std::default::Default for TileV2 {
    fn default() -> Self {
        TileV2(0)
    }
}

impl Cmd for CommandV2 {
    type TileType = TileV2;
    fn apply_to(&self, t: &TileV2) -> TileV2 {
        let mut u = t.clone();
        u.0 += self.amount;

        if u.0 < 0 {
            u.0 = 0;
        }

        u
    }
}

impl std::str::FromStr for CommandV2 {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "turn_on" => Ok(CommandV2 { amount: 1 }),
            "turn_off" => Ok(CommandV2 { amount: -1 }),
            "toggle" => Ok(CommandV2 { amount: 2 }),
            _ => Err(format_err!("invalid command: {}", s)),
        }
    }
}

fn main() -> Result<()> {
    println!("PART ONE");
    let instrs: Vec<Instruction<CommandV1>> =
        read_to_parsed_lines("data/day06/input", &|l: &str| l.parse())?;

    let mut board = Board::new();

    for instr in instrs.iter() {
        instr.apply_to(&mut board);
    }

    let counts = board.count();

    println!("Counts: {:?}", counts);

    println!("\nPART TWO");
    let instrs: Vec<Instruction<CommandV2>> =
        read_to_parsed_lines("data/day06/input", &|l: &str| l.parse())?;

    let mut board = Board::new();

    for instr in instrs.iter() {
        instr.apply_to(&mut board);
    }

    let brightness: i64 = board.tiles.values().map(|v| v.0).sum();
    println!("Total brightness: {}", brightness);
    Ok(())
}
