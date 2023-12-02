use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Context, Result};

fn parse_components(f: &str) -> Result<HashSet<(u8, u8)>> {
    let mut out = HashSet::new();

    let reader = BufReader::new(File::open(f).context("Open file")?);

    for line in reader.lines() {
        let line = line?;

        let (a, b) = line
            .split_once("/")
            .ok_or_else(|| anyhow!("Bad line: '{}'", line))?;

        let a = a.parse().context("parse first component")?;
        let b = b.parse().context("parse second component")?;

        out.insert((a, b));
    }

    Ok(out)
}

fn find_strongest(
    path: Vec<u8>,
    candidates: &HashSet<(u8, u8)>,
    current_strength: usize,
) -> (usize, Vec<u8>, Vec<u8>) {
    let head = *path.last().expect("Non-empty path");

    let mut best_score = current_strength;
    let mut best_path = path.clone();
    let mut longest_path = path.clone();

    for &(a, b) in candidates {
        let new_head = if a == head {
            b
        } else if b == head {
            a
        } else {
            // skip invalid candidates
            continue;
        };

        let mut new_path = path.clone();
        new_path.push(new_head);

        let mut new_candidates = candidates.clone();
        new_candidates.remove(&(a, b));

        let new_current_strength = current_strength + (a as usize) + (b as usize);
        // println!(
        //     "{}->{} score={} // {:?}",
        //     head, new_head, new_current_strength, new_candidates
        // );

        let (ret_score, ret_spath, ret_lpath) =
            find_strongest(new_path, &new_candidates, new_current_strength);

        if ret_score > best_score {
            best_score = ret_score;
            best_path = ret_spath;
            // println!("best is now {} with {:?}", best_score, best_path);
        }

        if ret_lpath.len() > longest_path.len()
            || (ret_lpath.len() == longest_path.len()
                && score_path(&ret_lpath) > score_path(&longest_path))
        {
            longest_path = ret_lpath;
        }
    }

    (best_score, best_path, longest_path)
}

fn score_path(path: &Vec<u8>) -> usize {
    path.into_iter().map(|v| *v as usize).sum::<usize>() * 2 - *path.last().unwrap_or(&0) as usize
}

fn main() -> Result<()> {
    let components = parse_components("data/day24/input")?;

    let (bs, bp, lp) = find_strongest(vec![0], &components, 0);

    println!("Part 1: {} {:?}", bs, bp);
    println!("Part 2: {} {:?}", score_path(&lp), lp);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        Ok(())
    }
}
