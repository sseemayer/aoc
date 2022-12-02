use std::collections::HashMap;

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
struct State {
    recipes: Vec<u8>,
    elves: Vec<usize>,
}

impl State {
    fn new() -> Self {
        Self {
            recipes: vec![3, 7],
            elves: vec![0, 1],
        }
    }

    fn step(&mut self) -> Result<()> {
        let scores: Vec<u8> = self
            .elves
            .iter()
            .filter_map(|i| self.recipes.get(*i).map(|s| *s))
            .collect();

        let score_sum = scores.iter().sum();

        if score_sum >= 10 {
            let a = score_sum % 10;
            let b = (score_sum - a) / 10;
            self.recipes.push(b);
            self.recipes.push(a);
        } else {
            self.recipes.push(score_sum);
        }

        for (e, score) in self.elves.iter_mut().zip(scores.into_iter()) {
            *e = (*e + score as usize + 1) % self.recipes.len();
        }

        Ok(())
    }

    fn score_after_steps(&mut self, n: usize) -> Result<String> {
        while self.recipes.len() < n + 10 {
            self.step()?;
        }

        let mut out = String::new();
        for r in self.recipes[n..n + 10].iter() {
            out.extend(r.to_string().chars());
        }

        Ok(out)
    }

    fn find_digits(&mut self, digits: &[u8]) -> Result<usize> {
        loop {
            if self.recipes.len() > digits.len() {
                if &self.recipes[self.recipes.len() - digits.len()..] == digits {
                    break Ok(self.recipes.len() - digits.len());
                }

                if &self.recipes[self.recipes.len() - digits.len() - 1..self.recipes.len() - 1]
                    == digits
                {
                    break Ok(self.recipes.len() - digits.len() - 1);
                }
            }
            self.step()?;
        }
    }
}

const BRACKETS: [(char, char); 2] = [('(', ')'), ('[', ']')];

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pos_to_bracket: HashMap<usize, (char, char)> = self
            .elves
            .iter()
            .enumerate()
            .map(|(i, pos)| (*pos, BRACKETS[i]))
            .collect();

        for (i, s) in self.recipes.iter().enumerate() {
            let (a, b) = pos_to_bracket.get(&i).unwrap_or(&(' ', ' '));

            write!(f, "{}{}{} ", a, s, b)?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let input = 286051;
    let input_digits = vec![2, 8, 6, 0, 5, 1];

    println!("Part 1: {}", State::new().score_after_steps(input)?);
    println!("Part 2: {}", State::new().find_digits(&input_digits[..])?);

    Ok(())
}
