#![feature(map_first_last)]

use std::{
    collections::{BTreeSet, HashMap},
    fs::File,
};

use anyhow::Result;
use aoc::map::{Map, ParseMapTile};

#[derive(Clone, PartialEq, Eq)]
struct Tile(u8);

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        Some(Tile((c as u8) - ('0' as u8)))
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

static OFFSETS: [[i64; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

fn shortest_path(map: &Map<[i64; 2], Tile>, start: &[i64; 2]) -> usize {
    let mut dists: HashMap<[i64; 2], usize> = HashMap::with_capacity(map.data.len());
    let mut queue: BTreeSet<(usize, [i64; 2])> = BTreeSet::new();

    dists.insert(start.clone(), 0);
    queue.insert((0, start.clone()));

    while let Some((dist, [i, j])) = queue.pop_first() {
        for [iofs, jofs] in OFFSETS {
            let pos = [i + iofs, j + jofs];
            if let Some(&Tile(d)) = map.get(&pos) {
                let alt_dist = dist + (d as usize);
                let cur_dist = *dists.get(&pos).unwrap_or(&usize::MAX);

                if alt_dist < cur_dist {
                    dists.insert(pos, alt_dist);
                    queue.remove(&(cur_dist, pos));
                    queue.insert((alt_dist, pos));
                }
            }
        }
    }

    let (_min, [imax, jmax]) = map.get_extent();

    *dists.get(&[imax, jmax]).unwrap()
}

fn expand(map: &Map<[i64; 2], Tile>) -> Map<[i64; 2], Tile> {
    let mut out = Map::new();

    let ([imin, jmin], [imax, jmax]) = map.get_extent();
    assert_eq!(imin, 0);
    assert_eq!(jmin, 0);

    let ie = imax + 1;
    let je = jmax + 1;

    for (&[i, j], &Tile(v)) in map.data.iter() {
        for it in 0..5 {
            for jt in 0..5 {
                let ii = it * ie + i;
                let jj = jt * je + j;
                let vv = ((v as i64 + it + jt - 1) % 9 + 1) as u8;

                out.set([ii, jj], Tile(vv));
            }
        }
    }

    out
}

fn main() -> Result<()> {
    let map: Map<[i64; 2], Tile> = Map::read(&mut File::open("data/day15/input")?)?;

    let dist1 = shortest_path(&map, &[0, 0]);
    println!("Part 1: {}", dist1);

    let map2 = expand(&map);

    let dist2 = shortest_path(&map2, &[0, 0]);
    println!("Part 2: {}", dist2);

    Ok(())
}
