use std::fs::File;
use std::io::{BufRead, BufReader};

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Number parsing error: {}", source))]
    ParseNumber { source: std::num::ParseIntError },
}

type Result<T> = std::result::Result<T, Error>;

fn next_departure(current_time: i64, bus_id: i64) -> i64 {
    ((current_time as f64 / bus_id as f64).ceil() as i64) * bus_id
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn parse_itinerary(s: &str) -> Result<Vec<Option<i64>>> {
    s.split(",")
        .map(|b| {
            if b == "x" {
                Ok(None)
            } else {
                Ok(Some(b.parse().context(ParseNumber)?))
            }
        })
        .collect::<Result<_>>()
}

fn calculate_buses_and_offsets(itinerary: &[Option<i64>]) -> (Vec<i64>, Vec<i64>) {
    let buses: Vec<i64> = itinerary.iter().filter_map(|b| *b).collect();
    let offsets: Vec<i64> = itinerary
        .iter()
        .enumerate()
        .filter_map(|(i, b)| match b {
            Some(bid) => Some(*bid as i64 - i as i64),
            None => None,
        })
        .collect();

    (buses, offsets)
}

fn solve_earliest_departure(itinerary: &[Option<i64>], time: i64) -> (i64, i64) {
    let (buses, offsets) = calculate_buses_and_offsets(&itinerary[..]);

    println!("t={} buses:\n{:#?}", time, buses);

    let mut earliest_departure: Option<(i64, i64)> = None;
    for bus in &buses {
        let departure = next_departure(time, *bus);
        if earliest_departure == None || departure < earliest_departure.unwrap().1 {
            earliest_departure = Some((*bus, departure));
        }
    }

    earliest_departure.unwrap()
}

fn solve_contest(itinerary: &[Option<i64>]) -> Option<i64> {
    let (buses, offsets) = calculate_buses_and_offsets(itinerary);
    let t = chinese_remainder(&offsets[..], &buses[..])?;
    Some(t)
}

fn main() -> Result<()> {
    let filename = "data/day13/input";
    let f = File::open(filename).context(Io)?;

    let lines: Vec<String> = BufReader::new(f)
        .lines()
        .map(|l| Ok(l.context(Io)?.to_string()))
        .collect::<Result<Vec<String>>>()?;

    let time: i64 = lines[0].parse().context(ParseNumber)?;
    let itinerary = parse_itinerary(&lines[1])?;

    let (bus_id, departure_time) = solve_earliest_departure(&itinerary, time);

    println!(
        "Part 1: Earliest departure with bus {} at {}. Answer={}",
        bus_id,
        departure_time,
        bus_id * (departure_time - time)
    );

    // part 2
    //
    // M = prod(bus ids)
    //
    // find t so that for all i
    // t = o[i] (mod b[i])
    //
    // o is offsets and b is bus ids (i.e. their schedules)

    let t = solve_contest(&itinerary).unwrap();

    println!("Part 2: Earliest time is {}", t);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_departure() {
        assert_eq!(next_departure(929, 7), 931);
        assert_eq!(next_departure(929, 13), 936);
        assert_eq!(next_departure(929, 59), 944);
        assert_eq!(next_departure(929, 31), 930);
        assert_eq!(next_departure(929, 19), 931);

        assert_eq!(next_departure(931, 7), 931);
        assert_eq!(next_departure(931, 13), 936);
        assert_eq!(next_departure(931, 59), 944);
        assert_eq!(next_departure(931, 31), 961);
        assert_eq!(next_departure(931, 19), 931);
    }

    #[test]
    fn test_contest() {
        assert_eq!(
            solve_contest(&parse_itinerary("7,13,x,x,59,x,31,19").unwrap()).unwrap(),
            1068781
        );
        // The earliest timestamp that matches the list 17,x,13,19 is 3417.
        assert_eq!(
            solve_contest(&parse_itinerary("17,x,13,19").unwrap()).unwrap(),
            3417
        );
        // 67,7,59,61 first occurs at timestamp 754018.
        assert_eq!(
            solve_contest(&parse_itinerary("67,7,59,61").unwrap()).unwrap(),
            754018
        );
        // 67,x,7,59,61 first occurs at timestamp 779210.
        assert_eq!(
            solve_contest(&parse_itinerary("67,x,7,59,61").unwrap()).unwrap(),
            779210
        );
        // 67,7,x,59,61 first occurs at timestamp 1261476.
        assert_eq!(
            solve_contest(&parse_itinerary("67,7,x,59,61").unwrap()).unwrap(),
            1261476
        );
        // 1789,37,47,1889 first occurs at timestamp 1202161486.
        assert_eq!(
            solve_contest(&parse_itinerary("1789,37,47,1889").unwrap()).unwrap(),
            1202161486
        );
    }
}
