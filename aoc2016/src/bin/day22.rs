use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

use aoc::map::Map;

lazy_static! {
    static ref RE_NODE: Regex =
        Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%$").unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    x: i8,
    y: i8,
    size: u16,
    used: u16,
    avail: u16,
    use_pct: u16,
    contains_goal: bool,
}

impl std::str::FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let captures = RE_NODE
            .captures(s.trim())
            .ok_or(anyhow!("Bad node: '{}'", s))?;

        let x = captures.get(1).unwrap().as_str();
        let y = captures.get(2).unwrap().as_str();
        let size = captures.get(3).unwrap().as_str();
        let used = captures.get(4).unwrap().as_str();
        let avail = captures.get(5).unwrap().as_str();
        let use_pct = captures.get(6).unwrap().as_str();

        let x: i8 = x.parse().context("Parse x")?;
        let y: i8 = y.parse().context("Parse y")?;
        let size: u16 = size.parse().context("Parse size")?;
        let used: u16 = used.parse().context("Parse used")?;
        let avail: u16 = avail.parse().context("Parse avail")?;
        let use_pct: u16 = use_pct.parse().context("Parse use_pct")?;

        Ok(Node {
            x,
            y,
            size,
            used,
            avail,
            use_pct,
            contains_goal: false,
        })
    }
}

impl Node {
    fn can_send_to(&self, target: &Node) -> bool {
        self.used > 0 && target.avail >= self.used
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = if self.used > 100 {
            "#"
        } else if self.used < self.size / 2 {
            "."
        } else {
            "/"
        };

        if self.contains_goal {
            write!(f, "!{}", symbol)
        } else {
            write!(f, " {}", symbol)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    steps: usize,
    map: Map<[i8; 2], Node>,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_offset(&self) -> [i8; 2] {
        match self {
            Direction::Up => [-1, 0],
            Direction::Down => [1, 0],
            Direction::Left => [0, -1],
            Direction::Right => [0, 1],
        }
    }
}

impl State {
    fn make_move(&self, pos_source: &[i8; 2], direction: &Direction) -> Option<Self> {
        let offset = direction.get_offset();
        let pos_target = [pos_source[0] + offset[0], pos_source[1] + offset[1]];

        let old_source = self.map.get(pos_source)?;
        let old_target = self.map.get(&pos_target)?;

        let sum = old_source.used + old_target.used;
        if sum > old_target.size {
            println!("Not enough space: {}", sum);
            return None;
        }

        let mut new_source = old_source.clone();
        let mut new_target = old_target.clone();

        new_target.used = sum;
        new_source.used = 0;

        new_target.contains_goal = new_source.contains_goal;
        new_source.contains_goal = false;

        let mut new_state = self.clone();
        new_state.steps += 1;
        new_state.map.set(*pos_source, new_source);
        new_state.map.set(pos_target, new_target);

        Some(new_state)
    }
}

fn main() -> Result<()> {
    let nodes: Vec<Node> = std::fs::read_to_string("data/day22/input")?
        .lines()
        .filter_map(|l| l.parse().ok())
        .collect();

    let mut n_viable = 0;
    for (i, n) in nodes.iter().enumerate() {
        for (j, m) in nodes.iter().enumerate() {
            if i != j && n.can_send_to(m) {
                n_viable += 1;
            }
        }
    }

    println!("Part 1: got {} viable pairs", n_viable);

    // convert to useful representation
    let mut map: Map<[i8; 2], Node> = Map::new();
    for n in nodes {
        map.set([n.y, n.x], n);
    }

    let (_, max) = map.get_extent();
    let mut target_pos = [0, max[1]];
    map.get_mut(&target_pos).unwrap().contains_goal = true;

    let mut empty_pos = None;
    let mut empty_used = std::u16::MAX;
    for (pos, n) in map.data.iter() {
        if n.used < empty_used {
            empty_pos = Some(pos);
            empty_used = n.used;
        }
    }

    let mut empty_pos = *empty_pos.expect("Found empty");
    let mut state = State { map, steps: 0 };

    println!("step {}:\n{}\n{:?}", state.steps, state.map, empty_pos);
    for _ in 0..4 {
        state = state
            .make_move(&[empty_pos[0], empty_pos[1] - 1], &Direction::Right)
            .expect("valid move");
        empty_pos[1] -= 1;
        println!("step {}:\n{}\n{:?}", state.steps, state.map, empty_pos);
    }

    while empty_pos[0] > 0 {
        state = state
            .make_move(&[empty_pos[0] - 1, empty_pos[1]], &Direction::Down)
            .expect("valid move");
        empty_pos[0] -= 1;
        println!("step {}:\n{}\n{:?}", state.steps, state.map, empty_pos);
    }

    while empty_pos[1] < target_pos[1] - 1 {
        state = state
            .make_move(&[empty_pos[0], empty_pos[1] + 1], &Direction::Left)
            .expect("valid move");
        empty_pos[1] += 1;
        println!("step {}:\n{}\n{:?}", state.steps, state.map, empty_pos);
    }

    while target_pos[1] > 0 {
        state = state
            .make_move(&target_pos, &Direction::Left)
            .expect("valid move");
        target_pos[1] -= 1;
        empty_pos[1] += 1;

        println!("step {}:\n{}\n{:?}", state.steps, state.map, empty_pos);

        if target_pos[1] == 0 {
            break;
        }

        state = state
            .make_move(&[empty_pos[0] + 1, empty_pos[1]], &Direction::Up)
            .expect("valid move");
        empty_pos[0] += 1;

        println!("step {}:\n{}\n{:?}", state.steps, state.map, empty_pos);

        for _ in 0..2 {
            state = state
                .make_move(&[empty_pos[0], empty_pos[1] - 1], &Direction::Right)
                .expect("valid move");
            empty_pos[1] -= 1;
            println!("step {}:\n{}\n{:?}", state.steps, state.map, empty_pos);
        }

        state = state
            .make_move(&[empty_pos[0] - 1, empty_pos[1]], &Direction::Down)
            .expect("valid move");
        empty_pos[0] -= 1;
        println!("step {}:\n{}\n{:?}", state.steps, state.map, empty_pos);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        Ok(())
    }
}
