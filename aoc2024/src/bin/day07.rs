use anyhow::{anyhow, Context, Error, Result};
use itertools::{repeat_n, Itertools};

#[derive(Debug, Clone)]
struct Equation {
    result: isize,
    terms: Vec<isize>,
}

impl Equation {
    fn is_true(&self, operators: &[Operator]) -> bool {
        let mut accum = self.terms[0];

        for (&term, op) in (&self.terms[1..]).iter().zip(operators.iter()) {
            accum = op.calculate(accum, term);
        }

        //println!(
        //    "{} =?= {} : {:?} {:?}",
        //    self.result, accum, self.terms, operators
        //);

        accum == self.result
    }

    fn solve(&self, operators: &[Operator]) -> Option<Vec<Operator>> {
        let solution = repeat_n(operators.iter().cloned(), self.terms.len() - 1)
            .multi_cartesian_product()
            .find(|operators| self.is_true(operators))
            .map(|v| v.to_owned());

        // if let Some(sol) = &solution {
        //     print!("{} = {}", self.result, self.terms[0]);

        //     for (term, op) in (&self.terms[1..]).iter().zip(sol.iter()) {
        //         print!(" {} {}", op, term);
        //     }

        //     println!();
        // }

        solution
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Cat,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Mul => write!(f, "*"),
            Operator::Cat => write!(f, "||"),
        }
    }
}

const OPERATORS_1: [Operator; 2] = [Operator::Add, Operator::Mul];
const OPERATORS_2: [Operator; 3] = [Operator::Add, Operator::Mul, Operator::Cat];

impl Operator {
    fn calculate(&self, lhs: isize, rhs: isize) -> isize {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
            Operator::Cat => lhs * 10isize.pow(rhs.ilog10() + 1) + rhs,
        }
    }
}

impl std::str::FromStr for Equation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (result, terms) = s.split_once(":").ok_or(anyhow!("expected :"))?;

        let result = result.parse().context("parse result")?;
        let terms: Vec<isize> = terms
            .split_whitespace()
            .map(|t| t.parse().context("parse term"))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { result, terms })
    }
}

fn solve(equations: &[Equation], operators: &[Operator]) -> Result<isize> {
    let sum: isize = equations
        .iter()
        .filter(|eq| eq.solve(operators).is_some())
        .map(|eq| eq.result)
        .sum();

    Ok(sum)
}

fn main() -> Result<()> {
    let equations: Vec<Equation> = aoc::io::read_lines((2024, 7))?;

    //let equations: Vec<Equation> = aoc::io::read_lines("data/day07/example")?;

    println!("Part 1: {}", solve(&equations[..], &OPERATORS_1[..])?);
    println!("Part 2: {}", solve(&equations[..], &OPERATORS_2[..])?);
    Ok(())
}
