use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Context, Result};
use colored::Colorize;

type Pos = [i32; 2];
type Map = aoc::map::Map<Pos, Tile>;

#[derive(Debug, Clone)]
enum Tile {
    Sand,
    Wall,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Sand => write!(f, "{}", "░".yellow()),
            Tile::Wall => write!(f, "█"),
        }
    }
}

fn parse(path: &str) -> Result<Map> {
    let mut map = Map::new();
    for line in BufReader::new(File::open(path)?).lines() {
        let points = line?
            .split(" -> ")
            .map(|p| {
                let (a, b) = p
                    .split_once(",")
                    .ok_or_else(|| anyhow!("No delimiter found: '{}'", p))?;

                let a: i32 = a.parse()?;
                let b: i32 = b.parse()?;

                Ok([a, b])
            })
            .collect::<Result<Vec<Pos>>>()?;

        for p in 0..points.len() - 1 {
            let [x0, y0] = points[p];
            let [x1, y1] = points[p + 1];

            let i0 = i32::min(y0, y1);
            let i1 = i32::max(y0, y1);
            let j0 = i32::min(x0, x1);
            let j1 = i32::max(x0, x1);

            for i in i0..=i1 {
                for j in j0..=j1 {
                    map.set([i, j], Tile::Wall);
                }
            }
        }
    }

    Ok(map)
}

fn step(map: &mut Map) -> bool {
    let (_, [max_i, _]) = map.get_extent();

    let mut i = 0;
    let mut j = 500;

    loop {
        if i > max_i {
            return false;
        } else if map.get(&[i + 1, j]).is_none() {
            i += 1;
        } else if map.get(&[i + 1, j - 1]).is_none() {
            i += 1;
            j -= 1;
        } else if map.get(&[i + 1, j + 1]).is_none() {
            i += 1;
            j += 1;
        } else {
            map.set([i, j], Tile::Sand);

            if i == 0 && j == 500 {
                return false;
            }

            return true;
        }
    }
}

fn simulate(mut map: Map) -> usize {
    let mut grains = 0;
    while step(&mut map) {
        grains += 1;
    }
    println!("{}", map);

    grains
}

fn main() -> Result<()> {
    let map = parse("data/day14/input")?;

    let part1 = simulate(map.clone());
    println!("Part 1: {}", part1);

    let mut map2 = map.clone();
    let ([_, min_j], [max_i, max_j]) = map2.get_extent();
    for j in min_j - max_i..max_j + max_i + 2 {
        map2.set([max_i + 2, j], Tile::Wall);
    }

    let part2 = simulate(map2) + 1;
    println!("Part 2: {}", part2);

    Ok(())
}
