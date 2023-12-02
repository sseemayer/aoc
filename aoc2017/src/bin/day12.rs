use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Context, Result};

#[derive(Debug)]
struct Connection {
    left: usize,
    right: Vec<usize>,
}

impl std::str::FromStr for Connection {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (left, right) = s.split_once(" <-> ").ok_or(anyhow!("Bad line: '{}'", s))?;

        let left: usize = left.parse().context("Parse LHS")?;
        let right: Vec<usize> = right
            .split(", ")
            .map(|v| Ok(v.parse().context("Parse RHS")?))
            .collect::<Result<Vec<usize>>>()?;

        Ok(Connection { left, right })
    }
}

#[derive(Debug)]
struct Graph {
    neighbors: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    fn from_connections(conns: &[Connection]) -> Self {
        let mut neighbors: HashMap<usize, HashSet<usize>> = HashMap::new();

        for conn in conns {
            neighbors
                .entry(conn.left)
                .or_default()
                .extend(conn.right.iter().cloned());

            for r in &conn.right {
                neighbors.entry(*r).or_default().insert(conn.left);
            }
        }

        Graph { neighbors }
    }

    fn find_group(&self, master: usize, seen: &mut HashSet<usize>) -> HashSet<usize> {
        let mut group = HashSet::new();
        let mut search = vec![master];
        while let Some(t) = search.pop() {
            seen.insert(t);
            group.insert(t);

            for n in &self.neighbors[&t] {
                if !seen.contains(&n) {
                    search.push(*n);
                }
            }
        }

        group
    }

    fn find_groups(&self) -> Vec<HashSet<usize>> {
        let mut todo: Vec<usize> = self.neighbors.keys().cloned().collect();

        let mut seen: HashSet<usize> = HashSet::new();
        let mut out: Vec<HashSet<usize>> = Vec::new();

        while let Some(master) = todo.pop() {
            if seen.contains(&master) {
                continue;
            }

            out.push(self.find_group(master, &mut seen));
        }

        out
    }
}

fn main() -> Result<()> {
    let connections: Vec<Connection> = aoc::io::read_lines("data/day12/input")?;

    let graph = Graph::from_connections(&connections[..]);

    let group0 = graph.find_group(0, &mut HashSet::new());
    let groups = graph.find_groups();

    println!("{:?}", groups);
    println!("Part 1: {}", group0.len());
    println!("Part 2: {}", groups.len());

    Ok(())
}
