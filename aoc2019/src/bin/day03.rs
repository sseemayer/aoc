use failure::{format_err, Error};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let sum = self.x.abs() + self.y.abs();
        let other_sum = other.x.abs() + other.y.abs();

        sum.cmp(&other_sum)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

trait Walk {
    fn walk(&self, pos: Pos, trace: &mut Vec<Pos>) -> Pos;
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Walk for Direction {
    fn walk(&self, pos: Pos, trace: &mut Vec<Pos>) -> Pos {
        let new_pos = match self {
            Direction::Up => Pos {
                x: pos.x,
                y: pos.y + 1,
            },
            Direction::Down => Pos {
                x: pos.x,
                y: pos.y - 1,
            },
            Direction::Left => Pos {
                x: pos.x - 1,
                y: pos.y,
            },
            Direction::Right => Pos {
                x: pos.x + 1,
                y: pos.y,
            },
        };

        trace.push(new_pos.clone());

        new_pos
    }
}

impl std::str::FromStr for Direction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(format_err!("Invalid direction: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    count: usize,
}

impl std::str::FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let direction: Direction = s[0..1].parse()?;
        let count: usize = s[1..].parse()?;

        Ok(Command { direction, count })
    }
}

impl Walk for Command {
    fn walk(&self, pos: Pos, trace: &mut Vec<Pos>) -> Pos {
        let mut new_pos = pos;
        for _ in 0..self.count {
            new_pos = self.direction.walk(new_pos, trace);
        }
        new_pos
    }
}

#[derive(Debug)]
struct Poly {
    commands: Vec<Command>,
}

impl Walk for Poly {
    fn walk(&self, pos: Pos, trace: &mut Vec<Pos>) -> Pos {
        let mut new_pos = pos;
        for cmd in self.commands.iter() {
            new_pos = cmd.walk(new_pos, trace);
        }
        new_pos
    }
}

impl std::str::FromStr for Poly {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let commands = s
            .split(",")
            .map(|c| c.parse())
            .collect::<Result<Vec<Command>>>()?;

        Ok(Poly { commands })
    }
}

fn intersect(traces: &Vec<Vec<Pos>>) -> Vec<(usize, Pos)> {
    let mut seen: HashMap<&Pos, usize> = HashMap::new();
    let mut intersections = Vec::new();

    for trace in traces {
        for (age, pos) in trace.iter().enumerate() {
            if seen.contains_key(&pos) {
                let other_age = seen[pos];
                intersections.push((age + other_age + 2, pos.clone()));
            }
        }

        for (age, pos) in trace.iter().enumerate() {
            if seen.contains_key(pos) {
                seen.insert(pos, seen[pos].min(age));
            } else {
                seen.insert(pos, age);
            }
        }
    }

    intersections
}

fn main() -> Result<()> {
    /*
    let ex1: Poly = "R75,D30,R83,U83,L12,D49,R71,U7,L72".parse()?;
    let mut trace1 = Vec::new();
    ex1.walk(Pos { x: 0, y: 0 }, &mut trace1);

    let ex2: Poly = "U62,R66,U55,R34,D71,R55,D58,R83".parse()?;
    let mut trace2 = Vec::new();
    ex2.walk(Pos { x: 0, y: 0 }, &mut trace2);

    println!("{:#?}", trace1);
    println!("{:#?}", trace2);
    let intersections = intersect(&vec![trace1, trace2]);

    // */

    // /*

    let f = File::open("data/day03/input")?;

    let polys = BufReader::new(f)
        .lines()
        .map(|v| v?.parse())
        .collect::<Result<Vec<Poly>>>()?;

    let traces: Vec<Vec<Pos>> = polys
        .iter()
        .map(|p| {
            let mut trace = Vec::new();
            p.walk(Pos { x: 0, y: 0 }, &mut trace);
            trace
        })
        .collect();

    let intersections = intersect(&traces);

    // */
    let closest_intersection = intersections.iter().map(|(_a, p)| p).min().unwrap();
    let youngest_intersection = intersections.iter().min().unwrap();

    println!(
        "Closest intersection: {:?} (distance {})",
        closest_intersection,
        closest_intersection.x.abs() + closest_intersection.y.abs()
    );

    println!(
        "Youngest intersection: {:?} (age {})",
        youngest_intersection.1, youngest_intersection.0
    );

    Ok(())
}
