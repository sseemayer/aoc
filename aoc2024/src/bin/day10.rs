use std::collections::{HashSet, VecDeque};

use anyhow::Result;
use aoc::{direction::Direction, map::ParseMapTile};
use colored::Colorize;
use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
struct Tile {
    elevation: u8,
    state: State,
}

#[derive(Debug, Clone)]
enum State {
    None,
    Visited,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bg: u8 = (255.0 * (self.elevation as f32 / 9.0)) as u8;

        let tile = format!("{}", self.elevation).on_truecolor(bg, bg, bg);

        let tile = match self.state {
            State::None => {
                let fg: u8 = if bg > 128 { 0 } else { 255 };
                tile.truecolor(fg, fg, fg)
            }

            State::Visited => tile.green(),
        };

        write!(f, "{}", tile)
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        let elevation = c.to_digit(10)? as u8;
        Some(Self {
            elevation,
            state: State::None,
        })
    }
}

type Map = aoc::map::Map<[i32; 2], Tile>;

fn find_trailheads(map: &Map) -> Vec<[i32; 2]> {
    map.find_all_where(|_pos, tile| tile.elevation == 0)
}

fn find_paths(map: &Map) -> Result<()> {
    let mut map = map.clone();
    let trailheads = find_trailheads(&map);

    let mut sum_peaks = 0;
    let mut sum_paths = 0;
    for [i0, j0] in trailheads {
        let mut seen_peaks: HashSet<[i32; 2]> = HashSet::new();
        let mut queue: VecDeque<[i32; 2]> = VecDeque::new();

        let mut paths = 0;

        queue.push_back([i0, j0]);

        while let Some([i, j]) = queue.pop_front() {
            let Some(&Tile { elevation, .. }) = map.get(&[i, j]) else {
                continue;
            };

            if elevation == 9 {
                seen_peaks.insert([i, j]);

                paths += 1;

                continue;
            }

            for direction in Direction::iter() {
                let [di, dj] = direction.dpos();
                let newpos = [i + di, j + dj];

                let Some(Tile {
                    elevation: new_elevation,
                    state: new_state,
                }) = map.get_mut(&newpos)
                else {
                    continue;
                };

                if *new_elevation != elevation + 1 {
                    continue;
                }

                *new_state = State::Visited;

                queue.push_back(newpos);
            }
        }

        sum_peaks += seen_peaks.len();
        sum_paths += paths;
    }

    println!("{}", map);
    println!("Part 1: {}", sum_peaks);
    println!("Part 2: {}", sum_paths);

    Ok(())
}

fn main() -> Result<()> {
    let map: Map = aoc::io::read_all((2024, 10))?.parse()?;

    find_paths(&map)?;
    Ok(())
}
