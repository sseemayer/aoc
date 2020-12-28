use snafu::{ResultExt, Snafu};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Int parsing error: {}", source))]
    ParseInt { source: std::num::ParseIntError },
}

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let f = File::open("data/day10/input").context(Io)?;
    let mut adapters: Vec<usize> = BufReader::new(f)
        .lines()
        .map(|l| l.context(Io)?.parse().context(ParseInt))
        .collect::<Result<_>>()?;

    adapters.sort();
    adapters.insert(0, 0);

    let max_joltage = adapters[adapters.len() - 1];

    adapters.push(max_joltage + 3);

    println!("Got adapters: {:?}", adapters);
    let mut deltas: HashMap<usize, usize> = HashMap::new();

    for i in 1..adapters.len() {
        let a = adapters.get(i - 1).unwrap(); // for the outlet
        let b = adapters.get(i).unwrap();
        let delta = b - a;
        let count = deltas.entry(delta).or_insert(0);
        *count += 1;
    }
    println!("Deltas: {:?}, answer: {}", deltas, deltas[&3] * deltas[&1]);

    let mut paths_to: Vec<usize> = adapters.iter().map(|_| 0).collect();
    paths_to[0] = 1;
    for (i, jolt_i) in adapters.iter().enumerate() {
        let n_inbound = paths_to[i];

        for j in i + 1..std::cmp::min(i + 4, adapters.len()) {
            let jolt_j = adapters[j];
            if jolt_j <= jolt_i + 3 {
                // println!("{} -> {} (x{})", jolt_i, jolt_j, n_inbound);
                paths_to[j] += n_inbound;
            }
        }
    }

    println!("Got {} arrangements", paths_to[paths_to.len() - 1]);

    Ok(())
}
