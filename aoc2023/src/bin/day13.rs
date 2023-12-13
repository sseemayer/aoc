use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

type Map = aoc::map::Map<[i32; 2], Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile;

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#")
    }
}

impl aoc::map::ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Self),
            _ => None,
        }
    }
}

fn load_maps(path: &str) -> Result<Vec<Map>> {
    let mut out = Vec::new();

    let mut buffer = String::new();
    for line in BufReader::new(File::open(path)?).lines() {
        let line = line?;

        if line.trim().is_empty() {
            if !buffer.is_empty() {
                let map: Map = buffer.parse()?;
                out.push(map);
                buffer.clear();
            }
        } else {
            buffer.extend(line.chars());
            buffer.push('\n');
        }
    }

    if !buffer.is_empty() {
        let map: Map = buffer.parse()?;
        out.push(map);
    }
    Ok(out)
}

fn check_mirror_i(map: &Map, i: i32, expected_mismatches: usize) -> bool {
    // the mirror planes are (0..=i).rev() and (i+1)..=imax
    //
    // i  k
    // 0  3|#...##..#
    // 1  2|#....#..#
    // 2  1|..##..###
    // 3  0|#####.##.
    // 4  0|#####.##.
    // 5  1|..##..###
    // 6  2|#....#..#
    //
    //

    let ([imin, jmin], [imax, jmax]) = map.get_extent();

    let height = i32::max(i, imax - i + 1);
    let mut mismatches = 0;

    for k in 0..height {
        let iup = i - k;
        let idn = i + k + 1;

        if iup < imin || idn > imax {
            continue;
        }

        for j in jmin..=jmax {
            let t_up = map.get(&[iup, j]);
            let t_dn = map.get(&[idn, j]);

            if t_up != t_dn {
                mismatches += 1;

                if mismatches > expected_mismatches {
                    return false;
                }
            }
        }
    }

    mismatches == expected_mismatches
}

fn check_mirror_j(map: &Map, j: i32, expected_mismatches: usize) -> bool {
    let ([imin, jmin], [imax, jmax]) = map.get_extent();

    let width = i32::max(j, jmax - j + 1);
    let mut mismatches = 0;

    for k in 0..width {
        let jl = j - k;
        let jr = j + k + 1;

        if jl < jmin || jr > jmax {
            continue;
        }

        for i in imin..=imax {
            let jl = map.get(&[i, jl]);
            let jr = map.get(&[i, jr]);

            if jl != jr {
                mismatches += 1;

                if mismatches > expected_mismatches {
                    return false;
                }
            }
        }
    }

    mismatches == expected_mismatches
}

fn find_mirror_i(map: &Map, expected_mismatches: usize) -> Option<i32> {
    let ([imin, _jmin], [imax, _jmax]) = map.get_extent();
    for i in imin..imax {
        if check_mirror_i(map, i, expected_mismatches) {
            return Some(i);
        }
    }
    None
}

fn find_mirror_j(map: &Map, expected_mismatches: usize) -> Option<i32> {
    let ([_imin, jmin], [_imax, jmax]) = map.get_extent();
    for j in jmin..jmax {
        if check_mirror_j(map, j, expected_mismatches) {
            return Some(j);
        }
    }
    None
}

fn find_mirrors(maps: &[Map], expected_mismatches: usize) -> i32 {
    let mut sum_i = 0;
    let mut sum_j = 0;

    for map in maps {
        if let Some(i) = find_mirror_i(map, expected_mismatches) {
            sum_i += i + 1;
        } else if let Some(j) = find_mirror_j(map, expected_mismatches) {
            sum_j += j + 1;
        }
    }

    sum_i * 100 + sum_j
}

fn main() -> Result<()> {
    let maps = load_maps("data/day13/input")?;

    println!("Part 1: {}", find_mirrors(&maps, 0));
    println!("Part 2: {}", find_mirrors(&maps, 1));

    Ok(())
}
