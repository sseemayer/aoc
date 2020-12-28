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

fn is_valid_hash(hash: &[u8]) -> bool {
    hash[0] == 0 && hash[1] == 0 && hash[2] & 0xf0 == 0
}

fn find_password(door_id: &str, length: usize) -> (String, String) {
    let mut password1 = String::new();
    let mut password2: Vec<char> = (0..length).map(|_| ' ').collect();
    let mut i = 0;

    while password1.len() < length || password2.iter().filter(|c| **c != ' ').count() < length {
        let attempt = format!("{}{}", door_id, i);
        let digest = md5::compute(attempt);
        if is_valid_hash(&(*digest)[..]) {
            let hex_digit_1 = digest[2] & 0x0f;
            let hex_digit_2 = (digest[3] & 0xf0) >> 4;

            let next_char = format!("{:x}", hex_digit_1);

            if password1.len() < length {
                password1.extend(next_char.chars());
            }

            if (hex_digit_1 as usize) < length && password2[hex_digit_1 as usize] == ' ' {
                password2[hex_digit_1 as usize] =
                    std::char::from_digit(hex_digit_2 as u32, 16).expect("hex digit");
            }

            println!("{} {}", password1, password2.iter().collect::<String>());
        }

        i += 1;
    }

    (password1, password2.iter().collect())
}

fn main() -> Result<()> {
    let door_id = "uqwqemis";

    let (part1, part2) = find_password(door_id, 8);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        Ok(())
    }
}
