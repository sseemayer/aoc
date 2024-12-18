use std::{
    cmp::Reverse,
    collections::{HashSet, VecDeque},
};

use anyhow::{anyhow, Context, Result};
use aoc::direction::Direction;
use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
enum Tile {
    Floor,
    Wall,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Floor => ".",
                Tile::Wall => "#",
            }
        )
    }
}

type Map = aoc::map::Map<[i32; 2], Tile>;

struct World {
    map: Map,

    extent: i32,

    coords: Vec<[i32; 2]>,
}

impl World {
    fn parse(s: &str, extent: i32) -> Result<Self> {
        let mut coords = Vec::new();
        for line in s.lines() {
            let (x, y) = line
                .trim()
                .split_once(',')
                .ok_or(anyhow!("Bad line: '{}'", line))?;

            let x = x.parse().context("Parse X")?;
            let y = y.parse().context("Parse Y")?;

            coords.push([y, x]);
        }

        let mut map = Map::new();
        map.fixed_extent = Some(([0, 0], [extent, extent]));

        for i in 0..=extent {
            for j in 0..=extent {
                map.set([i, j], Tile::Floor);
            }
        }

        Ok(Self {
            map,
            extent,
            coords,
        })
    }

    fn apply(&mut self, slice: std::ops::Range<usize>) {
        for &coord in &self.coords[slice] {
            self.map.set(coord, Tile::Wall);
        }
    }

    fn solve(&self) -> Result<usize> {
        let mut queue: VecDeque<(Reverse<usize>, [i32; 2])> = VecDeque::new();
        let mut seen: HashSet<[i32; 2]> = HashSet::new();
        queue.push_back((Reverse(0), [0, 0]));

        while let Some((time, [i, j])) = queue.pop_front() {
            if i == self.extent && j == self.extent {
                return Ok(time.0);
            }

            for direction in Direction::iter() {
                let [di, dj] = direction.dpos();
                let newpos = [i + di, j + dj];

                if !seen.insert(newpos) {
                    continue;
                }

                if let Some(Tile::Floor) = self.map.get(&newpos) {
                    queue.push_back((Reverse(time.0 + 1), newpos));
                }
            }
        }

        Err(anyhow!("Found no solution"))
    }
}

fn main() -> Result<()> {
    let mut world: World = World::parse(&aoc::io::read_all((2024, 18))?, 70)?;
    let mut time = 1024;

    //let mut world: World = World::parse(&aoc::io::read_all("data/day18/example")?, 6)?;
    //let mut time = 12;

    world.apply(0..time);

    let steps = world.solve()?;
    println!("Part 1: {}", steps);

    while world.solve().is_ok() {
        time += 1;
        world.apply(time..(time + 1));
    }

    let wall_coord = world.coords[time];

    println!("Part 2: {},{}", wall_coord[1], wall_coord[0]);

    Ok(())
}
