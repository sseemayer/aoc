use std::fs::File;
use std::io::{BufRead, BufReader};

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error on '{}': {}", filename, source))]
    Io {
        filename: String,
        source: std::io::Error,
    },

    #[snafu(display("Number parsing error: {}", source))]
    ParseNumber { source: std::num::ParseIntError },

    #[snafu(display("Instruction parsing error: {}", line))]
    ParseInstruction { line: String },
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_degrees(mut degrees: i64) -> Self {
        while degrees < 0 {
            degrees += 360;
        }

        match degrees % 360 {
            0 => Direction::East,
            90 => Direction::North,
            180 => Direction::West,
            270 => Direction::South,
            _ => panic!("Only support 90 degree increments!"),
        }
    }

    fn to_degrees(&self) -> i64 {
        match self {
            Direction::East => 0,
            Direction::North => 90,
            Direction::West => 180,
            Direction::South => 270,
        }
    }

    fn turn_by(&self, degrees: i64) -> Self {
        Direction::from_degrees(self.to_degrees() + degrees)
    }
}

#[derive(Debug)]
enum Instruction {
    Move { direction: Direction, units: i64 },
    Turn { degrees: i64 },
    Forward { units: i64 },
}

impl std::str::FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let letter = &s[0..1];
        let units: i64 = s[1..].parse().context(ParseNumber)?;

        Ok(match letter {
            "N" => Instruction::Move {
                direction: Direction::North,
                units,
            },
            "E" => Instruction::Move {
                direction: Direction::East,
                units,
            },
            "S" => Instruction::Move {
                direction: Direction::South,
                units,
            },
            "W" => Instruction::Move {
                direction: Direction::West,
                units,
            },
            "L" => Instruction::Turn { degrees: units },
            "R" => Instruction::Turn { degrees: -units },
            "F" => Instruction::Forward { units },
            _ => {
                return Err(Error::ParseInstruction {
                    line: s.to_string(),
                })
            }
        })
    }
}

#[derive(Debug)]
struct ShipState {
    x: i64,
    y: i64,
    direction: Direction,
}

impl ShipState {
    fn step(&mut self, instruction: &Instruction) {
        let my_dir = self.direction.clone();
        match instruction {
            Instruction::Move { direction, units } => self.move_by(&direction, *units),
            Instruction::Turn { degrees } => self.direction = self.direction.turn_by(*degrees),
            Instruction::Forward { units } => self.move_by(&my_dir, *units),
        }
    }

    fn move_by(&mut self, direction: &Direction, units: i64) {
        match direction {
            Direction::North => self.y -= units,
            Direction::East => self.x += units,
            Direction::South => self.y += units,
            Direction::West => self.x -= units,
        }
    }
}

#[derive(Debug)]
struct ShipAndWaypointState {
    ship_x: i64,
    ship_y: i64,
    waypoint_x: i64,
    waypoint_y: i64,
}

impl ShipAndWaypointState {
    fn step(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Move { direction, units } => self.move_waypoint_by(&direction, *units),
            Instruction::Turn { degrees } => self.turn_waypoint(*degrees),
            Instruction::Forward { units } => self.move_ship(*units),
        }
    }

    fn turn_waypoint(&mut self, degrees: i64) {
        let wx = self.waypoint_x;
        let wy = self.waypoint_y;
        match degrees {
            //       -y
            //        |
            // (-2, x |
            //  -5)   |
            //        |
            //        |    x(5, -2)
            //        |
            //-x -----+----- +x
            //        |
            //   x(-5,|
            //      2)|
            //        |
            //        | x(2, 5)
            //        |
            //       +y
            90 | -270 => {
                self.waypoint_x = wy;
                self.waypoint_y = -wx;
            }
            180 | -180 => {
                self.waypoint_x = -wx;
                self.waypoint_y = -wy;
            }
            270 | -90 => {
                self.waypoint_x = -wy;
                self.waypoint_y = wx;
            }
            _ => panic!("Only right angle turns supported"),
        }
    }

    fn move_ship(&mut self, units: i64) {
        self.ship_x += units * self.waypoint_x;
        self.ship_y += units * self.waypoint_y;
    }

    fn move_waypoint_by(&mut self, direction: &Direction, units: i64) {
        match direction {
            Direction::North => self.waypoint_y -= units,
            Direction::East => self.waypoint_x += units,
            Direction::South => self.waypoint_y += units,
            Direction::West => self.waypoint_x -= units,
        }
    }
}
fn main() -> Result<()> {
    let filename = "data/day12/input";
    let f = File::open(filename).context(Io {
        filename: filename.to_string(),
    })?;

    let br = BufReader::new(f);

    let instructions: Vec<Instruction> = br
        .lines()
        .map(|l| {
            l.context(Io {
                filename: filename.to_owned(),
            })
        })
        .map(|l| l?.parse())
        .collect::<Result<Vec<_>>>()?;

    let mut state = ShipState {
        x: 0,
        y: 0,
        direction: Direction::East,
    };

    for inst in &instructions {
        state.step(inst);
    }

    println!("Part 1: {:?}", state.x.abs() + state.y.abs());

    let mut state = ShipAndWaypointState {
        ship_x: 0,
        ship_y: 0,
        waypoint_x: 10,
        waypoint_y: -1,
    };

    for inst in &instructions {
        state.step(inst);
    }

    println!("Part 2: {:?}", state.ship_x.abs() + state.ship_y.abs());
    Ok(())
}
