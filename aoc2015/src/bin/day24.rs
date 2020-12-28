use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use aoc2015::Result;

fn solve(parcels: &[usize], target_weight: usize) -> Option<(usize, usize)> {
    let mut solution: Option<(usize, usize)> = None;
    for group1_length in 1..parcels.len() {
        for permutation in parcels.iter().combinations(group1_length) {
            let group1_weight = permutation.iter().map(|n| **n).sum::<usize>();

            if group1_weight != target_weight {
                continue;
            }

            let group1 = &permutation[..group1_length];

            let qe = group1.iter().map(|n| **n).product::<usize>();
            let n_parcels = group1.len();

            if let Some(sol) = solution {
                if sol.0 > n_parcels || sol.1 > qe {
                    solution = Some((n_parcels, qe))
                }
            } else {
                solution = Some((n_parcels, qe))
            }
        }

        if solution.is_some() {
            break;
        }
    }

    solution
}

fn main() -> Result<()> {
    let br = BufReader::new(File::open("data/day24/input")?);
    let parcels: Vec<usize> = br.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    // let parcels: Vec<usize> = "1 2 3 4 5 7 8 9 10 11"
    //     .split(" ")
    //     .map(|l| l.parse().unwrap())
    //     .collect();

    println!("Parcels: {:?}", parcels);

    let total_weight = parcels.iter().sum::<usize>();

    println!("Part 1: {:?}", solve(&parcels[..], total_weight / 3));
    println!("Part 2: {:?}", solve(&parcels[..], total_weight / 4));

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
}
