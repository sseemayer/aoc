use std::{
    collections::{BTreeSet, HashMap, HashSet},
    str::FromStr,
};

use anyhow::{Error, Result};
use colored::Colorize;

#[derive(Debug)]
struct Graph {
    label_to_index: HashMap<String, usize>,

    edges: HashMap<usize, HashSet<usize>>,
}

impl FromStr for Graph {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut n_labels = 0;
        let mut label_to_index = HashMap::new();
        let mut edges = HashMap::new();

        for line in s.lines() {
            if let Some((id, connections)) = line.trim().split_once(":") {
                let idx = *label_to_index.entry(id.to_string()).or_insert_with(|| {
                    let new_index = n_labels;
                    n_labels += 1;
                    new_index
                });

                let connections = connections
                    .split_whitespace()
                    .map(|s| s.trim())
                    .map(|label| {
                        *label_to_index.entry(label.to_string()).or_insert_with(|| {
                            let new_index = n_labels;
                            n_labels += 1;
                            new_index
                        })
                    })
                    .collect::<HashSet<usize>>();

                edges
                    .entry(idx)
                    .or_insert_with(HashSet::new)
                    .extend(connections);
            }
        }

        Ok(Self {
            label_to_index,
            edges,
        })
    }
}

impl Graph {
    /// Count the number of paths from start to dest that visit all nodes in via at least once.
    fn paths(&self, start: usize, dest: usize, via: &[usize]) -> usize {
        let mut memo: HashMap<(usize, BTreeSet<usize>), usize> = HashMap::new();

        fn inner(
            current: usize,
            dest: usize,
            via: BTreeSet<usize>,
            mut visited: BTreeSet<usize>,
            memo: &mut HashMap<(usize, BTreeSet<usize>), usize>,
            edges: &HashMap<usize, HashSet<usize>>,
        ) -> usize {
            if current == dest {
                if visited.len() == via.len() {
                    return 1;
                } else {
                    return 0;
                }
            }

            if via.contains(&current) {
                visited.insert(current);
            }

            let key = (current, visited.clone());

            if memo.contains_key(&key) {
                memo[&key]
            } else {
                let mut total_paths = 0;

                for &neighbor in &edges[&current] {
                    total_paths += inner(neighbor, dest, via.clone(), visited.clone(), memo, edges);
                }

                memo.insert(key, total_paths);

                total_paths
            }
        }

        inner(
            start,
            dest,
            via.iter().cloned().collect(),
            BTreeSet::new(),
            &mut memo,
            &self.edges,
        )
    }
}

fn main() -> Result<()> {
    let graph: Graph = aoc::io::read_all((2025, 11))?.parse()?;
    //let graph: Graph = aoc::io::read_all("data/day11/example")?.parse()?;
    //let graph: Graph = aoc::io::read_all("data/day11/example2")?.parse()?;

    let out = graph.label_to_index["out"];

    let you = graph.label_to_index["you"];
    let part1 = graph.paths(you, out, &[]);
    println!("{} {}", "Part 1:".bold().green(), part1);

    let svr = graph.label_to_index["svr"];
    let dac = graph.label_to_index["dac"];
    let fft = graph.label_to_index["fft"];
    let part2 = graph.paths(svr, out, &[dac, fft]);

    println!("{} {}", "Part 2:".bold().green(), part2);

    Ok(())
}
