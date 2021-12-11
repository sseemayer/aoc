use std::{collections::HashSet, fs::File};

use aoc::map::{Map, MapError, ParseMapTile};
use colored::Colorize;
use thiserror::Error;

#[derive(Error, Debug)]
enum Day11Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Map(#[from] MapError),
}

type Result<T> = std::result::Result<T, Day11Error>;

#[derive(Debug, Clone, Copy)]
struct Tile(u8);

const BOX_CHARACTERS: [char; 11] = ['*', '1', '2', '3', '4', '5', '6', '7', '8', '9', '█'];

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.0.min(10) as usize;
        let s = format!("{}", BOX_CHARACTERS[n]);

        if self.0 == 0 {
            write!(f, "{}", s.on_white())
        } else if self.0 > 9 {
            write!(f, "{}", s.on_red())
        } else {
            write!(f, "{}", s)
        }
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        Some(Tile(c as u8 - '0' as u8))
    }
}

const OFFSETS: [[i64; 2]; 8] = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [-1, 0],
    [1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
];

fn step(map: &mut Map<[i64; 2], Tile>) -> HashSet<[i64; 2]> {
    // increase all energy levels by 1
    for (_pos, tile) in map.data.iter_mut() {
        tile.0 += 1;
    }

    let mut flashes = HashSet::new();
    loop {
        let new_flashes = map.find_all_where(|p, t| t.0 > 9 && !flashes.contains(p));

        if new_flashes.is_empty() {
            break;
        } else {
            // perform flashes
            for [i, j] in new_flashes.into_iter() {
                for [iofs, jofs] in OFFSETS {
                    if let Some(t) = map.get_mut(&[i + iofs, j + jofs]) {
                        t.0 += 1;
                    }
                }

                flashes.insert([i, j]);
            }
        }
    }

    for pos in flashes.iter() {
        map.get_mut(pos).unwrap().0 = 0;
    }

    flashes
}

fn main() -> Result<()> {
    let mut map: Map<[i64; 2], Tile> = Map::read(&mut File::open("data/day11/input")?)?;

    let mut n_flashes = 0;
    let mut i = 0;
    loop {
        let new_flashes = step(&mut map).len();
        n_flashes += new_flashes;
        i += 1;

        println!(
            "Step {} ({} flashes, total {} flashes):\n{}",
            i, new_flashes, n_flashes, map
        );

        if new_flashes == map.data.len() {
            break;
        }
    }

    Ok(())
}
