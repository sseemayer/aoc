use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Context, Result};
use num::rational::Ratio;

type N = Ratio<i64>;

#[derive(Debug, Clone)]
enum Value {
    Const(N),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl std::str::FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once(" + ") {
            Ok(Value::Add(a.to_string(), b.to_string()))
        } else if let Some((a, b)) = s.split_once(" - ") {
            Ok(Value::Sub(a.to_string(), b.to_string()))
        } else if let Some((a, b)) = s.split_once(" * ") {
            Ok(Value::Mul(a.to_string(), b.to_string()))
        } else if let Some((a, b)) = s.split_once(" / ") {
            Ok(Value::Div(a.to_string(), b.to_string()))
        } else {
            let n: N = s.parse()?;
            Ok(Value::Const(n))
        }
    }
}

#[derive(Debug, Clone)]
enum Term {
    Human,
    Const(N),
    Add(Box<Term>, Box<Term>),
    Sub(Box<Term>, Box<Term>),
    Mul(Box<Term>, Box<Term>),
    Div(Box<Term>, Box<Term>),
}

impl Term {
    fn get_value(&self) -> Option<N> {
        match self {
            Term::Human => None,
            Term::Const(c) => Some(*c),
            Term::Add(a, b) => Some(a.get_value()? + b.get_value()?),
            Term::Sub(a, b) => Some(a.get_value()? - b.get_value()?),
            Term::Mul(a, b) => Some(a.get_value()? * b.get_value()?),
            Term::Div(a, b) => Some(a.get_value()? / b.get_value()?),
        }
    }

    fn simplify(self) -> Self {
        if let Some(v) = self.get_value() {
            return Term::Const(v);
        }

        // (x + 1) * 2 => 2 * x + 1*2
        // (x * 3) + 4

        match self.clone() {
            Term::Mul(a, b) => match (*a, *b) {
                (Term::Const(c), Term::Add(e, f)) | (Term::Add(e, f), Term::Const(c)) => {
                    ((Term::Const(c) * *e) + (Term::Const(c) * *f)).simplify()
                }
                (Term::Const(c), Term::Sub(e, f)) | (Term::Sub(e, f), Term::Const(c)) => {
                    ((Term::Const(c) * *e) - (Term::Const(c) * *f)).simplify()
                }
                (Term::Const(c), Term::Mul(e, f)) | (Term::Mul(e, f), Term::Const(c)) => {
                    if let Term::Const(g) = *e {
                        (Term::Const(c * g) * *f).simplify()
                    } else if let Term::Const(h) = *f {
                        (*e * Term::Const(c * h)).simplify()
                    } else {
                        self
                    }
                }
                (Term::Const(c), Term::Div(e, f)) | (Term::Div(e, f), Term::Const(c)) => {
                    if let Term::Const(g) = *e {
                        (Term::Const(c * g) / *f).simplify()
                    } else if let Term::Const(h) = *f {
                        (*e * Term::Const(c / h)).simplify()
                    } else {
                        self
                    }
                }
                _ => self,
            },
            Term::Div(a, b) => match (*a, *b) {
                (Term::Add(e, f), Term::Const(c)) => {
                    ((*e / Term::Const(c)) + (*f / Term::Const(c))).simplify()
                }
                (Term::Sub(e, f), Term::Const(c)) => {
                    ((*e / Term::Const(c)) - (*f / Term::Const(c))).simplify()
                }
                (Term::Mul(e, f), Term::Const(c)) => {
                    if let Term::Const(g) = *e {
                        (Term::Const(g / c) * *f).simplify()
                    } else if let Term::Const(h) = *f {
                        (*e * Term::Const(h / c)).simplify()
                    } else {
                        self
                    }
                }
                _ => self,
            },
            Term::Add(a, b) => match (*a, *b) {
                (Term::Add(e, f), Term::Const(c)) | (Term::Const(c), Term::Add(e, f)) => {
                    if let Term::Const(g) = *e {
                        (Term::Const(g + c) + *f).simplify()
                    } else if let Term::Const(h) = *f {
                        (*e + Term::Const(h + c)).simplify()
                    } else {
                        self
                    }
                }
                (Term::Sub(e, f), Term::Const(c)) | (Term::Const(c), Term::Sub(e, f)) => {
                    if let Term::Const(g) = *e {
                        (Term::Const(g + c) - *f).simplify()
                    } else if let Term::Const(h) = *f {
                        (*e + Term::Const(c - h)).simplify()
                    } else {
                        self
                    }
                }
                _ => self,
            },
            Term::Sub(a, b) => match (*a, *b) {
                (Term::Sub(e, f), Term::Const(c)) => {
                    if let Term::Const(g) = *e {
                        (Term::Const(g - c) - *f).simplify()
                    } else if let Term::Const(h) = *f {
                        (*e - Term::Const(h + c)).simplify()
                    } else {
                        self
                    }
                }
                (Term::Add(e, f), Term::Const(c)) => {
                    if let Term::Const(g) = *e {
                        (Term::Const(g - c) + *f).simplify()
                    } else if let Term::Const(h) = *f {
                        (*e + Term::Const(h - c)).simplify()
                    } else {
                        self
                    }
                }
                _ => self,
            },
            _ => self,
        }
    }
}

