use std::{fmt::Display, str::FromStr};

use anyhow::{Context, Error, Result};
use aoc::map::ParseMapTile;
use colored::Colorize;

type Map = aoc::map::Map<[i32; 2], Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile(u8);

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.0 + b'a') as char)
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        if c == '#' { Some(Tile(0)) } else { None }
    }
}

#[derive(Debug)]
struct Area {
    width: usize,
    height: usize,
    shape_counts: Vec<usize>,
}

/// an Area can be parsed from a string like "12x5: 1 0 1 0 2 2"
impl FromStr for Area {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (dims, counts) = s.split_once(':').context("Invalid area format")?;
        let (width, height) = dims.split_once('x').context("Invalid dimensions")?;
        let width: usize = width.trim().parse().context("Invalid width")?;
        let height: usize = height.trim().parse().context("Invalid height")?;

        let shape_counts = counts
            .trim()
            .split_whitespace()
            .map(|count_str| count_str.parse::<usize>().context("Invalid shape count"))
            .collect::<Result<Vec<_>>>()?;

        Ok(Area {
            width,
            height,
            shape_counts,
        })
    }
}

fn count_tiles(shape: &Map) -> usize {
    shape.data.len()
}

impl Area {
    fn area(&self) -> usize {
        self.width * self.height
    }

    /// A heuristic to check if the area can fit the shapes based on tile counts
    fn fits_shapes_heuristic(&self, shapes: &[Map]) -> bool {
        let needed_tiles = self
            .shape_counts
            .iter()
            .enumerate()
            .map(|(i, &count)| count * count_tiles(&shapes[i]))
            .sum::<usize>();

        needed_tiles <= self.area()
    }
}

#[derive(Debug)]
struct Problem {
    shapes: Vec<Map>,
    areas: Vec<Area>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let chunks = s.split("\n\n").collect::<Vec<_>>();

        let shapes = chunks[..chunks.len() - 1]
            .iter()
            .map(|chunk| {
                let (_num, map) = chunk.split_once(':').context("Invalid shape chunk")?;
                let map: Map = map.trim().parse()?;

                Ok(map)
            })
            .collect::<Result<Vec<_>>>()?;

        let areas: Vec<Area> = chunks
            .last()
            .context("Missing areas chunk")?
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<_>>>()?;

        Ok(Problem { shapes, areas })
    }
}

fn main() -> Result<()> {
    let problem: Problem = aoc::io::read_all((2025, 12))?.parse()?;

    let part1 = problem
        .areas
        .iter()
        .filter(|area| area.fits_shapes_heuristic(&problem.shapes))
        .count();

    println!("{} {}", "Part 1:".bold().green(), part1);
    Ok(())
}
