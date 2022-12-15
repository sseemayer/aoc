use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
struct Point(Vec<i32>);

impl std::str::FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Point(
            s.split(",")
                .map(|t| t.parse().context("Parsing coordinate"))
                .collect::<Result<Vec<i32>>>()?,
        ))
    }
}

impl Point {
    fn adjacent_to(&self, other: &Self) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| i32::abs_diff(*a, *b))
            .sum::<u32>()
            <= 3
    }
}

fn parse(path: &str) -> Result<Vec<Point>> {
    BufReader::new(File::open(path)?)
        .lines()
        .map(|line| line?.parse())
        .collect()
}

fn cluster(points: &[Point]) -> Vec<HashSet<usize>> {
    let mut clusters: Vec<HashSet<usize>> = Vec::new();

    for (i, p) in points.iter().enumerate() {
        let mut adjacent_clusters = HashSet::new();
        for (j, c) in clusters.iter().enumerate() {
            for k in c {
                let q = &points[*k];
                if p.adjacent_to(q) {
                    adjacent_clusters.insert(j);
                }
            }
        }

        if adjacent_clusters.is_empty() {
            // println!("New cluster {} with member {}", clusters.len(), i);

            let mut new_cluster = HashSet::new();
            new_cluster.insert(i);
            clusters.push(new_cluster);
        } else if adjacent_clusters.len() == 1 {
            let j = *adjacent_clusters.iter().next().unwrap();
            // println!("Add {} to cluster {}", i, j);
            clusters[j].insert(i);
        } else if adjacent_clusters.len() > 1 {
            let mut adjacent_clusters = adjacent_clusters.into_iter().collect::<Vec<_>>();
            adjacent_clusters.sort_by_key(|k| std::cmp::Reverse(*k));

            let j = adjacent_clusters.pop().unwrap();

            // println!("Merge {:?} into {}", adjacent_clusters, j);

            for k in adjacent_clusters {
                // println!("{:?} |= {:?}", clusters[j], clusters[k]);
                clusters[j] = &clusters[j] | &clusters[k];
                clusters.remove(k);
            }

            clusters[j].insert(i);
        }
    }

    clusters
}

fn main() -> Result<()> {
    let points = parse("data/day25/input")?;

    let clusters = cluster(&points[..]);
    println!("Part 1: {}", clusters.len());

    Ok(())
}
