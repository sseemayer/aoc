use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

type D = i32;
type Pos = [D; 3];

#[derive(Debug, Clone)]
struct Bot {
    pos: Pos,
    range: D,
}

lazy_static! {
    static ref RE_BOT: Regex = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
}

impl std::str::FromStr for Bot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE_BOT
            .captures(s)
            .ok_or_else(|| anyhow!("Bad bot definition: '{}'", s))?;

        let i: D = captures.get(1).unwrap().as_str().parse()?;
        let j: D = captures.get(2).unwrap().as_str().parse()?;
        let k: D = captures.get(3).unwrap().as_str().parse()?;

        let range: D = captures.get(4).unwrap().as_str().parse()?;

        Ok(Self {
            pos: [i, j, k],
            range,
        })
    }
}

impl Bot {
    fn distance_to(&self, pos: &Pos) -> D {
        self.pos
            .iter()
            .zip(pos.iter())
            .map(|(a, b)| D::abs_diff(*a, *b) as D)
            .sum()
    }

    fn can_see(&self, pos: &Pos) -> bool {
        self.distance_to(pos) <= self.range
    }
}

fn parse(path: &str) -> Result<Vec<Bot>> {
    BufReader::new(File::open(path)?)
        .lines()
        .map(|line| line?.trim().parse())
        .collect()
}

/// Transform from original coordinate system to one that is rotated 45 degrees on
/// X and Z axes to be aligned with the ranges of the bots.
fn transform_coord(pos: &Pos) -> Pos {
    //       y
    //      /
    //     /
    //    /
    //   /
    //  0-2-4-6-8-1-1->j
    //  |\ I      0 2
    //  2 \
    //  |  \    C
    //  4   \  B|D
    //  |    \A-+-E
    //  6     \H|F
    //  |      \G
    //  8       \
    // i|        x
    //  v
    //
    // perform coordinate transformation:
    // x =  i + j + k
    // y =  i - j + k
    // z = -i - j + k
    //
    //  |
    // -8
    //  |
    // -6
    //  |          C D E
    // -4           \ /
    //  |          B + F
    // -2   I       / \
    //  |          A H G
    //  0-2-4-6-8-1-1-1->x
    //  |         0 2 4

    let &[i, j, k] = pos;
    let x = i + j + k;
    let y = i - j + k;
    let z = -i - j + k;

    [x, y, z]
}

fn untransform_coord(pos: &Pos) -> Pos {
    let &[x, y, z] = pos;

    let i = (y - z) / 2;
    let j = (x - y) / 2;
    let k = (x + z) / 2;

    [i, j, k]
}

/// An axis-aligned 3D cuboid defined by minimum and maximum coordinates, inclusive.
#[derive(Debug, Clone, Copy)]
struct Cuboid {
    min: Pos,
    max: Pos,
}

impl Cuboid {
    fn from_bot(bot: &Bot) -> Self {
        let r = bot.range;

        // transform midpoint to new coordinate system
        let [x, y, z] = transform_coord(&bot.pos);

        // cuboid is defined by its axis-aligned extreme points
        let min = [x - r, y - r, z - r];
        let max = [x + r, y + r, z + r];

        Self { min, max }
    }

    fn to_bot(&self) -> Pos {
        untransform_coord(&self.min)
    }
}

/// Intersect cuboids
impl std::ops::BitAnd for &Cuboid {
    type Output = Option<Cuboid>;

    fn bitand(self, rhs: Self) -> Self::Output {
        let min = [
            D::max(self.min[0], rhs.min[0]),
            D::max(self.min[1], rhs.min[1]),
            D::max(self.min[2], rhs.min[2]),
        ];

        let max = [
            D::min(self.max[0], rhs.max[0]),
            D::min(self.max[1], rhs.max[1]),
            D::min(self.max[2], rhs.max[2]),
        ];

        if min.iter().zip(max.iter()).all(|(min, max)| min <= max) {
            Some(Cuboid { min, max })
        } else {
            None
        }
    }
}

fn central_position(bots: &[Bot]) -> Option<(D, Pos, usize)> {
    let cuboids = bots
        .iter()
        .map(|bot| Cuboid::from_bot(bot))
        .collect::<Vec<_>>();

    // make a list of most-overlapping cuboids
    let intersections: HashMap<usize, usize> = cuboids
        .iter()
        .enumerate()
        .map(|(i, a)| (i, cuboids.iter().filter(|b| (a & b).is_some()).count()))
        .collect();

    // go by descending number of candidates
    for n in (1..cuboids.len()).rev() {
        let mut min_solution = None;

        // pre-filter the cuboids to only the ones that have sufficient interactions
        let candidates = cuboids.iter().enumerate().filter_map(|(i, cuboid)| {
            if intersections[&i] >= n {
                Some(cuboid)
            } else {
                None
            }
        });

        for combination in candidates.combinations(n) {
            // create the intersection of all these cuboids
            let first = combination.first().copied().copied();
            let intersection: Option<Cuboid> = combination[1..]
                .iter()
                .fold(first, |accum, item| accum.and_then(|a| &a & item));

            if let Some(int) = intersection {
                // we found a solution! see if it is the minimal one.
                let pos = int.to_bot();
                let dist = pos.iter().map(|d| D::abs(*d)).sum::<D>();

                min_solution = match min_solution {
                    None => Some((dist, pos, n)),
                    Some((d, _, _)) if d > dist => Some((dist, pos, n)),
                    _ => min_solution,
                };
            }
        }

        if min_solution.is_some() {
            return min_solution;
        }
    }

    None
}

fn main() -> Result<()> {
    let bots = parse("data/day23/input")?;

    if let Some(strongest_bot) = bots.iter().max_by_key(|b| b.range) {
        let n_in_range = bots
            .iter()
            .filter(|b| strongest_bot.can_see(&b.pos))
            .count();
        println!("Part 1: {}", n_in_range);
    }

    if let Some((d, cpos, n)) = central_position(&bots[..]) {
        println!("Part 2: {} (at {:?}, covering {})", d, cpos, n);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transform() {
        for i in -10..=10 {
            for j in -10..=10 {
                for k in -10..=10 {
                    let ijk = [i, j, k];
                    let xyz = transform_coord(&ijk);
                    let ijk2 = untransform_coord(&xyz);

                    assert_eq!(ijk, ijk2);
                }
            }
        }
    }
}
