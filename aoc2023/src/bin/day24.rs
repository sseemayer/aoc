use std::{ops::RangeInclusive, str::FromStr};

use anyhow::{anyhow, bail, Context, Error, Result};
use itertools::Itertools;

use ndarray::prelude::*;
use ndarray_linalg::Solve;

#[derive(Debug, Clone)]
struct Hailstone {
    pos: [f64; 3],
    vel: [f64; 3],
}

impl FromStr for Hailstone {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (pos, vel) = s
            .split_once("@")
            .ok_or_else(|| anyhow!("Bad hailstone: '{}'", s))?;

        let pos: Vec<f64> = pos
            .split(",")
            .map(|s| s.trim().parse().context("Parse pos component"))
            .collect::<Result<Vec<_>>>()?;

        let vel: Vec<f64> = vel
            .split(",")
            .map(|s| s.trim().parse().context("Parse vel component"))
            .collect::<Result<Vec<_>>>()?;

        if pos.len() != 3 {
            bail!("Bad number of pos dimensions");
        }

        if vel.len() != 3 {
            bail!("Bad number of vel dimensions");
        }

        let pos = [pos[0], pos[1], pos[2]];
        let vel = [vel[0], vel[1], vel[2]];

        Ok(Self { pos, vel })
    }
}

impl Hailstone {
    fn intersect_xy(&self, other: &Hailstone) -> Option<[f64; 2]> {
        let [x1, y1, _] = self.pos;
        let [x2, y2] = [self.pos[0] + self.vel[0], self.pos[1] + self.vel[1]];

        let [x3, y3, _] = other.pos;
        let [x4, y4] = [other.pos[0] + other.vel[0], other.pos[1] + other.vel[1]];

        let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if denominator == 0.0 {
            // parallel lines
            return None;
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denominator;
        let u = ((x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2)) / denominator;

        if t < 0.0 || u < 0.0 {
            // cannot go back in time
            return None;
        }

        let x = x1 + t * (x2 - x1);
        let y = y1 + t * (y2 - y1);

        Some([x, y])
    }
}

fn part1(stones: &Vec<Hailstone>, window: RangeInclusive<f64>) -> usize {
    let mut count = 0;
    for (i, a) in stones.iter().enumerate() {
        for (_j, b) in stones[..i].iter().enumerate() {
            if let Some([x, y]) = a.intersect_xy(b) {
                if window.contains(&x) && window.contains(&y) {
                    count += 1;

                    // println!("{}-{:?} and {}-{:?} intersect at {:?}", i, a, j, b, [x, y]);
                }
            }
        }
    }

    count
}

/// find a stone s with starting position ps and velocity vs so that it will it the three hailstones
/// a, b, and c.
fn solve(a: &Hailstone, b: &Hailstone, c: &Hailstone) -> Result<Hailstone> {
    let [pax, pay, paz] = a.pos;
    let [vax, vay, vaz] = a.vel;
    let [pbx, pby, pbz] = b.pos;
    let [vbx, vby, vbz] = b.vel;
    let [pcx, pcy, pcz] = c.pos;
    let [vcx, vcy, vcz] = c.vel;

    // psx + t * vsx = pax + t * vax
    // psx - pax = t(vax - vsx)
    // t = (psx - pax) / (vax - vsx)

    // equating for t for x and y dimensions:
    // (psx - pax) / (vax - vsx) = (psy - pay) / (vay - vsy)
    // (psx - pax) * (vay - vsy) = (psy - pay) * (vax - vsx)
    // psx*vay - psx*vsy - pax*vay + pax*vsy = psy*vax - psy*vsx - pay*vax + pay*vsx

    // sort all elements independent of pa and va to the left:
    // psy*vsx - psx*vsy = pax*vay - psx*vay - pax*vsy + psy*vax - pay*vax + pay*vsx

    // equate right side for hailstones a and b:
    // pax*vay - psx*vay - pax*vsy + psy*vax - pay*vax + pay*vsx = pbx*vby - psx*vby - pbx*vsy + psy*vbx - pby*vbx + pby*vsx

    // substituting in symmetry to get 6 linear equations for 3 dimensions (xyz) and two pairs of hailstones (ab, ac):
    // psx(vby - vay) + psy(vax - vbx) +                  vsx(pay - pby) + vsy(pbx - pax)                  = pbx*vby - pby*vbx - pax*vay + pay*vax
    // psx(vbz - vaz) +                  psz(vax - vbx) + vsx(paz - pbz) +                  vsz(pbx - pax) = pbx*vbz - pbz*vbx - pax*vaz + paz*vax
    //                  psy(vbz - vaz) + psz(vay - vby) +                  vsy(paz - pbz) + vsz(pby - pay) = pby*vbz - pbz*vby - pay*vaz + paz*vay
    // psx(vcy - vay) + psy(vax - vcx) +                  vsx(pay - pcy) + vsy(pcx - pax)                  = pcx*vcy - pcy*vcx - pax*vay + pay*vax
    // psx(vcz - vaz) +                  psz(vax - vcx) + vsx(paz - pcz) +                  vsz(pcx - pax) = pcx*vcz - pcz*vcx - pax*vaz + paz*vax
    //                  psy(vcz - vaz) + psz(vay - vcy) +                  vsy(paz - pcz) + vsz(pcy - pay) = pcy*vcz - pcz*vcy - pay*vaz + paz*vay

    #[rustfmt::skip]
    let a: Array2<f64> = {
        array![
            //     psx        psy        psz        vsx        vsy        vsz
            [vby - vay, vax - vbx,        0., pay - pby, pbx - pax,        0.],
            [vbz - vaz,        0., vax - vbx, paz - pbz,        0., pbx - pax],
            [       0., vbz - vaz, vay - vby,        0., paz - pbz, pby - pay],
            [vcy - vay, vax - vcx,        0., pay - pcy, pcx - pax,        0.],
            [vcz - vaz,        0., vax - vcx, paz - pcz,        0., pcx - pax],
            [       0., vcz - vaz, vay - vcy,        0., paz - pcz, pcy - pay],
        ]
    };

    #[rustfmt::skip]
    let b: Array1<f64> = {
        array![
            pbx*vby - pby*vbx - pax*vay + pay*vax,
            pbx*vbz - pbz*vbx - pax*vaz + paz*vax,
            pby*vbz - pbz*vby - pay*vaz + paz*vay,
            pcx*vcy - pcy*vcx - pax*vay + pay*vax,
            pcx*vcz - pcz*vcx - pax*vaz + paz*vax,
            pcy*vcz - pcz*vcy - pay*vaz + paz*vay,
        ]
    };

    let x = a.solve_into(b)?.mapv(f64::round);

    Ok(Hailstone {
        pos: [x[0], x[1], x[2]],
        vel: [x[3], x[4], x[5]],
    })
}

fn part2(stones: &Vec<Hailstone>) -> Option<f64> {
    // generate a combination of three hailstones to plug into linear equation
    for combination in stones.iter().combinations(3) {
        if let Ok(solution) = solve(&combination[0], &combination[1], &combination[2]) {
            // return the first discovered solution
            return Some(solution.pos.iter().sum());
        }
    }

    None
}

fn main() -> Result<()> {
    let stones: Vec<Hailstone> = aoc::io::read_lines("data/day24/input")?;

    println!(
        "Part 1: {}",
        part1(&stones, 200_000_000_000_000.0f64..=400_000_000_000_000.0f64)
    );

    println!(
        "Part 2: {}",
        part2(&stones).ok_or_else(|| anyhow!("Could not solve"))?
    );

    Ok(())
}
