use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};
use colored::Colorize;

#[derive(Debug, Clone)]
struct State {
    /// input numbers in order they were read -- never changes
    numbers: Vec<i64>,

    /// current positions of numbers. keys are indices into numbers,
    /// and values are current positions
    positions: HashMap<usize, usize>,

    /// index into numbers of next place to change
    pos: usize,
}

impl State {
    fn new(numbers: Vec<i64>) -> Self {
        let positions = numbers.iter().enumerate().map(|(i, _n)| (i, i)).collect();

        Self {
            numbers,
            positions,
            pos: 0,
        }
    }

    fn parse(path: &str) -> Result<Self> {
        let numbers = BufReader::new(File::open(path)?)
            .lines()
            .map(|line| line?.parse().context("Parse number"))
            .collect::<Result<_>>()?;

        Ok(Self::new(numbers))
    }

    fn step(&mut self) {
        let nu = self.numbers.len();
        let n = nu as i64;

        // what number to move -- by how far
        let v = self.numbers[self.pos];

        // where n is currently
        let j = self.positions[&self.pos];

        // new destination
        //
        // A  B  C  D  E
        //-2 -1  0  1  2
        //
        let n = (((j as i64 + v) % (n - 1) + (n - 1)) % (n - 1)) as usize;

        // adjust positions between j and new_j
        if j < n {
            //      j   n
            //    01234567
            //      vvvvV
            //    01345627
            for k in self.positions.values_mut() {
                if *k > j && *k <= n {
                    *k = (*k + nu - 1) % nu;
                }
            }
        } else if j > n {
            //      n   j
            //    01234567
            //      Vvvvv
            //    01623457
            for k in self.positions.values_mut() {
                if *k >= n && *k < j {
                    *k = (*k + 1) % nu;
                }
            }
        }

        self.positions.insert(self.pos, n);
        self.pos = (self.pos + 1) % nu;
    }

    fn mix(&mut self) {
        for _ in 0..self.numbers.len() {
            self.step();
        }
    }

    fn coords(&self) -> i64 {
        let mut positions = self
            .positions
            .iter()
            .map(|(i, j)| (j, i, self.numbers[*i]))
            .collect::<Vec<_>>();

        positions.sort();

        let zero_pos = positions
            .iter()
            .find(|(_i, _j, n)| *n == 0)
            .expect("zero in data")
            .0;

        let n = positions.len();

        [1000, 2000, 3000]
            .iter()
            .map(|ofs| positions[(zero_pos + ofs) % n].2)
            .sum::<i64>()
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut positions = self
            .positions
            .iter()
            .map(|(i, j)| (j, i, self.numbers[*i]))
            .collect::<Vec<_>>();

        positions.sort();

        for (_i, j, n) in &positions {
            if **j == self.pos {
                write!(f, "{},", format!("{:3}", n).green())?;
            } else {
                write!(f, "{:3},", n)?;
            }
        }

        // write!(f, "\n        ")?;

        // for (i, _j, _n) in &positions {
        //     write!(f, "{:3},", i)?;
        // }

        Ok(())
    }
}

fn main() -> Result<()> {
    let state = State::parse("data/day20/input")?;

    {
        let mut state1 = state.clone();
        state1.mix();
        println!("Part 1: {}", state1.coords());
    }

    {
        let mut state2 = state.clone();
        state2.numbers.iter_mut().for_each(|n| *n *= 811589153);
        for _ in 1..=10 {
            state2.mix();
        }
        println!("Part 2: {}", state2.coords());
    }

    Ok(())
}
