use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Context, Result};

fn parse_coords(f: &str) -> Result<Points> {
    let mut points = Vec::new();
    for line in BufReader::new(File::open(f)?).lines() {
        let line = line?;

        let (x, y) = line
            .split_once(", ")
            .ok_or_else(|| anyhow!("Bad line: '{}'", line))?;

        let x: i16 = x.parse().context("Parse x coord")?;
        let y: i16 = y.parse().context("Parse y coord")?;
        points.push((x, y))
    }
    Ok(Points { points })
}

#[derive(Debug)]
struct Points {
    points: Vec<(i16, i16)>,
}

impl Points {
    /// Get all points that are closest to a coordinate
    fn closest(&self, x: i16, y: i16) -> Option<(i16, Vec<usize>)> {
        let mut min: Option<(i16, Vec<usize>)> = None;

        for (i, (px, py)) in self.points.iter().enumerate() {
            let dist = i16::abs(px - x) + i16::abs(py - y);

            min = Some(if let Some((current_distance, mut current_points)) = min {
                if dist < current_distance {
                    (dist, vec![i])
                } else if dist == current_distance {
                    current_points.push(i);
                    (current_distance, current_points)
                } else {
                    (current_distance, current_points)
                }
            } else {
                (dist, vec![i])
            })
        }

        min
    }

    fn extent(&self) -> (i16, i16) {
        let mut max_x = 0;
        let mut max_y = 0;

        for (px, py) in self.points.iter() {
            max_x = i16::max(max_x, *px);
            max_y = i16::max(max_y, *py);
        }

        (max_x, max_y)
    }

    fn largest_area(&self) -> Option<usize> {
        let (max_x, max_y) = self.extent();
        let mut area: HashMap<usize, usize> = HashMap::new();
        let mut finite: HashSet<usize> = (0..self.points.len()).collect();

        for x in 0..=max_x {
            for y in 0..=max_y {
                if let Some((_closest_distance, closest_points)) = self.closest(x, y) {
                    if closest_points.len() == 1 {
                        if let Some(&cp) = closest_points.first() {
                            *area.entry(cp).or_default() += 1;

                            if x == 0 || x == max_x || y == 0 || y == max_y {
                                finite.remove(&cp);
                            }
                        }
                    }
                }
            }
        }

        finite.iter().filter_map(|i| area.get(i)).max().map(|i| *i)
    }

    fn central_region(&self) -> usize {
        let (max_x, max_y) = self.extent();
        let mut area = 0;

        for x in 0..=max_x {
            for y in 0..=max_y {
                let distance: i16 = self
                    .points
                    .iter()
                    .map(|(px, py)| i16::abs(px - x) + i16::abs(py - y))
                    .sum();

                if distance < 10000 {
                    area += 1;
                }
            }
        }

        area
    }
}

fn main() -> Result<()> {
    let points = parse_coords("data/day06/input")?;

    println!("Part 1: {}", points.largest_area().expect("Largest area"));
    println!("Part 2: {}", points.central_region());

    Ok(())
}
