use anyhow::Result;
use aoc2021::io::read_lines;
use thiserror::Error;

#[derive(Error, Debug)]
enum ParseError {
    #[error("Bad line: '{}'", .0)]
    BadLine(String),

    #[error(transparent)]
    Int(#[from] std::num::ParseIntError),
}

#[derive(Clone, Copy)]
struct Cuboid {
    x: [i64; 2],
    y: [i64; 2],
    z: [i64; 2],
    sign: i64,
}

impl std::fmt::Debug for Cuboid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}[{}..{}/{}..{}/{}..{}]",
            if self.sign > 0 { "+" } else { "-" },
            self.x[0],
            self.x[1],
            self.y[0],
            self.y[1],
            self.z[0],
            self.z[1]
        )
    }
}

impl std::str::FromStr for Cuboid {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (sign, cuboid) = s
            .trim()
            .split_once(" ")
            .ok_or(ParseError::BadLine(s.to_string()))?;

        let sign = match sign {
            "on" => 1,
            "off" => -1,
            _ => return Err(ParseError::BadLine(s.to_string())),
        };

        let tokens = cuboid
            .trim()
            .split(",")
            .map(|t| {
                let (v1, v2) = t[2..]
                    .split_once("..")
                    .ok_or(ParseError::BadLine(s.to_string()))?;

                let v1: i64 = v1.parse()?;
                let v2: i64 = v2.parse()?;

                let (v1, v2) = if v1 < v2 { (v1, v2) } else { (v2, v1) };

                Ok([v1, v2])
            })
            .collect::<std::result::Result<Vec<[i64; 2]>, ParseError>>()?;

        if tokens.len() != 3 {
            return Err(ParseError::BadLine(s.to_string()));
        }

        let x = tokens[0];
        let y = tokens[1];
        let z = tokens[2];

        Ok(Cuboid { x, y, z, sign })
    }
}

impl Cuboid {
    fn in_init_region(&self) -> bool {
        self.x[0] >= -50
            && self.x[1] <= 50
            && self.y[0] >= -50
            && self.y[1] <= 50
            && self.z[0] >= -50
            && self.z[1] <= 50
    }

    fn volume(&self) -> i64 {
        (self.x[1] - self.x[0] + 1).max(0)
            * (self.y[1] - self.y[0] + 1).max(0)
            * (self.z[1] - self.z[0] + 1).max(0)
    }
}

/// Signed intersection of two cuboids.
impl std::ops::BitAnd for Cuboid {
    type Output = Option<Cuboid>;

    fn bitand(self, rhs: Cuboid) -> Self::Output {
        let x = [i64::max(self.x[0], rhs.x[0]), i64::min(self.x[1], rhs.x[1])];
        let y = [i64::max(self.y[0], rhs.y[0]), i64::min(self.y[1], rhs.y[1])];
        let z = [i64::max(self.z[0], rhs.z[0]), i64::min(self.z[1], rhs.z[1])];

        if x[1] < x[0] || y[1] < y[0] || z[1] < z[0] {
            // no intersection
            return None;
        }

        match (self.sign, rhs.sign) {
            // ll
            // lXr = vol(l) + vol(r) - vol(X)
            //  rr
            (1, 1) => Some(Cuboid { x, y, z, sign: -1 }),
            (1, -1) => Some(Cuboid { x, y, z, sign: -1 }),
            (-1, 1) => Some(Cuboid { x, y, z, sign: 1 }),
            (-1, -1) => Some(Cuboid { x, y, z, sign: 1 }),

            _ => panic!("Unknown sign combination"),
        }
    }
}

fn calculate<'a, I: Iterator<Item = &'a Cuboid>>(cuboids: I) -> i64 {
    let mut world: Vec<Cuboid> = Vec::new();

    for &c in cuboids {
        let mut new = Vec::new();
        for &d in world.iter() {
            new.extend(d & c);
        }

        world.extend(new.into_iter());

        if c.sign == 1 {
            world.push(c);
        }
    }

    world.iter().map(|c| c.volume() * c.sign).sum()
}

fn main() -> Result<()> {
    let cuboids: Vec<Cuboid> = read_lines("data/day22/input")?;

    println!(
        "Part 1: {}",
        calculate(cuboids.iter().filter(|c| c.in_init_region()))
    );

    println!("Part 2: {}", calculate(cuboids.iter()));

    Ok(())
}
