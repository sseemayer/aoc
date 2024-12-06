use std::collections::HashSet;

use anyhow::{anyhow, Result};
use aoc::{direction::Direction, map::ParseMapTile};
use colored::Colorize;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Ground { ground_type: GroundType },
    Wall { created: bool },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum GroundType {
    Blank,
    Visited,
    SimVisited,
    Start,
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Ground {
                ground_type: GroundType::Blank,
            }),
            '#' => Some(Tile::Wall { created: false }),
            '^' => Some(Tile::Ground {
                ground_type: GroundType::Start,
            }),
            _ => None,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ground { ground_type } => write!(
                f,
                "{}",
                match ground_type {
                    GroundType::Blank => "▒".white(),
                    GroundType::Visited => "▒".green(),
                    GroundType::SimVisited => "▒".blue(),
                    GroundType::Start => "@".green(),
                }
            ),
            Tile::Wall { created } => {
                write!(f, "{}", if *created { "█".red() } else { "█".white() })
            }
        }
    }
}

type Map = aoc::map::Map<[i32; 2], Tile>;

fn parse(data: &str) -> Result<(Map, [i32; 2])> {
    let map: Map = data.parse()?;

    let player_pos = map
        .find_one(&Tile::Ground {
            ground_type: GroundType::Start,
        })
        .ok_or(anyhow!("Could not find player start position"))?;

    Ok((map, player_pos))
}

fn simulate(
    map: &mut Map,
    start_pos: [i32; 2],
    start_dir: Direction,
    set_type: GroundType,
) -> (Vec<([i32; 2], Direction)>, bool) {
    let [mut i, mut j] = start_pos;
    let mut direction = start_dir;
    let mut trace = Vec::new();

    loop {
        if trace.contains(&([i, j], direction)) {
            return (trace, true);
        }
        trace.push(([i, j], direction));

        if let Some(Tile::Ground { ground_type }) = map.get_mut(&[i, j]) {
            if *ground_type != GroundType::Start {
                *ground_type = set_type.clone();
            }
        }

        //println!("{}/{} {:?}", i, j, direction);

        let [di, dj] = direction.dpos();
        let inew = i + di;
        let jnew = j + dj;

        match map.get(&[inew, jnew]) {
            Some(Tile::Wall { .. }) => {
                direction = direction.rot_right();
            }
            Some(Tile::Ground { .. }) => {
                i = inew;
                j = jnew;
            }
            _ => break,
        }
    }

    return (trace, false);
}

fn main() -> Result<()> {
    let data = &aoc::io::read_all((2024, 06))?;
    //let data = &aoc::io::read_all("data/day06/example")?;

    let (mut map, start_pos) = parse(data)?;
    let (trace, _) = simulate(&mut map, start_pos, Direction::North, GroundType::Visited);

    let seen: HashSet<[i32; 2]> = trace.iter().map(|(pos, _)| *pos).collect();
    println!("Part 1: {}", seen.len());

    let mut wall_positions: HashSet<[i32; 2]> = HashSet::new();
    for ([ti, tj], tdir) in trace {
        let mut map_mod = map.clone();
        let [di, dj] = tdir.dpos();
        let wpos = [ti + di, tj + dj];

        if let Some(tile) = map_mod.get_mut(&wpos) {
            if let Tile::Ground { .. } = tile {
                *tile = Tile::Wall { created: true };
            } else {
                continue;
            }
        } else {
            continue;
        }

        let (_, loops) = simulate(
            &mut map_mod,
            start_pos,
            Direction::North,
            GroundType::SimVisited,
        );

        if loops {
            // println!("{}", map_mod);

            wall_positions.insert(wpos);
        }
    }

    println!("Part 2: {}", wall_positions.len());

    Ok(())
}
