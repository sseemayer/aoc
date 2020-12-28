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

fn main() -> Result<()> {
    let digits: Vec<usize> = std::fs::read_to_string("data/day01/input")
        .context(Io)?
        .trim()
        .chars()
        .map(|c| ((c as u8) - ('0' as u8)) as usize)
        .collect();

    let n = digits.len();

    let mut sum1 = 0;
    let mut sum2 = 0;
    for (i, d) in digits.iter().enumerate() {
        let j = (i + 1) % n;
        let k = (i + n / 2) % n;
        let e = digits[j];
        let f = digits[k];

        if *d == e {
            sum1 += d;
        }
        if *d == f {
            sum2 += d;
        }
    }

    println!("Part 1: {}", sum1);
    println!("Part 2: {}", sum2);

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
