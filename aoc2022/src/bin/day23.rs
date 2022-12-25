use std::{collections::HashMap, fs::File, io::Read};

use anyhow::Result;

use aoc::direction::Direction;

type Pos = [i32; 2];
type Map = aoc::map::Map<Pos, Tile>;

#[derive(Debug, Clone, Copy)]
struct Tile;

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#")
    }
}

impl aoc::map::ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Tile),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct State {
    map: Map,
    directions: Vec<Direction>,
}

impl State {
    fn parse(path: &str) -> Result<Self> {
        let mut buf = String::new();
        File::open(path)?.read_to_string(&mut buf)?;

        let map: Map = buf.parse()?;
        let directions = vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];

        Ok(Self { map, directions })
    }

    fn step(&mut self) -> bool {
        // proposal phase
        let move_choices = self
            .map
            .data
            .keys()
            .filter_map(|pos| {
                if !self.scan_around(pos) {
                    // don't move if no elves anywhere around
                    return None;
                }

                for dir in &self.directions {
                    if !self.scan_direction(pos, *dir) {
                        return Some((*pos, *dir));
                    }
                }

                // no valid move direction
                None
            })
            .collect::<HashMap<Pos, Direction>>();

        // figure out where elves would be
        let mut destinations: HashMap<Pos, Vec<Pos>> = HashMap::new();
        for ([i, j], direction) in move_choices {
            let [di, dj] = direction.dpos();
            let [ni, nj] = [i + di, j + dj];
            destinations.entry([ni, nj]).or_default().push([i, j]);
        }

        // move non-colliding elves
        let mut had_movement = false;
        for (n, origins) in destinations {
            if origins.len() == 1 {
                let o = origins[0];
                self.map.set(n, Tile);
                self.map.remove(&o);
                had_movement = true;
            }
        }

        let first_dir = self.directions.remove(0);
        self.directions.push(first_dir);

        had_movement
    }

    fn scan_at(&self, pos: &Pos) -> bool {
        self.map.get(pos).is_some()
    }

    fn scan_direction(&self, &[i, j]: &Pos, direction: Direction) -> bool {
        match direction {
            Direction::North => [[i - 1, j - 1], [i - 1, j], [i - 1, j + 1]],
            Direction::East => [[i - 1, j + 1], [i, j + 1], [i + 1, j + 1]],
            Direction::South => [[i + 1, j - 1], [i + 1, j], [i + 1, j + 1]],
            Direction::West => [[i - 1, j - 1], [i, j - 1], [i + 1, j - 1]],
        }
        .into_iter()
        .any(|pos| self.scan_at(&pos))
    }

    fn scan_around(&self, &[i, j]: &Pos) -> bool {
        [
            [i - 1, j - 1],
            [i - 1, j],
            [i - 1, j + 1],
            [i, j + 1],
            [i + 1, j + 1],
            [i + 1, j],
            [i + 1, j - 1],
            [i, j - 1],
        ]
        .into_iter()
        .any(|pos| self.scan_at(&pos))
    }

    fn empty_ground(&self) -> usize {
        let ([imin, jmin], [imax, jmax]) = self.map.get_extent();

        let mut empty_tiles = 0;
        for i in imin..=imax {
            for j in jmin..=jmax {
                if self.map.get(&[i, j]).is_none() {
                    empty_tiles += 1;
                }
            }
        }
        empty_tiles
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:?}", self.map, self.directions)
    }
}

fn main() -> Result<()> {
    let mut state = State::parse("data/day23/input")?;

    for _i in 1..=10 {
        state.step();
    }

    println!("Part 1: {}", state.empty_ground());

    let mut i = 11;
    while state.step() {
        i += 1;
    }

    println!("Part 2: {}", i);

    Ok(())
}
