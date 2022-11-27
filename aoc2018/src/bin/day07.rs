use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{bail, Result};

fn parse_steps(f: &str) -> Result<HashMap<String, Vec<String>>> {
    let mut out: HashMap<String, Vec<String>> = HashMap::new();
    let mut seen_tasks = HashSet::new();

    for line in BufReader::new(File::open(f)?).lines() {
        let line = line?;

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() != 10 {
            bail!("Bad line with wrong number of tokens: '{}'", line);
        }

        let prerequisite = tokens[1].to_string();
        let task = tokens[7].to_string();

        seen_tasks.insert(prerequisite.clone());
        seen_tasks.insert(task.clone());

        out.entry(task).or_default().push(prerequisite);
    }

    for t in seen_tasks {
        out.entry(t).or_default();
    }

    Ok(out)
}

#[derive(Debug)]
struct State1 {
    rules: HashMap<String, Vec<String>>,
    finished: HashSet<String>,
}

impl State1 {
    fn new(rules: HashMap<String, Vec<String>>) -> Self {
        Self {
            rules,
            finished: HashSet::new(),
        }
    }

    fn step(&mut self) -> Option<&str> {
        let mut next_rule = None;

        for (candidate, prerequisites) in self.rules.iter() {
            if self.finished.contains(candidate) {
                // do not start rules that are already finished
                continue;
            }

            if prerequisites.iter().any(|p| !self.finished.contains(p)) {
                // do not start rules for which the prerequisites are not met
                continue;
            }

            if let Some(nr) = next_rule {
                if nr < candidate.as_str() {
                    continue;
                }
            }

            next_rule = Some(candidate.as_str())
        }

        if let Some(nr) = next_rule {
            self.finished.insert(nr.to_string());
        }

        next_rule
    }

    fn solve(mut self) -> String {
        let mut out = String::new();

        while let Some(nr) = self.step() {
            out.extend(nr.chars());
        }

        out
    }
}

#[derive(Debug)]
struct State2 {
    /// the static rulebase
    rules: HashMap<String, Vec<String>>,

    /// what the various workers are busy with
    workers: Vec<Option<String>>,

    /// the status of ongoing work
    ongoing: HashMap<String, (i64, usize)>,

    /// work items that are finished
    finished: HashSet<String>,

    /// time passed
    time: usize,
}

impl State2 {
    fn new(rules: HashMap<String, Vec<String>>, n_workers: usize) -> Self {
        let workers: Vec<Option<String>> = (0..n_workers).map(|_| None).collect();

        Self {
            rules,
            workers,
            ongoing: HashMap::new(),
            finished: HashSet::new(),
            time: 0,
        }
    }

    fn get_free_worker(&self) -> Option<usize> {
        self.workers
            .iter()
            .enumerate()
            .find_map(|(i, t)| if t.is_none() { Some(i) } else { None })
    }

    fn is_task_startable(&self, task: &str) -> bool {
        if self.finished.contains(task) {
            // cannot start a task that is already finished
            return false;
        }
        if self.ongoing.contains_key(task) {
            // cannot start a task that we are already working on
            return false;
        }

        if !self
            .rules
            .get(task)
            .expect("Rules for task")
            .iter()
            .all(|prerequisite| self.finished.contains(prerequisite))
        {
            // cannot start a task if not all prerequisites are met
            return false;
        }

        true
    }

    fn step(&mut self) {
        // println!("t={}", self.time);

        self.time += 1;

        let mut finished_tasks = Vec::new();
        for (task, (time_left, worker)) in self.ongoing.iter_mut() {
            *time_left -= 1;

            if *time_left <= 0 {
                finished_tasks.push((task.to_string(), *worker));
            }
        }

        finished_tasks.sort();
        for (task, worker) in finished_tasks {
            // println!("Worker {} finishes {}", worker, task);

            self.ongoing.remove(&task);
            self.finished.insert(task);
            self.workers[worker] = None;
        }

        let mut available_tasks: Vec<&str> = self
            .rules
            .keys()
            .filter(|candidate| self.is_task_startable(candidate))
            .map(|c| c.as_str())
            .collect();

        available_tasks.sort_by_key(|k| Reverse(*k));

        while let Some(worker) = self.get_free_worker() {
            if let Some(nr) = available_tasks.pop() {
                let task = nr.chars().next().expect("Single-letter tasks");
                let duration = ((task as u8) - ('A' as u8) + 61) as i64;

                self.workers[worker] = Some(nr.to_string());
                self.ongoing.insert(nr.to_string(), (duration, worker));

                // println!("Worker {} starts {} for {}", worker, task, duration);
            } else {
                break;
            }
        }
    }

    fn solve(mut self) -> usize {
        while self.finished.len() < self.rules.len() {
            self.step();
        }

        self.time
    }
}

fn main() -> Result<()> {
    let rules = parse_steps("data/day07/input")?;

    let state1 = State1::new(rules.clone());
    println!("Part 1: {}", state1.solve());

    let state2 = State2::new(rules.clone(), 5);
    println!("Part 2: {}", state2.solve() - 1);

    Ok(())
}
