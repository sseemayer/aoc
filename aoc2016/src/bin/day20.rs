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

    #[snafu(display("IP range parsing error for '{}'", data))]
    ParseIPRange { data: String },
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct IPRange {
    start: usize,
    end: usize,
}

impl std::str::FromStr for IPRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<&str> = s.split("-").collect();
        if tokens.len() != 2 {
            return Err(Error::ParseIPRange {
                data: s.to_string(),
            });
        }
        let start: usize = tokens[0].parse().context(ParseInt {
            data: tokens[0].to_string(),
        })?;
        let end: usize = tokens[1].parse().context(ParseInt {
            data: tokens[1].to_string(),
        })?;

        Ok(IPRange { start, end })
    }
}

fn main() -> Result<()> {
    let mut ranges: Vec<IPRange> = std::fs::read_to_string("data/day20/input")
        .context(Io)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_>>()?;

    let max_value = 4294967295;

    //let mut ranges: Vec<IPRange> = vec![(0, 2), (4, 7), (5, 8)]
    //     .into_iter()
    //     .map(|v| IPRange {
    //         start: v.0,
    //         end: v.1,
    //     })
    //     .collect();
    // let max_value = 10;

    ranges.sort();

    let mut min_allowed = 0;
    let mut last_end = 0;
    let mut n_allowed = 0;
    for r in &ranges {
        if min_allowed >= r.start && min_allowed <= r.end {
            min_allowed = r.end + 1;
        }

        if last_end < r.start {
            n_allowed += r.start - last_end - 1;
        }

        last_end = std::cmp::max(r.end, last_end);
    }

    if last_end <= max_value {
        n_allowed += max_value - last_end;
    }

    println!("Part 1: minimum non-blocked IP is {}", min_allowed);
    println!("Part 2: number of available IPs is {}", n_allowed);

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
