use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
struct Valve {
    rate: usize,
    connections: HashSet<String>,
}

#[derive(Debug, Clone)]
struct World {
    valves: HashMap<String, Valve>,

    distance: HashMap<(String, String), usize>,
}

lazy_static! {
    static ref RE_VALVE: Regex =
        Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Za-z, ]+)")
            .unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    time: usize,
    pressure_released: usize,
    path: Vec<String>,
}

impl World {
    fn new(valves: HashMap<String, Valve>) -> Self {
        let mut distance = HashMap::new();

        for (a, v) in valves.iter() {
            for b in v.connections.iter() {
                distance.insert((a.to_string(), b.to_string()), 1);
            }

            distance.insert((a.to_string(), a.to_string()), 0);
        }

        for k in valves.keys() {
            for i in valves.keys() {
                if let Some(&dik) = distance.get(&(i.to_string(), k.to_string())) {
                    for j in valves.keys() {
                        if let Some(&dkj) = distance.get(&(k.to_string(), j.to_string())) {
                            let dij = distance
                                .entry((i.to_string(), j.to_string()))
                                .or_insert(usize::MAX);
                            *dij = usize::min(*dij, dik + dkj);
                        }
                    }
                }
            }
        }

        Self { valves, distance }
    }

    fn parse(path: &str) -> Result<Self> {
        let mut valves = HashMap::new();
        for line in BufReader::new(File::open(path)?).lines() {
            let line = line?;
            let captures = RE_VALVE
                .captures(&line)
                .ok_or_else(|| anyhow!("Expected valve definition: '{}'", line))?;

            let valve_id = captures.get(1).unwrap().as_str().to_string();
            let rate: usize = captures.get(2).unwrap().as_str().parse()?;
            let connections: HashSet<String> = captures
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|c| c.to_string())
                .collect();

            valves.insert(valve_id, Valve { rate, connections });
        }

        Ok(Self::new(valves))
    }

    fn filter(&self, select: &HashSet<&String>) -> Self {
        let valves = self
            .valves
            .iter()
            .filter(|(vid, _v)| select.contains(vid))
            .map(|(vid, v)| (vid.to_string(), v.clone()))
            .collect();

        let distance = self
            .distance
            .iter()
            .filter(|((a, b), _)| select.contains(a) && select.contains(b))
            .map(|((a, b), d)| ((a.to_string(), b.to_string()), *d))
            .collect();

        Self { valves, distance }
    }

    fn pathfind(&self, max_time: usize) -> Option<State> {
        let mut queue: BinaryHeap<State> = BinaryHeap::new();

        queue.push(State {
            time: 0,
            pressure_released: 0,
            path: vec!["AA".to_string()],
        });

        let mut solutions: Vec<State> = Vec::new();

        while let Some(state) = queue.pop() {
            if state.time >= max_time {
                solutions.push(state);
                continue;
            }

            let loc = state.path.last().expect("Non empty path");
            let open_valves: HashSet<&String> = state.path.iter().collect();

            // find next open destinations
            let mut found_next = false;
            for (vid, v) in self
                .valves
                .iter()
                .filter(|(vid, v)| !open_valves.contains(vid) && v.rate > 0)
            {
                let mut new_state = state.clone();

                if let Some(dist) = self.distance.get(&(loc.to_string(), vid.to_string())) {
                    new_state.path.push(vid.to_string());
                    new_state.time += dist + 1;

                    if new_state.time > max_time {
                        continue;
                    }

                    new_state.pressure_released += v.rate * (max_time - new_state.time);
                    queue.push(new_state);
                    found_next = true;
                } else {
                    panic!("Cannot find path between {} and {}!", loc, vid);
                }
            }

            if !found_next {
                // no more valid extensions to this step were found, so wait it out
                let mut wait_step = state.clone();
                wait_step.time = max_time;
                solutions.push(wait_step);
            }
        }

        solutions.into_iter().max_by_key(|s| s.pressure_released)
    }
}

fn main() -> Result<()> {
    let world = World::parse("data/day16/input")?;

    if let Some(s) = world.pathfind(30) {
        println!("Part 1: {:?}", s);
    }

    let ids = world
        .valves
        .iter()
        .filter(|(_vid, v)| v.rate > 0)
        .map(|(vid, _v)| vid)
        .collect::<HashSet<_>>();

    let mut best = 0;
    for n_items in 1..=ids.len() / 2 {
        println!("{} / {}", n_items, ids.len() / 2);
        for ids_a in ids.iter().combinations(n_items) {
            let mut ids_a = ids_a.into_iter().copied().collect::<HashSet<&String>>();
            let mut ids_b = &ids - &ids_a;

            let aa = "AA".to_string();
            ids_a.insert(&aa);
            ids_b.insert(&aa);

            let world_a = world.filter(&ids_a);
            let world_b = world.filter(&ids_b);

            if let (Some(pa), Some(pb)) = (world_a.pathfind(26), world_b.pathfind(26)) {
                let score = pa.pressure_released + pb.pressure_released;

                if score > best {
                    best = score;
                }
            }
        }
    }

    println!("Part 2: {}", best);

    Ok(())
}
