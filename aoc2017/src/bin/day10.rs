use aoc2017::knothash::KnotHash;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("data/day10/input")?;

    let lengths: Vec<usize> = input
        .split(",")
        .map(|n| n.trim().parse().map_err(|e| anyhow!("Bad int: {}", e)))
        .collect::<Result<_>>()?;

    // let lengths = vec![3, 4, 1, 5];

    // step length input             output         new skip
    // 1    3      ([0] 1 2) 3 4     2 1 0 [3] 4    1
    // 2    4      2 1) 0 ([3] 4     4 3 0 [1] 2    2
    // 3    1      4 3 0 ([1]) 2     4 [3] 0 1 2    3
    // 4    5      4) ([3] 0 1 2     3 4 2 1 [0]    4
    //

    let mut state = KnotHash::new(256);
    for l in &lengths {
        state.step(*l);
    }

    println!(
        "Part 1: {}",
        state.numbers[0] as usize * state.numbers[1] as usize
    );

    let state = KnotHash::from_str(input.trim());
    let hash = state.hash();
    println!("Part 2: {}", hash);

    Ok(())
}
