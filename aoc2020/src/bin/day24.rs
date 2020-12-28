use std::collections::HashMap;

use snafu::{ResultExt, Snafu};

use aoc2020::map::{Map, MapCoordinate};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Number parsing error for '{}': {}", data, source))]
    ParseNumber {
        data: String,
        source: std::num::ParseIntError,
    },

    #[snafu(display("Cannot parse direction: '{}", data))]
    ParseDirection { data: String },
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn all() -> Vec<Self> {
        vec![
            Direction::East,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
            Direction::NorthEast,
        ]
    }

    fn to_axial_coords(&self) -> HexPosition {
        let (q, r) = match self {
            Direction::East => (1, 0),
            Direction::SouthEast => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (0, -1),
            Direction::NorthEast => (1, -1),
        };

        HexPosition { q, r }
    }
}

#[derive(Debug)]
struct HexPosition {
    q: i64,
    r: i64,
}

impl std::str::FromStr for HexPosition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut q: i64 = 0;
        let mut r: i64 = 0;

        let mut s = s.chars();
        while let Some(c) = s.next() {
            let dir = match c {
                'e' => Direction::East,
                'w' => Direction::West,
                'n' | 's' => {
                    let d = s.next().ok_or(Error::ParseDirection {
                        data: format!("'{}' without character after", c),
                    })?;

                    match (c, d) {
                        ('n', 'w') => Direction::NorthWest,
                        ('n', 'e') => Direction::NorthEast,
                        ('s', 'w') => Direction::SouthWest,
                        ('s', 'e') => Direction::SouthEast,
                        _ => {
                            return Err(Error::ParseDirection {
                                data: format!("{}{}", c, d),
                            })
                        }
                    }
                }
                _ => {
                    return Err(Error::ParseDirection {
                        data: format!("{}", c),
                    })
                }
            };

            let axdir = dir.to_axial_coords();
            q += axdir.q;
            r += axdir.r;
        }

        Ok(Self { q, r })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    White,
    Black,
}

fn conway_step(map: &mut Map<[i64; 2], Tile>) {
    let (min, max) = map.get_extent();

    let mut neighbors: HashMap<[i64; 2], usize> = HashMap::new();

    for q in min[0] - 2..max[0] + 2 {
        for r in min[1] - 2..max[1] + 2 {
            let mut n_neighbors = 0;
            for ofs in Direction::all().into_iter().map(|d| d.to_axial_coords()) {
                if let Tile::Black = map.get(&[q + ofs.q, r + ofs.r]).unwrap_or(&Tile::White) {
                    n_neighbors += 1;
                }

                neighbors.insert([q, r], n_neighbors);
            }
        }
    }

    for q in min[0] - 2..max[0] + 2 {
        for r in min[1] - 2..max[1] + 2 {
            let n = neighbors[&[q, r]];
            let t = map.get(&[q, r]).unwrap_or(&Tile::White);

            if t == &Tile::Black && (n == 0 || n > 2) {
                // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
                map.set([q, r], Tile::White);
            } else if t == &Tile::White && n == 2 {
                // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black
                map.set([q, r], Tile::Black);
            }
        }
    }
}

fn count_black(map: &Map<[i64; 2], Tile>) -> usize {
    let mut n_black = 0;
    for t in map.data.values() {
        if let Tile::Black = t {
            n_black += 1;
        }
    }
    n_black
}

fn main() -> Result<()> {
    let dirs: Vec<HexPosition> = std::fs::read_to_string("data/day24/input")
        .context(Io)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_>>()?;

    let mut map: Map<[i64; 2], Tile> = Map::new();
    for d in &dirs {
        let t = *map.get(&[d.q, d.r]).unwrap_or(&Tile::White);

        map.set(
            [d.q, d.r],
            if let Tile::White = t {
                Tile::Black
            } else {
                Tile::White
            },
        );
    }

    println!("Part 1: {}", count_black(&map));

    for i in 1..=100 {
        conway_step(&mut map);
        println!("day {:3}: {}", i, count_black(&map));
    }

    println!("Part 2: {}", count_black(&map));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
