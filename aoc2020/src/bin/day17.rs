use std::collections::{HashMap, HashSet};

use snafu::{ResultExt, Snafu};

use aoc2020::map::{Map, MapCoordinate, MapError, MapTile, ParseMapTile};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Map parsing error: {}", source))]
    ParseMap { source: MapError },

    #[snafu(display("Field parsing error"))]
    ParseField,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Inactive,
    Active,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Inactive => ".",
                Tile::Active => "#",
            }
        )
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '.' => Tile::Inactive,
            '#' => Tile::Active,
            _ => return None,
        })
    }
}

trait NeighborsIter
where
    Self: MapCoordinate,
{
    fn get_neighbor_positions(&self) -> Vec<Self>;
    fn get_in_range_positions<T: MapTile>(map: &Map<Self, T>) -> Vec<Self>;
}

impl NeighborsIter for [i64; 3] {
    fn get_neighbor_positions(&self) -> Vec<Self> {
        let mut out = Vec::new();
        for iofs in -1..=1 {
            for jofs in -1..=1 {
                for kofs in -1..=1 {
                    if iofs == 0 && jofs == 0 && kofs == 0 {
                        continue;
                    }
                    out.push([self[0] + iofs, self[1] + jofs, self[2] + kofs]);
                }
            }
        }
        out
    }

    fn get_in_range_positions<T: MapTile>(map: &Map<Self, T>) -> Vec<Self> {
        let (min, max) = map.get_extent();

        let mut out = Vec::new();
        for i in min[0] - 1..=max[0] + 1 {
            for j in min[1] - 1..=max[1] + 1 {
                for k in min[2] - 1..=max[2] + 1 {
                    out.push([i, j, k]);
                }
            }
        }
        out
    }
}

impl NeighborsIter for [i64; 4] {
    fn get_neighbor_positions(&self) -> Vec<Self> {
        let mut out = Vec::new();
        for iofs in -1..=1 {
            for jofs in -1..=1 {
                for kofs in -1..=1 {
                    for lofs in -1..=1 {
                        if iofs == 0 && jofs == 0 && kofs == 0 && lofs == 0 {
                            continue;
                        }
                        out.push([
                            self[0] + iofs,
                            self[1] + jofs,
                            self[2] + kofs,
                            self[3] + lofs,
                        ]);
                    }
                }
            }
        }
        out
    }

    fn get_in_range_positions<T: MapTile>(map: &Map<Self, T>) -> Vec<Self> {
        let (min, max) = map.get_extent();

        let mut out = Vec::new();
        for i in min[0] - 1..=max[0] + 1 {
            for j in min[1] - 1..=max[1] + 1 {
                for k in min[2] - 1..=max[2] + 1 {
                    for l in min[3] - 1..=max[3] + 1 {
                        out.push([i, j, k, l]);
                    }
                }
            }
        }
        out
    }
}

fn count_neighbors<P: NeighborsIter>(map: &Map<P, Tile>, pos: P) -> usize {
    let mut n_active = 0;
    for n in pos.get_neighbor_positions() {
        if let Some(Tile::Active) = map.get(&n) {
            n_active += 1;
        }
    }
    n_active
}

fn count_active<C: MapCoordinate>(map: &Map<C, Tile>) -> usize {
    let mut n_active = 0;
    for tile in map.data.values() {
        if tile == &Tile::Active {
            n_active += 1;
        }
    }
    n_active
}

fn step<P: NeighborsIter>(map: &mut Map<P, Tile>) {
    let neighbors: HashMap<P, usize> = P::get_in_range_positions::<Tile>(&map)
        .into_iter()
        .map(|pos| (pos, count_neighbors(&map, pos)))
        .collect();

    for (pos, n) in neighbors {
        let current = map.get(&pos).unwrap_or(&Tile::Inactive).clone();
        map.set(
            pos,
            match (n, current) {
                (n, Tile::Active) if n >= 2 && n <= 3 => Tile::Active,
                (3, Tile::Inactive) => Tile::Active,
                _ => Tile::Inactive,
            },
        );
    }
}

fn main() -> Result<()> {
    let map2d: Map<[i64; 2], Tile> = std::fs::read_to_string("data/day17/input")
        .context(Io)?
        .parse()
        .context(ParseMap)?;

    let mut map3d: Map<[i64; 3], Tile> = Map::from_2d(&map2d);
    let mut map4d: Map<[i64; 4], Tile> = Map::from_3d(&map3d);

    for cycle in 1..=6 {
        println!("\nCYCLE {} \\\\\\\\\\\\\\\\\\", cycle);
        step(&mut map3d);
        step(&mut map4d);

        // println!("{}", map3d);
        // println!("{}", map4d);

        println!(
            "Part 1: Got {} active tiles, extent {:?}",
            count_active(&map3d),
            map3d.get_extent()
        );
        println!(
            "Part 2: Got {} active tiles, extent {:?}",
            count_active(&map4d),
            map4d.get_extent()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
