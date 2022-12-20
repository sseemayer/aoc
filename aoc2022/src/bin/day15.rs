use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

type Pos = [i32; 2];

#[derive(Debug, Clone)]
struct World {
    sensors: HashMap<Pos, u32>,
    beacons: HashSet<Pos>,

    extent: (Pos, Pos),
}

lazy_static! {
    static ref RE_LINE: Regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
}

impl World {
    fn parse(path: &str) -> Result<Self> {
        let mut sensors = HashMap::new();
        let mut beacons = HashSet::new();
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        for line in BufReader::new(File::open(path)?).lines() {
            let line = line?;

            let captures = RE_LINE
                .captures(&line)
                .ok_or_else(|| anyhow!("Expected regex match: '{}'", line))?;

            let sx: i32 = captures.get(1).unwrap().as_str().parse()?;
            let sy: i32 = captures.get(2).unwrap().as_str().parse()?;
            let bx: i32 = captures.get(3).unwrap().as_str().parse()?;
            let by: i32 = captures.get(4).unwrap().as_str().parse()?;

            let dist: u32 = i32::abs_diff(sx, bx) + i32::abs_diff(sy, by);
            let d: i32 = dist as i32;

            min_x = i32::min(min_x, sx - d);
            min_y = i32::min(min_y, sy - d);
            max_x = i32::max(max_x, sx + d);
            max_y = i32::max(max_y, sy + d);

            sensors.insert([sx, sy], dist);
            beacons.insert([bx, by]);
        }

        let extent = ([min_x, min_y], ([max_x, max_y]));

        Ok(Self {
            sensors,
            beacons,
            extent,
        })
    }

    fn scan_line(&self, y: i32) -> Vec<(i32, i32)> {
        let ([xmin, ymin], [xmax, ymax]) = self.extent;

        let mut covered = self
            .sensors
            .iter()
            .filter_map(|(&[sx, sy], &sd)| {
                let xsteps = sd as i32 - i32::abs_diff(sy, y) as i32;

                if xsteps >= 0 {
                    Some((sx - xsteps, sx + xsteps))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        covered.sort();

        for i in 0..covered.len() {
            let mut j = i + 1;
            while j < covered.len() {
                let (x2, x3) = covered[j];
                let (x0, x1) = covered.get_mut(i).unwrap();

                if x2 >= *x0 && x2 <= *x1 {
                    // j starts inside of i
                    // possibly extend i to envelop j
                    *x1 = i32::max(*x1, x3);
                    covered.remove(j);
                } else {
                    j += 1;
                }
            }
        }

        covered
    }

    fn no_beacon_spots(&self, y: i32) -> i32 {
        let covered: i32 = self.scan_line(y).iter().map(|&(x0, x1)| x1 - x0 + 1).sum();
        let beacons = self.beacons.iter().filter(|[_x, by]| *by == y).count();

        covered - beacons as i32
    }

    fn find_free(&self, search_space: (Pos, Pos)) -> Option<Pos> {
        let ([xmin, ymin], [xmax, ymax]) = search_space;

        for y in ymin..=ymax {
            let covered_spots = self.scan_line(y);

            let mut x = xmin;

            for (x0, x1) in covered_spots {
                if x >= x0 && x <= x1 {
                    x = x1 + 1;
                }

                if self.beacons.contains(&[x, y]) {
                    x += 1;
                }
            }

            if x <= xmax {
                return Some([x, y]);
            }
        }

        None
    }
}

fn main() -> Result<()> {
    // let world = World::parse("data/day15/example")?;
    // println!("Part 1: {}", world.no_beacon_spots(10));

    // if let Some([x, y]) = world.find_free(([0, 0], [20, 20])) {
    //     println!("Part 2: {} ({}, {})", x * 4000000 + y, x, y);
    // }

    let world = World::parse("data/day15/input")?;
    println!("Part 1: {}", world.no_beacon_spots(2_000_000));

    if let Some([x, y]) = world.find_free(([0, 0], [4_000_000, 4_000_000])) {
        println!(
            "Part 2: {} ({}, {})",
            x as usize * 4_000_000 + y as usize,
            x,
            y
        );
    }

    Ok(())
}
