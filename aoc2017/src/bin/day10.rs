use std::collections::HashMap;

use snafu::{ResultExt, Snafu};

use colored::Colorize;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Int format error for '{}': {}", data, source))]
    ParseInt {
        data: String,
        source: std::num::ParseIntError,
    },
}

#[derive(Debug)]
struct State {
    next: HashMap<u16, u16>,
    prev: HashMap<u16, u16>,

    current: u16,
    first: u16,
    skip_size: usize,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "forward: ")?;
        self.write_number(self.first, f)?;

        let mut cur = self.next[&self.first];
        while cur != self.first {
            write!(f, " -> ")?;
            self.write_number(cur, f)?;

            cur = self.next[&cur];
        }
        write!(f, " (-> ")?;
        self.write_number(self.first, f)?;

        write!(f, ")\nreverse: ")?;

        self.write_number(self.first, f)?;

        let mut cur = self.prev[&self.first];
        while cur != self.first {
            write!(f, " -> ")?;
            self.write_number(cur, f)?;

            cur = self.prev[&cur];
        }
        write!(f, " (-> ")?;
        self.write_number(self.first, f)?;

        write!(f, ")\nskip_size={}", self.skip_size)?;
        Ok(())
    }
}

impl State {
    fn new(n_numbers: usize) -> Self {
        let mut next: HashMap<u16, u16> = HashMap::with_capacity(n_numbers);
        let mut prev: HashMap<u16, u16> = HashMap::with_capacity(n_numbers);
        for i in 0..n_numbers {
            let j = (i + 1) % n_numbers;
            next.insert(i as u16, j as u16);
            prev.insert(j as u16, i as u16);
        }

        State {
            next,
            prev,

            skip_size: 0,
            current: 0,
            first: 0,
        }
    }

    fn skip_forward(&self, from: u16, by: usize) -> u16 {
        let mut current = from;
        for _ in 0..by {
            current = self.next[&current];
        }
        current
    }

    fn write_number(&self, n: u16, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if n == self.current {
            write!(f, "{}", format!("{}", n).green())
        } else {
            write!(f, "{}", n)
        }
    }

    fn step(&mut self, length: usize) {
        let pinch_start = self.current;
        let pinch_prev = self.prev[&pinch_start];
        let pinch_end = self.skip_forward(pinch_start, length);
        let pinch_last = self.prev[&pinch_end];

        // reverse from pinch_start to pinch_end, not including pinch_end

        //   ps  le
        // 0123456789
        //   265437
        //   pl  se

        let mut current = pinch_start;
        loop {
            let temp = self.prev[&current];
            self.prev.insert(current, self.next[&current]);
            self.next.insert(current, temp);

            if current == pinch_last {
                break;
            }

            current = self.prev[&current];
        }

        if pinch_start != pinch_end {
            self.next.insert(pinch_prev, pinch_last);
            self.prev.insert(pinch_last, pinch_prev);

            self.next.insert(pinch_start, pinch_end);
            self.prev.insert(pinch_end, pinch_start);

            self.current = self.skip_forward(pinch_end, self.skip_size);
        } else {
            self.next.insert(pinch_start, pinch_last);
            self.prev.insert(pinch_last, pinch_start);

            self.current = self.skip_forward(pinch_last, self.skip_size);
        }

        // increase skip size
        self.skip_size += 1;
    }
}

fn main() -> Result<()> {
    let lengths: Vec<usize> = std::fs::read_to_string("data/day10/input")
        .context(Io)?
        .split(",")
        .map(|n| {
            n.trim().parse().context(ParseInt {
                data: n.to_string(),
            })
        })
        .collect::<Result<_>>()?;

    let lengths = vec![3, 4, 1, 5];

    // step length input             output         new skip
    // 1    3      ([0] 1 2) 3 4     2 1 0 [3] 4    1
    // 2    4      2 1) 0 ([3] 4     4 3 0 [1] 2    2
    // 3    1      4 3 0 ([1]) 2     4 [3] 0 1 2    3
    // 4    5      4) ([3] 0 1 2     3 4 2 1 [0]    4
    //

    let mut state = State::new(5);
    println!("{}", state);
    for l in &lengths {
        println!("STEP {}", l);
        state.step(*l);
        println!("{}", state);
    }

    let first = state.current;
    let next = state.next[&first];

    println!("Part 1: {}", first * next);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        Ok(())
    }
}
