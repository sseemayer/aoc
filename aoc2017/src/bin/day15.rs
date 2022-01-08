use snafu::Snafu;

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

fn step1(vals: (usize, usize)) -> (usize, usize) {
    ((vals.0 * 16807) % 2147483647, (vals.1 * 48271) % 2147483647)
}

fn step_mod(val: usize, factor: usize, modulus: usize) -> usize {
    let mut out = val;
    loop {
        out = (out * factor) % 2147483647;
        if out % modulus == 0 {
            return out;
        }
    }
}

fn step2(vals: (usize, usize)) -> (usize, usize) {
    let a = step_mod(vals.0, 16807, 4);
    let b = step_mod(vals.1, 48271, 8);

    (a, b)
}

fn matches(vals: &(usize, usize)) -> bool {
    let m = 2 << 15;

    vals.0 % m == vals.1 % m
}

fn main() -> Result<()> {
    let input = (618, 814);
    //let input = (65, 8921);

    let mut state = input.clone();
    let mut n_matching = 0;
    for _ in 0..=40_000_000 {
        if matches(&state) {
            n_matching += 1;
        }
        state = step1(state);
    }
    println!("Part 1: {}", n_matching);

    let mut state = input.clone();
    let mut n_matching = 0;
    for _ in 0..=5_000_000 {
        if matches(&state) {
            n_matching += 1;
        }
        state = step2(state);
    }
    println!("Part 2: {}", n_matching);

    Ok(())
}
