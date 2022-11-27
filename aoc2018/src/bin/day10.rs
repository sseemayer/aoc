use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_POINT: Regex =
        Regex::new(r"position=<([0-9 -]+), ([0-9 -]+)> velocity=<([0-9 -]+), ([0-9 -]+)>").unwrap();
}

#[derive(Debug, Clone)]
struct Point {
    velocity: [i32; 2],
}

#[derive(Default, Debug, Clone)]
struct Tile {
    points: Vec<Point>,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#")
    }
}

type Map = aoc::map::Map<[i32; 2], Tile>;

fn parse_points(f: &str) -> Result<Map> {
    let mut out = Map::new();

    for line in BufReader::new(File::open(f)?).lines() {
        let line = line?;

        let captures = RE_POINT
            .captures(&line)
            .ok_or_else(|| anyhow!("Bad line: '{}'", line))?;

        let px: i32 = captures.get(1).unwrap().as_str().trim().parse()?;
        let py: i32 = captures.get(2).unwrap().as_str().trim().parse()?;
        let vx: i32 = captures.get(3).unwrap().as_str().trim().parse()?;
        let vy: i32 = captures.get(4).unwrap().as_str().trim().parse()?;

        out.data
            .entry([py, px])
            .or_default()
            .points
            .push(Point { velocity: [vy, vx] });
    }

    Ok(out)
}

fn step(map: &Map) -> Map {
    let mut new: HashMap<[i32; 2], Tile> = HashMap::new();

    for (&[i, j], tile) in map.data.iter() {
        for point in &tile.points {
            let new_pos = [i + point.velocity[0], j + point.velocity[1]];
            new.entry(new_pos).or_default().points.push(point.clone());
        }
    }

    Map {
        data: new,
        fixed_extent: None,
    }
}

fn calculate_area(map: &Map) -> usize {
    let ([imin, jmin], [imax, jmax]) = map.get_extent();
    let area = ((imax - imin) as usize) * ((jmax - jmin) as usize);

    area
}

fn main() -> Result<()> {
    let mut map = parse_points("data/day10/input")?;

    let mut smallest_area = calculate_area(&map);
    let mut steps = 0;
    loop {
        let new_map = step(&map);
        let new_area = calculate_area(&new_map);

        if new_area < smallest_area {
            smallest_area = new_area;
            map = new_map;
            steps += 1;
        } else {
            println!("{}\nSteps: {}", map, steps);
            break;
        }
    }

    Ok(())
}
