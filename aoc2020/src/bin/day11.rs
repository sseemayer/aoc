use std::collections::HashMap;
use std::fs::File;

use snafu::{ResultExt, Snafu};

use aoc2020::map::{Map, MapError, ParseMapTile};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error on '{}': {}", filename, source))]
    Io {
        filename: String,
        source: std::io::Error,
    },

    #[snafu(display("Map error: {}", source))]
    MapLoading { source: MapError },
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Floor,
    Chair { occupied: bool },
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Floor),
            'L' => Some(Tile::Chair { occupied: false }),
            '#' => Some(Tile::Chair { occupied: true }),
            _ => None,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Floor => "░",
                Tile::Chair { occupied: false } => "○",
                Tile::Chair { occupied: true } => "●",
            }
        )
    }
}

fn step(map: &mut Map<[usize; 2], Tile>, cast_ray: bool, max_neighbors: usize) {
    // count number of per-chair neighbors
    let mut neighbors: HashMap<[usize; 2], usize> = HashMap::new();
    for ([i, j], tile) in map.data.iter() {
        if let Tile::Chair { .. } = tile {
            let mut n = 0;
            for idir in -1..=1 {
                for jdir in -1..=1 {
                    if idir == 0 && jdir == 0 {
                        continue;
                    }

                    let mut k = 1;
                    loop {
                        let ic = (*i as i64) + k * idir;
                        let jc = (*j as i64) + k * jdir;
                        if ic < 0 || jc < 0 {
                            break;
                        }

                        match map.get(&[ic as usize, jc as usize]) {
                            Some(Tile::Chair { occupied }) => {
                                if *occupied {
                                    n += 1;
                                }
                                break;
                            }
                            None => break,
                            _ => {}
                        }

                        if !cast_ray {
                            break;
                        }
                        k += 1;
                    }
                }
            }

            neighbors.insert([*i, *j], n);
        }
    }

    // apply update rule
    for (coord, tile) in map.data.iter_mut() {
        if let Tile::Chair { ref mut occupied } = tile {
            let filled_neighbors = neighbors[coord];
            if !*occupied && filled_neighbors == 0 {
                *occupied = true;
            } else if *occupied && filled_neighbors >= max_neighbors {
                *occupied = false;
            }
        }
    }
}

fn loop_until_stabilized(
    mut map: Map<[usize; 2], Tile>,
    cast_ray: bool,
    max_neighbors: usize,
) -> Map<[usize; 2], Tile> {
    loop {
        let map_last = map.clone();
        step(&mut map, cast_ray, max_neighbors);
        println!("{}", map);
        if map == map_last {
            return map;
        }
    }
}

fn count_filled_seats(map: &Map<[usize; 2], Tile>) -> usize {
    let mut filled_seats = 0;
    for tile in map.data.values() {
        if let Tile::Chair { occupied: true } = tile {
            filled_seats += 1;
        }
    }
    filled_seats
}

fn main() -> Result<()> {
    let filename = "data/day11/input";
    let mut f = File::open(filename).context(Io {
        filename: filename.to_string(),
    })?;

    let map_original = Map::<[usize; 2], Tile>::read(&mut f).context(MapLoading)?;

    let map1 = loop_until_stabilized(map_original.clone(), false, 4);
    println!("Part 1: Got {} filled seats", count_filled_seats(&map1));

    let map2 = loop_until_stabilized(map_original.clone(), true, 5);
    println!("Part 2: Got {} filled seats", count_filled_seats(&map2));

    Ok(())
}
