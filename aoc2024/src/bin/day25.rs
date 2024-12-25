use anyhow::{bail, Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile(bool);

impl aoc::map::ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Tile(true)),
            '.' => Some(Tile(false)),
            _ => None,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile(true) => "#",
                Tile(false) => ".",
            }
        )
    }
}

type Map = aoc::map::Map<[i32; 2], Tile>;

#[derive(Debug, Clone)]
struct Matrix {
    code: Vec<usize>,
    height: usize,
    is_lock: bool,
}

impl std::str::FromStr for Matrix {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let map: Map = s.parse()?;
        let ([imin, jmin], [height, width]) = map.get_extent();

        let width = (width as usize) + 1;
        let height = (height as usize) + 1;

        if imin != 0 || jmin != 0 {
            bail!("Extent should start at 0");
        }

        let is_lock = (0..width).all(|j| map.get(&[0, j as i32]) == Some(&Tile(true)));

        let mut code = vec![0; width];

        for ([_, j], Tile(b)) in map.data.into_iter() {
            if b {
                code[j as usize] += 1;
            }
        }

        Ok(Self {
            code,
            height,
            is_lock,
        })
    }
}

impl Matrix {
    fn fits(&self, other: &Matrix) -> bool {
        if self.is_lock == other.is_lock {
            return false;
        }

        if self.height != other.height {
            return false;
        }

        if self.code.len() != other.code.len() {
            return false;
        }

        self.code
            .iter()
            .zip(&other.code)
            .all(|(a, b)| a + b <= self.height)
    }
}

fn main() -> Result<()> {
    let matrices: Vec<Matrix> = aoc::io::read_all((2024, 25))?
        .split("\n\n")
        .map(|s| s.parse())
        .collect::<Result<Vec<_>>>()?;

    let mut sum = 0;
    for (i, m) in matrices.iter().enumerate() {
        for n in matrices.iter().take(i) {
            if m.fits(n) {
                sum += 1;
            }
        }
    }

    println!("Part 1: {}", sum);

    Ok(())
}
