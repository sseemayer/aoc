use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use anyhow::Result;
use aoc::summed_area_table::SummedAreaTable;
use colored::Colorize;
use strum::{EnumIter, IntoEnumIterator};

type D = i16;
type Pos = [D; 2];

#[derive(Debug, Clone)]
enum TileType {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Debug, Clone)]
struct Tile {
    tile_type: TileType,
    on_path: bool,
}

impl std::fmt::Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileType::Rocky => write!(f, "."),
            TileType::Wet => write!(f, "="),
            TileType::Narrow => write!(f, "|"),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.on_path {
            write!(f, "{}", format!("{}", self.tile_type).green())
        } else {
            write!(f, "{}", self.tile_type)
        }
    }
}

impl Tile {
    fn from_erosion_level(el: usize) -> Self {
        let tile_type = match el % 3 {
            0 => TileType::Rocky,
            1 => TileType::Wet,
            2 => TileType::Narrow,
            _ => panic!("Should not happen"),
        };

        Self {
            tile_type,
            on_path: false,
        }
    }

    fn risk_level(&self) -> usize {
        match self.tile_type {
            TileType::Rocky => 0,
            TileType::Wet => 1,
            TileType::Narrow => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, Hash)]
enum Tool {
    ClimbingGear,
    Torch,
    Neither,
}

impl Tool {
    fn is_suitable_for(&self, tile: &Tile) -> bool {
        match (&tile.tile_type, self) {
            (TileType::Rocky, Tool::ClimbingGear | Tool::Torch) => true,
            (TileType::Wet, Tool::ClimbingGear | Tool::Neither) => true,
            (TileType::Narrow, Tool::Torch | Tool::Neither) => true,
            _ => false,
        }
    }
}

type Map = aoc::map::Map<Pos, Tile>;

struct State {
    map: Map,
    target: Pos,

    sat: SummedAreaTable<D, D>,
}

const DIRECTIONS: [Pos; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

impl State {
    fn new(depth: usize, target: Pos, overbuild: D) -> Self {
        let mut map = Map::new();
        let [ti, tj] = target;

        let imax = ti + overbuild;
        let jmax = tj + overbuild;

        let mut last_row = (0..=jmax)
            .map(|j| {
                let geo_idx = j as usize * 16807;
                let ero_lvl = (geo_idx + depth) % 20183;
                ero_lvl
            })
            .collect::<Vec<_>>();

        for j in 0..=jmax {
            map.set([0, j], Tile::from_erosion_level(last_row[j as usize]))
        }

        for i in 1..=imax {
            let first_geo_idx = i as usize * 48271;
            let mut last_ero_lvl = (first_geo_idx + depth) % 20183;
            let mut new_row = Vec::new();
            new_row.push(last_ero_lvl);
            map.set([i, 0], Tile::from_erosion_level(last_ero_lvl));

            for j in 1..=jmax {
                let geo_idx = last_ero_lvl * last_row[j as usize];
                let ero_lvl = (geo_idx + depth) % 20183;

                new_row.push(ero_lvl);

                map.set([i, j], Tile::from_erosion_level(new_row[j as usize]));

                last_ero_lvl = ero_lvl;
            }

            last_row = new_row;
        }

        // force geologic index of 0 for target after the fact
        map.set([ti, tj], Tile::from_erosion_level(depth % 20183));

        let sat = SummedAreaTable::new(([0 as D, 0 as D], [imax as D, jmax as D]), |[i, j]| {
            let tile = map.get(&[i, j]).expect("Tile at position");
            tile.risk_level() as D
        });

        Self { map, target, sat }
    }

    fn distance_to_target(&self, &[i, j]: &Pos) -> u16 {
        let [ti, tj] = self.target;

        (i16::abs_diff(ti, i) + i16::abs_diff(tj, j)) as u16
    }

    fn search_path(&self) -> Option<(usize, Vec<(usize, Pos, Tool)>)> {
        let mut queue: BinaryHeap<(Reverse<usize>, Reverse<u16>, Pos, Tool)> = BinaryHeap::new();
        let mut traceback: HashMap<(Pos, Tool), (usize, Pos, Tool)> = HashMap::new();

        let start = [0, 0];

        traceback.insert((start, Tool::Torch), (0, start, Tool::Torch));

        queue.push((
            Reverse(0),
            Reverse(self.distance_to_target(&start)),
            start,
            Tool::Torch,
        ));

        while let Some((Reverse(time), Reverse(dtt), pos, tool)) = queue.pop() {
            if let Some((t, _, _)) = traceback.get(&(pos, tool)) {
                if *t < time {
                    continue;
                }
            }

            // println!(
            //     "t={} d={} {}/{} {:?} qlen={}",
            //     time,
            //     dtt,
            //     pos[0],
            //     pos[1],
            //     tool,
            //     queue.len()
            // );

            if pos == self.target && tool == Tool::Torch {
                let mut current = (pos, tool);
                let mut path = Vec::new();
                while let Some(&(t, p, l)) = traceback.get(&current) {
                    path.push((t, p, l));
                    current = (p, l);

                    if p == [0, 0] && l == Tool::Torch {
                        break;
                    }
                }

                return Some((time, path));
            }

            let tile = self.map.get(&pos).expect("Valid tile");

            for [di, dj] in DIRECTIONS {
                let new_pos = [pos[0] + di, pos[1] + dj];
                if let Some(new_tile) = self.map.get(&new_pos) {
                    for new_tool in Tool::iter() {
                        if !new_tool.is_suitable_for(tile) || !new_tool.is_suitable_for(new_tile) {
                            continue;
                        }

                        // we could move from (pos, tool, time) to (new_pos, new_tool, new_time)
                        let new_time = time + if new_tool == tool { 1 } else { 1 + 7 };

                        let (prev_time, prev_tb_pos, prev_tb_tool) = traceback
                            .entry((new_pos, new_tool))
                            .or_insert((usize::MAX, pos, tool));

                        if new_time < *prev_time {
                            *prev_time = new_time;
                            *prev_tb_pos = pos;
                            *prev_tb_tool = tool;
                            queue.push((
                                Reverse(new_time),
                                Reverse(self.distance_to_target(&new_pos)),
                                new_pos,
                                new_tool,
                            ));
                        }
                    }
                }
            }

            // special case for target -- allow tool change if arrived (normally tool changes bound
            // to tile changes)
            if pos == self.target {
                let new_time = time + 7;
                traceback
                    .entry((pos, Tool::Torch))
                    .or_insert((new_time, pos, tool));

                queue.push((Reverse(new_time), Reverse(0), pos, Tool::Torch));
            }
        }

        None
    }
}

fn main() -> Result<()> {
    // let mut state = State::new(510, [10, 10], 3); // example input

    let mut state = State::new(8112, [743, 13], 50); // my input

    let risk_level = state
        .sat
        .get_range_sum(([0, 0], [state.target[0] as D, state.target[1] as D]));
    let (time, path) = state.search_path().expect("BFS solution");

    for (_t, p, _l) in &path {
        state.map.data.get_mut(p).expect("Path tile").on_path = true;
    }

    println!("{}", state.map);

    println!("Part 1: {}", risk_level);
    println!("Part 2: {}", time);

    // for (t, pos, tool) in path {
    //     println!("{}: {:?} [{:?}]", t, pos, tool);
    // }

    Ok(())
}
