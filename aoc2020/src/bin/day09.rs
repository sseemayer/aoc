use snafu::{ResultExt, Snafu};
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

fn can_sum(prefix: &[usize], target: usize) -> bool {
    for i in 0..prefix.len() {
        for j in i + 1..prefix.len() {
            if prefix[i] + prefix[j] == target {
                return true;
            }
        }
    }
    false
}

fn validate(numbers: &[usize], window_size: usize) -> Option<usize> {
    for i in window_size..numbers.len() {
        let target = numbers[i];
        let prefix = &numbers[i - window_size..i];
        if !can_sum(prefix, target) {
            return Some(i);
        }
    }

    None
}

fn find_summands(numbers: &[usize], target: usize) -> Option<Vec<usize>> {
    let mut out = Vec::new();
    let mut sum;

    for i in 0..numbers.len() {
        out.clear();
        out.push(numbers[i]);
        sum = numbers[i];

        for j in i + 1..numbers.len() {
            out.push(numbers[j]);
            sum += numbers[j];

            if sum == target {
                out.sort();
                return Some(out);
            } else if sum > target {
                break;
            }
        }
    }

    None
}

fn main() -> Result<()> {
    let f = File::open("data/day09/input").context(Io)?;
    let numbers: Vec<usize> = BufReader::new(f)
        .lines()
        .map(|l| l.context(Io)?.parse().context(ParseInt))
        .collect::<Result<Vec<_>>>()?;

    if let Some(i) = validate(&numbers[..], 25) {
        let n = numbers[i];
        println!("Cannot validate {} at position {}", n, i);
        if let Some(v) = find_summands(&numbers, n) {
            println!("Summands for {} are {:?}", n, v);

            println!("Solution is {}", v[0] + v[v.len() - 1]);
        }
    }

    Ok(())
}
