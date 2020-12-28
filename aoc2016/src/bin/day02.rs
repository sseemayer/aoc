use snafu::{ResultExt, Snafu};

use aoc2016::map::{Map, MapError, ParseMapTile};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
    c: char,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.c)
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        if c == ' ' {
            None
        } else {
            Some(Tile { c })
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Map parsing error: {}", source))]
    ParseMap { source: MapError },

    #[snafu(display("Invalid direction"))]
    ParseDirection,
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::str::FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(Error::ParseDirection),
        }
    }
}

impl Direction {
    fn walk(&self, pos: &[i64; 2]) -> [i64; 2] {
        let [i, j] = pos;

        match self {
            Direction::Up => [i - 1, *j],
            Direction::Down => [i + 1, *j],
            Direction::Left => [*i, j - 1],
            Direction::Right => [*i, j + 1],
        }
    }
}

fn follow_instructions(
    map: &Map<[i64; 2], Tile>,
    start_pos: [i64; 2],
    instructions: &Vec<Vec<Direction>>,
) -> String {
    let mut out: String = String::new();
    let mut pos = start_pos.clone();

    for line in instructions {
        for d in line {
            let new_pos = d.walk(&pos);
            if map.get(&new_pos).is_some() {
                pos = new_pos;
            }
        }

        if let Some(t) = map.get(&pos) {
            out.push(t.c);
        }
    }

    out
}

fn main() -> Result<()> {
    let instructions: Vec<Vec<Direction>> = std::fs::read_to_string("data/day02/input")
        .context(Io)?
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| format!("{}", c).parse())
                .collect::<Result<Vec<Direction>>>()
        })
        .collect::<Result<_>>()?;

    let keypad1: Map<[i64; 2], Tile> = "123\n456\n789".parse().context(ParseMap)?;
    let five_pos1 = keypad1.find_one(&Tile { c: '5' }).expect("Need 5 key");
    let digits1 = follow_instructions(&keypad1, five_pos1, &instructions);
    println!("Part 1: {}", digits1);

    let keypad2: Map<[i64; 2], Tile> = "  1  \n 234 \n56789\n ABC \n  D  "
        .parse()
        .context(ParseMap)?;
    let five_pos2 = keypad2.find_one(&Tile { c: '5' }).expect("Need 5 key");
    let digits2 = follow_instructions(&keypad2, five_pos2, &instructions);
    println!("Part 2: {}", digits2);
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
