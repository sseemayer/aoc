use std::{collections::VecDeque, str::FromStr};

use anyhow::{Context, Error, Result, anyhow};
use colored::Colorize;

use good_lp::{Expression, Solution, SolverModel, solvers::scip::scip, variable, variables};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_MACHINE: Regex =
        Regex::new(r"\[([.#]+)\]\s*((?:\([0-9,]+\)\s*)+)\s*\{([0-9,]+)\}").unwrap();
}

#[derive(Debug)]
struct Machine {
    target_lamps: usize,

    switches: Vec<usize>,
    target_joltage: Vec<usize>,
}

impl FromStr for Machine {
    type Err = Error;

    /// Parses a machine line such as [..#..] (1,2,3,4) (0,1,2) {1,2}
    fn from_str(s: &str) -> Result<Self> {
        let m = RE_MACHINE
            .captures(s)
            .with_context(|| format!("Could not parse machine line: {}", s))?;

        // read target lamps into a bit vector
        let target_lamps = m
            .get(1)
            .unwrap()
            .as_str()
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                '#' => 1 << i,
                '.' => 0,
                _ => unreachable!(),
            })
            .sum::<usize>();

        // read switches
        let switches = m
            .get(2)
            .unwrap()
            .as_str()
            .replace("(", "")
            .replace(")", "")
            .split_whitespace()
            .map(|s| {
                s.split(',')
                    .map(|s| Ok(1 << s.parse::<usize>()?))
                    .sum::<Result<usize, _>>()
            })
            .collect::<Result<Vec<usize>>>()?;

        // read target joltages
        let target_joltage = m
            .get(3)
            .unwrap()
            .as_str()
            .split(',')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()
            .with_context(|| format!("Could not parse target joltages in line: {}", s))?;

        Ok(Self {
            target_lamps,
            switches,
            target_joltage,
        })
    }
}

impl Machine {
    fn solve_lamps(&self) -> Option<usize> {
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));

        while let Some((steps, state)) = queue.pop_front() {
            if state == self.target_lamps {
                return Some(steps);
            }

            for &switch in &self.switches {
                let new_state = state ^ switch;
                queue.push_back((steps + 1, new_state));
            }
        }

        None
    }

    fn solve_joltage(&self) -> Result<usize> {
        let mut problem = variables! {};
        let mut variables = Vec::new();
        let mut total: Expression = Expression::from(0);
        for _ in 0..self.switches.len() {
            let var = problem.add(variable().integer().min(0));
            total += var;
            variables.push(var);
        }

        let mut model = problem.minimise(total).using(scip);

        for joltage in 0..self.target_joltage.len() {
            let mut expr = Expression::from(0);

            for (i, &switch) in self.switches.iter().enumerate() {
                if (switch & (1 << joltage)) != 0 {
                    expr += variables[i];
                }
            }

            model.add_constraint(expr.eq(self.target_joltage[joltage] as f64));
        }

        let solution = model.solve()?;

        let total_switches = variables
            .into_iter()
            .map(|v| solution.value(v))
            .sum::<f64>() as usize;

        Ok(total_switches)
    }
}

fn main() -> Result<()> {
    //let input: Vec<Machine> = aoc::io::read_lines("data/day10/example")?;
    let input: Vec<Machine> = aoc::io::read_lines((2025, 10))?;

    let mut part1 = 0;
    let mut part2 = 0;
    for machine in &input {
        part1 += machine
            .solve_lamps()
            .ok_or(anyhow!("No solution for lamps"))?;
        part2 += machine.solve_joltage()?;
    }

    println!("{} {}", "Part 1:".bold().green(), part1);
    println!("{} {}", "Part 2:".bold().green(), part2);

    Ok(())
}
