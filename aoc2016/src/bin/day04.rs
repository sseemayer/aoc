use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;
use snafu::{ResultExt, Snafu};

type Result<T> = std::result::Result<T, Error>;

lazy_static! {
    static ref RE_ROOM: Regex = Regex::new(r"^([a-z-]+)-([0-9]+)\[([a-z]+)\]$").unwrap();
}

#[derive(Debug)]
struct Room {
    name: String,
    id: usize,
    checksum: String,
}

impl std::str::FromStr for Room {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let captures = RE_ROOM.captures(s).ok_or(Error::ParseRoom {
            data: s.to_string(),
        })?;

        let name = captures.get(1).unwrap().as_str().to_string();
        let id = captures.get(2).unwrap().as_str();
        let id: usize = id.parse().context(ParseInt {
            data: id.to_string(),
        })?;
        let checksum = captures.get(3).unwrap().as_str().to_string();

        Ok(Room { name, id, checksum })
    }
}

impl Room {
    fn is_valid(&self) -> bool {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for c in self.name.chars() {
            if c == '-' {
                continue;
            }
            *counts.entry(c).or_insert(0) += 1;
        }

        let mut counts: Vec<(usize, char)> = counts.into_iter().map(|(c, n)| (n, c)).collect();
        counts.sort_by_key(|c| (std::cmp::Reverse(c.0), c.1));

        // println!(
        //     "{} {}",
        //     self.checksum,
        //     counts[..self.checksum.len()]
        //         .iter()
        //         .map(|c| c.1)
        //         .collect::<String>()
        // );

        for (a, (_, b)) in self.checksum.chars().zip(counts) {
            if a != b {
                return false;
            }
        }

        true
    }

    fn decrypt(&self) -> String {
        let mut out = String::new();
        for c in self.name.chars() {
            let d = if c == '-' {
                ' '
            } else {
                (((((c as usize) - ('a' as usize) + self.id) % 26) + ('a' as usize)) as u8) as char
            };

            out.push(d);
        }

        out
    }
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Int format error for '{}': {}", data, source))]
    ParseInt {
        data: String,
        source: std::num::ParseIntError,
    },

    #[snafu(display("Room format error for '{}'", data))]
    ParseRoom { data: String },
}

fn main() -> Result<()> {
    let rooms: Vec<Room> = std::fs::read_to_string("data/day04/input")
        .context(Io)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_>>()?;

    let valid_rooms: Vec<Room> = rooms.into_iter().filter(|r| r.is_valid()).collect();

    let sector_id_sum = valid_rooms.iter().map(|r| r.id).sum::<usize>();

    println!("Part 1: Sector ID sum is {}", sector_id_sum);

    let mut room_id = 0;
    for room in valid_rooms {
        let decrypted = room.decrypt();
        println!("room {}: {}", room.id, room.decrypt());

        if decrypted == "northpole object storage" {
            room_id = room.id;
        }
    }

    println!("\nPart 2: Correct Sector ID is {}", room_id);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_validation() -> Result<()> {
        assert_eq!(
            "aaaaa-bbb-z-y-x-123[abxyz]"
                .parse::<Room>()
                .unwrap()
                .is_valid(),
            true
        );

        assert_eq!(
            "totally-real-room-200[decoy]"
                .parse::<Room>()
                .unwrap()
                .is_valid(),
            false
        );

        Ok(())
    }

    #[test]
    fn test_room_decryption() -> Result<()> {
        assert_eq!(
            "qzmt-zixmtkozy-ivhz-343[asdf]"
                .parse::<Room>()
                .unwrap()
                .decrypt(),
            "very encrypted name".to_string()
        );

        Ok(())
    }
}
