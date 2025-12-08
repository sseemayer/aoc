use std::{
    collections::{BTreeSet, HashMap, HashSet},
    str::FromStr,
};

use anyhow::{Context, Error, Result, bail};
use colored::Colorize;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Box([isize; 3]);

impl FromStr for Box {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let dims: Vec<isize> = s
            .split(',')
            .map(|part| part.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()
            .context("parsing box dimensions")?;

        if dims.len() != 3 {
            bail!("expected 3 dimensions, got {}", dims.len());
        }

        Ok(Box([dims[0], dims[1], dims[2]]))
    }
}

impl Box {
    fn square_distance(a: &Box, b: &Box) -> isize {
        let dx = a.0[0] - b.0[0];
        let dy = a.0[1] - b.0[1];
        let dz = a.0[2] - b.0[2];
        dx * dx + dy * dy + dz * dz
    }
}

fn main() -> Result<()> {
    //let boxes: Vec<Box> = aoc::io::read_lines("data/day08/example")?;
    let boxes: Vec<Box> = aoc::io::read_lines((2025, 8))?;

    let mut distances: BTreeSet<(isize, usize, usize)> = BTreeSet::new();
    for i in 0..boxes.len() {
        for j in 0..i {
            let d = Box::square_distance(&boxes[i], &boxes[j]);
            distances.insert((d, i, j));
        }
    }

    let mut not_connected: HashSet<usize> = (0..boxes.len()).collect();
    let mut member_to_circuit: HashMap<usize, usize> = HashMap::new();
    let mut circuit_to_members: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut circuit_count = 0;
    let mut steps = 0;
    while let Some((_d, i, j)) = distances.pop_first() {
        let ci = member_to_circuit.get(&i).cloned();
        let cj = member_to_circuit.get(&j).cloned();

        match (ci, cj) {
            // Neither member is in a circuit yet; create a new one
            (None, None) => {
                not_connected.remove(&i);
                not_connected.remove(&j);
                member_to_circuit.insert(i, circuit_count);
                member_to_circuit.insert(j, circuit_count);
                circuit_to_members.insert(circuit_count, [i, j].into_iter().collect());
                circuit_count += 1;
            }
            // i is in circuit ci, j is not; add j to ci
            (Some(ci), None) => {
                not_connected.remove(&j);
                member_to_circuit.insert(j, ci);
                circuit_to_members.get_mut(&ci).unwrap().insert(j);
            }
            // j is in circuit cj, i is not; add i to cj
            (None, Some(cj)) => {
                not_connected.remove(&i);
                member_to_circuit.insert(i, cj);
                circuit_to_members.get_mut(&cj).unwrap().insert(i);
            }
            // Both members are in different circuits; merge cj into ci
            (Some(ci), Some(cj)) if ci != cj => {
                let members_j = circuit_to_members.remove(&cj).unwrap();
                for member in &members_j {
                    member_to_circuit.insert(*member, ci);
                }
                circuit_to_members.get_mut(&ci).unwrap().extend(members_j);
            }
            _ => {}
        }

        steps += 1;
        if steps == 1000 {
            let mut circuit_sizes: Vec<usize> = circuit_to_members
                .values()
                .map(|members| members.len())
                .collect();

            circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));

            let part1 = circuit_sizes[0..3].iter().product::<usize>();

            println!("{} {}", "Part 1:".bold().green(), part1);
        }

        if not_connected.is_empty() {
            let bi = &boxes[i];
            let bj = &boxes[j];

            let part2 = bi.0[0] * bj.0[0];

            println!("{} {}", "Part 2:".bold().green(), part2);
            break;
        }
    }

    Ok(())
}
