use anyhow::Result;
use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Read},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum ParseError {
    #[error("Bad line: '{}'", .0)]
    BadLine(String),

    #[error(transparent)]
    Int(#[from] std::num::ParseIntError),
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl std::fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:>5},{:>5},{:>5}>", self.x, self.y, self.z)
    }
}

impl std::ops::Mul<i64> for Coordinate {
    type Output = Coordinate;

    fn mul(self, rhs: i64) -> Self::Output {
        Coordinate {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Neg for Coordinate {
    type Output = Coordinate;

    fn neg(self) -> Self::Output {
        self * -1
    }
}

impl std::ops::Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::str::FromStr for Coordinate {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let xyz = s
            .split(",")
            .map(|n| Ok(n.parse()?))
            .collect::<std::result::Result<Vec<i64>, Self::Err>>()?;

        if xyz.len() != 3 {
            return Err(ParseError::BadLine(s.to_string()));
        }

        let x = xyz[0];
        let y = xyz[1];
        let z = xyz[2];

        Ok(Coordinate { x, y, z })
    }
}

impl Coordinate {
    fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Debug, Clone, Copy)]
struct Matrix([[i64; 3]; 3]);

lazy_static! {
    static ref TRANSFORMATIONS: Vec<Matrix> = {
        let mut out = Vec::new();

        let directions = [
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
            [-1, 0, 0],
            [0, -1, 0],
            [0, 0, -1],
        ];

        for x in directions.iter() {
            for y in directions.iter() {
                if x != y {
                    let z = [
                        x[1] * y[2] - x[2] * y[1],
                        x[2] * y[0] - x[0] * y[2],
                        x[0] * y[1] - x[1] * y[0],
                    ];

                    out.push(Matrix([x.clone(), y.clone(), z]));
                }
            }
        }

        out
    };
}

impl std::ops::Mul<Matrix> for Coordinate {
    type Output = Coordinate;
    fn mul(self, m: Matrix) -> Coordinate {
        Coordinate {
            x: self.x * m.0[0][0] + self.y * m.0[0][1] + self.z * m.0[0][2],
            y: self.x * m.0[1][0] + self.y * m.0[1][1] + self.z * m.0[1][2],
            z: self.x * m.0[2][0] + self.y * m.0[2][1] + self.z * m.0[2][2],
        }
    }
}

#[derive(Debug, Default)]
struct Scanner {
    signals: Vec<Coordinate>,
    coord_to_idx: HashMap<Coordinate, usize>,
    deltas: HashMap<Coordinate, HashSet<[usize; 2]>>,
}

fn parse_scanners<R: Read>(r: &mut BufReader<R>) -> Result<VecDeque<Scanner>> {
    let mut out = VecDeque::new();

    let mut signals = Vec::new();

    for line in r.lines() {
        let line = line?;

        if line.trim().is_empty() {
            continue;
        } else if line.starts_with("---") {
            if !signals.is_empty() {
                out.push_back(Scanner::from_signals(signals));
                signals = Vec::new();
            }
        } else {
            let signal: Coordinate = line.trim().parse()?;
            signals.push(signal);
        }
    }

    if !signals.is_empty() {
        out.push_back(Scanner::from_signals(signals));
    }

    Ok(out)
}

impl std::ops::Mul<Matrix> for &Scanner {
    type Output = Scanner;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let signals = self.signals.iter().map(|s| *s * rhs).collect();
        Scanner::from_signals(signals)
    }
}

impl Scanner {
    fn from_signals(mut signals: Vec<Coordinate>) -> Self {
        signals.sort();

        let mut deltas: HashMap<Coordinate, HashSet<[usize; 2]>> = HashMap::new();
        let mut coord_to_idx = HashMap::new();

        for (i, &c) in signals.iter().enumerate() {
            coord_to_idx.insert(c, i);
            for (j, &d) in signals[..i].iter().enumerate() {
                let delta = d - c;
                deltas.entry(delta).or_default().insert([i, j]);
            }
        }

        Scanner {
            signals,
            coord_to_idx,
            deltas,
        }
    }

    fn insert(&mut self, c: Coordinate) -> usize {
        if let Some(&i) = self.coord_to_idx.get(&c) {
            i
        } else {
            let i = self.signals.len();

            self.signals.push(c);
            self.coord_to_idx.insert(c, i);

            for (j, &d) in self.signals.iter().enumerate() {
                let delta = d - c;
                self.deltas.entry(delta).or_default().insert([i, j]);
                self.deltas.entry(-delta).or_default().insert([j, i]);
            }

            i
        }
    }

    fn try_absorb(&mut self, other: &Scanner) -> Option<Coordinate> {
        for transform in TRANSFORMATIONS.iter() {
            let transformed = other * *transform;

            let mut overlaps: HashSet<usize> = HashSet::new();
            let mut offsets: HashMap<Coordinate, usize> = HashMap::new();
            for (other_delta, other_pairs) in transformed.deltas.iter() {
                if let Some(self_pairs) = self.deltas.get(other_delta) {
                    for &[i, j] in self_pairs.iter() {
                        let c = self.signals[i];
                        let d = self.signals[j];
                        for &[k, l] in other_pairs.iter() {
                            let e = transformed.signals[k];
                            let f = transformed.signals[l];

                            let v1 = e - c;
                            let v2 = f - d;

                            assert_eq!(v1, v2);

                            *offsets.entry(v1).or_default() += 1;

                            overlaps.insert(k);
                            overlaps.insert(l);
                        }
                    }

                    if overlaps.len() >= 12 {
                        break;
                    }
                }
            }

            if overlaps.len() >= 12 {
                let mut offsets: Vec<(Coordinate, usize)> = offsets.into_iter().collect();
                offsets.sort_by_key(|(_, n)| std::cmp::Reverse(*n));

                let ofs = offsets.first().expect("Got offset").0;
                for (i, &coord) in transformed.signals.iter().enumerate() {
                    if !overlaps.contains(&i) {
                        let coord_transformed = coord - ofs;
                        self.insert(coord_transformed);
                    }
                }

                return Some(ofs);
            }
        }

        None
    }
}

fn main() -> Result<()> {
    let mut reader = BufReader::new(File::open("data/day19/input")?);
    let mut scanners = parse_scanners(&mut reader)?;

    let mut universe = scanners.pop_front().unwrap();
    let mut scanner_positions: Vec<Coordinate> = vec![Coordinate { x: 0, y: 0, z: 0 }];

    while let Some(scanner) = scanners.pop_front() {
        if let Some(pos) = universe.try_absorb(&scanner) {
            scanner_positions.push(pos);
        } else {
            scanners.push_back(scanner);
        }
    }

    println!("Part 1: {}", universe.signals.len());

    let mut max_dist = 0;
    for (i, &a) in scanner_positions.iter().enumerate() {
        for &b in scanner_positions[..i].iter() {
            let delta = (b - a).manhattan();

            if delta > max_dist {
                max_dist = delta;
            }
        }
    }

    println!("Part 2: {}", max_dist);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
}
