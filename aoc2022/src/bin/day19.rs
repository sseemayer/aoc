use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    ops::{AddAssign, SubAssign},
};

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_BLUEPRINT: Regex = Regex::new(r"Blueprint (\d+): (.*)").unwrap();
    static ref RE_ROBOT: Regex = Regex::new(r"Each (\w+) robot costs (.*)").unwrap();
    static ref RESOURCE_KEYS: HashMap<&'static str, char> = vec![
        ("ore", 'o'),
        ("clay", 'c'),
        ("obsidian", 'n'),
        ("geode", 'g')
    ]
    .into_iter()
    .collect();
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    plans: Vec<Robot>,
}

impl std::str::FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE_BLUEPRINT
            .captures(s)
            .ok_or_else(|| anyhow!("Expected blueprint: '{}'", s))?;

        let id: usize = captures.get(1).unwrap().as_str().parse()?;
        let plans: Vec<Robot> = captures
            .get(2)
            .unwrap()
            .as_str()
            .split(".")
            .filter(|r| !r.trim().is_empty())
            .map(|r| r.trim().parse())
            .collect::<Result<Vec<Robot>>>()?;

        Ok(Self { id, plans })
    }
}

impl std::fmt::Display for Blueprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Blueprint #{}:\n", self.id)?;
        for plan in &self.plans {
            write!(f, "\t{}\n", plan)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    time: u8,
    robots: Resources,
    resources: Resources,
}

impl State {
    fn hash_key(&self) -> String {
        format!("{:?} {:?}", self.robots, self.resources)
    }

    fn most_geodes_possible(&self, plans: &[Robot], time_left: u8) -> u16 {
        let mut resources = self.resources.clone();
        let mut robots = self.robots.clone();

        for _ in 0..time_left {
            let mut next_resources = resources.clone();
            next_resources += &robots;

            for plan in plans {
                if resources.can_afford(&plan.costs) {
                    *robots.0.entry(plan.mines).or_default() += 1;
                }
            }

            resources = next_resources;
        }

        resources.0.get(&'g').copied().unwrap_or_default()
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "t={} bots={:?} res={:?}\n",
            self.time, self.robots, self.resources
        )
    }
}

impl Blueprint {
    fn max_geodes(&self, max_time: u8) -> u16 {
        // it does not make sense to produce more of a resource than can be consumed in a step
        let mut max_resources = HashMap::new();
        for bp in &self.plans {
            for (k, v) in bp.costs.0.iter() {
                max_resources
                    .entry(*k)
                    .and_modify(|n| *n = u16::max(*n, *v))
                    .or_insert(*v);
            }
        }
        // max geode is fine though
        max_resources.insert('g', u16::MAX);

        let mut queue = VecDeque::new();
        queue.push_back(State {
            time: 0,
            robots: "1 ore".parse().unwrap(),
            resources: Resources::default(),
        });
        let mut max_geodes = 0;

        let mut seen = HashSet::new();

        while let Some(state) = queue.pop_front() {
            // println!("{:?} q={} s={}", state, queue.len(), seen.len());

            let geodes = state.resources.0.get(&'g').copied().unwrap_or_default();

            if geodes + 2 < max_geodes {
                // if we are 2 behind best state, don't explore this further
                continue;
            }

            if geodes > max_geodes {
                // println!(
                //     "{} after {}. q={}, s={}",
                //     geodes,
                //     state.time,
                //     queue.len(),
                //     seen.len()
                // );
                // println!("{}", state);
                max_geodes = geodes;
            }

            if state.time >= max_time {
                continue;
            }

            if geodes + state.most_geodes_possible(&self.plans, max_time - state.time) < max_geodes
            {
                continue;
            }

            // calculate what resources will be collected at the end of the current step
            let mut next_state = state.clone();
            next_state.time += 1;
            next_state.resources.add_assign(&state.robots);

            for bp in &self.plans {
                if state.resources.can_afford(&bp.costs) {
                    if next_state
                        .robots
                        .0
                        .get(&bp.mines)
                        .copied()
                        .unwrap_or_default()
                        + 1
                        > max_resources.get(&bp.mines).copied().unwrap_or(u16::MAX)
                    {
                        // do not choose productions that would make more than we can consume
                        continue;
                    }

                    let mut build_state = next_state.clone();
                    build_state.resources -= &bp.costs;
                    *build_state.robots.0.entry(bp.mines).or_default() += 1;

                    if seen.insert(build_state.hash_key()) {
                        queue.push_back(build_state);
                    }
                }
            }

            // also ok to wait
            if seen.insert(next_state.hash_key()) {
                queue.push_back(next_state);
            }
        }

        max_geodes
    }
}

#[derive(Debug)]
struct Robot {
    mines: char,
    costs: Resources,
}

impl std::str::FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE_ROBOT
            .captures(s.trim())
            .ok_or_else(|| anyhow!("Expected robot: '{}'", s))?;

        let mines = *RESOURCE_KEYS
            .get(captures.get(1).unwrap().as_str())
            .unwrap();
        let costs: Resources = captures.get(2).unwrap().as_str().parse()?;

        Ok(Self { mines, costs })
    }
}

impl std::fmt::Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.mines, self.costs)
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
struct Resources(HashMap<char, u16>);

impl std::fmt::Debug for Resources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut items: Vec<_> = self.0.iter().collect();
        items.sort();

        for (k, v) in items {
            write!(f, "{}{} ", v, k)?;
        }
        Ok(())
    }
}

impl Resources {
    fn can_afford(&self, other: &Resources) -> bool {
        for (k, v) in other.0.iter() {
            if let Some(n) = self.0.get(k) {
                if n < v {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

impl AddAssign<&Resources> for Resources {
    fn add_assign(&mut self, rhs: &Resources) {
        for (k, v) in rhs.0.iter() {
            *self.0.entry(*k).or_default() += *v;
        }
    }
}

impl SubAssign<&Resources> for Resources {
    fn sub_assign(&mut self, rhs: &Resources) {
        for (k, v) in rhs.0.iter() {
            *self.0.entry(*k).or_default() -= *v;
        }
    }
}

impl std::str::FromStr for Resources {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let resources = s
            .split(" and ")
            .map(|t| {
                let (n, r) = t
                    .trim()
                    .split_once(" ")
                    .ok_or_else(|| anyhow!("Bad resource delimiter '{}'", s))?;

                let n: u16 = n.parse()?;
                let r = *RESOURCE_KEYS.get(r).unwrap();

                Ok((r, n))
            })
            .collect::<Result<_>>()?;

        Ok(Self(resources))
    }
}

fn parse(path: &str) -> Result<Vec<Blueprint>> {
    BufReader::new(File::open(path)?)
        .lines()
        .map(|line| line?.trim().parse())
        .collect()
}

fn main() -> Result<()> {
    let blueprints = parse("data/day19/input")?;

    let mut quality_level_sum = 0;
    for bp in &blueprints {
        let ql = bp.max_geodes(24) as usize * bp.id;
        println!("{:?} quality_level={}", bp, ql);
        quality_level_sum += ql;
    }

    println!("Part 1: {}", quality_level_sum);

    let mut quality_level_prod = 1;
    for bp in blueprints.iter().take(3) {
        // println!("{}", bp);

        let ql = bp.max_geodes(32);
        println!("{:?} quality_level={}", bp, ql);
        quality_level_prod *= ql;
    }
    println!("Part 2: {}", quality_level_prod);

    Ok(())
}
