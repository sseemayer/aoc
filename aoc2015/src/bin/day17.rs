use aoc2015::util::read_to_parsed_lines;
use aoc2015::{format_err, Error, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let jars: Vec<u16> = read_to_parsed_lines("data/day17/input", &|l| {
        l.parse().map_err(|e: std::num::ParseIntError| e.into())
    })?;

    let mut ways = 0;
    let mut min_number = jars.len();
    let mut min_count = 0;
    for n in 1..=jars.len() {
        for combo in jars.iter().combinations(n) {
            let total_volume: u16 = combo.iter().map(|v| **v).sum();
            println!("{} = {:?}", total_volume, combo);

            if total_volume == 150 {
                ways += 1;

                if combo.len() < min_number {
                    min_number = combo.len();
                    min_count = 1;
                } else if combo.len() == min_number {
                    min_count += 1;
                }
            }
        }
    }

    println!(
        "Found {} combinations, smallest combinations ({}) use {} jars",
        ways, min_count, min_number
    );

    Ok(())
}
