use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
struct Graph {
    links: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn parse(path: &str) -> Result<Self> {
        let mut links: HashMap<String, HashSet<String>> = HashMap::new();

        for line in aoc::io::read_all(path)?.lines() {
            let (src, dests) = line
                .trim()
                .split_once(":")
                .ok_or_else(|| anyhow!("Cannot split on ':': '{}", line))?;

            let dests: Vec<String> = dests
                .split_whitespace()
                .map(|s| s.trim().to_string())
                .collect();

            for dest in dests {
                links
                    .entry(src.to_string())
                    .or_default()
                    .insert(dest.clone());
                links.entry(dest).or_default().insert(src.to_string());
            }
        }

        Ok(Self { links })
    }

    fn find_furthest(&self, start: &str) -> String {
        let mut seen: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(start.to_string());

        let mut furthest = start.to_string();
        while let Some(current) = queue.pop_front() {
            furthest = current.to_string();

            if let Some(neighbors) = self.links.get(&current) {
                for neighbor in neighbors {
                    if seen.contains(neighbor) {
                        continue;
                    }

                    seen.insert(neighbor.to_string());
                    queue.push_back(neighbor.to_string());
                }
            }
        }

        furthest
    }

    fn bfs_non_repeating(&self, start: &str, end: &str) -> usize {
        let mut seen: HashSet<String> = HashSet::new();
        let mut used: HashSet<(String, String)> = HashSet::new();

        for _ in 0..4 {
            seen.clear();

            let mut queue = VecDeque::new();
            queue.push_back(vec![start.to_string()]);

            while let Some(path) = queue.pop_front() {
                let current = path.last().expect("non empty paths");

                // check if we reached the goal
                if current == end {
                    // println!("Found path: {:?}", path);

                    // disallow all parts of this solution for future iterations
                    for i in 1..path.len() {
                        used.insert((path[i - 1].to_string(), path[i].to_string()));
                    }

                    // try again in next iteration
                    break;
                }

                if let Some(neighbors) = self.links.get(current) {
                    for neighbor in neighbors {
                        if used.contains(&(current.to_string(), neighbor.to_string()))
                            || used.contains(&(neighbor.to_string(), current.to_string()))
                            || seen.contains(neighbor)
                        {
                            continue;
                        }

                        seen.insert(neighbor.to_string());

                        let mut new_path = path.clone();
                        new_path.push(neighbor.to_string());

                        queue.push_back(new_path);
                    }
                }
            }

            // println!("reached {} nodes", seen.len());
        }

        seen.len() * (self.links.len() - seen.len())
    }
}

fn part1(graph: &Graph) -> usize {
    // start a BFS from an arbitrary node and find a maximally-distant start node
    //
    let seed = graph.links.keys().next().expect("non empty graph");
    let start = graph.find_furthest(seed);
    let end = graph.find_furthest(&start);

    // println!("chose start={} end={}", start, end);

    graph.bfs_non_repeating(&start, &end)
}

fn main() -> Result<()> {
    let graph = Graph::parse("data/day25/input")?;

    println!("Part 1: {}", part1(&graph));

    Ok(())
}
