use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Error, Result};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Graph {
    edges: HashSet<(usize, usize)>,
    index_to_id: HashMap<usize, String>,
}

impl std::str::FromStr for Graph {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut edges: HashSet<(usize, usize)> = Default::default();
        let mut id_to_index: HashMap<String, usize> = Default::default();
        let mut index_to_id: HashMap<usize, String> = Default::default();
        for line in s.lines() {
            let (a, b) = line.split_once("-").ok_or(anyhow!("expect delimiter"))?;

            let i = *id_to_index
                .entry(a.to_string())
                .or_insert(index_to_id.len());

            index_to_id.insert(i, a.to_string());

            let j = *id_to_index
                .entry(b.to_string())
                .or_insert(index_to_id.len());

            index_to_id.insert(j, b.to_string());

            if i < j {
                edges.insert((i, j));
            } else {
                edges.insert((j, i));
            }
        }

        Ok(Self { edges, index_to_id })
    }
}

impl Graph {
    fn part1(&self) -> Result<usize> {
        let mut out = 0;

        for (&i, a) in &self.index_to_id {
            for (&j, b) in &self.index_to_id {
                for (&k, c) in &self.index_to_id {
                    if self.edges.contains(&(i, j))
                        && self.edges.contains(&(i, k))
                        && self.edges.contains(&(j, k))
                        && (a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
                    {
                        out += 1;
                    }
                }
            }
        }

        Ok(out)
    }

    fn part2(&self) -> Result<String> {
        let mut cliques: Vec<HashSet<usize>> = Vec::new();

        for &i in self.index_to_id.keys() {
            let mut singleton = HashSet::new();
            singleton.insert(i);
            cliques.push(singleton);
        }

        for &i in self.index_to_id.keys() {
            for clique in cliques.iter_mut() {
                if clique
                    .iter()
                    .all(|&j| self.edges.contains(&(i, j)) || self.edges.contains(&(j, i)))
                {
                    clique.insert(i);
                }
            }
        }

        let biggest = cliques
            .into_iter()
            .max_by_key(|c| c.len())
            .ok_or(anyhow!("expect at least one node"))?;

        let mut tokens: Vec<&String> = biggest.iter().map(|i| &self.index_to_id[i]).collect();
        tokens.sort();

        Ok(tokens.iter().join(","))
    }
}

fn main() -> Result<()> {
    let graph: Graph = aoc::io::read_all((2024, 23))?.parse()?;
    //let graph: Graph = aoc::io::read_all("data/day23/example")?.parse()?;

    println!("Part 1: {}", graph.part1()?);
    println!("Part 2: {}", graph.part2()?);
    Ok(())
}
