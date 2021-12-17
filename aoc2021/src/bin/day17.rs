use anyhow::Result;
use aoc2021::io::read_all;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("bad input: '{}'", .0)]
    BadInput(String),
}

fn simulate(vx0: i64, vy0: i64, xmin: i64, xmax: i64, ymin: i64, ymax: i64) -> Option<i64> {
    let mut x = 0;
    let mut y = 0;
    let mut vx = vx0;
    let mut vy = vy0;

    let mut max_y_pos = 0;

    loop {
        x += vx;
        y += vy;

        max_y_pos = max_y_pos.max(y);

        vx -= vx.signum();
        vy -= 1;

        if xmin <= x && x <= xmax && ymin <= y && y <= ymax {
            return Some(max_y_pos);
        }

        if y < ymin {
            return None;
        }
    }
}

fn main() -> Result<()> {
    let data = read_all("data/day17/input")?;
    // let data = "target area: x=20..30, y=-10..-5";

    let data: Vec<&str> = data.split_whitespace().collect();
    let (xmin, xmax) = data[2][2..data[2].len() - 1]
        .split_once("..")
        .ok_or(Error::BadInput(data[2].to_string()))?;
    let (xmin, xmax): (i64, i64) = (xmin.parse()?, xmax.parse()?);

    let (ymin, ymax) = data[3][2..]
        .split_once("..")
        .ok_or(Error::BadInput(data[3].to_string()))?;
    let (ymin, ymax): (i64, i64) = (ymin.parse()?, ymax.parse()?);

    let mut n_hitting = 0;
    let mut max_y_pos = 0;

    for vx0 in xmin.signum()..xmin.signum() * 1000 {
        for vy0 in -100..100 {
            if let Some(ym) = simulate(vx0, vy0, xmin, xmax, ymin, ymax) {
                max_y_pos = max_y_pos.max(ym);
                n_hitting += 1;
            }
        }
    }

    println!("Part 1: {}", max_y_pos);
    println!("Part 2: {}", n_hitting);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
