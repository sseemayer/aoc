use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
};

use anyhow::{Context, Result};

use aoc::map::ParseMapTile;
use colored::Colorize;

type Pos = [i32; 2];
type Map = aoc::map::Map<Pos, Tile>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Start,
    Goal,
    Ground(u8),
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Start => write!(f, "{}", "S".green()),
            Tile::Goal => write!(f, "{}", "E".red()),
            Tile::Ground(h) => write!(f, "{}", ('a' as u8 + h) as char),
        }
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'S' => Some(Tile::Start),
            'E' => Some(Tile::Goal),
            'a'..='z' => Some(Tile::Ground(c as u8 - 'a' as u8)),
            _ => None,
        }
    }
}

impl Tile {
    fn height(&self) -> i8 {
        match self {
            Tile::Start => 0,
            Tile::Goal => 25,
            Tile::Ground(h) => *h as i8,
        }
    }
}

const DIRECTIONS: [Pos; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

fn parse(path: &str) -> Result<Map> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;

    buffer.parse().context("Parse map")
}

fn pathfind(map: &Map, start: Pos, dest: Pos) -> Option<usize> {
    let mut traceback: HashMap<Pos, Pos> = HashMap::new();
    let mut queue: VecDeque<(usize, i8, Pos)> = VecDeque::new();
    queue.push_back((0, 0, start));

    while let Some((dist, height, pos)) = queue.pop_front() {
        if pos == dest {
            return Some(dist);
        }

        for [di, dj] in DIRECTIONS {
            let new_pos = [pos[0] + di, pos[1] + dj];

            if traceback.contains_key(&new_pos) {
                continue;
            }

            if let Some(tile) = map.get(&new_pos) {
                let new_height = tile.height();

                if new_height - height > 1 {
                    continue;
                }

                traceback.insert(new_pos, pos);
                queue.push_back((dist + 1, new_height, new_pos));
            }
        }
    }

    None
}

fn main() -> Result<()> {
    let map = parse("data/day12/input")?;

    println!("{}", map);

    let start = map.find_one(&Tile::Start).expect("Start position");
    let dest = map.find_one(&Tile::Goal).expect("Goal position");

    if let Some(d) = pathfind(&map, start, dest) {
        println!("Part 1: {}", d);
    }

    let mut shortest = usize::MAX;
    for start in map.find_all(&Tile::Ground(0)) {
        if let Some(d) = pathfind(&map, start, dest) {
            if d < shortest {
                shortest = d;
            }
        }
    }

    println!("Part 2: {}", shortest);
    Ok(())
}
