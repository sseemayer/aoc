use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

use anyhow::{bail, Result};
use aoc::{direction::Direction, map::ParseMapTile};
use colored::Colorize;

type Map = aoc::map::Map<[i32; 2], Tile>;

#[derive(Debug, Clone)]
struct Tile(usize, bool);

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = format!("{}", self.0);
        write!(f, "{}", if self.1 { c.green() } else { c.normal() })
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        let n: usize = c.to_digit(10)? as usize;
        Some(Self(n, false))
    }
}

#[derive(Debug, Clone)]
struct State {
    pos: [i32; 2],
    heat_loss: usize,
    direction: Direction,

    history: Vec<[i32; 2]>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.heat_loss.partial_cmp(&other.heat_loss) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.pos.partial_cmp(&other.pos)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("Always comparable")
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.heat_loss == other.heat_loss
    }
}

impl Eq for State {}

impl State {
    fn hash_key(&self) -> ([i32; 2], Direction) {
        (self.pos.clone(), self.direction)
    }
}

fn pathfind(map: &Map, min_steps: usize, max_steps: usize) -> Result<(usize, Vec<[i32; 2]>)> {
    let ([imin, jmin], [imax, jmax]) = map.get_extent();

    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    queue.push(Reverse(State {
        pos: [imin, jmin],
        heat_loss: 0,
        direction: Direction::East,
        history: vec![[0, 0]],
    }));

    queue.push(Reverse(State {
        pos: [imin, jmin],
        heat_loss: 0,
        direction: Direction::South,
        history: vec![[0, 0]],
    }));

    while let Some(Reverse(state)) = queue.pop() {
        if seen.contains(&state.hash_key()) {
            continue;
        } else {
            seen.insert(state.hash_key());
        }

        let [i, j] = state.pos;

        if i == imax && j == jmax {
            return Ok((state.heat_loss, state.history));
        }

        let directions = vec![state.direction.rot_left(), state.direction.rot_right()];

        for direction in directions {
            let [di, dj] = direction.dpos();

            let mut ni = i;
            let mut nj = j;
            let mut heat_loss = state.heat_loss;
            let mut history = state.history.clone();
            for i in 1..=max_steps {
                ni += di;
                nj += dj;

                if let Some(tile) = map.get(&[ni, nj]) {
                    heat_loss += tile.0;

                    history.push([ni, nj]);

                    if i >= min_steps {
                        queue.push(Reverse(State {
                            pos: [ni, nj],
                            heat_loss,
                            direction,
                            history: history.clone(),
                        }));
                    }
                }
            }
        }
    }

    bail!("Could not find a path")
}

fn paint_with_history(map: &mut Map, history: &[[i32; 2]]) {
    for pos in history {
        if let Some(t) = map.get_mut(pos) {
            t.1 = true;
        }
    }
}

fn main() -> Result<()> {
    let mut map: Map = aoc::io::read_all("data/day17/input")?.parse()?;

    let (heat_loss, _history) = pathfind(&map, 1, 3)?;

    println!("Part 1: {:?}", heat_loss);

    let (heat_loss, history) = pathfind(&map, 4, 10)?;

    paint_with_history(&mut map, &history[..]);

    println!("{}", map);

    println!("Part 2: {:?}", heat_loss);
    Ok(())
}
