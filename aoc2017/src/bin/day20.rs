use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use aoc::io::read_lines;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:>3},{:>3},{:>3}>", self.x, self.y, self.z)
    }
}

impl std::str::FromStr for Vector {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<&str> = s[1..s.len() - 1].split(",").collect();

        if tokens.len() != 3 {
            return Err(anyhow!("Expected three dimensions: '{}'", s));
        }

        let x: i64 = tokens[0].trim().parse().context("Parse X")?;
        let y: i64 = tokens[1].trim().parse().context("Parse Y")?;
        let z: i64 = tokens[2].trim().parse().context("Parse Z")?;

        Ok(Vector { x, y, z })
    }
}

impl std::ops::AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, rhs: &Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Clone)]
struct Particle {
    pos: Vector,
    vel: Vector,
    acc: Vector,
}

impl std::str::FromStr for Particle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.trim().split(", ").collect();

        if tokens.len() != 3 {
            return Err(anyhow!("Bad particle: '{}'", s));
        }

        let pos: Vector = tokens[0][2..].trim().parse()?;
        let vel: Vector = tokens[1][2..].trim().parse()?;
        let acc: Vector = tokens[2][2..].trim().parse()?;

        Ok(Particle { pos, vel, acc })
    }
}

impl std::fmt::Debug for Particle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.pos)
    }
}

impl Particle {
    fn step(&mut self) {
        self.vel += &self.acc;
        self.pos += &self.vel;
    }

    fn manhattan_distance(&self) -> i64 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }
}

fn main() -> Result<()> {
    let particles: Vec<Particle> = read_lines("data/day20/input").context("Reading particles")?;

    let mut state = particles.clone();
    for _ in 0..1000 {
        for p in state.iter_mut() {
            p.step();
        }
    }

    let (i_min, _) = state
        .iter()
        .enumerate()
        .min_by_key(|(_, p)| p.manhattan_distance())
        .expect("Min particle");

    println!("Part 1: {}", i_min);

    let mut state: Vec<(usize, Particle)> = particles.into_iter().enumerate().collect();

    for step in 0..100 {
        println!("step {}, count={}", step, state.len());

        // do updates and store re-occurring positions -> state index lookups
        let mut positions: HashMap<Vector, Vec<usize>> = HashMap::new();
        for (i, (_, p)) in state.iter_mut().enumerate() {
            p.step();

            positions.entry(p.pos.clone()).or_default().push(i);
        }

        // purge colliding
        let mut colliding = Vec::new();
        for v in positions.into_values() {
            if v.len() > 1 {
                colliding.extend(v);
            }
        }
        colliding.sort();
        while let Some(i) = colliding.pop() {
            state.remove(i);
        }
    }

    println!("Part 2: {}", state.len());

    Ok(())
}
