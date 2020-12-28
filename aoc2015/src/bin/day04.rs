use aoc2015::util::read_to_string;
use aoc2015::Result;
use md5::Digest;

fn check_5(prefix: &str, n: usize) -> bool {
    let hash: Digest = md5::compute(format!("{}{}", prefix, n).as_bytes());

    if hash.0[0] != 0 {
        return false;
    }
    if hash.0[1] != 0 {
        return false;
    }
    if hash.0[2] & 0b11110000 != 0 {
        return false;
    }

    true
}

fn check_6(prefix: &str, n: usize) -> bool {
    let hash: Digest = md5::compute(format!("{}{}", prefix, n).as_bytes());

    if hash.0[0] != 0 {
        return false;
    }
    if hash.0[1] != 0 {
        return false;
    }
    if hash.0[2] != 0 {
        return false;
    }

    true
}

fn main() -> Result<()> {
    let data = read_to_string("data/day04/input")?.trim().to_owned();

    let mut n = 0;
    while !check_5(&data, n) {
        n += 1;
    }

    println!("First result: {}", n);

    while !check_6(&data, n) {
        n += 1;
    }

    println!("Second result: {}", n);

    Ok(())
}
