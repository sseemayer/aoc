use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;
use aoc::{direction::Direction, map::ParseMapTile};
use strum::IntoEnumIterator;

#[derive(Clone, Debug)]
struct Tile(char);

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        Some(Self(c))
    }
}

type Map = aoc::map::Map<[i32; 2], Tile>;

#[derive(Clone, Debug)]
struct Region {
    area: usize,
    circumference: usize,
    corners: usize,
}

impl Region {
    fn price(&self) -> usize {
        self.area * self.circumference
    }

    fn price_bulk(&self) -> usize {
        self.area * self.corners
    }
}

fn find_regions(map: &Map) -> Result<Vec<Region>> {
    let mut regions: Vec<Region> = Vec::new();

    let mut todo: HashSet<[i32; 2]> = map.data.keys().cloned().collect();

    while !todo.is_empty() {
        let seed_pos = todo
            .take(&todo.iter().next().cloned().expect("non empty"))
            .expect("exists");

        let &Tile(letter) = map.get(&seed_pos).expect("tile exists");

        // build up tiles and neighbors via BFS
        let mut tiles: HashSet<[i32; 2]> = HashSet::new();
        let mut neighbors: HashMap<[i32; 2], HashSet<[i32; 2]>> = HashMap::new();
        let mut queue: VecDeque<[i32; 2]> = VecDeque::new();
        queue.push_back(seed_pos);
        while let Some([i, j]) = queue.pop_front() {
            tiles.insert([i, j]);
            for direction in Direction::iter() {
                let [di, dj] = direction.dpos();
                let npos = [i + di, j + dj];

                if let Some(&Tile(other_letter)) = map.get(&npos) {
                    if other_letter != letter {
                        continue;
                    }
                } else {
                    continue;
                }

                neighbors.entry([i, j]).or_default().insert(npos);
                neighbors.entry(npos).or_default().insert([i, j]);

                if let Some(..) = todo.take(&npos) {
                    queue.push_back(npos);
                }
            }
        }

        let circumference = tiles
            .iter()
            .map(|p| 4 - neighbors.get(p).map(|n| n.len()).unwrap_or(0))
            .sum();

        let mut corners = 0;
        for &[i, j] in &tiles {
            let mut mask = [[false; 3]; 3];
            for di in 0..3 {
                for dj in 0..3 {
                    mask[di][dj] = tiles.contains(&[i + (di as i32) - 1, j + (dj as i32) - 1]);
                }
            }

            let [[tl, t, tr], [l, _, r], [bl, b, br]] = mask;

            // top left corner
            if (!l && !t) || (l && t && !tl) {
                corners += 1;
            }

            // top right corner
            if (!r && !t) || (r && t && !tr) {
                corners += 1;
            }

            // bottom left corner
            if (!l && !b) || (l && b && !bl) {
                corners += 1;
            }

            // bottom right corner
            if (!r && !b) || (r && b && !br) {
                corners += 1;
            }
        }

        regions.push(Region {
            area: tiles.len(),
            circumference,
            corners,
        });
    }

    Ok(regions)
}

fn main() -> Result<()> {
    let map: Map = aoc::io::read_all((2024, 12))?.parse()?;
    //let map: Map = aoc::io::read_all("data/day12/example2")?.parse()?;

    let regions = find_regions(&map)?;

    println!(
        "Part 1: {}",
        regions.iter().map(|r| r.price()).sum::<usize>()
    );

    println!(
        "Part 2: {}",
        regions.iter().map(|r| r.price_bulk()).sum::<usize>()
    );

    Ok(())
}
