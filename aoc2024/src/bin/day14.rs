use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Context, Error, Result};
use colored::Colorize;
use lazy_static::lazy_static;
use nalgebra::Vector2;
use regex::Regex;

lazy_static! {
    static ref RE_ROBOT: Regex =
        Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").expect("valid regex");
}

#[derive(Debug, Clone)]
struct Extent {
    size: Vector2<i32>,
    bound: Vector2<i32>,
}

impl Extent {
    fn new(size: Vector2<i32>) -> Self {
        // 01234 5 6789a
        // 01234 5 6789a
        // 01234 5 6789a
        //
        // 01234 5 6789a
        // 01234 5 6789a
        // 01234 5 6789a

        let bound = size.component_div(&Vector2::from_element(2));

        Self { size, bound }
    }

    fn get_quadrant(&self, pos: Vector2<i32>) -> Option<u8> {
        let mut out = 0;

        if pos[0] == self.bound[0] {
            return None;
        } else if pos[0] > self.bound[0] {
            out += 1;
        }

        if pos[1] == self.bound[1] {
            return None;
        } else if pos[1] > self.bound[1] {
            out += 2;
        }

        Some(out)
    }
}

#[derive(Debug, Clone)]
struct Robot {
    pos: Vector2<i32>,
    vel: Vector2<i32>,
}

impl std::str::FromStr for Robot {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let m = RE_ROBOT
            .captures(s)
            .ok_or(anyhow!("Bad robot definition: '{}'", s))?;

        let components: Vec<i32> = (1..=4)
            .map(|i| m.get(i).ok_or(anyhow!("missing regex component {}", i)))
            .map(|r| r.and_then(|n| n.as_str().parse::<i32>().context("parse coord")))
            .collect::<Result<Vec<_>>>()?;

        let pos = Vector2::new(components[0], components[1]);
        let vel = Vector2::new(components[2], components[3]);

        Ok(Self { pos, vel })
    }
}

impl Robot {
    fn position_at(&self, time: i32, extent: &Extent) -> Vector2<i32> {
        // ( pos + time * vel ) % extent.size
        (self.pos + time * self.vel + time * extent.size).zip_map(&extent.size, |a, b| a % b)
    }
}

fn part1(robots: &[Robot], extent: &Extent) {
    let mut quadrant_counts: HashMap<u8, usize> = HashMap::new();

    for robot in robots {
        let pos = robot.position_at(100, extent);

        if let Some(quad) = extent.get_quadrant(pos) {
            *quadrant_counts.entry(quad).or_default() += 1;
        }
    }

    let safety = quadrant_counts.into_values().product::<usize>();

    println!("Part 1: {}", safety);
}

#[derive(Debug, Clone)]
struct Tile;

type Map = aoc::map::Map<[i32; 2], Tile>;

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "█".green())
    }
}

fn draw(positions: &HashSet<Vector2<i32>>) {
    let mut map = Map::new();

    for pos in positions {
        let &[j, i] = pos.as_slice() else {
            continue;
        };

        map.set([i, j], Tile);
    }

    println!("{}", map);
}

fn part2(robots: &[Robot], extent: &Extent) {
    let mut time = 0;

    'outer: loop {
        let mut positions: HashSet<Vector2<i32>> = HashSet::new();
        for robot in robots {
            let pos = robot.position_at(time, extent);
            if positions.contains(&pos) {
                time += 1;
                continue 'outer;
            }

            positions.insert(pos);
        }

        draw(&positions);

        println!("Part 2: {}", time);
        break;
    }
}

fn main() -> Result<()> {
    let robots: Vec<Robot> = aoc::io::read_lines((2024, 14))?;
    let extent = Extent::new(Vector2::new(101, 103));

    //let robots: Vec<Robot> = aoc::io::read_lines("data/day14/example")?;
    //let extent = Extent::new(Vector2::new(11, 7));

    part1(&robots, &extent);
    part2(&robots, &extent);

    Ok(())
}
