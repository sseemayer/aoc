use aoc2015::util::read_to_string;
use aoc2015::{format_err, Error, Result};
use std::collections::HashSet;

struct Pos {
    i: i64,
    j: i64,
}

impl From<(i64, i64)> for Pos {
    fn from(v: (i64, i64)) -> Self {
        Pos { i: v.0, j: v.1 }
    }
}

fn walk(mut pos: Vec<Pos>, steps: &str) -> HashSet<(i64, i64)> {
    let mut cur_walker = 0;
    let mut seen = HashSet::new();
    seen.insert((pos[0].i, pos[0].j));

    for c in steps.chars() {
        match c {
            '>' => {
                pos[cur_walker].j += 1;
            }
            '<' => {
                pos[cur_walker].j -= 1;
            }
            '^' => {
                pos[cur_walker].i -= 1;
            }
            'v' => {
                pos[cur_walker].i += 1;
            }
            _ => panic!("Invalid direction {}", c),
        }

        seen.insert((pos[cur_walker].i, pos[cur_walker].j));

        cur_walker = (cur_walker + 1) % pos.len();
    }

    seen
}

fn main() -> Result<()> {
    let steps = read_to_string("data/day03/input")?;

    println!(
        "Part 1: visited {} houses",
        walk(vec![(0, 0).into()], &steps).len()
    );

    println!(
        "Part 2: visited {} houses",
        walk(vec![(0, 0).into(), (0, 0).into()], &steps).len()
    );
    Ok(())
}
