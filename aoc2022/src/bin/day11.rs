use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{bail, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test_divisible: usize,
    destinations: HashMap<bool, usize>,

    n_inspections: usize,
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?} %{} {:?} #{}",
            self.items, self.operation, self.test_divisible, self.destinations, self.n_inspections
        )
    }
}

impl Monkey {
    fn step(
        &mut self,
        allow_relaxation: bool,
        modulo: Option<usize>,
    ) -> HashMap<usize, Vec<usize>> {
        let mut thrown_items: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut items = Vec::new();
        std::mem::swap(&mut self.items, &mut items);
        for item in items {
            self.n_inspections += 1;

            let item_inspected = self.operation.apply_to(item);
            let item_relaxed = if allow_relaxation {
                item_inspected / 3
            } else {
                item_inspected
            };

            let item_modulated = if let Some(m) = modulo {
                item_relaxed % m
            } else {
                item_relaxed
            };

            let test_value = (item_modulated % self.test_divisible) == 0;
            let dest = self
                .destinations
                .get(&test_value)
                .expect("Valid destination");

            thrown_items.entry(*dest).or_default().push(item_modulated);
        }

        thrown_items
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}

impl std::str::FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_whitespace().collect::<Vec<_>>();

        if tokens.len() != 5 || &tokens[..3] != &["new", "=", "old"][..] {
            bail!("Bad operation: '{}'", s);
        }

        match (tokens[3], tokens[4]) {
            ("+", _) => Ok(Operation::Add(tokens[4].parse()?)),
            ("*", "old") => Ok(Operation::Square),
            ("*", _) => Ok(Operation::Mul(tokens[4].parse()?)),
            _ => bail!("Unknown operator: '{}'", s),
        }
    }
}

impl std::default::Default for Operation {
    fn default() -> Self {
        Operation::Square
    }
}

impl Operation {
    fn apply_to(&self, item: usize) -> usize {
        match self {
            Operation::Add(n) => item + n,
            Operation::Mul(n) => item * n,
            Operation::Square => item * item,
        }
    }
}

#[derive(Debug, Clone)]
struct Jungle {
    monkeys: Vec<Monkey>,
}

lazy_static! {
    static ref RE_MONKEY: Regex = Regex::new(r"Monkey (\d+)").unwrap();
    static ref RE_STARTING: Regex = Regex::new(r"\s*Starting items: ([0-9, ]+)").unwrap();
    static ref RE_OPERATION: Regex = Regex::new(r"\s*Operation: (.+)").unwrap();
    static ref RE_TEST: Regex = Regex::new(r"\s*Test: divisible by (\d+)").unwrap();
    static ref RE_DEST: Regex = Regex::new(r"\s*If (true|false): throw to monkey (\d+)").unwrap();
}

impl Jungle {
    fn parse(path: &str) -> Result<Self> {
        let mut monkeys = Vec::new();
        let mut monkey = None;

        for line in BufReader::new(File::open(path)?).lines() {
            let line = line?;

            if let Some(_captures) = RE_MONKEY.captures(&line) {
                if let Some(m) = monkey {
                    monkeys.push(m);
                }
                monkey = Some(Monkey::default());
            } else if let Some(captures) = RE_STARTING.captures(&line) {
                let monkey = monkey.as_mut().expect("Monkey initialized");
                monkey.items = captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .trim()
                    .split(", ")
                    .map(|i| i.parse::<usize>().context("Parse starting items"))
                    .collect::<Result<Vec<usize>>>()?;
            } else if let Some(captures) = RE_OPERATION.captures(&line) {
                let monkey = monkey.as_mut().expect("Monkey initialized");
                monkey.operation = captures.get(1).unwrap().as_str().parse()?;
            } else if let Some(captures) = RE_TEST.captures(&line) {
                let monkey = monkey.as_mut().expect("Monkey initialized");
                monkey.test_divisible = captures.get(1).unwrap().as_str().parse()?;
            } else if let Some(captures) = RE_DEST.captures(&line) {
                let monkey = monkey.as_mut().expect("Monkey initialized");
                let k: bool = captures.get(1).unwrap().as_str().parse()?;
                let v: usize = captures.get(2).unwrap().as_str().parse()?;
                monkey.destinations.insert(k, v);
            }
        }

        if let Some(m) = monkey {
            monkeys.push(m);
        }

        Ok(Self { monkeys })
    }

    fn step(&mut self, allow_relaxation: bool, modulo: Option<usize>) {
        for i in 0..self.monkeys.len() {
            let flying_items = self.monkeys[i].step(allow_relaxation, modulo);

            for (dest, items) in flying_items {
                self.monkeys[dest].items.extend(items);
            }
        }
    }

    fn monkey_business(&self) -> usize {
        let mut inspections = self
            .monkeys
            .iter()
            .map(|m| m.n_inspections)
            .collect::<Vec<_>>();
        inspections.sort_by_key(|k| std::cmp::Reverse(*k));

        inspections[..2].iter().product::<usize>()
    }
}

impl std::fmt::Display for Jungle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, m) in self.monkeys.iter().enumerate() {
            write!(f, "Monkey {}: {}\n", i, m)?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let jungle = Jungle::parse("data/day11/input")?;
    let common_multiple = jungle
        .monkeys
        .iter()
        .map(|m| m.test_divisible)
        .product::<usize>();

    let mut jungle_part1 = jungle.clone();
    for _ in 0..20 {
        jungle_part1.step(true, Some(common_multiple));
    }

    println!("Part 1: {}", jungle_part1.monkey_business());

    let mut jungle_part2 = jungle.clone();
    for _ in 0..10_000 {
        jungle_part2.step(false, Some(common_multiple));
    }

    println!("Part 2: {}", jungle_part2.monkey_business());

    Ok(())
}
