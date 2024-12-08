use std::collections::{HashMap, HashSet};

use anyhow::Result;
use aoc::map::ParseMapTile;
use colored::Colorize;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Tile {
    tile_type: TileType,
    has_node: bool,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_node {
            write!(
                f,
                "{}",
                match self.tile_type {
                    TileType::Floor => '#',
                    TileType::Antenna(c) => c,
                }
                .to_string()
                .red()
            )
        } else {
            write!(f, "{}", self.tile_type)
        }
    }
}

#[derive(Debug, Clone)]
enum TileType {
    Floor,
    Antenna(char),
}

impl std::fmt::Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileType::Floor => write!(f, "{}", "░"),
            TileType::Antenna(c) => write!(f, "{}", c),
        }
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile {
                tile_type: TileType::Floor,
                has_node: false,
            }),
            '0'..='9' | 'A'..='Z' | 'a'..='z' => Some(Tile {
                tile_type: TileType::Antenna(c),
                has_node: false,
            }),
            _ => None,
        }
    }
}

type Map = aoc::map::Map<[i32; 2], Tile>;

fn find_antennae(map: &Map) -> HashMap<char, Vec<[i32; 2]>> {
    let mut out: HashMap<char, Vec<[i32; 2]>> = HashMap::new();

    for (pos, tile) in map.data.iter() {
        if let TileType::Antenna(c) = tile.tile_type {
            out.entry(c).or_default().push(pos.clone());
        }
    }

    out
}

fn part1(map: &Map, anntenna_pos: &HashMap<char, Vec<[i32; 2]>>) -> Result<()> {
    let mut nodes: HashSet<[i32; 2]> = HashSet::new();
    let mut map = map.clone();

    for positions in anntenna_pos.values() {
        for pair in positions.into_iter().combinations(2) {
            let [ia, ja] = *pair[0];
            let [ib, jb] = *pair[1];
            let [di, dj] = [ib - ia, jb - ja];

            let n = [ia - di, ja - dj];
            let m = [ib + di, jb + dj];

            if let Some(tile) = map.get_mut(&n) {
                nodes.insert(n);
                tile.has_node = true;
            }

            if let Some(tile) = map.get_mut(&m) {
                nodes.insert(m);
                tile.has_node = true;
            }
        }
    }

    println!("{}", map);

    println!("Part 1: {}", nodes.len());

    Ok(())
}

fn part2(map: &Map, anntenna_pos: &HashMap<char, Vec<[i32; 2]>>) -> Result<()> {
    let mut nodes: HashSet<[i32; 2]> = HashSet::new();
    let mut map = map.clone();

    for positions in anntenna_pos.values() {
        for pair in positions.into_iter().combinations(2) {
            let [ia, ja] = *pair[0];
            let [ib, jb] = *pair[1];
            let [di, dj] = simplify([ib - ia, jb - ja]);

            let mut s = 0;
            loop {
                let mut found = false;
                let n = [ia + s * di, ja + s * dj];
                let m = [ia - s * di, ja - s * dj];
                if let Some(tile) = map.get_mut(&n) {
                    nodes.insert(n);
                    tile.has_node = true;
                    found = true;
                }

                if let Some(tile) = map.get_mut(&m) {
                    nodes.insert(m);
                    tile.has_node = true;
                    found = true;
                }

                if !found {
                    break;
                }

                s += 1;
            }
        }
    }

    println!("{}", map);

    println!("Part 2: {}", nodes.len());

    Ok(())
}

fn simplify([mut i, mut j]: [i32; 2]) -> [i32; 2] {
    for d in 2..i32::min(i.abs(), j.abs()) {
        if i % d == 0 && j % d == 0 {
            i /= d;
            j /= d;
        }
    }

    [i, j]
}

fn main() -> Result<()> {
    let map: Map = aoc::io::read_all((2024, 8))?.parse()?;
    //let map: Map = aoc::io::read_all("data/day08/example")?.parse()?;

    let anntenna_pos = &find_antennae(&map);
    part1(&map, anntenna_pos)?;
    part2(&map, anntenna_pos)?;

    Ok(())
}
