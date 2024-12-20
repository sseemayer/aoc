use std::collections::{HashMap, VecDeque};

use anyhow::{anyhow, Result};
use aoc::{direction::Direction, map::ParseMapTile};
use colored::Colorize;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Floor,
    Wall,
    Start,
    End,
}

impl Tile {
    fn can_pass(&self) -> bool {
        match self {
            Tile::Floor => true,
            Tile::Wall => false,
            Tile::Start => true,
            Tile::End => true,
        }
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Floor),
            '#' => Some(Tile::Wall),
            'S' => Some(Tile::Start),
            'E' => Some(Tile::End),
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
                Tile::Floor => " ".black(),
                Tile::Wall => "█".white(),
                Tile::Start => "S".green().on_black(),
                Tile::End => "E".red().on_black(),
            }
        )
    }
}

type Map = aoc::map::Map<[i32; 2], Tile>;

#[derive(Debug, Clone)]
struct World {
    distance_to_end: HashMap<[i32; 2], usize>,
}

impl World {
    fn new(map: &Map) -> Result<Self> {
        let end_pos = map
            .find_one(&Tile::End)
            .ok_or(anyhow!("Could not find end!"))?;

        let mut distance_to_end: HashMap<[i32; 2], usize> = HashMap::new();
        distance_to_end.insert(end_pos, 0);

        let mut queue: VecDeque<(usize, [i32; 2])> = VecDeque::new();
        queue.push_back((0, end_pos));
        while let Some((distance, [i, j])) = queue.pop_front() {
            for direction in Direction::iter() {
                let [di, dj] = direction.dpos();

                let new_pos = [i + di, j + dj];

                let Some(tile) = map.get(&new_pos) else {
                    continue;
                };

                if !tile.can_pass() {
                    continue;
                }

                if distance_to_end.contains_key(&new_pos) {
                    continue;
                }

                distance_to_end.insert(new_pos, distance + 1);
                queue.push_back((distance + 1, new_pos));
            }
        }

        Ok(Self { distance_to_end })
    }

    fn find_skips(&self, max_duration: usize, min_savings: usize) -> usize {
        let mut out = 0;
        let md = max_duration as i32;

        for di in -md..=md {
            for dj in -md..=md {
                let skip_steps = (di.abs() + dj.abs()) as usize;
                if skip_steps > max_duration {
                    continue;
                }

                for (&[i, j], &dist) in &self.distance_to_end {
                    let new_pos = [i + di, j + dj];

                    let Some(&skip_dist) = self.distance_to_end.get(&new_pos) else {
                        continue;
                    };

                    let Some(savings) = usize::checked_sub(dist, skip_dist + skip_steps) else {
                        continue;
                    };

                    if savings >= min_savings {
                        //println!(
                        //    "found {:?} -> {:?} skip={} savings={}",
                        //    [i, j],
                        //    new_pos,
                        //    skip_steps,
                        //    savings
                        //);

                        out += 1;
                    }
                }
            }
        }

        out
    }
}

fn main() -> Result<()> {
    let map: Map = aoc::io::read_all((2024, 20))?.parse()?;
    //let map: Map = aoc::io::read_all("data/day20/example")?.parse()?;

    let world = World::new(&map)?;

    // let mut dte: Vec<([i32; 2], usize)> = world
    //     .distance_to_end
    //     .iter()
    //     .map(|(k, v)| (*k, *v))
    //     .collect();
    // dte.sort();

    // for (k, v) in dte {
    //     println!("{:?} {}", k, v);
    // }

    println!("Part 1: {}", world.find_skips(2, 100));
    println!("Part 2: {}", world.find_skips(20, 100));

    Ok(())
}
