use failure::{format_err, Error};
use std::collections::HashMap;
type Result<T> = std::result::Result<T, Error>;

use aoc2019::util::read_to_parsed_lines;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn sum_abs(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "<{:>3}, {:>3}, {:>3}>", self.x, self.y, self.z)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl std::fmt::Debug for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Moon[pos={:?}, vel={:?}]", self.pos, self.vel)
    }
}

impl std::str::FromStr for Moon {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens = s
            .replace("<", "")
            .replace(">", "")
            .replace("x=", "")
            .replace("y=", "")
            .replace("z=", "")
            .split(",")
            .map(|t| {
                t.trim()
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.into())
            })
            .collect::<Result<Vec<i64>>>()?;

        if tokens.len() != 3 {
            return Err(format_err!("Positions have to be 3 values exactly"));
        }

        let pos = Vec3 {
            x: tokens[0],
            y: tokens[1],
            z: tokens[2],
        };

        Ok(Moon {
            pos,
            vel: Vec3 { x: 0, y: 0, z: 0 },
        })
    }
}

fn grav_1dim(a: i64, b: i64) -> i64 {
    if a < b {
        1
    } else if a == b {
        0
    } else {
        -1
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    moons: Vec<Moon>,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:#?} e={}", self.moons, self.energy())
    }
}

impl State {
    fn simulate(&mut self) {
        let n_moons = self.moons.len();

        // GRAVITY STEP
        for i in 0..n_moons {
            let m = self.moons.get(i).unwrap();

            let mut x = m.vel.x;
            let mut y = m.vel.y;
            let mut z = m.vel.z;

            for j in 0..n_moons {
                if i == j {
                    continue;
                }

                let n = self.moons.get(j).unwrap();

                x += grav_1dim(m.pos.x, n.pos.x);
                y += grav_1dim(m.pos.y, n.pos.y);
                z += grav_1dim(m.pos.z, n.pos.z);
            }

            let m = self.moons.get_mut(i).unwrap();
            m.vel.x = x;
            m.vel.y = y;
            m.vel.z = z;
        }

        // VELOCITY STEP
        for m in self.moons.iter_mut() {
            m.pos = m.pos + m.vel;
        }
    }

    fn energy(&self) -> i64 {
        let mut out = 0;

        for m in self.moons.iter() {
            let e_kin = m.pos.sum_abs();
            let e_pot = m.vel.sum_abs();

            out += e_kin * e_pot;
        }

        out
    }
}

#[derive(Debug)]
struct PeriodFinder1D {
    seen: HashMap<Vec<(i64, i64)>, usize>,
    period: Option<usize>,
}

impl PeriodFinder1D {
    fn new() -> Self {
        PeriodFinder1D {
            seen: HashMap::new(),
            period: None,
        }
    }

    fn step(&mut self, step: usize, pos_vel: Vec<(i64, i64)>) {
        if let Some(_) = self.period {
            return;
        }

        if self.seen.contains_key(&pos_vel) {
            let last_seen = self.seen[&pos_vel];
            self.period = Some(step - last_seen);

            println!("Found 1D period: {:?}", self.period);
        } else {
            self.seen.insert(pos_vel, step);
        }
    }
}

#[derive(Debug)]
struct PeriodFinder3D {
    pf_x: PeriodFinder1D,
    pf_y: PeriodFinder1D,
    pf_z: PeriodFinder1D,
    period: Option<usize>,
}

impl PeriodFinder3D {
    fn new() -> Self {
        PeriodFinder3D {
            pf_x: PeriodFinder1D::new(),
            pf_y: PeriodFinder1D::new(),
            pf_z: PeriodFinder1D::new(),
            period: None,
        }
    }

    fn step(&mut self, step: usize, moons: &Vec<Moon>) {
        if let Some(_) = self.period {
            return;
        }

        let pv_x = moons.iter().map(|m| (m.pos.x, m.vel.x)).collect();
        let pv_y = moons.iter().map(|m| (m.pos.y, m.vel.y)).collect();
        let pv_z = moons.iter().map(|m| (m.pos.z, m.vel.z)).collect();

        self.pf_x.step(step, pv_x);
        self.pf_y.step(step, pv_y);
        self.pf_z.step(step, pv_z);

        if let (Some(px), Some(py), Some(pz)) =
            (self.pf_x.period, self.pf_y.period, self.pf_z.period)
        {
            let period = num::integer::lcm(num::integer::lcm(px, py), pz);
            self.period = Some(period);
            println!("Found 3D period: {:?}", self.period);
        }
    }
}

fn main() -> Result<()> {
    let moons = read_to_parsed_lines("data/day12/input", &|l: &str| l.parse())?;

    let mut state = State { moons };
    let mut step = 0;
    let mut pf = PeriodFinder3D::new();

    loop {
        println!("AFTER {} STEPS:", step);
        println!("{:#?}", state);
        pf.step(step, &state.moons);

        if let Some(p) = pf.period {
            println!("FOUND PERIOD: {}", p);
            break;
        }

        state.simulate();
        step += 1;
    }

    Ok(())
}
