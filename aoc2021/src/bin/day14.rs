use anyhow::Result;
use aoc2021::io::read_lines_bufreader;
use thiserror::Error;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Rule {
    l1: char,
    l2: char,
    insert: char,
}

#[derive(Error, Debug)]
enum ReadRuleError {
    #[error("Bad line: '{}'", .0)]
    BadLine(String),
}

impl std::str::FromStr for Rule {
    type Err = ReadRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s
            .split_once(" -> ")
            .ok_or(ReadRuleError::BadLine(s.to_string()))?;

        let l: Vec<char> = l.trim().chars().collect();
        let r: Vec<char> = r.trim().chars().collect();

        if l.len() != 2 || r.len() != 1 {
            return Err(ReadRuleError::BadLine(s.to_string()));
        }

        let l1 = l[0];
        let l2 = l[1];

        let insert = r[0];

        Ok(Rule { l1, l2, insert })
    }
}

#[derive(Debug)]
struct State {
    pair_counts: HashMap<(char, char), i64>,
    single_counts: HashMap<char, i64>,
}

impl State {
    fn from(state: &str) -> Self {
        let state: Vec<char> = state.trim().chars().collect();

        let mut pair_counts = HashMap::new();
        let mut single_counts = HashMap::new();

        for i in 0..(state.len() - 1) {
            let a = state[i];
            let b = state[i + 1];

            *pair_counts.entry((a, b)).or_default() += 1;
            *single_counts.entry(a).or_default() += 1;
        }

        *single_counts.entry(state[state.len() - 1]).or_default() += 1;

        Self {
            pair_counts,
            single_counts,
        }
    }

    fn score(&self) -> i64 {
        let min = *self.single_counts.values().min().unwrap_or(&0);
        let max = *self.single_counts.values().max().unwrap_or(&0);

        max - min
    }

    fn step(&mut self, rules: &HashMap<(char, char), char>) {
        let mut added = Vec::new();
        for (&(a, b), &n) in self.pair_counts.iter() {
            if let Some(&c) = rules.get(&(a, b)) {
                added.push(((a, c), n));
                added.push(((c, b), n));
                added.push(((a, b), -n));

                *self.single_counts.entry(c).or_default() += n;
            }
        }

        for ((a, b), n) in added {
            *self.pair_counts.entry((a, b)).or_default() += n;
        }
    }
}

fn main() -> Result<()> {
    let mut reader = BufReader::new(File::open("data/day14/input")?);

    let mut base = String::new();
    reader.read_line(&mut base)?;

    // ignore next line
    reader.read_line(&mut String::new())?;

    let rules: Vec<Rule> = read_lines_bufreader(reader)?;
    let rules: HashMap<(char, char), char> = rules
        .into_iter()
        .map(|r| ((r.l1, r.l2), r.insert))
        .collect();

    let mut state = State::from(&base[..]);

    for i in 0..=40 {
        println!("step {}: {} {:?}", i, state.score(), state.single_counts);

        state.step(&rules);
    }

    Ok(())
}
