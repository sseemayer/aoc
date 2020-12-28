use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Number parsing error for '{}': {}", data, source))]
    ParseNumber {
        data: String,
        source: std::num::ParseIntError,
    },
}

type Result<T> = std::result::Result<T, Error>;

fn transform(subject: usize, loop_size: usize) -> usize {
    // To transform a subject number, start with the value 1.
    let mut v = 1;

    // Then, a number of times called the loop size, perform the following steps:
    for _ in 0..loop_size {
        // Set the value to itself multiplied by the subject number.
        // Set the value to the remainder after dividing the value by 20201227.
        v = (v * subject) % 20201227;
    }

    v
}

fn find_loop_size(pubkey: usize, subject: usize) -> usize {
    let mut ls = 0;
    let mut v = 1;

    while v != pubkey {
        ls += 1;
        v = (v * subject) % 20201227;
    }

    ls
}

fn main() -> Result<()> {
    let nums: Vec<usize> = std::fs::read_to_string("data/day25/input")
        .context(Io)?
        .lines()
        .map(|l| {
            l.parse().context(ParseNumber {
                data: l.to_string(),
            })
        })
        .collect::<Result<_>>()?;

    let card_pubkey = nums[0];
    let door_pubkey = nums[1];

    let card_loop_size = find_loop_size(card_pubkey, 7);
    let door_loop_size = find_loop_size(door_pubkey, 7);

    let ec1 = transform(door_pubkey, card_loop_size);
    let ec2 = transform(card_pubkey, door_loop_size);

    println!("{} {}", ec1, ec2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_loop_size() {
        assert_eq!(find_loop_size(5764801, 7), 8);
        assert_eq!(find_loop_size(17807724, 7), 11);
    }
}
