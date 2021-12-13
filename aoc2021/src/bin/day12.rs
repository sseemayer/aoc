use aoc2021::io::{read_lines, ReadLinesError};
use std::collections::{HashMap, VecDeque};
use thiserror::Error;

#[derive(Error, Debug)]
enum Day12Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Read(#[from] ReadLinesError<Connection>),
}

type Result<T> = std::result::Result<T, Day12Error>;

#[derive(Debug)]
struct Connection {
    l: String,
    r: String,
}

#[derive(Error, Debug)]
enum ParseConnectionError {
    #[error("Bad line: '{}'", .0)]
    BadLine(String),
}

impl std::str::FromStr for Connection {
    type Err = ParseConnectionError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (l, r) = s
            .split_once("-")
            .ok_or(ParseConnectionError::BadLine(s.to_string()))?;

        let l = l.to_string();
        let r = r.to_string();

        Ok(Connection { l, r })
    }
}

#[derive(Debug)]
struct Graph {
    name_to_idx: HashMap<String, usize>,
    idx_to_name: Vec<String>,
    big: Vec<bool>,
    connections: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn from_connections(conns: &[Connection]) -> Self {
        let mut idx_to_name = Vec::new();
        let mut name_to_idx = HashMap::new();
        let mut big = Vec::new();
        let mut connections: HashMap<usize, Vec<usize>> = HashMap::new();
        for c in conns.iter() {
            let il = *name_to_idx.entry(c.l.to_string()).or_insert_with(|| {
                idx_to_name.push(c.l.to_string());
                big.push(c.l.to_uppercase() == c.l);
                big.len() - 1
            });

            let ir = *name_to_idx.entry(c.r.to_string()).or_insert_with(|| {
                idx_to_name.push(c.r.to_string());
                big.push(c.r.to_uppercase() == c.r);
                big.len() - 1
            });

            connections.entry(il).or_default().push(ir);
            connections.entry(ir).or_default().push(il);
        }

        Graph {
            name_to_idx,
            idx_to_name,
            big,
            connections,
        }
    }

    fn paths(&self, allow_revisit: bool) -> Vec<Vec<usize>> {
        let mut queue: VecDeque<(Vec<usize>, HashMap<usize, usize>)> = VecDeque::new();

        let start = self.name_to_idx["start"];
        let end = self.name_to_idx["end"];

        queue.push_back((vec![start], vec![(start, 1)].into_iter().collect()));

        let mut solutions = Vec::new();

        while let Some((path, counts)) = queue.pop_front() {
            let last = *path.last().unwrap();

            if last == end {
                solutions.push(path);
            } else {
                for neighbor in &self.connections[&last] {
                    let mut new_path = path.clone();
                    new_path.push(*neighbor);

                    let mut new_counts = counts.clone();
                    if !self.big[*neighbor] {
                        // we are entering a small cave - check if we are allowed to
                        let neighbor_count = counts.get(neighbor).unwrap_or(&0);

                        if !allow_revisit && *neighbor_count > 0 {
                            // part 1 - do not allow visiting small caves that
                            // have been visited before
                            continue;
                        }

                        if *neighbor_count >= 2 {
                            // do not allow visiting a cave more than 2 times
                            continue;
                        }

                        if *neighbor_count >= 1 && (*neighbor == start || *neighbor == end) {
                            // do not allow re-visiting start or end
                            continue;
                        }

                        let max_count = counts.values().max().unwrap();

                        if *max_count >= 2 && *neighbor_count >= 1 {
                            // another cave has been visited twice - cannot visit this cave twice
                            continue;
                        }

                        *new_counts.entry(*neighbor).or_default() += 1;
                    }

                    queue.push_back((new_path, new_counts));
                }
            }
        }

        solutions
    }
}

fn main() -> Result<()> {
    let lines = read_lines("data/day12/input")?;
    let graph = Graph::from_connections(&lines[..]);

    let paths1 = graph.paths(false);
    println!("Part 1: got {} paths", paths1.len());

    let paths2 = graph.paths(true);
    println!("Part 2: got {} paths", paths2.len());

    Ok(())
}
