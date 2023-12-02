use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Context, Result};
use aoc::map::ParseMapTile;
use colored::Colorize;

type Map = aoc::map::Map<[i16; 2], Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    On,
    Off,
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

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::On => "#".green(),
                Tile::Off => ".".white(),
            }
        )
    }
}

#[derive(Debug)]
struct RuleBase {
    rules2: HashMap<usize, Map>,
    rules3: HashMap<usize, Map>,
}

fn print_rules(rules: &HashMap<usize, Map>) {
    for (k, v) in rules.iter() {
        println!("{}\n{}\n\n", k, v);
    }
}

impl RuleBase {
    fn print(&self) {
        print_rules(&self.rules2);
        print_rules(&self.rules3);
    }

    fn step(&self, map: &Map) -> Result<Map> {
        let (min, max) = map.get_extent();
        let height = max[0] - min[0] + 1;
        let width = max[1] - min[1] + 1;
        assert!(width == height);

        let step = if width % 2 == 0 { 2 } else { 3 };
        let mut out = Map::new();

        // i and j are cell indices
        for i in 0..(height / step) {
            for j in 0..(width / step) {
                let mut cell = Map::new();
                for ic in 0..step {
                    for jc in 0..step {
                        cell.set(
                            [ic, jc],
                            *map.get(&[i * step + ic, j * step + jc])
                                .unwrap_or(&Tile::Off),
                        );
                    }
                }

                let out_cell = self.match_rule(&cell)?;

                let (out_min, out_max) = out_cell.get_extent();
                let out_height = out_max[0] - out_min[0] + 1;
                let out_width = out_max[1] - out_min[1] + 1;

                for (pos, tile) in out_cell.data.iter() {
                    out.set([out_height * i + pos[0], out_width * j + pos[1]], *tile);
                }
            }
        }

        Ok(out)
    }

    fn match_rule(&self, cell: &Map) -> Result<&Map> {
        let cell_hash = canonical_hash_from(&cell);

        let out = if cell.get_extent().1[0] == 1 {
            self.rules2
                .get(&cell_hash)
                .ok_or(anyhow!("No 2-cell rule for hash '{}'", cell_hash))?
        } else {
            self.rules3
                .get(&cell_hash)
                .ok_or(anyhow!("No 3-cell rule for hash '{}'", cell_hash))?
        };

        Ok(out)
    }
}

fn hash_from(from: &Map) -> usize {
    let mut out = 0;

    let (min, max) = from.get_extent();
    for i in num::iter::range_inclusive(min[0], max[0]) {
        for j in num::iter::range_inclusive(min[1], max[1]) {
            out <<= 1;
            let is_on = from.get(&[i, j]).unwrap_or(&Tile::Off) == &Tile::On;

            if is_on {
                out += 1;
            }
        }
    }

    out
}

fn canonical_hash_from(from: &Map) -> usize {
    let mut current = from.clone();
    let mut min_hash = usize::MAX;

    for _ in 0..4 {
        let hash = hash_from(&current);
        min_hash = usize::min(min_hash, hash);
        current = current.rotate_right();
    }

    current = current.flip(0);

    for _ in 0..4 {
        let hash = hash_from(&current);
        min_hash = usize::min(min_hash, hash);
        current = current.rotate_right();
    }

    return min_hash;
}

fn parse_map_line(map: &str) -> Result<Map> {
    map.replace('/', "\n").parse().context("Reading map")
}

fn load_input(path: &str) -> Result<RuleBase> {
    let reader = BufReader::new(File::open(path).context("Loading input")?);
    let mut rules2 = HashMap::new();
    let mut rules3 = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let (from, to) = line
            .split_once(" => ")
            .ok_or_else(|| anyhow!("Cannot split line: '{}", line))?;

        let from = parse_map_line(from)?;
        let to = parse_map_line(to)?;

        let (_min, max) = from.get_extent();
        let from_hash = canonical_hash_from(&from);
        if max[0] == 1 {
            rules2.insert(from_hash, to);
        } else if max[0] == 2 {
            rules3.insert(from_hash, to);
        } else {
            return Err(anyhow!("Incorrect dimensionality: '{}'", line));
        }
    }

    Ok(RuleBase { rules2, rules3 })
}

fn main() -> Result<()> {
    let rules = load_input("data/day21/input")?;
    let mut state = parse_map_line(".#./..#/###")?;

    rules.print();

    println!(
        "After {}:\n{}{} are on\n",
        0,
        state,
        state.find_all(&Tile::On).len()
    );

    for i in 0..18 {
        state = rules.step(&state)?;
        println!(
            "After {}:\n{}{} are on\n",
            i + 1,
            state,
            state.find_all(&Tile::On).len()
        );
    }

    Ok(())
}
