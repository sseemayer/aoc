use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::{Context, Result};
use colored::Colorize;
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

#[derive(Default, Clone, Debug)]
struct Tile(Vec<usize>);

type Map = aoc::map::Map<[usize; 2], Tile>;

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.len() == 0 {
            write!(f, " ")
        } else if self.0.len() == 1 {
            write!(f, "{}", "1".green())
        } else if self.0.len() == 2 {
            write!(f, "{}", "2".yellow())
        } else {
            write!(f, "{}", "!".red())
        }
    }
}

#[derive(Debug, Error)]
enum Error {
    #[error("I/O error: {}", _0)]
    Io(#[from] std::io::Error),

    #[error("Int parsing error: {}", _0)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Bad rect definition: '{}'", _0)]
    BadRect(String),
}

lazy_static! {
    static ref RE_RECT: Regex = Regex::new(r"#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)").unwrap();
}

#[derive(Debug)]
struct Rect {
    id: usize,

    left: usize,
    top: usize,

    width: usize,
    height: usize,
}

impl Rect {
    fn intersects_with(&self, other: &Rect) -> bool {
        //  01234567
        // 0........
        // 1...2222.
        // 2...2222.
        // 3.11XX22.
        // 4.11XX22.
        // 5.111133.
        // 6.111133.
        // 7........

        let intersect_x = (self.left <= other.left + other.width - 1)
            && (self.left + self.width - 1 >= other.left);

        let intersect_y =
            (self.top <= other.top + other.height - 1) && (self.top + self.height - 1 >= other.top);

        intersect_x && intersect_y
    }
}

impl FromStr for Rect {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let m = RE_RECT
            .captures(s)
            .ok_or_else(|| Error::BadRect(s.to_string()))?;

        let id = m.get(1).unwrap().as_str().parse()?;
        let left = m.get(2).unwrap().as_str().parse()?;
        let top = m.get(3).unwrap().as_str().parse()?;
        let width = m.get(4).unwrap().as_str().parse()?;
        let height = m.get(5).unwrap().as_str().parse()?;

        Ok(Rect {
            id,
            left,
            top,
            width,
            height,
        })
    }
}

fn main() -> Result<()> {
    let rects = BufReader::new(File::open("data/day03/input").context("Open input")?)
        .lines()
        .map(|l| l.context("Read line")?.parse().context("Read rect"))
        .collect::<Result<Vec<Rect>>>()
        .context("Read rects")?;

    let mut map = Map::new();

    for rect in rects.iter() {
        for i in rect.top..rect.top + rect.height {
            for j in rect.left..rect.left + rect.width {
                map.data.entry([i, j]).or_default().0.push(rect.id);
            }
        }
    }

    //println!("{}", map);

    println!(
        "Part 1: {}",
        map.find_all_where(|_c, t| t.0.len() >= 2).len()
    );

    for (i, ra) in rects.iter().enumerate() {
        let mut intersecting = false;

        for (j, rb) in rects.iter().enumerate() {
            if i == j {
                continue;
            }
            if ra.intersects_with(rb) {
                intersecting = true;
                // println!("{:?} intersects with {:?}", ra, rb);
                break;
            }
        }

        if !intersecting {
            println!("Part 2: {}", ra.id);
        }
    }

    Ok(())
}
