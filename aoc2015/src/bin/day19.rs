use aoc2015::util::read_to_lines;
use aoc2015::{format_err, Error, Result};
use std::collections::HashSet;

struct Reaction {
    educt: String,
    product: String,
}

impl std::fmt::Debug for Reaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} => {}", self.educt, self.product)
    }
}

impl std::str::FromStr for Reaction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut tokens: Vec<_> = s.split(" => ").collect();

        if tokens.len() != 2 {
            return Err(format_err!("Invalid number of tokens: '{}'", s));
        }

        let educt = tokens.remove(0).to_owned();
        let product = tokens.remove(0).to_owned();

        Ok(Reaction { educt, product })
    }
}

impl Reaction {
    fn apply_to(&self, state: &str) -> HashSet<String> {
        let mut products = HashSet::new();

        if state.len() < self.educt.len() {
            return products;
        }

        for ofs in 0..(state.len() - self.educt.len() + 1) {
            if state[ofs..ofs + self.educt.len()] == self.educt {
                let mut product = state[0..ofs].to_owned();
                product += &self.product;
                product += &state[ofs + self.educt.len()..state.len()];

                products.insert(product);
            }
        }

        products
    }

    fn reverse_apply(&self, state: &str) -> HashSet<String> {
        let mut educts = HashSet::new();

        if state.len() < self.product.len() {
            return educts;
        }

        for ofs in 0..(state.len() - self.product.len() + 1) {
            if state[ofs..ofs + self.product.len()] == self.product {
                let mut educt = state[0..ofs].to_owned();
                educt += &self.educt;
                educt += &state[ofs + self.product.len()..state.len()];

                educts.insert(educt);
            }
        }

        educts
    }
}

fn parse() -> Result<(Vec<Reaction>, String)> {
    let lines = read_to_lines("data/day19/input")?;

    let mut rxns = Vec::new();
    let mut state = None;

    for l in lines {
        if l.trim().len() == 0 {
            continue;
        }

        if let Ok(rxn) = l.parse() {
            rxns.push(rxn)
        } else {
            state = Some(l.trim().to_owned());
        }
    }

    if let Some(state) = state {
        Ok((rxns, state))
    } else {
        Err(format_err!("Did not find an initial state!"))
    }
}

fn retrosynthesis(rxns: &Vec<Reaction>, initial: &str, target: &str) -> Option<usize> {
    let mut queue = vec![(0, initial.to_owned())];
    let mut seen = HashSet::new();

    let mut most_steps = 0;

    while !queue.is_empty() {
        queue.sort_by_key(|k| k.1.len());

        let (steps, cur) = queue.remove(0);

        if steps > most_steps {
            println!(
                "Searching {}-step reactions, seen {} products so far",
                steps,
                seen.len()
            );
            most_steps = steps;
        }

        println!("{}", cur);

        if cur == target {
            return Some(steps);
        }

        let mut eds = HashSet::new();
        for rxn in rxns {
            for ed in rxn.reverse_apply(&cur) {
                eds.insert(ed);
            }
        }

        for ed in eds {
            if !seen.contains(&ed) {
                seen.insert(ed.clone());
                queue.push((steps + 1, ed));
            }
        }
    }

    None
}

fn main() -> Result<()> {
    let (rxns, state) = parse()?;

    println!("State: {}\nReactions:\n{:#?}", state, rxns);

    let mut all_products = HashSet::new();

    for rxn in &rxns {
        for prd in rxn.apply_to(&state) {
            all_products.insert(prd);
        }
    }

    println!("PART ONE: Got {} products in one step", all_products.len());

    if let Some(n) = retrosynthesis(&rxns, &state, "e") {
        println!("PART TWO: Made target in {} steps", n);
    }

    Ok(())
}
