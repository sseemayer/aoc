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
    numbers: Vec<u8>,

    current: usize,
    skip_size: usize,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (i, n) in self.numbers.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            self.write_number(*n, f)?;
        }

        write!(f, "] skip_size={}", self.skip_size)?;
        Ok(())
    }
}

impl State {
    fn new(n_numbers: usize) -> Self {
        let numbers = (0..n_numbers).map(|n| n as u8).collect();

        State {
            numbers,

            skip_size: 0,
            current: 0,
        }
    }

    fn from_str(s: &str) -> Self {
        let numbers = s.as_bytes().to_vec();
        State {
            numbers,

            skip_size: 0,
            current: 0,
        }
    }

    fn step(&mut self, length: usize) {
        let n = self.numbers.len();

        for i in 0..(length / 2) {
            let a = (self.current + i) % n;
            let b = (self.current + length - i - 1) % n;
            // println!("swap {} {}", a, b);
            self.numbers.swap(a, b);
        }

        self.current = (self.current + length + self.skip_size) % n;

        // increase skip size
        self.skip_size += 1;
    }

    fn write_number(&self, n: u8, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.numbers[self.current] == n {
            write!(f, "{}", format!("{}", n).green())
        } else {
            write!(f, "{}", n)
        }
    }

    fn hash(&self) -> String {
        self.numbers
            .chunks(16)
            .map(|c| c.iter().fold(0, |a, b| a ^ *b))
            .map(|c| format!("{:02x}", c))
            .collect()
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("data/day10/input").context(Io)?;

    let lengths: Vec<usize> = input
        .split(",")
        .map(|n| {
            n.trim().parse().context(ParseInt {
                data: n.to_string(),
            })
        })
        .collect::<Result<_>>()?;

    // let lengths = vec![3, 4, 1, 5];

    // step length input             output         new skip
    // 1    3      ([0] 1 2) 3 4     2 1 0 [3] 4    1
    // 2    4      2 1) 0 ([3] 4     4 3 0 [1] 2    2
    // 3    1      4 3 0 ([1]) 2     4 [3] 0 1 2    3
    // 4    5      4) ([3] 0 1 2     3 4 2 1 [0]    4
    //

    let mut state = State::new(256);
    for l in &lengths {
        state.step(*l);
    }

    println!(
        "Part 1: {}",
        state.numbers[0] as usize * state.numbers[1] as usize
    );

    let mut state = State::new(256);
    let mut lengths: Vec<usize> = input.trim().bytes().map(|b| b as usize).collect();
    lengths.extend(vec![17, 31, 73, 47, 23]);

    for _round in 0..64 {
        for l in &lengths {
            state.step(*l);
        }
    }

    let hash = state.hash();
    println!("Part 2: {}", hash);

    Ok(())
}
