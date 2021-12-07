use aoc2021::io::read_all;

use thiserror::Error;

#[derive(Error, Debug)]
enum Day07Error {
    #[error(transparent)]
    Int(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, Day07Error>;

fn load_positions(s: &str) -> Result<Vec<i64>> {
    let mut positions = s
        .trim()
        .split(",")
        .map(|v| Ok(v.parse()?))
        .collect::<Result<Vec<_>>>()?;

    positions.sort();

    Ok(positions)
}

fn part1_distance(d: i64) -> i64 {
    d.abs()
}

fn part2_distance(d: i64) -> i64 {
    d.abs() * (d.abs() + 1) / 2
}

fn main() -> Result<()> {
    let positions = load_positions(&read_all("data/day07/input")?)?;
    // let positions = load_positions("16,1,2,0,4,2,7,1,2,14")?;

    // median minimizes $\sum_{i} |position[i] - p|$
    let part1_pos = positions[positions.len() / 2];
    let part1_dst: i64 = positions
        .iter()
        .map(|p| part1_distance(p - part1_pos))
        .sum();

    println!("part 1: dst={} (pos={})", part1_dst, part1_pos);

    //   min_p \sum_i |p_i - p| * (|p_i - p| + 1| / 2
    // = min_p 0.5 * \sum_i [ (p_i - p)^2 + |p_i - p| ]
    //
    // d/dp = \sum_i [ 2*(p_i - p) + sign(p_i - p)]
    //      = -2np + 2*\sum_i [ p_i ] + \sum_i[sign(p_i - p)]
    //
    // d/dp := 0:   2np=2\sum_i[p_i] + \sum_i[sign(p_i) - p)]
    //                p=\sum_i[p_i]/n + \sum_i[sign(p_i - p)]/(2n)
    //                  ^^^^^^^^^^^^^   ^^^^^^^^^^^^^^^^^^^^^^^^^^
    //                  mean(positions)  this value is in [-1, 1]

    let part2_pos_rough =
        (positions.iter().sum::<i64>() as f64 / positions.len() as f64).floor() as i64;

    let part2_pos = (-1..=1)
        .map(|ofs| part2_pos_rough + ofs)
        .min_by_key(|v| positions.iter().map(|p| part2_distance(p - v)).sum::<i64>())
        .unwrap();

    let part2_dst: i64 = positions
        .iter()
        .map(|p| part2_distance(p - part2_pos))
        .sum();

    println!("part 2: dst={} (pos={})", part2_dst, part2_pos);

    Ok(())
}
