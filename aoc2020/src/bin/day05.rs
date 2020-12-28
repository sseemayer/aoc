use snafu::{ResultExt, Snafu};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error on '{}': {}", filename, source))]
    Io {
        filename: String,
        source: std::io::Error,
    },
}

type Result<T> = std::result::Result<T, Error>;

struct BinaryRange {
    lower: usize,
    upper: usize,
}

impl BinaryRange {
    fn new(upper: usize) -> Self {
        BinaryRange { lower: 0, upper }
    }

    fn mid(&self) -> usize {
        (self.lower + self.upper) / 2
    }

    fn lower_half(&mut self) {
        self.upper = self.mid();
    }

    fn upper_half(&mut self) {
        self.lower = self.mid() + 1;
    }
}

impl std::fmt::Display for BinaryRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.lower, self.upper)
    }
}

fn find_seat(seat_string: &str) -> (usize, usize, usize) {
    let mut row = BinaryRange::new(127);
    let mut col = BinaryRange::new(7);

    for c in seat_string.chars() {
        // print!("{} {} --{}--> ", row, col, c);
        match c {
            'F' => row.lower_half(),
            'B' => row.upper_half(),
            'L' => col.lower_half(),
            'R' => col.upper_half(),
            _ => unreachable!(),
        }
        // println!("{} {}", row, col);
    }

    let row = row.lower;
    let col = col.lower;

    (row, col, row * 8 + col)
}

fn main() -> Result<()> {
    let filename = "data/day05/input";
    let br = BufReader::new(File::open(filename).context(Io {
        filename: filename.to_string(),
    })?);

    let mut highest_id = 0;
    let mut found_seats: HashSet<usize> = HashSet::new();
    for line in br.lines() {
        let line = line.context(Io {
            filename: filename.to_string(),
        })?;

        let (_row, _col, seat_id) = find_seat(&line);
        //println!("{}: {} / {}: {}", line, row, col, seat_id);

        found_seats.insert(seat_id);
        if seat_id > highest_id {
            highest_id = seat_id;
        }
    }

    println!("Highest ID is {}", highest_id);

    for i in 0..highest_id {
        if found_seats.contains(&i)
            && !found_seats.contains(&(i + 1))
            && found_seats.contains(&(i + 2))
        {
            println!("Found free seat {}", i + 1);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partitioning() {
        assert_eq!(find_seat("FBFBBFFRLR"), (44, 5, 357));
        assert_eq!(find_seat("BFFFBBFRRR"), (70, 7, 567));
        assert_eq!(find_seat("FFFBBBFRRR"), (14, 7, 119));
        assert_eq!(find_seat("BBFFBBFRLL"), (102, 4, 820));
    }
}
