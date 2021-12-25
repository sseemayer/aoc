use anyhow::Result;
use aoc::map::ParseMapTile;
use std::fs::File;

type Map = aoc::map::Map<[i64; 2], Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Right,
    Down,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "░"),
            Tile::Right => write!(f, ">"),
            Tile::Down => write!(f, "v"),
        }
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Empty),
            '>' => Some(Tile::Right),
            'v' => Some(Tile::Down),
            _ => None,
        }
    }
}

fn step(map: &mut Map) -> bool {
    let mut moving_right = Vec::new();

    let ([imin, jmin], [imax, jmax]) = map.get_extent();

    let irange = imax - imin + 1;
    let jrange = jmax - jmin + 1;

    let wrap_i = |i| (i - imin) % irange + imin;
    let wrap_j = |j| (j - jmin) % jrange + jmin;

    for (&[i, j], &t) in map.data.iter() {
        if t == Tile::Right {
            if map.get(&[i, wrap_j(j + 1)]) == Some(&Tile::Empty) {
                moving_right.push([i, j]);
            }
        }
    }

    for &[i, j] in &moving_right {
        map.set([i, j], Tile::Empty);
        map.set([i, wrap_j(j + 1)], Tile::Right);
    }

    let mut moving_down = Vec::new();

    for (&[i, j], &t) in map.data.iter() {
        if t == Tile::Down {
            if map.get(&[wrap_i(i + 1), j]) == Some(&Tile::Empty) {
                moving_down.push([i, j]);
            }
        }
    }

    for &[i, j] in &moving_down {
        map.set([i, j], Tile::Empty);
        map.set([wrap_i(i + 1), j], Tile::Down);
    }

    moving_right.is_empty() && moving_down.is_empty()
}

fn main() -> Result<()> {
    let mut map = Map::read(&mut File::open("data/day25/input")?)?;

    let mut n = 1;

    while !step(&mut map) {
        n += 1;
        // println!("Step {}:\n{}", n, map);
    }

    println!("Part 1: {}", n);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
}
