use anyhow::Result;
use aoc::map::ParseMapTile;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    On,
    Off,
}

impl Tile {
    fn to_bin(&self) -> usize {
        match self {
            Tile::On => 1,
            Tile::Off => 0,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::On => write!(f, "█"),
            Tile::Off => write!(f, "░"),
        }
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Tile::On),
            '.' => Some(Tile::Off),
            _ => None,
        }
    }
}

type Map = aoc::map::Map<[i64; 2], Tile>;

fn enhance(map: &Map, outside: Tile, iea: &Map) -> (Map, Tile) {
    let ([imin, jmin], [imax, jmax]) = map.get_extent();

    let mut out = Map::new();
    for i in imin - 1..=imax + 1 {
        for j in jmin - 1..=jmax + 1 {
            let mut code = 0;
            for iofs in -1..=1 {
                for jofs in -1..=1 {
                    let tile = map.get(&[i + iofs, j + jofs]).unwrap_or(&outside);
                    code <<= 1;
                    code += tile.to_bin();
                }
            }

            out.set([i, j], *iea.get(&[0, code as i64]).unwrap());
        }
    }

    let new_outside = match outside {
        Tile::On => *iea.get(&[0, 511]).unwrap(),
        Tile::Off => *iea.get(&[0, 0]).unwrap(),
    };

    (out, new_outside)
}

fn main() -> Result<()> {
    let mut reader = BufReader::new(File::open("data/day20/input")?);

    let iea = {
        let mut iea = String::new();
        reader.read_line(&mut iea)?;
        Map::read(&mut iea.as_bytes())?
    };

    println!("IEA: {}", iea);

    reader.read_line(&mut String::new())?;

    let mut map = Map::read(&mut reader)?;
    let mut outside = Tile::Off;

    for i in 0..=50 {
        let n_on = map.find_all(&Tile::On).len();
        println!(
            "After step {} ({} on) (extent {:?}):\n{}",
            i,
            n_on,
            map.get_extent(),
            map
        );
        let (m, o) = enhance(&map, outside, &iea);
        map = m;
        outside = o;
    }

    Ok(())
}
