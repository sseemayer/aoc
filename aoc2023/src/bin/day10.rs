use std::collections::{HashSet, VecDeque};

use anyhow::{anyhow, bail, Result};

use aoc::{direction::Direction, map::ParseMapTile};
use colored::Colorize;

type Map = aoc::map::Map<[i32; 2], Tile>;

#[derive(Debug, Clone)]
struct Tile {
    distance: Option<usize>,
    tile: TileType,
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        let tile = match c {
            'S' => TileType::AnimalStart,
            '|' => TileType::Vertical,
            '-' => TileType::Horizontal,
            'L' => TileType::NorthEast,
            'J' => TileType::NorthWest,
            '7' => TileType::SouthWest,
            'F' => TileType::SouthEast,
            _ => return None,
        };

        Some(Self {
            distance: None,
            tile,
        })
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tile = match self.tile {
            TileType::AnimalStart => "S",
            TileType::Inner => "█",
            TileType::Vertical => "│",
            TileType::Horizontal => "─",
            TileType::NorthEast => "└",
            TileType::NorthWest => "┘",
            TileType::SouthWest => "┐",
            TileType::SouthEast => "┌",
        };

        write!(
            f,
            "{}",
            if self.tile == TileType::Inner {
                tile.blue()
            } else if let Some(d) = self.distance {
                if d == 0 {
                    tile.green()
                } else if d < 4000 {
                    tile.yellow()
                } else if d < 6500 {
                    tile.red()
                } else {
                    tile.purple()
                }
            } else {
                tile.normal()
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TileType {
    AnimalStart, // S
    Inner,
    Vertical,   // |
    Horizontal, // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
}

impl TileType {
    fn connects_to(&self, dir: Direction) -> bool {
        match (self, dir) {
            (TileType::Vertical, Direction::North | Direction::South) => true,
            (TileType::Horizontal, Direction::East | Direction::West) => true,
            (TileType::NorthEast, Direction::North | Direction::East) => true,
            (TileType::NorthWest, Direction::North | Direction::West) => true,
            (TileType::SouthWest, Direction::South | Direction::West) => true,
            (TileType::SouthEast, Direction::South | Direction::East) => true,
            _ => false,
        }
    }

    fn get_connections(&self) -> &[Direction] {
        match self {
            TileType::AnimalStart | TileType::Inner => &[],
            TileType::Vertical => &[Direction::North, Direction::South],
            TileType::Horizontal => &[Direction::West, Direction::East],
            TileType::NorthEast => &[Direction::North, Direction::East],
            TileType::NorthWest => &[Direction::North, Direction::West],
            TileType::SouthWest => &[Direction::South, Direction::West],
            TileType::SouthEast => &[Direction::South, Direction::East],
        }
    }
}

fn split_out_start(mut map: Map) -> Result<(Map, [i32; 2])> {
    let [i, j] = map
        .find_one_where(|_, t| &t.tile == &TileType::AnimalStart)
        .ok_or_else(|| anyhow!("Cannot find start"))?;

    let north = map
        .get(&[i - 1, j])
        .is_some_and(|t| t.tile.connects_to(Direction::South));

    let east = map
        .get(&[i, j + 1])
        .is_some_and(|t| t.tile.connects_to(Direction::West));

    let south = map
        .get(&[i + 1, j])
        .is_some_and(|t| t.tile.connects_to(Direction::North));

    let west = map
        .get(&[i, j - 1])
        .is_some_and(|t| t.tile.connects_to(Direction::East));

    let tile_at_start = match (north, east, south, west) {
        (true, false, true, false) => TileType::Vertical,
        (false, true, false, true) => TileType::Horizontal,
        (true, true, false, false) => TileType::NorthEast,
        (true, false, false, true) => TileType::NorthWest,
        (false, false, true, true) => TileType::SouthWest,
        (false, true, true, false) => TileType::SouthEast,
        _ => bail!(
            "Bad connections: N={} E={} S={} W={}",
            north,
            east,
            south,
            west
        ),
    };

    map.set(
        [i, j],
        Tile {
            distance: Some(0),
            tile: tile_at_start,
        },
    );

    Ok((map, [i, j]))
}

fn mark_distances(map: &mut Map, start: [i32; 2]) -> Result<(usize, [i32; 2])> {
    let mut furthest = (0, start);
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    seen.insert(start);
    queue.push_back((start, 0));

    while let Some(([i, j], dist)) = queue.pop_front() {
        let tile = map
            .get_mut(&[i, j])
            .ok_or_else(|| anyhow!("Moved to invalid tile at {:?}", [i, j]))?;

        tile.distance = Some(dist);

        if dist > furthest.0 {
            furthest = (dist, [i, j]);
        }

        for dir in tile.tile.get_connections() {
            let [di, dj] = dir.dpos();
            let [si, sj] = [i + di, j + dj];

            if !seen.contains(&[si, sj]) {
                queue.push_back(([si, sj], dist + 1));
                seen.insert([si, sj]);
            }
        }
    }

    Ok(furthest)
}

fn mark_inner(map: &mut Map) -> Result<usize> {
    let ([imin, jmin], [imax, jmax]) = map.get_extent();
    let mut filled = 0;

    for i in imin..=imax {
        let mut inside = false;
        let mut last_cross = None;
        for j in jmin..=jmax {
            if let Some(tile) = map.get(&[i, j]).filter(|t| t.distance.is_some()) {
                let is_crossing = match &tile.tile {
                    TileType::Vertical => true,
                    TileType::Inner | TileType::Horizontal | TileType::AnimalStart => false,
                    _ => match (last_cross, &tile.tile) {
                        (Some(TileType::NorthEast), TileType::NorthWest)
                        | (Some(TileType::SouthEast), TileType::SouthWest)
                        | (Some(TileType::NorthWest), TileType::NorthEast)
                        | (Some(TileType::SouthWest), TileType::SouthEast) => {
                            last_cross = None;
                            false
                        }
                        (
                            None,
                            TileType::NorthEast
                            | TileType::NorthWest
                            | TileType::SouthWest
                            | TileType::SouthEast,
                        ) => {
                            last_cross = Some(tile.tile.clone());
                            false
                        }
                        (Some(TileType::NorthEast), TileType::SouthWest)
                        | (Some(TileType::SouthEast), TileType::NorthWest)
                        | (Some(TileType::NorthWest), TileType::SouthEast)
                        | (Some(TileType::SouthWest), TileType::NorthEast) => {
                            last_cross = None;
                            true
                        }
                        (_, _) => unreachable!(),
                    },
                };

                if is_crossing {
                    inside = !inside;
                }
            } else {
                if inside {
                    filled += 1;
                    map.set(
                        [i, j],
                        Tile {
                            distance: None,
                            tile: TileType::Inner,
                        },
                    );
                }
            }
        }
    }

    Ok(filled)
}

fn main() -> Result<()> {
    let map: Map = aoc::io::read_all("data/day10/input")?.parse()?;

    let (mut map, start) = split_out_start(map)?;

    let (furthest_dist, furthest_pos) = mark_distances(&mut map, start)?;
    let filled = mark_inner(&mut map)?;

    println!("{}", map);

    println!("Part 1: {} @ {:?}", furthest_dist, furthest_pos);
    println!("Part 2: {}", filled);

    Ok(())
}
