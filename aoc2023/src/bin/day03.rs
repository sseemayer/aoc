use std::{collections::HashMap, fmt::Display, ops::Deref};

use anyhow::Result;

use aoc::map::ParseMapTile;

type Map = aoc::map::Map<[i32; 2], Tile>;

#[derive(Debug, Clone)]
struct Tile(char);

impl Deref for Tile {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        if c != '.' {
            Some(Tile(c))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Part {
    symbol: char,
    part_numbers: Vec<u32>,
}

/// scan through map to find contiguous numbers
/// this is not filtered by symbol adjacency yet
fn find_numbers(map: &Map) -> Vec<([i32; 2], i32, u32)> {
    let ([imin, jmin], [imax, jmax]) = map.get_extent();

    let mut current_number = None;
    let mut out = Vec::new();
    for i in imin..=imax {
        // overscan by 1 to ensure we end numbers
        for j in jmin..=(jmax + 1) {
            match (current_number.as_mut(), map.get(&[i, j])) {
                (None, Some(t)) if t.is_digit(10) => {
                    // starting a new number
                    let d = t.to_digit(10).expect("valid digit");
                    current_number = Some((j, d));
                }
                (Some((_js, n)), Some(t)) if t.is_digit(10) => {
                    // continuing a number
                    let d = t.to_digit(10).expect("valid digit");
                    *n = (*n * 10) + d;
                }
                (Some((js, n)), _) => {
                    // ending a number
                    out.push(([i, *js], j - 1, *n));
                    current_number = None;
                }
                (None, _) => {
                    // not in a number and not starting one
                }
            }
        }
    }

    out
}

/// using previously found numbers, find adjacent parts
/// will group all part numbers by part position
fn find_parts(map: &Map, numbers: &Vec<([i32; 2], i32, u32)>) -> HashMap<[i32; 2], Part> {
    let mut out = HashMap::new();

    for ([i, js], je, part_number) in numbers {
        let adjacent_symbol = (*js..=*je).find_map(|j| {
            (-1..=1).find_map(|di| {
                (-1..=1).find_map(|dj| {
                    let spos = [i + di, j + dj];

                    if let Some(t) = map.get(&spos) {
                        if !t.is_digit(10) {
                            return Some((spos, j, t.0));
                        }
                    }

                    None
                })
            })
        });

        if let Some((pos, _j, symbol)) = adjacent_symbol {
            //println!("{:?}: adjacent {}", [*i, j], symbol);

            out.entry(pos)
                .or_insert_with(|| Part {
                    symbol,
                    part_numbers: Vec::new(),
                })
                .part_numbers
                .push(*part_number);
        }
    }

    out
}

fn main() -> Result<()> {
    let map: Map = aoc::io::read_all("data/day03/input")?.parse()?;
    let numbers = find_numbers(&map);

    let parts = find_parts(&map, &numbers);

    let part_number_sum: u32 = parts
        .values()
        .map(|p| p.part_numbers.iter().sum::<u32>())
        .sum();

    println!("Part 1: {}", part_number_sum);

    let gear_ratio_sum: u32 = parts
        .values()
        .filter_map(|p| {
            if p.symbol != '*' || p.part_numbers.len() != 2 {
                return None;
            }

            Some(p.part_numbers.iter().product::<u32>())
        })
        .sum();

    println!("Part 2: {}", gear_ratio_sum);

    Ok(())
}
