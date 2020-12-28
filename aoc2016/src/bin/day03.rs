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

struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}

impl Triangle {
    fn is_valid(&self) -> bool {
        (self.a + self.b) > self.c && (self.a + self.c) > self.b && (self.b + self.c) > self.a
    }
}

fn main() -> Result<()> {
    let matrix: Vec<Vec<usize>> = std::fs::read_to_string("data/day03/input")
        .context(Io)?
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|t| {
                    t.parse::<usize>().context(ParseInt {
                        data: t.to_string(),
                    })
                })
                .collect::<Result<_>>()
        })
        .collect::<Result<_>>()?;

    let triangles1: Vec<Triangle> = matrix
        .iter()
        .map(|l| Triangle {
            a: l[0],
            b: l[1],
            c: l[2],
        })
        .collect();

    let valid1 = triangles1.iter().filter(|t| t.is_valid()).count();
    println!("Part 1: {} valid", valid1);

    let mut triangles2: Vec<Triangle> = Vec::new();
    for i in (0..matrix.len()).step_by(3) {
        let row0 = &matrix[i];
        let row1 = &matrix[i + 1];
        let row2 = &matrix[i + 2];

        for ((a, b), c) in row0.iter().zip(row1.iter()).zip(row2.iter()) {
            triangles2.push(Triangle {
                a: *a,
                b: *b,
                c: *c,
            });
        }
    }
    let valid2 = triangles2.iter().filter(|t| t.is_valid()).count();
    println!("Part 2: {} valid", valid2);

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
