use std::{
    collections::HashMap,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Result};

fn parse_state(state: &str) -> Vec<bool> {
    state.trim().chars().map(|c| c == '#').collect()
}

#[derive(Debug, Clone)]
struct State {
    state: HashMap<i32, bool>,
    rules: HashMap<Vec<bool>, bool>,

    index_sum: i32,
}

fn vec_to_hash(state: &Vec<bool>) -> HashMap<i32, bool> {
    state
        .iter()
        .enumerate()
        .map(|(i, c)| (i as i32, *c))
        .collect()
}

impl State {
    fn parse(path: &str) -> Result<Self> {
        let mut lines = BufReader::new(File::open(path)?).lines();

        let first_line = lines.next().ok_or_else(|| anyhow!("read first line"))??;
        let state = vec_to_hash(&parse_state(
            first_line.trim_start_matches("initial state: "),
        ));

        // ignore second line
        lines.next().ok_or_else(|| anyhow!("read second line"))??;

        let mut rules = HashMap::new();
        for line in lines {
            let line = line?;

            let (pattern, result) = line
                .split_once(" => ")
                .ok_or_else(|| anyhow!("Split rule"))?;

            let pattern = parse_state(pattern);
            let result = result == "#";

            rules.insert(pattern, result);
        }

        let index_sum = state
            .iter()
            .filter_map(|(k, v)| if *v { Some(k) } else { None })
            .sum();

        Ok(Self {
            state,
            rules,
            index_sum,
        })
    }

    fn get_window(&self, offset: i32) -> Vec<bool> {
        (offset..offset + 5)
            .map(|i| *self.state.get(&i).unwrap_or(&false))
            .collect()
    }

    fn step(&mut self) {
        let (min_i, max_i) = self.extent();

        let new_state = (min_i - 1..=max_i + 1)
            .filter_map(|i| {
                // calculate state for pot i
                let window = self.get_window(i - 2);

                let pot = *self.rules.get(&window).unwrap_or(&false);

                if pot {
                    Some((i, pot))
                } else {
                    None
                }
            })
            .collect();

        self.state = new_state;
        self.index_sum += self.count()
    }

    fn count(&self) -> i32 {
        self.state
            .iter()
            .filter_map(|(k, v)| if *v { Some(k) } else { None })
            .sum()
    }

    fn extent(&self) -> (i32, i32) {
        let min_i = *self.state.keys().min().unwrap_or(&0);
        let max_i = *self.state.keys().max().unwrap_or(&0);

        (min_i, max_i)
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_i, max_i) = self.extent();

        // write!(f, "{:5} ", min_i)?;

        for i in min_i..=max_i {
            if *self.state.get(&i).unwrap_or(&false) {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }

        write!(f, "\n")
    }
}

fn part1() -> Result<()> {
    let mut state = State::parse("data/day12/input")?;
    let mut last_score = 0;

    print!("Step   0: {}", state);
    for i in 1..=20 {
        state.step();
        let score = state.count();
        print!(
            "Step {:3}: {}score={}, delta={}\n",
            i,
            state,
            score,
            score - last_score
        );
        last_score = score;
    }

    println!("Part 1: {}", state.count());

    Ok(())
}

fn part2() -> Result<()> {
    let mut state = State::parse("data/day12/input")?;
    let mut seen = HashMap::new();
    let mut step = 0;

    let (_last_step, _last_state) = loop {
        state.step();
        step += 1;

        let hash_key = format!("{}", state);
        if let Some(s) = seen.get(&hash_key) {
            break s;
        }

        seen.insert(hash_key, (step, state.clone()));
    };

    let score_delta = {
        let mut next_state = state.clone();
        next_state.step();
        next_state.count() - state.count()
    };

    println!(
        "Pattern will repeat starting from step {}, increasing score by {} each step",
        step, score_delta
    );

    let score = state.count();

    let final_score = score as usize + (score_delta as usize * (50_000_000_000usize - step));

    println!("Part 2: {}", final_score);

    Ok(())
}

fn main() -> Result<()> {
    part1()?;
    part2()?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bit_stuff() {
        let state = State {
            state: vec_to_hash(&vec![true, false, true, true, false, false, true]),
            rules: HashMap::new(),
            index_sum: 0,
        };

        assert_eq!(state.get_window(0), vec![true, false, true, true, false]);
        assert_eq!(state.get_window(1), vec![false, true, true, false, false]);
        assert_eq!(state.get_window(2), vec![true, true, false, false, true]);
    }
}
