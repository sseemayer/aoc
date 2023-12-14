use std::{cmp::Reverse, collections::HashMap};

use anyhow::Result;
use aoc::{direction::Direction, map::ParseMapTile};

type Map = aoc::map::Map<[i32; 2], Tile>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Boulder,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Wall => '#',
                Tile::Boulder => 'O',
            }
        )
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Tile::Wall),
            'O' => Some(Tile::Boulder),
            _ => None,
        }
    }
}

fn tilt(map: &mut Map, direction: Direction) {
    let ([imin, jmin], [imax, jmax]) = map.get_extent();

    let mut boulder_pos = map.find_all_where(|_c, t| *t == Tile::Boulder);

    match direction {
        Direction::North => boulder_pos.sort_by_key(|pos| pos[0]),
        Direction::East => boulder_pos.sort_by_key(|pos| Reverse(pos[1])),
        Direction::South => boulder_pos.sort_by_key(|pos| Reverse(pos[0])),
        Direction::West => boulder_pos.sort_by_key(|pos| pos[1]),
    }

    let [di, dj] = direction.dpos();

    for pos in boulder_pos {
        let [mut i, mut j] = pos;
        map.data.remove(&pos);
        loop {
            if map.get(&[i + di, j + dj]).is_some()
                || i + di < imin
                || i + di > imax
                || j + dj < jmin
                || j + dj > jmax
            {
                map.set([i, j], Tile::Boulder);
                break;
            } else {
                i += di;
                j += dj;
            }
        }
    }
}

fn calculate_load(map: &Map) -> isize {
    let ([_imin, _jmin], [imax, _jmax]) = map.get_extent();

    let mut sum = 0;
    for ([i, _j], tile) in &map.data {
        if tile == &Tile::Boulder {
            sum += (imax - i + 1) as isize;
        }
    }

    sum
}

fn cycle(map: &mut Map) {
    tilt(map, Direction::North);
    tilt(map, Direction::West);
    tilt(map, Direction::South);
    tilt(map, Direction::East);
}

fn cycle_loop(map: &mut Map, count: usize) {
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut step = 0;

    while step < count {
        let key = format!("{}", map);
        if let Some(s) = seen.get(&key) {
            // skip ahead if we find a loop
            let loop_length = step - s;
            step += loop_length * ((count - step) / loop_length);
        } else {
            seen.insert(key, step);
        }

        cycle(map);
        step += 1;
    }
}

fn main() -> Result<()> {
    let mut map: Map = aoc::io::read_all("data/day14/input")?.parse()?;

    tilt(&mut map, Direction::North);

    println!("Part 1: {}", calculate_load(&map));

    cycle_loop(&mut map, 1_000_000_000);

    println!("Part 2: {}", calculate_load(&map));

    Ok(())
}
