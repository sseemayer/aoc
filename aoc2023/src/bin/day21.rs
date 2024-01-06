use std::collections::HashSet;

use anyhow::{anyhow, Result};
use aoc::{direction::Direction, map::ParseMapTile};
use image::{ImageBuffer, Rgb};
use strum::IntoEnumIterator;

type Map = aoc::map::Map<[i32; 2], Tile>;

#[derive(Debug, Clone)]
enum Tile {
    Elf,
    Plot,
    Rock,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Elf => write!(f, "🯅"),
            Tile::Plot => write!(f, "░"),
            Tile::Rock => write!(f, "█"),
        }
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'S' => Some(Tile::Elf),
            '.' => Some(Tile::Plot),
            '#' => Some(Tile::Rock),
            _ => None,
        }
    }
}

fn step(map: &Map, elf_positions: HashSet<[i32; 2]>) -> HashSet<[i32; 2]> {
    let ([imin, jmin], [imax, jmax]) = map.get_extent();
    assert_eq!(imin, 0);
    assert_eq!(jmin, 0);
    let height = imax + 1;
    let width = jmax + 1;

    let mut out = HashSet::new();
    for [i, j] in elf_positions {
        for direction in Direction::iter() {
            let [di, dj] = direction.dpos();

            let check_pos = [wrap(i + di, height), wrap(j + dj, width)];

            if let Some(Tile::Plot) = map.get(&check_pos) {
                out.insert([i + di, j + dj]);
            }
        }
    }

    out
}

/// i % dim, but also dealing with negative i
fn wrap(i: i32, dim: i32) -> i32 {
    (i + (i32::abs(i / dim) + 1) * dim) % dim
}

fn part1(map: &Map, start_pos: [i32; 2]) -> usize {
    let mut positions = HashSet::new();
    positions.insert(start_pos);
    for _i in 0..64 {
        positions = step(&map, positions);
    }

    positions.len()
}

fn part2(map: &Map, [istart, jstart]: [i32; 2]) -> usize {
    let ([imin, jmin], [imax, jmax]) = map.get_extent();
    assert_eq!(imin, 0);
    assert_eq!(jmin, 0);
    let width = jmax + 1;
    let height = imax + 1;

    let mut positions = HashSet::new();
    positions.insert([istart, jstart]);

    // one tile is 131 wide.
    // final number of steps is 202300 full-tile steps plus one half-tile (65) step,
    // which means that the last step will have the border of a tile boundary exactly hit.
    //
    // instead of simulating all, we can simulate five half-steps to get repeatable cells
    //
    //  01234
    // 0.🮣🮧🮢.
    // 1🮣🮠O🮡🮢
    // 2🮤OEO🮥
    // 3🮡🮢O🮣🮠
    // 4.🮡🮦🮠.

    for i in 0..(width * 2 + jmax / 2) {
        if (i + 1 + jmax / 2) % width == 0 {
            let img = display(map, &positions);
            img.save(format!("data/day21/step{}.png", i + 1))
                .expect("save image");
        }

        positions = step(map, positions);

        // println!("{} {}", i + 1, positions.len());
    }

    let ipmin = *positions.iter().map(|[i, _]| i).min().unwrap_or(&0);
    let jpmin = *positions.iter().map(|[_, j]| j).min().unwrap_or(&0);

    assert_eq!(ipmin, -height * 2 - imax / 2 + istart);
    assert_eq!(jpmin, -width * 2 - jmax / 2 + jstart);

    // slice up the positions into repeatable cells
    let mut cell_positions = [[0usize; 5]; 5];
    for [i, j] in positions {
        let ci = ((i - ipmin) / height) as usize;
        let cj = ((j - jpmin) / width) as usize;

        cell_positions[ci][cj] += 1;
    }

    let [
        [zero0, tl_odd0, t_even, tr_odd0, zero1], // a
        [tl_odd1, tl_even, odd0, tr_even, tr_odd1], // b
        [l_even, odd1, even, odd2, r_even],  // c
        [bl_odd0, bl_even, odd3, br_even, br_odd0], // d
        [zero2, bl_odd1, b_even, br_odd1, zero3], // e
    ] = cell_positions;

    assert_eq!(odd0, odd1);
    assert_eq!(odd0, odd2);
    assert_eq!(odd0, odd3);
    assert_eq!(tl_odd0, tl_odd1);
    assert_eq!(tr_odd0, tr_odd1);
    assert_eq!(bl_odd0, bl_odd1);
    assert_eq!(br_odd0, br_odd1);
    assert_eq!(zero0, 0);
    assert_eq!(zero1, 0);
    assert_eq!(zero2, 0);
    assert_eq!(zero3, 0);

    // expand the total number of positions using repeatable tiles
    //
    // step    even     odd   odd_diag    even_diag
    // 0.5     0        0     0           0
    //(1.5     1        0     0           4           )
    // 2.5     1        4     8           4
    //(3.5     9        4     8           12          )
    // 4.5     9        16    16          12
    //
    // n.5     (n-1)^2  n^2   4n          4(n-1)

    let expansions = 202300;

    let total = odd0 * expansions * expansions + // odd squarees
        even * (expansions - 1) * (expansions - 1) +  // even squarees
        tl_odd0 * expansions + tl_even * (expansions - 1) + // top-left diagonals
        tr_odd0 * expansions + tr_even * (expansions - 1) + // top-right diagonals
        bl_odd0 * expansions + bl_even * (expansions - 1) + // bottom-left diagonals
        br_odd0 * expansions + br_even * (expansions - 1) + // bottom-right diagonals
        t_even + r_even + b_even + l_even; // tips

    total
}

