use std::{collections::HashSet, fmt::Display};

use anyhow::{anyhow, Result};
use aoc::{direction::Direction, map::ParseMapTile};
use colored::Colorize;

type Map = aoc::map::Map<[i32; 2], Tile>;

#[derive(Debug, Clone)]
struct Tile {
    energized: bool,
    tile_type: TileType,
}

#[derive(Debug, Clone)]
enum TileType {
    Empty,
    Diagonal1,
    Diagonal2,
    Vertical,
    Horizontal,
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        let tile_type = match c {
            '.' => TileType::Empty,
            '/' => TileType::Diagonal1,
            '\\' => TileType::Diagonal2,
            '|' => TileType::Vertical,
            '-' => TileType::Horizontal,
            _ => return None,
        };

        Some(Self {
            energized: false,
            tile_type,
        })
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.tile_type {
            TileType::Empty => '░',
            TileType::Diagonal1 => '╱',
            TileType::Diagonal2 => '╲',
            TileType::Vertical => '│',
            TileType::Horizontal => '─',
        }
        .to_string();

        write!(
            f,
            "{}",
            if self.energized {
                c.yellow()
            } else {
                c.normal()
            }
        )
    }
}

fn raycast(
    map: &mut Map,
    origin: [i32; 2],
    direction: Direction,
    cache: &mut HashSet<([i32; 2], Direction)>,
) -> Result<()> {
    let ([imin, jmin], [imax, jmax]) = map.get_extent();
    let [mut i, mut j] = origin;
    let [di, dj] = direction.dpos();

    while i >= imin && j >= jmin && i <= imax && j <= jmax {
        if cache.contains(&([i, j], direction)) {
            break;
        } else {
            cache.insert(([i, j], direction));
        }

        let t = map
            .get_mut(&[i, j])
            .ok_or_else(|| anyhow!("Shot ray into emptiness @ {:?}", [i, j]))?;

        t.energized = true;

        match (&t.tile_type, direction) {
            (TileType::Empty, _) => {}
            (TileType::Diagonal1, Direction::North) => {
                return raycast(map, [i, j + 1], Direction::East, cache)
            }
            (TileType::Diagonal1, Direction::East) => {
                return raycast(map, [i - 1, j], Direction::North, cache)
            }
            (TileType::Diagonal1, Direction::South) => {
                return raycast(map, [i, j - 1], Direction::West, cache)
            }
            (TileType::Diagonal1, Direction::West) => {
                return raycast(map, [i + 1, j], Direction::South, cache)
            }
            (TileType::Diagonal2, Direction::North) => {
                return raycast(map, [i, j - 1], Direction::West, cache)
            }
            (TileType::Diagonal2, Direction::East) => {
                return raycast(map, [i + 1, j], Direction::South, cache)
            }
            (TileType::Diagonal2, Direction::South) => {
                return raycast(map, [i, j + 1], Direction::East, cache)
            }
            (TileType::Diagonal2, Direction::West) => {
                return raycast(map, [i - 1, j], Direction::North, cache)
            }
            (TileType::Vertical, Direction::North | Direction::South) => {}
            (TileType::Horizontal, Direction::West | Direction::East) => {}
            (TileType::Vertical, Direction::East | Direction::West) => {
                // split the ray north-south
                raycast(map, [i - 1, j], Direction::North, cache)?;
                raycast(map, [i + 1, j], Direction::South, cache)?;
                break;
            }
            (TileType::Horizontal, Direction::North | Direction::South) => {
                // split the ray east-west
                raycast(map, [i, j + 1], Direction::East, cache)?;
                raycast(map, [i, j - 1], Direction::West, cache)?;
                break;
            }
        }

        i += di;
        j += dj;
    }

    Ok(())
}

fn simulate(map: &Map, origin: [i32; 2], direction: Direction) -> Result<usize> {
    let mut map = map.clone();
    let mut cache = HashSet::new();
    raycast(&mut map, origin, direction, &mut cache)?;

    let energized = map.data.values().filter(|t| t.energized).count();

    Ok(energized)
}

fn simulate_all(map: &Map) -> Result<usize> {
    let ([imin, jmin], [imax, jmax]) = map.get_extent();

    let mut best = 0;

    for i in imin..=imax {
        best = usize::max(best, simulate(map, [i, jmin], Direction::East)?);
        best = usize::max(best, simulate(map, [i, jmax], Direction::West)?);
    }

    for j in jmin..=jmax {
        best = usize::max(best, simulate(map, [imin, j], Direction::South)?);
        best = usize::max(best, simulate(map, [imax, j], Direction::North)?);
    }

    Ok(best)
}

fn main() -> Result<()> {
    let map: Map = aoc::io::read_all("data/day16/input")?.parse()?;

    println!("Part 1: {}", simulate(&map, [0, 0], Direction::East)?);

    println!("Part 2: {}", simulate_all(&map)?);

    Ok(())
}
