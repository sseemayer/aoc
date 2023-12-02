use std::{collections::HashSet, io::Write};

use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Item {
    Chip(char),
    Generator(char),

    ChipAndGenerator,
}

impl std::str::FromStr for Item {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 2 {
            return Err(anyhow!("Bad item: '{}' - wrong token number", s));
        }

        match chars[1] {
            'M' => Ok(Item::Chip(chars[0])),
            'G' => Ok(Item::Generator(chars[0])),
            _ => Err(anyhow!("Bad item: '{}' - unknown type", s)),
        }
    }
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Chip(id) => write!(f, "{}M", id),
            Item::Generator(id) => write!(f, "{}G", id),
            Item::ChipAndGenerator => write!(f, "<>"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    elevator: usize,
    floors: Vec<HashSet<Item>>,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, floor) in self.floors.iter().enumerate() {
            write!(
                f,
                "{} {}: {:?}\n",
                i,
                if self.elevator == i { "E" } else { " " },
                floor
            )?;
        }
        Ok(())
    }
}

fn would_fry(items: &HashSet<Item>) -> bool {
    for item in items {
        match item {
            Item::Chip(id) => {
                if items.contains(&Item::Generator(*id)) {
                    // chip is protected by generator
                    continue;
                }
                for other_item in items {
                    if let Item::Generator(other_id) = other_item {
                        // TODO: we might not need this if
                        if other_id != id {
                            // chip gets fried by another generator
                            return true;
                        }
                    }
                }
            }
            Item::Generator(_) => {}
            Item::ChipAndGenerator => {}
        }
    }
    false
}

impl State {
    fn score(&self) -> usize {
        self.floors
            .iter()
            .enumerate()
            .map(|(i, f)| f.len() * (i + 1) * 10)
            .sum::<usize>()
    }

    fn is_success(&self) -> bool {
        for floor in &self.floors[..self.floors.len() - 1] {
            if !floor.is_empty() {
                return false;
            }
        }
        true
    }

    fn get_neighbors(&self) -> Vec<State> {
        let mut out = Vec::new();

        // calculate valid floors that the elevator can move to
        let mut valid_destinations = Vec::new();
        if self.elevator > 0 {
            valid_destinations.push(self.elevator - 1);
        };
        if self.elevator < self.floors.len() - 1 {
            valid_destinations.push(self.elevator + 1);
        }

        for num_items in 1..=2 {
            // generate sets of items that can be taken from current floor - none, one, or two
            for moved_items in self.floors[self.elevator].iter().combinations(num_items) {
                let moved_items: HashSet<Item> = moved_items.into_iter().cloned().collect();
                for destination in &valid_destinations {
                    // take moved_items from self.elevator to destination

                    let current_floor: HashSet<Item> = self.floors[self.elevator]
                        .difference(&moved_items)
                        .cloned()
                        .collect();
                    let destination_floor: HashSet<Item> = self.floors[*destination]
                        .union(&moved_items)
                        .cloned()
                        .collect();

                    // do not perform invalid moves
                    if would_fry(&current_floor) || would_fry(&destination_floor) {
                        continue;
                    }

                    let mut new_state: State = self.clone();
                    new_state.floors[self.elevator] = current_floor;
                    new_state.floors[*destination] = destination_floor;
                    new_state.elevator = *destination;

                    out.push(new_state);
                }
            }
        }

        out
    }
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.elevator.hash(state);

        let floors: Vec<Vec<Item>> = self
            .floors
            .iter()
            .map(|items| {
                // build list of ids for which both Chip and Generator are seen
                let merge: Vec<char> = items
                    .iter()
                    .filter_map(|i| {
                        if let Item::Chip(id) = i {
                            if items.contains(&Item::Generator(*id)) {
                                return Some(*id);
                            }
                        }
                        None
                    })
                    .collect();

                let mut items = items.clone();
                for id in &merge {
                    items.remove(&Item::Chip(*id));
                    items.remove(&Item::Generator(*id));
                }

                let mut items: Vec<Item> = items.into_iter().collect();
                for _ in &merge {
                    items.push(Item::ChipAndGenerator);
                }
                items.sort();
                items
            })
            .collect();
        floors.hash(state);
    }
}

fn solve(input: &[&str]) -> Result<usize> {
    // -> Result<Vec<State>> {
    let start = State {
        elevator: 0,
        floors: input
            .iter()
            .map(|l| {
                if l.trim().is_empty() {
                    Ok(HashSet::new())
                } else {
                    l.split(",")
                        .map(|i| i.parse())
                        .collect::<Result<HashSet<Item>>>()
                }
            })
            .collect::<Result<_>>()?,
    };

    //let mut queue = vec![(0, Vec::new(), start.clone())];
    let mut queue = vec![(0, start.clone())];
    let mut seen: HashSet<State> = HashSet::new();

    let mut max_steps = 0;
    let mut best_score = 0;
    while !queue.is_empty() {
        //let (steps, path, state) = queue.remove(0);
        let (steps, state) = queue.remove(0);
        //println!("{}\n{}", steps, state);

        if max_steps < steps {
            max_steps = steps;
            print!(".");
            std::io::stdout().flush().unwrap();
        }

        for next_state in state.get_neighbors() {
            if seen.contains(&next_state) {
                continue;
            }

            let score = next_state.score();
            if score > best_score {
                best_score = score;
            }

            // dirty heuristic: don't explore very bad states
            if score < best_score - 40 {
                continue;
            }

            //let mut next_path = path.clone();
            //next_path.push(next_state.clone());

            seen.insert(next_state.clone());
            //queue.push((steps + 1, next_path, next_state));
            queue.push((steps + 1, next_state));
        }

        if state.is_success() {
            //return Ok(path);
            return Ok(steps);
        }
    }

    panic!("No solution")
}

fn main() -> Result<()> {
    //let input = vec![
    //    "HM,LM", // The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
    //    "HG",    // The second floor contains a hydrogen generator.
    //    "LG",    // The third floor contains a lithium generator.
    //    "",      // The fourth floor contains nothing relevant.
    //];

    // p plutonium
    // P promethium
    // r ruthenium
    // s strontium
    // t thulium
    let input1 = vec![
        "tG,tM,pG,sG", // The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, and a strontium generator.
        "pM,sM", // The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
        "PG,PM,rG,rM", // The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
        "",            // The fourth floor contains nothing relevant.
    ];

    if let Ok(path) = solve(&input1[..]) {
        //println!("\npart 1: solution in {} steps", path.len());
        println!("\npart 1: solution in {} steps", path);

        //for (i, step) in path.into_iter().enumerate() {
        //    println!("STEP {}:\n{}\n", i, step);
        //}
    }

    // d dilithium
    // e elerium
    // p plutonium
    // P promethium
    // r ruthenium
    // s strontium
    // t thulium
    let input2 = vec!["dG,dM,eG,eM,tG,tM,pG,sG", "pM,sM", "PG,PM,rG,rM", ""];

    if let Ok(path) = solve(&input2[..]) {
        //println!("\npart 2: solution in {} steps", path.len());
        println!("\npart 2: solution in {} steps", path);

        //for (i, step) in path.into_iter().enumerate() {
        //    println!("STEP {}:\n{}\n", i, step);
        //}
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
