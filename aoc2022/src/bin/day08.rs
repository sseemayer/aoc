use std::{fs::File, io::Read};

use anyhow::{Context, Result};

use aoc::map::ParseMapTile;
use colored::Colorize;

#[derive(Debug, Clone)]
struct Tile {
    height: u8,
    visible: Option<bool>,
    most_scenic: bool,
    scenic_score: Option<u32>,
}

type Map = aoc::map::Map<[i32; 2], Tile>;

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = format!("{}", self.height);

        if self.most_scenic {
            return write!(f, "{}", height.blue());
        }

        match self.visible {
            Some(true) => write!(f, "{}", height.red()),
            Some(false) => write!(f, "{}", height.green()),
            None => write!(f, "{}", height),
        }
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        c.to_string().parse::<u8>().ok().map(|t| Tile {
            height: t,
            visible: None,
            most_scenic: false,
            scenic_score: None,
        })
    }
}

fn parse(path: &str) -> Result<Map> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;

    buffer.parse().context("Parse map")
}

/// cast a ray from a starting pos into a direction
/// returns true if the ray hits the outside of the map
/// returns false if the ray was obstructed by another tree
fn raycast(map: &Map, height: u8, pos: [i32; 2], dir: [i32; 2]) -> (bool, usize) {
    let ([min_i, min_j], [max_i, max_j]) = map.get_extent();

    let mut trees = 0;
    let [mut i, mut j] = pos;
    let [di, dj] = dir;
    i += di;
    j += dj;

    while i >= min_i && i <= max_i && j >= min_j && j <= max_j {
        if let Some(t) = map.get(&[i, j]) {
            trees += 1;
            if t.height >= height {
                return (false, trees);
            }
        }

        i += di;
        j += dj;
    }

    (true, trees)
}

const RAY_DIRECTIONS: [[i32; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

fn visibility_scan(map: &mut Map) {
    let map_read = map.clone();
    for (pos, tile) in map.data.iter_mut() {
        let height = map_read.get(&pos).map(|t| t.height).unwrap_or(0);
        let rays: Vec<(bool, usize)> = RAY_DIRECTIONS
            .iter()
            .map(|dir| raycast(&map_read, height, *pos, *dir))
            .collect();

        let is_visible = rays.iter().any(|(v, _)| *v);
        let scenic_score: u32 = rays.iter().map(|(_, trees)| *trees as u32).product();

        tile.visible = Some(is_visible);
        tile.scenic_score = Some(scenic_score);
    }
}

fn main() -> Result<()> {
    let mut map = parse("data/day08/input")?;
    visibility_scan(&mut map);

    if let Some((&pos, score)) = map
        .data
        .iter()
        .filter_map(|(p, t)| t.scenic_score.map(|ss| (p, ss)))
        .max_by_key(|(_p, ss)| *ss)
    {
        if let Some(t) = map.get_mut(&pos) {
            t.most_scenic = true;
        }

        println!("{}", map);

        println!(
            "Part 1: {}",
            map.find_all_where(|_, tile| tile.visible.unwrap_or(false))
                .len()
        );

        println!("Part 2: {} (located at {:?})", score, pos);
    }

    Ok(())
}
