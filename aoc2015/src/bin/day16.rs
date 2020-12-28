use aoc2015::util::read_to_parsed_lines;
use aoc2015::{format_err, Error, Result};

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Sue {
    knowledge: HashMap<String, i64>,
}

impl std::str::FromStr for Sue {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        // Sue 1: cars: 9, akitas: 3, goldfish: 0
        // Sue 2: akitas: 9, children: 3, samoyeds: 9
        // Sue 3: trees: 6, cars: 6, children: 4

        let mut tokens: Vec<_> = s.split(" ").collect();
        tokens.remove(0); // Sue
        tokens.remove(0); // #:

        let mut knowledge = HashMap::new();
        while !tokens.is_empty() {
            let key = tokens.remove(0).replace(":", "").to_owned();
            let value: i64 = tokens.remove(0).replace(",", "").parse()?;

            knowledge.insert(key, value);
        }

        Ok(Sue { knowledge })
    }
}

impl Sue {
    fn matches(&self, description: &HashMap<String, i64>) -> bool {
        for (k, v) in description.iter() {
            if let Some(vs) = self.knowledge.get(k) {
                if v != vs {
                    // the current Sue has a different number of items than in the description
                    return false;
                }
            }
        }

        for (ks, vs) in self.knowledge.iter() {
            if description.get(ks) == None {
                // the current Sue has something that is not in the description
                return false;
            }
        }

        true
    }

    fn matches_pt2(&self, description: &HashMap<String, i64>) -> bool {
        for (k, v) in description.iter() {
            if let Some(vs) = self.knowledge.get(k) {
                if k == "cats" || k == "trees" {
                    // current Sue has more than the specified amount
                    if v >= vs {
                        return false;
                    }
                } else if k == "pomeranians" || k == "goldfish" {
                    // current Sue has fewer than the specified amount
                    if v <= vs {
                        return false;
                    }
                } else {
                    if v != vs {
                        // the current Sue has a different number of items than in the description
                        return false;
                    }
                }
            }
        }

        for (ks, vs) in self.knowledge.iter() {
            if description.get(ks) == None {
                // the current Sue has something that is not in the description
                println!("Filtered out -- shouldn't have {}", ks);
                return false;
            }
        }

        true
    }
}

fn main() -> Result<()> {
    let mut sues: Vec<Sue> = read_to_parsed_lines("data/day16/input", &|l| l.parse())?;

    let description: Sue = "Sue X: children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1".parse()?;

    println!("Search description: {:#?}", description.knowledge);

    println!("PART ONE");
    let filtered_sues1: Vec<_> = sues
        .iter()
        .enumerate()
        .filter(|(i, s)| s.matches(&description.knowledge))
        .collect();

    println!("Got filtered Sues: {:#?}", filtered_sues1);

    let filtered_sues2: Vec<_> = sues
        .iter()
        .enumerate()
        .filter(|(i, s)| s.matches_pt2(&description.knowledge))
        .collect();

    println!("Got filtered Sues: {:#?}", filtered_sues2);

    Ok(())
}
