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

fn get_decompressed_length(s: &str, expand_recursive: bool) -> Result<usize> {
    let mut pos = 0;
    let mut total_length = 0;

    let s: Vec<char> = s.chars().collect();

    while pos < s.len() {
        let c = s[pos];
        match c {
            '(' => {
                let mut x_pos = pos + 1;
                while s[x_pos] != 'x' {
                    x_pos += 1;
                }
                let mut closing_pos = x_pos + 1;
                while s[closing_pos] != ')' {
                    closing_pos += 1;
                }

                let n_read: String = s[pos + 1..x_pos].iter().collect();
                let n_repeat: String = s[x_pos + 1..closing_pos].iter().collect();

                let n_read: usize = n_read.parse().context(ParseInt { data: n_read })?;
                let n_repeat: usize = n_repeat.parse().context(ParseInt { data: n_repeat })?;

                let inner_length = if expand_recursive {
                    let data: String = s[closing_pos + 1..closing_pos + 1 + n_read]
                        .iter()
                        .collect();

                    get_decompressed_length(&data, expand_recursive)?
                } else {
                    n_read
                };

                pos = closing_pos + 1 + n_read;
                total_length += inner_length * n_repeat;
            }
            _ => {
                total_length += 1;
                pos += 1;
            }
        }
    }

    Ok(total_length)
}

fn main() -> Result<()> {
    let input: String = std::fs::read_to_string("data/day09/input")
        .context(Io)?
        .trim()
        .to_string();

    println!(
        "Part 1: {} characters",
        get_decompressed_length(&input, false)?
    );
    println!(
        "Part 2: {} characters",
        get_decompressed_length(&input, true)?
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(get_decompressed_length("(6x1)(1x3)A", false).unwrap(), 6);
        assert_eq!(
            get_decompressed_length("X(8x2)(3x3)ABCY", false).unwrap(),
            18
        );
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            get_decompressed_length("X(8x2)(3x3)ABCY", true).unwrap(),
            "XABCABCABCABCABCABCY".len()
        );
        assert_eq!(
            get_decompressed_length("(27x12)(20x12)(13x14)(7x10)(1x12)A", true).unwrap(),
            241920
        );
        Ok(())
    }
}
