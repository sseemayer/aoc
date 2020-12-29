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

fn run1(mut offsets: Vec<i64>) -> usize {
    let mut ic: i64 = 0;
    let mut steps = 0;
    while ic >= 0 && (ic as usize) < offsets.len() {
        let ofs = offsets.get_mut(ic as usize).unwrap();
        ic += *ofs;
        *ofs += 1;
        steps += 1;
    }
    steps
}

fn run2(mut offsets: Vec<i64>) -> usize {
    let mut ic: i64 = 0;
    let mut steps = 0;
    while ic >= 0 && (ic as usize) < offsets.len() {
        let ofs = offsets.get_mut(ic as usize).unwrap();
        ic += *ofs;

        if *ofs >= 3 {
            *ofs -= 1;
        } else {
            *ofs += 1;
        }
        steps += 1;
    }
    steps
}

fn main() -> Result<()> {
    let offsets: Vec<i64> = std::fs::read_to_string("data/day05/input")
        .context(Io)?
        .lines()
        .map(|l| {
            l.parse().context(ParseInt {
                data: l.to_string(),
            })
        })
        .collect::<Result<_>>()?;

    println!("Part 1: {}", run1(offsets.clone()));
    println!("Part 2: {}", run2(offsets.clone()));

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
