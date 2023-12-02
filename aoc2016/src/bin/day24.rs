use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
};

use anyhow::Result;
use aoc::map::{Map, ParseMapTile};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Floor,
    Wall,
    Waypoint { id: u8 },
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Floor),
            '#' => Some(Tile::Wall),
            '0'..='9' => {
                let id = (c as u8) - ('0' as u8);
                Some(Tile::Waypoint { id })
            }
            _ => None,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Floor => write!(f, "."),
            Tile::Wall => write!(f, "#"),
            Tile::Waypoint { id } => write!(f, "{}", id),
        }
    }
}

fn shortest_paths(from: &[i16; 2], map: &Map<[i16; 2], Tile>) -> HashMap<[i16; 2], usize> {
    let mut dist: HashMap<[i16; 2], usize> = HashMap::new();
    let mut queue: VecDeque<([i16; 2], usize)> = VecDeque::new();
    queue.push_back((*from, 0));

    while let Some((current, d)) = queue.pop_front() {
        if !dist.contains_key(&current) {
            dist.insert(current, d);

            for (iofs, jofs) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let n = [current[0] + iofs, current[1] + jofs];
                if let Some(t) = map.get(&n) {
                    if t != &Tile::Wall {
                        queue.push_back((n, d + 1));
                    }
                }
            }
        }
    }

    dist
}

fn shortest_tour(
    from: u8,
    return_to: Option<u8>,
    goals: &HashSet<u8>,
    wpsp: &HashMap<(u8, u8), usize>,
) -> Option<usize> {
    let mut queue: BinaryHeap<(Reverse<usize>, Vec<u8>)> = BinaryHeap::new();
    queue.push((Reverse(0), vec![from]));

    while let Some((dist, path)) = queue.pop() {
        let dist = dist.0;

        if let Some(rt) = return_to {
            if path.len() == goals.len() + 1 && path.last().unwrap() == &rt {
                println!("Solution: {:?}", path);
                return Some(dist);
            }
        } else {
            if path.len() == goals.len() {
                println!("Solution: {:?}", path);
                return Some(dist);
            }
        }

        for next_wp in goals {
            let is_return = return_to
                .map(|rt| path.len() == goals.len() && next_wp == &rt)
                .unwrap_or(false);

            if !path.contains(next_wp) || is_return {
                let last = path.last().unwrap();
                let d = wpsp[&(*last, *next_wp)];
                let mut new_path = path.clone();
                new_path.push(*next_wp);

                queue.push((Reverse(dist + d), new_path));
            }
        }
    }

    None
}

fn main() -> Result<()> {
    let map: Map<[i16; 2], Tile> = Map::read(&mut File::open("data/day24/input")?)?;

    // let map: Map<[i16; 2], Tile> =
    //     "###########\n#0.1.....2#\n#.#######.#\n#4.......3#\n###########"
    //         .parse()?;

    let start_pos = map
        .find_one(&Tile::Waypoint { id: 0 })
        .expect("start position");

    println!("{}\nstart @ {:?}", map, start_pos,);

    let waypoint_positions: HashMap<u8, [i16; 2]> = map
        .find_all_where(|_p, t| {
            if let Tile::Waypoint { .. } = t {
                true
            } else {
                false
            }
        })
        .into_iter()
        .filter_map(|pos| {
            if let Some(Tile::Waypoint { id }) = map.get(&pos) {
                Some((*id, pos))
            } else {
                None
            }
        })
        .collect();

    println!("Waypoints @ {:?}", waypoint_positions);

    let mut wpsp: HashMap<(u8, u8), usize> = HashMap::new();
    for (i, p) in waypoint_positions.iter() {
        let sp = shortest_paths(p, &map);
        for (j, q) in waypoint_positions.iter() {
            wpsp.insert((*i, *j), sp[q]);
        }
    }

    let waypoints: HashSet<u8> = waypoint_positions.keys().cloned().collect();

    if let Some(dist) = shortest_tour(0, None, &waypoints, &wpsp) {
        println!("Part 1: {} steps", dist);
    }

    if let Some(dist) = shortest_tour(0, Some(0), &waypoints, &wpsp) {
        println!("Part 2: {} steps", dist);
    }

    // TODO solve shortest path using BFS and binary heap queue

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
