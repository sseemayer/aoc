use std::collections::HashMap;

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Number parsing error: {}", source))]
    ParseNumber { source: std::num::ParseIntError },
}

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let input = "7,14,0,17,11,1,2";
    // let input = "0,3,6";
    let mut numbers: Vec<usize> = input
        .split(",")
        .map(|n| n.parse().context(ParseNumber))
        .collect::<Result<_>>()?;

    let mut history: HashMap<usize, usize> = HashMap::new();
    let mut number = numbers.remove(0);
    for i in 1..=30000000 {
        //println!("{:5}: {} {:?}", i, number, history);
        if i == 2020 || i % 1000000 == 0 {
            println!("{:8}: {}", i, number);
        }

        let next_number = if !numbers.is_empty() {
            numbers.remove(0)
        } else {
            if let Some(h) = history.get(&number) {
                // println!("Last saw {} on turn {}", number, h);
                i - h
            } else {
                // println!("Never saw {} before", number);
                0
            }
        };

        history.insert(number, i);

        number = next_number;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
