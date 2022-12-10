use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Result};
use colored::Colorize;

type Pos = [i32; 2];

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default, Clone)]
struct Tile {
    tail_seen: bool,
    links: Vec<usize>,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_link = self.links.iter().min();
        if let Some(ml) = min_link {
            if *ml == 0 {
                write!(f, "{}", "@".red())
            } else {
                write!(f, "{}", format!("{}", ml).green())
            }
        } else if self.tail_seen {
            write!(f, ".")
        } else {
            write!(f, " ")
        }
    }
}

type Map = aoc::map::Map<Pos, Tile>;

impl std::str::FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(anyhow!("Bad direction: '{}'", s)),
        }
    }
}

impl Direction {
    fn step(&self, &[i, j]: &Pos) -> Pos {
        match self {
            Direction::Up => [i - 1, j],
            Direction::Down => [i + 1, j],
            Direction::Left => [i, j - 1],
            Direction::Right => [i, j + 1],
        }
    }
}

#[derive(Debug)]
struct Step {
    direction: Direction,
    count: usize,
}

impl std::str::FromStr for Step {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, count) = s
            .split_once(" ")
            .ok_or_else(|| anyhow!("Expected two tokens"))?;

        let direction: Direction = direction.parse()?;
        let count: usize = count.parse()?;

        Ok(Self { direction, count })
    }
}

fn parse(path: &str) -> Result<Vec<Step>> {
    BufReader::new(File::open(path)?)
        .lines()
        .map(|line| line?.trim().parse())
        .collect()
}

#[derive(Debug)]
struct Rope {
    links: Vec<Pos>,

    tail_seen: HashSet<Pos>,
}

impl std::fmt::Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = Map::new();

        for (i, pos) in self.links.iter().enumerate() {
            let tile = map.data.entry(*pos).or_default();
            tile.links.push(i);
        }

        for pos in &self.tail_seen {
            let tile = map.data.entry(*pos).or_default();
            tile.tail_seen = true;
        }

        write!(f, "{}", map)
    }
}

impl Rope {
    fn new(n_links: usize) -> Self {
        let links = (0..n_links).map(|_| [0, 0]).collect();
        let mut tail_seen = HashSet::new();
        tail_seen.insert([0, 0]);

        Self { links, tail_seen }
    }

    fn step(&mut self, direction: &Direction) {
        // move head
        self.links[0] = direction.step(&self.links[0]);

        for i in 1..self.links.len() {
            let [pi, pj] = self.links[i - 1];
            let [mi, mj] = &mut self.links[i];

            let di = pi - *mi;
            let dj = pj - *mj;

            //    ddvdd
            //    d...d
            //    >.H.<
            //    d...d
            //    dd^dd

            if di.abs() > 1 || dj.abs() > 1 {
                *mi += di.signum();
                *mj += dj.signum();
            }
        }

        if let Some(tail_pos) = self.links.last() {
            self.tail_seen.insert(*tail_pos);
        }
    }

    fn process(&mut self, steps: &[Step]) {
        for step in steps {
            for _ in 0..step.count {
                self.step(&step.direction);
            }
        }
    }
}

fn main() -> Result<()> {
    let steps = parse("data/day09/input")?;

    let mut rope = Rope::new(2);
    rope.process(&steps[..]);

    println!("{}", rope);
    println!("Part 1: {}", rope.tail_seen.len());

    let mut rope = Rope::new(10);
    rope.process(&steps[..]);
    println!("{}", rope);
    println!("Part 2: {}", rope.tail_seen.len());

    Ok(())
}
