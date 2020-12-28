use std::collections::HashMap;

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Number parsing error for '{}': {}", data, source))]
    ParseNumber {
        data: String,
        source: std::num::ParseIntError,
    },
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct State {
    next_cup: HashMap<u32, u32>,
    current: u32,
    max: u32,
}

fn decrease_wrapping(v: u32, min: u32, max: u32) -> u32 {
    ((max - min + 1) + (v - min) - 1) % (max - min + 1) + min
}

impl State {
    fn new(cups: &[u32], expand_to: usize) -> Self {
        let mut max = *cups.iter().max().unwrap();

        let mut next_cup: HashMap<u32, u32> =
            HashMap::with_capacity(std::cmp::max(expand_to, cups.len()));
        let mut last_cup = cups[0];
        for c in &cups[1..] {
            next_cup.insert(last_cup, *c);
            last_cup = *c;
        }

        while next_cup.len() < expand_to {
            next_cup.insert(last_cup, max + 1);
            last_cup = max + 1;
            max = last_cup;
        }

        if expand_to > cups.len() {
            last_cup -= 1;
            max -= 1;
        }

        next_cup.insert(last_cup, cups[0]);

        State {
            next_cup,
            current: cups[0],
            max,
        }
    }

    fn play_move(&mut self) {
        // * The crab picks up the three cups that are immediately clockwise of the current cup.
        //   They are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
        let cur = self.current;
        let max = self.max;

        let d = self.next_cup[&cur];
        let e = self.next_cup[&d];
        let f = self.next_cup[&e];
        let g = self.next_cup[&f];

        // * The crab selects a destination cup: the cup with a label equal to the current cup's label minus one.
        //   If this would select one of the cups that was just picked up, the crab will keep subtracting one
        //   until it finds a cup that wasn't just picked up. If at any point in this process the value goes
        //   below the lowest value on any cup's label, it wraps around to the highest value on any cup's label instead.
        let mut dest_val = decrease_wrapping(cur, 1, max);
        while dest_val == d || dest_val == e || dest_val == f {
            dest_val = decrease_wrapping(dest_val, 1, max);
        }

        // * The crab places the cups it just picked up so that they are immediately clockwise of the destination cup.
        //   They keep the same order as when they were picked up.

        // Cdefg...Di...
        //
        // Cg...Ddefi...

        let i = self.next_cup[&dest_val];

        self.next_cup.insert(cur, g);
        self.next_cup.insert(dest_val, d);
        self.next_cup.insert(f, i);

        // * The crab selects a new current cup: the cup which is immediately clockwise of the current cup.
        self.current = g;
    }

    fn labels_after(&self, v: u32) -> String {
        let mut out = String::new();
        let mut current = self.next_cup[&v];

        while current != v {
            out.extend(format!("{}", current).chars());
            current = self.next_cup[&current];
        }

        out
    }
}

fn main() -> Result<()> {
    // let input = "389125467";
    let input = "219347865";

    println!("input: {}", input);
    let cups: Vec<u32> = input.bytes().map(|c| (c - ('0' as u8)) as u32).collect();

    let mut state = State::new(&cups, 0);

    for _ in 0..100 {
        state.play_move();
    }

    println!("Part 1: {}", state.labels_after(1));

    let mut state = State::new(&cups, 1_000_000);

    for _ in 0..10_000_000 {
        state.play_move();
    }

    let a = state.next_cup[&1] as usize;
    let b = state.next_cup[&(a as u32)] as usize;

    println!("Part 2: {} * {} =  {}", a, b, a * b);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrease_wrapping() {
        assert_eq!(decrease_wrapping(4, 2, 9), 3);
        assert_eq!(decrease_wrapping(3, 2, 9), 2);
        assert_eq!(decrease_wrapping(2, 2, 9), 9);
    }
}
