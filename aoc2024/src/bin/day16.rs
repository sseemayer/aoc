use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use anyhow::{anyhow, bail, Context, Error, Result};
use aoc::direction::Direction;
use colored::Colorize;
use strum::IntoEnumIterator;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum Tile {
    Floor(Option<Direction>),
    #[default]
    Wall,
    Start,
    End,
}

impl Tile {
    fn is_blocking(&self) -> bool {
        match self {
            Tile::Wall => true,
            _ => false,
        }
    }
}

impl aoc::map::ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Floor(None)),
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
                Tile::Floor(None) => " ".on_black(),
                Tile::Floor(Some(Direction::North)) => "^".yellow().on_black(),
                Tile::Floor(Some(Direction::East)) => ">".yellow().on_black(),
                Tile::Floor(Some(Direction::South)) => "v".yellow().on_black(),
                Tile::Floor(Some(Direction::West)) => "<".yellow().on_black(),
                Tile::Wall => "█".white(),
                Tile::Start => "S".green().on_black(),
                Tile::End => "E".red().on_black(),
            }
        )
    }
}

type Map = aoc::map::Map<[i32; 2], Tile>;

type NavState = ([i32; 2], Direction);

struct World {
    map: Map,
    start_pos: [i32; 2],
    end_pos: [i32; 2],

    connections: HashMap<NavState, HashMap<NavState, usize>>,
}

impl World {
    fn show_solution(&self, path: &Vec<NavState>) {
        let mut map = self.map.clone();
        for &(pos, dir) in path {
            map.set(pos, Tile::Floor(Some(dir)))
        }

        println!("{}", map);
    }

    fn solve(&self) -> Result<()> {
        let mut queue: BinaryHeap<(Reverse<usize>, Vec<NavState>)> = BinaryHeap::new();

        queue.push((Reverse(0), vec![(self.start_pos, Direction::East)]));
        queue.push((Reverse(1000), vec![(self.start_pos, Direction::North)]));
        queue.push((Reverse(1000), vec![(self.start_pos, Direction::South)]));

        let mut solution_positions: HashSet<[i32; 2]> = HashSet::new();
        let mut earliest: HashMap<NavState, usize> = HashMap::new();
        let mut best_steps = None;

        while let Some((score, path)) = queue.pop() {
            let &(pos, direction) = path.last().expect("non-empty path");

            // println!("{} {:?} {:?} {}", score.0, pos, direction, queue.len());

            // don't explore search paths further away than an already-found solution
            if let Some(bs) = best_steps {
                if score.0 > bs {
                    break;
                }
            }

            // don't explore search paths that arrive at a state that was encountered earlier
            if let Some(&bs) = earliest.get(&(pos, direction)) {
                if score.0 > bs {
                    continue;
                }
            } else {
                earliest.insert((pos, direction), score.0);
            }

            if pos == self.end_pos {
                if best_steps.is_none() {
                    println!("Part 1: {}", score.0);
                    best_steps = Some(score.0);
                }

                let mut expanded_path = Vec::new();
                for s in 1..path.len() {
                    let ([i0, j0], direction) = path[s - 1];
                    let ([i1, j1], _) = path[s];

                    let imin = i32::min(i0, i1);
                    let imax = i32::max(i0, i1);
                    let jmin = i32::min(j0, j1);
                    let jmax = i32::max(j0, j1);

                    for i in imin..=imax {
                        for j in jmin..=jmax {
                            expanded_path.push(([i, j], direction));
                        }
                    }
                }

                // println!("Solution with score {}", score.0);
                // self.show_solution(&expanded_path);

                for (p, _) in expanded_path {
                    solution_positions.insert(p);
                }

                continue;
            }

            let Some(connections) = self.connections.get(&(pos, direction)) else {
                continue;
            };

            for (&new_state, &steps) in connections {
                if path.contains(&new_state) {
                    continue;
                }

                let mut new_path = path.clone();

                new_path.push(new_state);

                queue.push((Reverse(score.0 + steps), new_path))
            }
        }

        println!("Part 2: {}", solution_positions.len());

        Ok(())
    }
}

impl std::str::FromStr for World {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let map: Map = s.parse()?;

        let start_pos = map
            .find_one_where(|_, t| t == &Tile::Start)
            .ok_or(anyhow!("Found no start"))?;
        let end_pos = map
            .find_one_where(|_, t| t == &Tile::End)
            .ok_or(anyhow!("Found no end"))?;

        // nodes are all points that are not on a straight line (i.e. corners and crossings)
        let mut nodes: HashSet<[i32; 2]> = map
            .data
            .iter()
            .filter_map(|(&pos, tile)| {
                if tile.is_blocking() {
                    return None;
                }

                let [i, j] = pos;

                let is_blocking = |dir: Direction| {
                    let [di, dj] = dir.dpos();
                    map.get(&[i + di, j + dj])
                        .map(|t| t.is_blocking())
                        .unwrap_or(true)
                };

                if Direction::iter().any(|dir| {
                    !is_blocking(dir)
                        && (!is_blocking(dir.rot_left()) || !is_blocking(dir.rot_right()))
                }) {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect();

        nodes.insert(start_pos);
        nodes.insert(end_pos);

        // build up connections between nodes by raycasting from start nodes
        let mut connections: HashMap<NavState, HashMap<NavState, usize>> = HashMap::new();
        for &[i, j] in &nodes {
            for direction in Direction::iter() {
                let [di, dj] = direction.dpos();

                let mut step = 1;
                loop {
                    let pos = [i + step as i32 * di, j + step as i32 * dj];
                    if map.get(&pos).map(|t| t.is_blocking()).unwrap_or(true) {
                        break;
                    }

                    if nodes.contains(&pos) {
                        let outbounds = connections.entry(([i, j], direction)).or_default();

                        if pos == end_pos {
                            outbounds.insert((pos, direction), step);
                            break;
                        }

                        let [di, dj] = direction.dpos();
                        if !map
                            .get(&[pos[0] + di, pos[1] + dj])
                            .map(|t| t.is_blocking())
                            .unwrap_or(true)
                        {
                            outbounds.insert((pos, direction), step);
                        }

                        let [di, dj] = direction.rot_left().dpos();
                        if !map
                            .get(&[pos[0] + di, pos[1] + dj])
                            .map(|t| t.is_blocking())
                            .unwrap_or(true)
                        {
                            outbounds.insert((pos, direction.rot_left()), step + 1000);
                        }

                        let [di, dj] = direction.rot_right().dpos();
                        if !map
                            .get(&[pos[0] + di, pos[1] + dj])
                            .map(|t| t.is_blocking())
                            .unwrap_or(true)
                        {
                            outbounds.insert((pos, direction.rot_right()), step + 1000);
                        }

                        break;
                    }

                    step += 1;
                }
            }
        }

        //let mut conns: Vec<_> = connections.iter().collect();
        //conns.sort_by_key(|(k, _v)| *k);

        //for ((pos, dir), conns) in &conns {
        //    println!("{:?} {:?}: {:?}", pos, dir, conns);
        //}

        Ok(Self {
            map,
            start_pos,
            end_pos,
            connections,
        })
    }
}

fn main() -> Result<()> {
    let world: World = aoc::io::read_all((2024, 16))?.parse()?;
    //let world: World = aoc::io::read_all("data/day16/example2")?.parse()?;

    println!("{}", &world.map);

    world.solve()?;

    Ok(())
}
