use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // fwft (72) -> ktlj, cntj, xhth
    static ref RE_PROGRAM: Regex = Regex::new(r"^(\w+)\s+\((\d+)\)(?:\s+->\s+([a-z, ]+))?$").unwrap();
}

#[derive(Debug, Clone)]
struct Program {
    name: String,
    weight: usize,
    children: Vec<String>,
}

impl std::str::FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let caps = RE_PROGRAM
            .captures(s)
            .ok_or(anyhow!("Bad program: '{}'", s))?;

        let name = caps.get(1).unwrap().as_str().to_string();
        let weight = caps.get(2).unwrap().as_str();
        let weight: usize = weight.parse().context("Parse weight")?;

        let children = if let Some(c) = caps.get(3) {
            c.as_str().split(", ").map(|s| s.to_string()).collect()
        } else {
            Vec::new()
        };

        Ok(Program {
            name,
            weight,
            children,
        })
    }
}

impl Program {
    fn get_total_weight(&self, id_to_program: &HashMap<String, Program>) -> usize {
        return self.weight
            + self
                .children
                .iter()
                .map(|cid| id_to_program[cid].get_total_weight(id_to_program))
                .sum::<usize>();
    }

    fn find_unbalanced(&self, id_to_program: &HashMap<String, Program>) -> Option<(String, usize)> {
        if self.children.is_empty() {
            println!("{} is balanced (no children)", self.name);
            return None;
        }

        for cid in &self.children {
            let c = &id_to_program[cid];
            if let Some(ub) = c.find_unbalanced(id_to_program) {
                return Some(ub);
            }
        }

        let mut child_weights: Vec<(usize, &str)> = self
            .children
            .iter()
            .map(|cid| (id_to_program[cid].get_total_weight(id_to_program), &cid[..]))
            .collect();

        child_weights.sort();

        if child_weights[0].0 == child_weights[child_weights.len() - 1].0 {
            println!(
                "{} is balanced (all children have weight {})",
                self.name, child_weights[0].0
            );
            return None;
        }

        let outlier = if child_weights[0].0 == child_weights[1].0 {
            child_weights[child_weights.len() - 1].1
        } else {
            child_weights[0].1
        };

        println!("{:?}", child_weights);

        Some((outlier.to_string(), child_weights[1].0))
    }
}

fn main() -> Result<()> {
    let programs: Vec<Program> = aoc::io::read_lines("data/day07/input")?;

    let mut id_to_parent: HashMap<String, String> = HashMap::new();
    let mut id_to_program: HashMap<String, Program> = HashMap::new();

    for p in programs {
        for c in &p.children {
            id_to_parent.insert(c.clone(), p.name.clone());
        }
        id_to_program.insert(p.name.clone(), p);
    }

    let root = id_to_program
        .keys()
        .filter(|p| !id_to_parent.contains_key(&p[..]))
        .next()
        .unwrap();

    println!("Part 1: {}", root);

    if let Some((ub, target_weight)) = id_to_program[root].find_unbalanced(&id_to_program) {
        let ubp = &id_to_program[&ub];
        let total_weight = ubp.get_total_weight(&id_to_program);
        let corrected_weight = ubp.weight as i64 + (target_weight as i64 - total_weight as i64);

        println!("{:?} is unbalanced, should weigh {}", ubp, corrected_weight);
    }

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
