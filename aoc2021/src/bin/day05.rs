use std::collections::HashSet;

use aoc2021::io::{read_lines, ReadLinesError};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
struct Line {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

#[derive(Error, Debug)]
enum LineError {
    #[error("Bad line: '{}'", .0)]
    BadLine(String),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

impl std::str::FromStr for Line {
    type Err = LineError;
    fn from_str(s: &str) -> Result<Line, LineError> {
        if let Some((xy1, xy2)) = s.trim().split_once(" -> ") {
            if let (Some((x1, y1)), Some((x2, y2))) = (xy1.split_once(","), xy2.split_once(",")) {
                let x1 = x1.parse()?;
                let y1 = y1.parse()?;
                let x2 = x2.parse()?;
                let y2 = y2.parse()?;

                return Ok(Line { x1, y1, x2, y2 });
            }
        }
        Err(LineError::BadLine(s.to_string()))
    }
}

impl Line {
    fn is_straight(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn covered(&self) -> HashSet<(i64, i64)> {
        let mut out = HashSet::new();

        let dx = self.x2 - self.x1;
        let dy = self.y2 - self.y1;

        if dx.abs() == dy.abs() || dx == 0 || dy == 0 {
            let steps = i64::max(dx.abs(), dy.abs());
            let dir_x = dx / steps;
            let dir_y = dy / steps;

            for s in 0..=steps {
                out.insert((self.x1 + dir_x * s, self.y1 + dir_y * s));
            }
        } else {
            todo!()
        }

        out
    }
}

#[derive(Error, Debug)]
enum Day05Error {
    #[error(transparent)]
    ReadLines(#[from] ReadLinesError<Line>),
}

fn find_junctions(lines: &[Line]) -> HashSet<(i64, i64)> {
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut junctions: HashSet<(i64, i64)> = HashSet::new();

    for line in lines {
        let covered = line.covered();

        let new_junctions: HashSet<(i64, i64)> = visited.intersection(&covered).cloned().collect();

        visited.extend(covered.iter());
        junctions.extend(new_junctions.iter());
    }

    junctions
}

fn main() -> Result<(), ReadLinesError<Line>> {
    let lines = read_lines("data/day05/input")?;

    let straight_lines: Vec<Line> = lines.iter().filter(|l| l.is_straight()).cloned().collect();

    println!(
        "Part 1: Got {} junctions",
        find_junctions(&straight_lines[..]).len()
    );
    println!("Part 2: Got {} junctions", find_junctions(&lines[..]).len());

    Ok(())
}
