use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{bail, Result};
use colored::Colorize;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
enum Tile {
    Spring,
    Clay,
    StillWater,
    MovingWater,
}

type Map = aoc::map::Map<[i32; 2], Tile>;

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Spring => write!(f, "{}", "+".red()),
            Tile::Clay => write!(f, "█"),
            Tile::StillWater => write!(f, "{}", "▒".blue()),
            Tile::MovingWater => write!(f, "{}", "░".blue()),
        }
    }
}

impl Tile {
    fn is_solid(&self) -> bool {
        match self {
            Tile::Clay | Tile::StillWater => true,
            _ => false,
        }
    }
}

lazy_static! {
    static ref RE_HORIZONTAL: Regex = Regex::new(r"y=(\d+), x=(\d+)\.\.(\d+)").unwrap();
    static ref RE_VERTICAL: Regex = Regex::new(r"x=(\d+), y=(\d+)\.\.(\d+)").unwrap();
}

fn parse(path: &str) -> Result<Map> {
    let mut map = Map::new();
    for line in BufReader::new(File::open(path)?).lines() {
        let line = line?;

        if let Some(c) = RE_HORIZONTAL.captures(&line) {
            let i: i32 = c.get(1).expect("i").as_str().parse()?;
            let j0: i32 = c.get(2).expect("j0").as_str().parse()?;
            let j1: i32 = c.get(3).expect("j1").as_str().parse()?;

            for j in j0..=j1 {
                map.set([i, j], Tile::Clay);
            }
        } else if let Some(c) = RE_VERTICAL.captures(&line) {
            let j: i32 = c.get(1).expect("j").as_str().parse()?;
            let i0: i32 = c.get(2).expect("i0").as_str().parse()?;
            let i1: i32 = c.get(3).expect("i1").as_str().parse()?;

            for i in i0..=i1 {
                map.set([i, j], Tile::Clay);
            }
        } else {
            bail!("Bad line: '{}'", line);
        }
    }

    map.set([0, 500], Tile::Spring);

    Ok(map)
}

#[derive(Debug, PartialEq, Eq)]
enum FillResult {
    NewDrip(i32),
    OldDrip,
    Wall,
}

fn fill(
    map: &mut Map,
    source_coord: [i32; 2],
    jdir: i32,
    reached: &mut HashSet<[i32; 2]>,
) -> FillResult {
    let [i, mut j] = source_coord;
    j += jdir;

    loop {
        let solid_underneath = map.get(&[i, j]).map(|t| t.is_solid()).unwrap_or(false);
        let solid_there = map.get(&[i - 1, j]).map(|t| t.is_solid()).unwrap_or(false);

        match (solid_there, solid_underneath) {
            (true, _) => {
                // we have hit a wall on the left - stop filling
                return FillResult::Wall;
            }
            (false, true) => {
                // no wall yet - keep filling
                map.set([i - 1, j], Tile::MovingWater);
                reached.insert([i - 1, j]);
            }
            (false, false) => {
                // hole underneath
                if let Some(Tile::Spring) = map.get(&[i - 1, j]) {
                    return FillResult::OldDrip;
                } else {
                    map.set([i - 1, j], Tile::Spring);
                    return FillResult::NewDrip(j);
                }
            }
        }

        j += jdir;
    }
}

fn process_source(map: &mut Map, source_coord: [i32; 2]) -> bool {
    // print!("{}", map);
    let ([_, _], [imax, _]) = map.get_extent();

    let [mut i, j] = source_coord;
    i += 1;

    // go down until hitting something solid
    loop {
        if i > imax {
            return false;
        }

        if let Some(t) = map.get(&[i, j]) {
            if t.is_solid() {
                break;
            }
        }

        map.set([i, j], Tile::MovingWater);
        i += 1;
    }

    // we have hit something -- now time to fill!
    loop {
        // print!("{}", map);

        let mut reached = HashSet::new();
        reached.insert([i - 1, j]);

        let mut j_left = fill(map, [i, j], -1, &mut reached);
        let mut j_right = fill(map, [i, j], 1, &mut reached);

        while let FillResult::NewDrip(j) = j_left {
            // println!("drip {:?} -> {:?}", source_coord, [i - 1, j]);
            if process_source(map, [i - 1, j]) {
                map.set([i - 1, j], Tile::MovingWater);
                reached.insert([i - 1, j]);
                j_left = fill(map, [i, j], -1, &mut reached);
            } else {
                break;
            }
        }

        while let FillResult::NewDrip(j) = j_right {
            // println!("drip {:?} -> {:?}", source_coord, [i - 1, j]);
            if process_source(map, [i - 1, j]) {
                map.set([i - 1, j], Tile::MovingWater);
                reached.insert([i - 1, j]);
                j_right = fill(map, [i, j], 1, &mut reached);
            } else {
                break;
            }
        }

        if j_left == FillResult::Wall && j_right == FillResult::Wall {
            // current layer has left and right wall - make it solid
            for coord in reached {
                map.set(coord, Tile::StillWater);
            }

            // println!("fill {:?}", [i - 1, j]);
        } else {
            // we had an overflow, so don't fill higher
            return false;
        }

        i -= 1;

        if i <= source_coord[0] + 1 {
            // we managed to fill up to the current source
            return true;
        }
    }
}

fn main() -> Result<()> {
    let mut map = parse("data/day17/input")?;

    process_source(&mut map, [0, 500]);
    println!("{}", map);

    let min_i = map
        .data
        .iter()
        .filter_map(|(pos, tile)| {
            if let Tile::Clay = tile {
                Some(pos[0])
            } else {
                None
            }
        })
        .min()
        .unwrap_or(0);

    let water_reachable = map
        .data
        .iter()
        .filter(|(pos, tile)| {
            if pos[0] < min_i {
                return false;
            }

            match tile {
                Tile::Spring | Tile::StillWater | Tile::MovingWater => true,
                _ => false,
            }
        })
        .count();

    let water_steady = map
        .data
        .iter()
        .filter(|(_pos, tile)| matches!(tile, Tile::StillWater))
        .count();

    println!("Part 1: {}", water_reachable);
    println!("Part 2: {}", water_steady);

    Ok(())
}
