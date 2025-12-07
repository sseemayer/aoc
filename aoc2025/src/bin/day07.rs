use std::{collections::HashMap, fmt::Display};

use anyhow::Result;
use aoc::map::ParseMapTile;
use colored::Colorize;

type Map = aoc::map::Map<[isize; 2], Tile>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Start,
    Empty,
    Splitter,
    Beam,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Start => write!(f, "{}", "S".bold().yellow()),
            Tile::Empty => write!(f, "."),
            Tile::Splitter => write!(f, "{}", "^".bold().blue()),
            Tile::Beam => write!(f, "{}", "│".yellow(),),
        }
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'S' => Some(Tile::Start),
            '.' => Some(Tile::Empty),
            '^' => Some(Tile::Splitter),
            _ => None,
        }
    }
}

fn simulate(map: &mut Map) -> Result<(usize, usize)> {
    let mut beams: HashMap<[isize; 2], usize> = HashMap::new();

    for [i, j] in map.find_all(&Tile::Start) {
        beams.insert([i + 1, j], 1);
    }

    let mut n_splits = 0;
    let mut n_timelines = 0;

    while !beams.is_empty() {
        let mut new_beams = HashMap::new();

        n_timelines = beams.values().sum::<usize>();

        for ([i, j], count) in beams {
            let Some(tile) = map.get_mut(&[i, j]) else {
                continue;
            };

            // println!(
            //     "{:?} at {} with count {}",
            //     tile,
            //     format!("[{}, {}]", i, j).bold().green(),
            //     count
            // );

            match tile {
                Tile::Empty | Tile::Beam => {
                    *tile = Tile::Beam;

                    new_beams
                        .entry([i + 1, j])
                        .and_modify(|e| *e += count)
                        .or_insert(count);
                }
                Tile::Splitter => {
                    n_splits += 1;

                    new_beams
                        .entry([i + 1, j - 1])
                        .and_modify(|e| *e += count)
                        .or_insert(count);

                    new_beams
                        .entry([i + 1, j + 1])
                        .and_modify(|e| *e += count)
                        .or_insert(count);
                }
                _ => {}
            }
        }

        beams = new_beams;

        // println!("{}", map);
        // println!(
        //     "splitted: {}, paths: {}, sum: {}",
        //     n_splits,
        //     n_timelines,
        //     beams.values().sum::<usize>()
        // );
        // println!("{:?}", beams);
        // println!("\n\n");
    }

    Ok((n_splits, n_timelines))
}

fn main() -> Result<()> {
    //let input = aoc::io::read_all("data/day07/example")?;
    let input = aoc::io::read_all((2025, 7))?;

    let mut map: Map = input.parse()?;

    let (n_splits, n_timelines) = simulate(&mut map)?;

    println!("{}", map);

    println!("{} {}", "Part 1:".bold().green(), n_splits);
    println!("{} {}", "Part 2:".bold().green(), n_timelines);
    Ok(())
}
