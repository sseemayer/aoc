use std::fs::File;
use std::io::{BufRead, BufReader};

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error on '{}': {}", filename, source))]
    Io {
        filename: String,
        source: std::io::Error,
    },

    #[snafu(display("Number parsing error: {}", source))]
    ParseNumber { source: std::num::ParseIntError },
}

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let f = File::open("data/day01/input").context(Io {
        filename: "data/day01/input".to_string(),
    })?;

    let br = BufReader::new(f);

    let lines: Vec<String> = br
        .lines()
        .map(|l| {
            l.context(Io {
                filename: "data/day01/input".to_owned(),
            })
        })
        .collect::<Result<Vec<String>>>()?;

    let numbers: Vec<u64> = lines
        .iter()
        .map(|v| v.parse().context(ParseNumber))
        .collect::<Result<Vec<_>>>()?;

    'outer2: for a in &numbers {
        for b in &numbers {
            if (a + b) == 2020 {
                println!("{} * {} = {}", a, b, a * b);
                break 'outer2;
            }
        }
    }

    'outer3: for a in &numbers {
        for b in &numbers {
            for c in &numbers {
                if (a + b + c) == 2020 {
                    println!("{} * {} * {} = {}", a, b, c, a * b * c);
                    break 'outer3;
                }
            }
        }
    }

    Ok(())
}
