use std::{collections::HashMap, fs::File, hash::Hash, io::Read};

use anyhow::{Context, Result};
use colored::Colorize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Open,
    Trees,
    Lumberyard,
}

impl aoc::map::ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Open),
            '|' => Some(Tile::Trees),
            '#' => Some(Tile::Lumberyard),
            _ => None,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Open => write!(f, "{}", "░".yellow()),
            Tile::Trees => write!(f, "{}", "▓".green()),
            Tile::Lumberyard => write!(f, "{}", "█".red()),
        }
    }
}

type Map = aoc::map::Map<[i32; 2], Tile>;

fn parse(path: &str) -> Result<Map> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;

    buffer.parse().context("Parse map")
}

const AROUND: [[i32; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
    [1, 0],
    [1, -1],
    [0, -1],
];

fn count_neighborhood(map: &Map, [i, j]: &[i32; 2]) -> HashMap<Tile, usize> {
    let mut out = HashMap::new();

    for [io, jo] in AROUND {
        if let Some(tile) = map.get(&[i + io, j + jo]) {
            *out.entry(*tile).or_default() += 1
        }
    }

    out
}

fn step(map: Map) -> Map {
    let mut out = Map::new();

    for (pos, tile) in map.data.iter() {
        let neighborhood = count_neighborhood(&map, &pos);

        let new_tile = match tile {
            Tile::Open => {
                // An open acre will become filled with trees if
                // three or more adjacent acres contained trees.
                // Otherwise, nothing happens.

                let n_trees = *neighborhood.get(&Tile::Trees).unwrap_or(&0);

                if n_trees >= 3 {
                    Tile::Trees
                } else {
                    Tile::Open
                }
            }
            Tile::Trees => {
                // An acre filled with trees will become a lumberyard if
                // three or more adjacent acres were lumberyards.
                // Otherwise, nothing happens.
                let n_lumberyards = *neighborhood.get(&Tile::Lumberyard).unwrap_or(&0);

                if n_lumberyards >= 3 {
                    Tile::Lumberyard
                } else {
                    Tile::Trees
                }
            }
            Tile::Lumberyard => {
                // An acre containing a lumberyard will remain a lumberyard if
                // it was adjacent to at least one other lumberyard
                // and at least one acre containing trees.
                // Otherwise, it becomes open.

                let n_trees = *neighborhood.get(&Tile::Trees).unwrap_or(&0);
                let n_lumberyards = *neighborhood.get(&Tile::Lumberyard).unwrap_or(&0);

                if n_lumberyards >= 1 && n_trees >= 1 {
                    Tile::Lumberyard
                } else {
                    Tile::Open
                }
            }
        };

        out.set(*pos, new_tile);
    }

    out
}

fn count_all(map: &Map) -> HashMap<Tile, usize> {
    let mut out = HashMap::new();

    for tile in map.data.values() {
        *out.entry(*tile).or_default() += 1;
    }

    out
}

fn main() -> Result<()> {
    let mut map = parse("data/day18/input")?;

    println!("{}", map);

    for i in 0..10 {
        println!("Step {}:\n{}", i, map);
        map = step(map);
    }

    println!("Final:\n{}", map);

    let counts = count_all(&map);

    let n_trees = *counts.get(&Tile::Trees).unwrap_or(&0);
    let n_lumberyards = *counts.get(&Tile::Lumberyard).unwrap_or(&0);

    let resource_value = n_trees * n_lumberyards;
    println!("Part 1: {}", resource_value);

    let mut seen_states: HashMap<String, usize> = HashMap::new();
    let mut frames: HashMap<usize, Map> = HashMap::new();
    let mut i = 10;
    let (i0, i1) = loop {
        let map_key = format!("{}", map);
        if let Some(last_step) = seen_states.get(&map_key) {
            // found a loop
            println!(
                "{} and {} are the same, period {}",
                i,
                last_step,
                i - last_step
            );
            break (last_step, i);
        } else {
            seen_states.insert(map_key, i);
        }

        frames.insert(i, map.clone());

        map = step(map);
        i += 1;
    };

    let target = 1000000000;
    let loop_duration = i1 - i0;
    let equivalent_step = (target - i0) % loop_duration + i0;

    println!("Equivalent step to {} is {}", target, equivalent_step);

    let equivalent_map = frames.get(&equivalent_step).expect("should have looped");
    let counts = count_all(&equivalent_map);

    let n_trees = *counts.get(&Tile::Trees).unwrap_or(&0);
    let n_lumberyards = *counts.get(&Tile::Lumberyard).unwrap_or(&0);

    let resource_value = n_trees * n_lumberyards;
    println!("Part 2: {}", resource_value);

    Ok(())
}