fn display(map: &Map, positions: &HashSet<[i32; 2]>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let ([immin, jmmin], [immax, jmmax]) = map.get_extent();
    assert_eq!(immin, 0);
    assert_eq!(jmmin, 0);

    let width = jmmax + 1;
    let height = immax + 1;

    let ipmin = *positions.iter().map(|[i, _]| i).min().unwrap_or(&0);
    let jpmin = *positions.iter().map(|[_, j]| j).min().unwrap_or(&0);
    let ipmax = *positions.iter().map(|[i, _]| i).max().unwrap_or(&0);
    let jpmax = *positions.iter().map(|[_, j]| j).max().unwrap_or(&0);

    let mut image = ImageBuffer::new((jpmax - jpmin + 1) as u32, (ipmax - ipmin + 1) as u32);

    for i in ipmin..=ipmax {
        for j in jpmin..=jpmax {
            let ci = ((i - ipmin) / height) as usize;
            let cj = ((j - jpmin) / width) as usize;

            let even_cell = (ci + cj) % 2 == 0;

            let luma = match map.get(&[wrap(i, height), wrap(j, width)]) {
                Some(Tile::Plot | Tile::Elf) => 255,
                Some(Tile::Rock) => 128,
                None => 64,
            };

            let color = if positions.contains(&[i, j]) {
                if even_cell {
                    Rgb([0, luma, 0])
                } else {
                    Rgb([luma, luma, 0])
                }
            } else if (i - ipmin) % height == 0 || (j - jpmin) % width == 0 {
                Rgb([0, 0, luma])
            } else {
                Rgb([luma, luma, luma])
            };

            *image.get_pixel_mut((j - jpmin) as u32, (i - ipmin) as u32) = color;
        }
    }

    image
}

fn load_map(path: &str) -> Result<(Map, [i32; 2])> {
    let mut map: Map = aoc::io::read_all(path)?.parse()?;
    let start_pos = map
        .find_one_where(|_p, t| matches!(t, Tile::Elf))
        .ok_or_else(|| anyhow!("Cannot find start position"))?;

    map.set(start_pos, Tile::Plot);

    Ok((map, start_pos))
}

fn main() -> Result<()> {
    let (map, start_pos) = load_map("data/day21/input")?;

    println!("Part 1: {}", part1(&map, start_pos));

    println!("Part 2: {}", part2(&map, start_pos));

    Ok(())
}
