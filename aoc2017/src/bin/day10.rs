use std::collections::HashMap;

use snafu::{ResultExt, Snafu};

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
    next: HashMap<usize, usize>,
    prev: HashMap<usize, usize>,

    first: usize,
    skip: usize,
    current: usize,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " forward: [{}]", self.current)?;

        let mut cur = self.next[&self.current];
        while cur != self.current {
            write!(f, " -> {}", cur)?;
            cur = self.next[&cur];
        }
        write!(f, " (-> {})", cur)?;

        write!(f, "\nbackward: [{}]", self.current)?;

        let mut cur = self.prev[&self.current];
        while cur != self.current {
            write!(f, " -> {}", cur)?;
            cur = self.prev[&cur];
        }
        write!(f, " (-> {})", cur)?;

        write!(f, "\nskip={}, first={}", self.skip, self.first)?;
        Ok(())
    }
}

impl State {
    fn new(n_numbers: usize) -> Self {
        let mut next: HashMap<usize, usize> = HashMap::with_capacity(n_numbers);
        let mut prev: HashMap<usize, usize> = HashMap::with_capacity(n_numbers);
        for i in 0..n_numbers {
            next.insert(i, (i + 1) % n_numbers);
            prev.insert((i + 1) % n_numbers, i);
        }

        State {
            next,
            prev,
            first: 0,
            skip: 0,
            current: 0,
        }
    }

    fn skip_forward(&self, from: usize, amount: usize) -> usize {
        let mut current = from;
        for _ in 1..amount {
            current = self.next[&current];
        }
        current
    }

    fn link(&mut self, from: usize, to: usize) {
        println!("set {} -> {}", from, to);
        self.next.insert(from, to);
        self.prev.insert(to, from);
    }

    fn reverse(&mut self, from: usize, to: usize) {
        //f0
        // 01
        //  12
        //   23
        //    34
        //     4t
        //
        //     0f
        //    10
        //   21
        //  32
        // 43
        //t4

        let mut last = from;
        let mut current = self.next[&from];
        while current != to {
            let next = self.next[&current];
            self.link(current, last);
            last = current;
            current = next;
        }
        self.link(current, last);
    }

    fn step(&mut self, length: usize) {
        //  /-l-\
        // bc012de
        // bd210ce

        let b = self.prev[&self.current];
        let c = self.current;

        let d = self.skip_forward(c, length);
        let e = self.next[&d];

        println!("b={} c={} d={} e={}", b, c, d, e);

        if c == d {
            // don't do anything for lists of 1
        } else if c != e {
            // we are in a proper sublist
            self.reverse(c, d);
            self.link(b, d);
            self.link(c, e);
        } else {
            // we are in a loop around c
            self.reverse(c, c);
        }

        self.current = self.skip_forward(e, self.skip);
        self.skip += 1;
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

    // ([0] 1 2) 3 4
    // ([3] 4 2 1) 0
    // ([3] 0 1 2 4)
    // ([2]) 4 3 0 1
    // ([3] 0 1 2 4)
    // 3 4 2 1 [0]

    let mut state = State::new(5);
    println!("{}", state);
    for l in &lengths {
        println!("STEP {}", l);
        state.step(*l);
        println!("{}", state);
    }

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
