use std::fs::File;
use std::io::{BufRead, BufReader};

use failure::Error;

type Result<T> = std::result::Result<T, Error>;

fn fuel_for_mass(m: &u32) -> u32 {
    return std::cmp::max(0, *m as i64 / 3 - 2) as u32;
}

fn fuel_for_mass_and_fuel(m: &u32) -> u32 {
    let mut total_fuel = fuel_for_mass(m);

    let mut extra_fuel = fuel_for_mass(&total_fuel);
    while extra_fuel > 0 {
        total_fuel += extra_fuel;
        extra_fuel = fuel_for_mass(&extra_fuel);
    }

    total_fuel
}

fn main() -> Result<()> {
    let f = File::open("data/day01/input")?;
    let br = BufReader::new(f);

    let masses: Vec<u32> = br
        .lines()
        .map(|l| l?.parse().map_err(|e: std::num::ParseIntError| e.into()))
        .collect::<Result<Vec<u32>>>()?;

    let fuel_for_modules: u32 = masses.iter().map(fuel_for_mass).sum();
    println!("Fuel needed for modules: {}", fuel_for_modules);

    let fuel_total: u32 = masses.iter().map(fuel_for_mass_and_fuel).sum();
    println!("Fuel total needed: {}", fuel_total);
    Ok(())
}
