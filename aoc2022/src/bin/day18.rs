use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, bail, Context, Result};
use itertools::Itertools;

type Pos = [i32; 3];

const OFFSETS: [Pos; 6] = [
    [-1, 0, 0],
    [0, 1, 0],
    [1, 0, 0],
    [0, -1, 0],
    [0, 0, -1],
    [0, 0, 1],
];

#[derive(Debug, Clone)]
struct Scan {
    points: HashSet<Pos>,
}

impl Scan {
    fn parse(path: &str) -> Result<Self> {
        let points = BufReader::new(File::open(path)?)
            .lines()
            .map(|line| {
                let line = line?;

                let tokens: Vec<i32> = line
                    .split(",")
                    .map(|t| t.parse::<i32>().context("Parse coordinate"))
                    .collect::<Result<Vec<i32>>>()?;

                if tokens.len() != 3 {
                    bail!("Expected exactly 3 tokens: {}", line);
                }

                Ok([tokens[0], tokens[1], tokens[2]])
            })
            .collect::<Result<HashSet<Pos>>>()?;

        Ok(Self { points })
    }

    fn surface_area_1(&self) -> usize {
        self.points
            .iter()
            .map(|&[i, j, k]| {
                OFFSETS
                    .iter()
                    .filter(move |&[di, dj, dk]| !self.points.contains(&[i + di, j + dj, k + dk]))
            })
            .flatten()
            .count()
    }

    fn extent(&self) -> (Pos, Pos) {
        let (min_i, max_i) = self
            .points
            .iter()
            .map(|&[i, _, _]| i)
            .minmax()
            .into_option()
            .expect("Have points");

        let (min_j, max_j) = self
            .points
            .iter()
            .map(|&[_, j, _]| j)
            .minmax()
            .into_option()
            .expect("Have points");

        let (min_k, max_k) = self
            .points
            .iter()
            .map(|&[_, _, k]| k)
            .minmax()
            .into_option()
            .expect("Have points");

        (
            [min_i - 1, min_j - 1, min_k - 1],
            [max_i + 1, max_j + 1, max_k + 1],
        )
    }

    fn surface_area_2(&self) -> usize {
        let (min_pos, max_pos) = self.extent();
        let mut queue = vec![min_pos];
        let mut visited = HashSet::new();
        let mut surfaces = 0;
        while let Some([i, j, k]) = queue.pop() {
            if !visited.insert([i, j, k]) {
                continue;
            }

            for [di, dj, dk] in OFFSETS {
                let new_pos = [i + di, j + dj, k + dk];

                if self.points.contains(&new_pos) {
                    surfaces += 1;
                } else if min_pos.iter().zip(new_pos.iter()).all(|(min, p)| p >= min)
                    && max_pos.iter().zip(new_pos.iter()).all(|(max, p)| p <= max)
                {
                    queue.push(new_pos);
                }
            }
        }

        surfaces
    }
}

fn main() -> Result<()> {
    let scan = Scan::parse("data/day18/input")?;

    println!("Part 1: {}", scan.surface_area_1());
    println!("Part 2: {}", scan.surface_area_2());

    Ok(())
}