impl std::ops::Add for Term {
    type Output = Term;

    fn add(self, rhs: Self) -> Self::Output {
        Term::Add(Box::new(self), Box::new(rhs)).simplify()
    }
}

impl std::ops::Sub for Term {
    type Output = Term;

    fn sub(self, rhs: Self) -> Self::Output {
        Term::Sub(Box::new(self), Box::new(rhs)).simplify()
    }
}

impl std::ops::Mul for Term {
    type Output = Term;

    fn mul(self, rhs: Self) -> Self::Output {
        Term::Mul(Box::new(self), Box::new(rhs)).simplify()
    }
}

impl std::ops::Div for Term {
    type Output = Term;

    fn div(self, rhs: Self) -> Self::Output {
        Term::Div(Box::new(self), Box::new(rhs)).simplify()
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Human => write!(f, "x"),
            Term::Const(n) => write!(f, "{}", n),
            Term::Add(a, b) => write!(f, "({})+({})", a, b),
            Term::Sub(a, b) => write!(f, "({})-({})", a, b),
            Term::Mul(a, b) => write!(f, "({})*({})", a, b),
            Term::Div(a, b) => write!(f, "({})/({})", a, b),
        }
    }
}

#[derive(Debug)]
struct Monkeys {
    monkeys: HashMap<String, Value>,
}

impl Monkeys {
    fn parse(path: &str) -> Result<Self> {
        let monkeys = BufReader::new(File::open(path)?)
            .lines()
            .map(|line| {
                let line = line?;

                let (k, v) = line
                    .trim()
                    .split_once(": ")
                    .ok_or_else(|| anyhow!("Bad line: '{}'", line))?;

                let v: Value = v.parse()?;

                Ok((k.to_string(), v))
            })
            .collect::<Result<_>>()?;

        Ok(Self { monkeys })
    }

    fn op<F>(&self, id0: &str, id1: &str, f: F) -> Option<N>
    where
        F: Fn(N, N) -> N,
    {
        let a = self.get_value(id0)?;
        let b = self.get_value(id1)?;

        Some(f(a, b))
    }

    fn opt<F>(&self, id0: &str, id1: &str, f: F) -> Option<Term>
    where
        F: Fn(Box<Term>, Box<Term>) -> Term,
    {
        let a = self.get_term(id0)?;
        let b = self.get_term(id1)?;

        Some(f(Box::new(a), Box::new(b)).simplify())
    }

    fn get_value(&self, id: &str) -> Option<N> {
        let value = self.monkeys.get(id)?;

        match value {
            Value::Const(c) => Some(*c),
            Value::Add(a, b) => self.op(a, b, |a, b| a + b),
            Value::Sub(a, b) => self.op(a, b, |a, b| a - b),
            Value::Mul(a, b) => self.op(a, b, |a, b| a * b),
            Value::Div(a, b) => self.op(a, b, |a, b| a / b),
        }
    }

    fn get_term(&self, id: &str) -> Option<Term> {
        if id == "humn" {
            return Some(Term::Human);
        }

        let value = self.monkeys.get(id)?;

        if id == "root" {
            if let Value::Add(a, b) = value {
                let a = self.get_term(a)?.simplify();
                let b = self.get_term(b)?.simplify();
                return Some(Term::Sub(Box::new(a), Box::new(b)).simplify());
            } else {
                panic!("Root must be addition");
            }
        }

        match value {
            Value::Const(c) => Some(Term::Const(*c)),
            Value::Add(a, b) => self.opt(a, b, |a, b| Term::Add(a, b)),
            Value::Sub(a, b) => self.opt(a, b, |a, b| Term::Sub(a, b)),
            Value::Mul(a, b) => self.opt(a, b, |a, b| Term::Mul(a, b)),
            Value::Div(a, b) => self.opt(a, b, |a, b| Term::Div(a, b)),
        }
    }
}

fn main() -> Result<()> {
    let monkeys = Monkeys::parse("data/day21/input")?;

    println!("Part 1: {}", monkeys.get_value("root").expect("Root value"));

    let root = monkeys.get_term("root").expect("Root term");
    println!("Part 2: {}=0", root);
    Ok(())
}
