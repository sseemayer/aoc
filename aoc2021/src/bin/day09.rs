use std::{collections::HashSet, fs::File};

use aoc::map::{Map, MapError, ParseMapTile};
use colored::Colorize;
use thiserror::Error;

#[derive(Error, Debug)]
enum Day09Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Map(#[from] MapError),
}

type Result<T> = std::result::Result<T, Day09Error>;

#[derive(Debug, Clone, Copy)]
struct Tile(u8, Option<usize>, bool);

const BOX_CHARACTERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '█'];

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{}", BOX_CHARACTERS[self.0 as usize]);
        if self.2 {
            write!(f, "{}", s.bright_white().on_black())
        } else if self.1.is_some() {
            let c = self.1.unwrap() % 6;

            match c {
                0 => write!(f, "{}", s.on_green()),
                1 => write!(f, "{}", s.on_yellow()),
                2 => write!(f, "{}", s.on_red()),
                3 => write!(f, "{}", s.on_purple()),
                4 => write!(f, "{}", s.on_blue()),
                5 => write!(f, "{}", s.on_cyan()),
                _ => unreachable!(),
            }
        } else {
            write!(f, "{}", s)
        }
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        Some(Tile(c as u8 - '0' as u8, None, false))
    }
}

const OFFSETS: [[i64; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

fn find_minima(map: &Map<[i64; 2], Tile>) -> Vec<([i64; 2], Tile)> {
    let mut out = Vec::new();

    for ([i, j], tile) in map.data.iter() {
        let is_minimum = OFFSETS.iter().all(|[iofs, jofs]| {
            if let Some(t) = map.data.get(&[i + iofs, j + jofs]) {
                t.0 > tile.0
            } else {
                true
            }
        });

        if is_minimum {
            out.push(([*i, *j], *tile));
        }
    }

    out
}

fn fill_basin(
    map: &Map<[i64; 2], Tile>,
    basin: &[i64; 2],
    seen: &mut HashSet<[i64; 2]>,
) -> HashSet<[i64; 2]> {
    let mut frontier = vec![basin.clone()];
    let mut out: HashSet<[i64; 2]> = HashSet::new();
    out.insert(basin.clone());

    while let Some([i, j]) = frontier.pop() {
        let t = map.get(&[i, j]).unwrap();

        for [iofs, jofs] in OFFSETS.iter() {
            let p = [i + *iofs, j + *jofs];

            if let Some(n) = map.get(&p) {
                if n.0 >= t.0 && n.0 < 9 {
                    if !seen.insert(p.clone()) {
                        // skip if already seen
                        continue;
                    }
                    frontier.push(p.clone());
                    out.insert(p.clone());
                }
            } else {
                continue;
            }
        }
    }

    out
}

fn find_basins(map: &Map<[i64; 2], Tile>, minima: &[([i64; 2], Tile)]) -> Vec<HashSet<[i64; 2]>> {
    let mut seen: HashSet<[i64; 2]> = HashSet::new();
    let mut basins: Vec<HashSet<[i64; 2]>> = minima
        .iter()
        .map(|(basin, _)| fill_basin(&map, basin, &mut seen))
        .collect();

    basins.sort_by_key(|b| std::cmp::Reverse(b.len()));

    basins
}

fn main() -> Result<()> {
    let mut map: Map<[i64; 2], Tile> = Map::read(&mut File::open("data/day09/input")?)?;

    let minima = find_minima(&map);

    let risk_sum = minima.iter().map(|(_, t)| t.0 as i64 + 1).sum::<i64>();

    let basins = find_basins(&map, &minima[..]);

    // mark minima
    for (p, _) in minima.iter() {
        if let Some(t) = map.get_mut(p) {
            t.2 = true;
        }
    }

    // mark basins
    for (i, basin) in basins.iter().enumerate() {
        for p in basin.iter() {
            if let Some(t) = map.get_mut(p) {
                t.1 = Some(i)
            }
        }
    }
    println!("{}", map);

    let largest_basins = basins.iter().take(3).map(|b| b.len()).product::<usize>();

    println!("Part 1: {}", risk_sum);
    println!("Part 2: {} [{} basins total]", largest_basins, basins.len());

    Ok(())
}
