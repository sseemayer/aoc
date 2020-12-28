use std::collections::{HashMap, HashSet};

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

fn make_hash(n: usize, salt: &str, stretching: usize) -> [u8; 16] {
    let mut hash = *md5::compute(format!("{}{}", salt, n));

    for _ in 0..stretching {
        let newval = hex::encode(hash);
        hash = *md5::compute(newval.as_bytes());
    }

    hash
}

/// convert 16 bytes of data into 32 hexadecimal digits
fn to_hex_digits(data: &[u8; 16]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, d) in data.iter().enumerate() {
        out[i * 2] = (d & 0xf0) >> 4;
        out[i * 2 + 1] = d & 0x0f;
    }
    out
}

fn find_hex_tuples(data: &[u8; 16], search_length: usize, only_one: bool) -> HashSet<u8> {
    let data = to_hex_digits(data);
    let mut current = data[0];
    let mut length = 1;
    let mut out = HashSet::new();
    for digit in &data[1..] {
        if *digit != current {
            current = *digit;
            length = 1;
        } else {
            length += 1;

            if length >= search_length {
                out.insert(current);
                length = 0;

                if only_one {
                    break;
                }
            }
        }
    }
    out
}

fn derive_key(input: &str, stretching: usize) -> usize {
    let mut quintuplets: HashMap<usize, HashSet<u8>> = HashMap::new();
    let mut triplets: HashMap<usize, HashSet<u8>> = HashMap::new();

    let mut i = 0;
    let mut found = 0;

    loop {
        let future = make_hash(i, input, stretching);

        quintuplets.insert(i, find_hex_tuples(&future, 5, false));
        triplets.insert(i, find_hex_tuples(&future, 3, true));

        if i >= 1000 {
            let j = i - 1000;

            'outer: for candidate_triplet in &triplets[&j] {
                for k in (i - 999)..=i {
                    if quintuplets[&k].contains(&candidate_triplet) {
                        println!(
                            "Key #{:3}: {} has 3x{:x} and {} has 5x{:x}:\n{:8}: {:x?}\n{:8}: {:x?}\n",
                            found + 1,
                            j,
                            candidate_triplet,
                            k,
                            candidate_triplet,
                            j,
                            make_hash(j, input, stretching),
                            k,
                            make_hash(k, input, stretching)
                        );

                        found += 1;

                        if found >= 64 {
                            return j;
                        }

                        break 'outer;
                    }
                }
            }
        }

        i += 1;
    }
}

fn main() -> Result<()> {
    //let input = "abc";
    let input = "ngcjuoqr";

    println!("Part 1: {}", derive_key(input, 0));
    println!("Part 2: {}", derive_key(input, 2016));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digest() -> Result<()> {
        assert_eq!(
            make_hash(18, "abc", 0),
            [
                0x00, 0x34, 0xe0, 0x92, 0x3c, 0xc3, 0x88, 0x87, 0xa5, 0x7b, 0xd7, 0xb1, 0xd4, 0xf9,
                0x53, 0xdf
            ]
        );

        assert_eq!(
            make_hash(0, "abc", 0),
            [
                0x57, 0x75, 0x71, 0xbe, 0x4d, 0xe9, 0xdc, 0xce, 0x85, 0xa0, 0x41, 0xba, 0x04, 0x10,
                0xf2, 0x9f,
            ]
        );

        assert_eq!(
            make_hash(0, "abc", 1),
            [
                0xee, 0xc8, 0x0a, 0x0c, 0x92, 0xdc, 0x8a, 0x07, 0x77, 0xc6, 0x19, 0xd9, 0xbb, 0x51,
                0xe9, 0x10,
            ]
        );

        assert_eq!(
            make_hash(0, "abc", 2016),
            [
                0xa1, 0x07, 0xff, 0x63, 0x48, 0x56, 0xbb, 0x30, 0x01, 0x38, 0xca, 0xc6, 0x56, 0x8c,
                0x0f, 0x24,
            ]
        );

        Ok(())
    }

    #[test]
    fn test_hex_digits() -> Result<()> {
        assert_eq!(
            to_hex_digits(&[
                0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54,
                0x32, 0x10
            ]),
            [
                0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x0,
                0xf, 0xe, 0xd, 0xc, 0xb, 0xa, 0x9, 0x8, 0x7, 0x6, 0x5, 0x4, 0x3, 0x2, 0x1, 0x0
            ]
        );

        Ok(())
    }

    #[test]
    fn test_hex_quintuplets() -> Result<()> {
        assert_eq!(
            find_hex_tuples(
                &[
                    0x00, 0x0a, 0xaa, 0xaa, 0xbb, 0xcc, 0xcc, 0xcc, 0x01, 0x02, 0x03, 0x04, 0x05,
                    0x06, 0x07, 0x08
                ],
                5,
                false
            ),
            vec![0xa, 0xc].into_iter().collect()
        );

        Ok(())
    }

    #[test]
    fn test_derive_key() -> Result<()> {
        assert_eq!(derive_key("abc", 0), 22728);
        assert_eq!(derive_key("ngcjuoqr", 0), 18626);
        // assert_eq!(derive_key("abc", 2016), 22551);

        Ok(())
    }
}
