use aoc2017::knothash::KnotHash;
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

fn main() -> Result<()> {
    let input = std::fs::read_to_string("data/day10/input").context(Io)?;

    let lengths: Vec<usize> = input
        .split(",")
        .map(|n| {
            n.trim().parse().context(ParseInt {
                data: n.to_string(),
            })
        })
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
