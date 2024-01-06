use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{anyhow, Result};
use aoc::{direction::Direction, map::ParseMapTile};
use colored::Colorize;
use strum::IntoEnumIterator;

type Map = aoc::map::Map<[i32; 2], Tile>;

#[derive(Debug, Clone)]
enum Tile {
    Floor,
    Wall,
    Slope(Direction),
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Floor => "░".normal(),
                Tile::Wall => "█".normal(),
                Tile::Slope(d) => match d {
                    Direction::North => "🮧",
                    Direction::East => "🮥",
                    Direction::South => "🮦",
                    Direction::West => "🮤",
                }
                .green(),
            }
        )
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Floor),
            '#' => Some(Tile::Wall),
            '^' => Some(Tile::Slope(Direction::North)),
            '>' => Some(Tile::Slope(Direction::East)),
            'v' => Some(Tile::Slope(Direction::South)),
            '<' => Some(Tile::Slope(Direction::West)),
            _ => None,
        }
    }
}

fn find_crossings(map: &Map, ignore_slopes: bool) -> Vec<([i32; 2], Vec<Direction>)> {
    let ([imin, jmin], [imax, jmax]) = map.get_extent();

    let mut out = Vec::new();

    // add a pseudo crossing at the start and finish
    out.push(([imin, jmin + 1], vec![Direction::South]));
    out.push(([imax, jmax - 1], vec![]));

    'outer: for (&[i, j], tile) in &map.data {
        if let Tile::Floor = tile {
            let mut outbounds = Vec::new();
            for direction in Direction::iter() {
                let [di, dj] = direction.dpos();
                match map.get(&[i + di, j + dj]) {
                    Some(Tile::Slope(d)) => {
                        if d == &direction || ignore_slopes {
                            outbounds.push(direction);
                        }
                    }
                    Some(Tile::Wall) => {}
                    _ => continue 'outer,
                }
            }

            out.push(([i, j], outbounds))
        }
    }

    out
}

fn pathfind(
    map: &Map,
    crossings: &Vec<([i32; 2], Vec<Direction>)>,
) -> HashMap<usize, HashMap<usize, usize>> {
    let mut out: HashMap<usize, HashMap<usize, usize>> = HashMap::new();

    let pos_to_crossing: HashMap<[i32; 2], usize> = crossings
        .iter()
        .enumerate()
        .map(|(n, (pos, _))| (*pos, n))
        .collect();

    for (a, ([ai, aj], directions)) in crossings.iter().enumerate() {
        let mut seen: HashSet<[i32; 2]> = HashSet::new();
        seen.insert([*ai, *aj]);

        let mut queue = VecDeque::new();
        for dir in directions {
            let [di, dj] = dir.dpos();
            queue.push_back((1, [ai + di, aj + dj]));
        }

        while let Some((steps, [i, j])) = queue.pop_front() {
            if let Some(b) = pos_to_crossing.get(&[i, j]) {
                out.entry(a).or_default().insert(*b, steps);
                continue;
            }

            for dir in Direction::iter() {
                let [di, dj] = dir.dpos();
                let newpos = [i + di, j + dj];

                if seen.contains(&newpos) {
                    continue;
                }

                match map.get(&newpos) {
                    Some(Tile::Floor | Tile::Slope(_)) => {}
                    _ => continue,
                }

                seen.insert(newpos);

                queue.push_back((steps + 1, newpos));
            }
        }
    }

    out
}

fn find_longest_path(paths: &HashMap<usize, HashMap<usize, usize>>) -> Result<usize> {
    let mut longest = 0;

    let mut stack = vec![(vec![0], 0)];

    while let Some((path, steps)) = stack.pop() {
        let current = path
            .last()
            .ok_or_else(|| anyhow!("Encountered empty path"))?;

        if *current == 1 {
            longest = usize::max(longest, steps);
            continue;
        }

        let options = paths
            .get(current)
            .ok_or_else(|| anyhow!("No steps to go from {}", current))?;

        for (next, extra_steps) in options {
            if path.contains(next) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(*next);

            stack.push((new_path, steps + extra_steps));
        }
    }

    Ok(longest)
}

fn main() -> Result<()> {
    let map: Map = aoc::io::read_all("data/day23/input")?.parse()?;

    {
        let crossings = find_crossings(&map, false);
        let pathmap = pathfind(&map, &crossings);
        println!("Part 1: {}", find_longest_path(&pathmap)?);
    }

    {
        let crossings = find_crossings(&map, true);
        let pathmap = pathfind(&map, &crossings);
        println!("Part 2: {}", find_longest_path(&pathmap)?);
    }
    Ok(())
}
