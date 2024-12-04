use anyhow::Result;
use aoc::map::ParseMapTile;
use colored::Colorize;

#[derive(Debug, Clone)]
struct Tile {
    letter: char,
    active: bool,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = self.letter.to_string();
        write!(
            f,
            "{}",
            if self.active {
                letter.on_green()
            } else {
                letter.white()
            }
        )
    }
}

impl ParseMapTile for Tile {
    fn from_char(letter: char) -> Option<Self> {
        Some(Tile {
            letter,
            active: false,
        })
    }
}

type Map = aoc::map::Map<[i32; 2], Tile>;

const DIRECTIONS: [[i32; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

fn part1(map: &Map) -> Result<()> {
    let mut map = map.to_owned();

    let ([imin, jmin], [imax, jmax]) = map.get_extent();

    let search: Vec<char> = "XMAS".chars().collect();

    let mut found = 0;
    for i0 in imin..=imax {
        for j0 in jmin..=jmax {
            'direction: for [di, dj] in DIRECTIONS.iter() {
                for (n, a) in search.iter().enumerate() {
                    let i = i0 + n as i32 * di;
                    let j = j0 + n as i32 * dj;

                    if i < imin || i > imax || j < jmin || j > jmax {
                        continue 'direction;
                    }

                    if let Some(tile) = map.get(&[i, j]) {
                        if tile.letter != *a {
                            continue 'direction;
                        }
                    } else {
                        continue 'direction;
                    }
                }

                found += 1;
                for n in 0..search.len() {
                    let i = i0 + n as i32 * di;
                    let j = j0 + n as i32 * dj;

                    if let Some(tile) = map.get_mut(&[i, j]) {
                        tile.active = true;
                    }
                }
            }
        }
    }

    println!("{}\nPart 1: {}", map, found);

    Ok(())
}

const CROSS_DIRECTIONS: [[i32; 2]; 4] = [[-1, -1], [-1, 1], [1, -1], [1, 1]];

fn part2(map: &Map) -> Result<()> {
    let mut map = map.to_owned();

    let ([imin, jmin], [imax, jmax]) = map.get_extent();

    let mut found = 0;
    for i0 in imin..=imax {
        for j0 in jmin..=jmax {
            let Some(tile) = map.get(&[i0, j0]) else {
                continue;
            };

            if tile.letter != 'A' {
                continue;
            }

            let mut seen = 0;
            for [di, dj] in CROSS_DIRECTIONS {
                let a = map.get(&[i0 + di, j0 + dj]).map(|tile| tile.letter);
                let b = map.get(&[i0 - di, j0 - dj]).map(|tile| tile.letter);

                if let (Some('M'), Some('S')) = (a, b) {
                    seen += 1;
                }
            }

            if seen == 2 {
                found += 1;

                for [di, dj] in CROSS_DIRECTIONS {
                    if let Some(tile) = map.get_mut(&[i0 + di, j0 + dj]) {
                        tile.active = true;
                    }
                }

                if let Some(tile) = map.get_mut(&[i0, j0]) {
                    tile.active = true;
                }
            }
        }
    }

    println!("{}\nPart 2: {}", map, found);

    Ok(())
}

fn main() -> Result<()> {
    let map: Map = aoc::io::read_all((2024, 04))?.parse()?;
    //let map: Map = aoc::io::read_all("data/day04/example")?.parse()?;

    part1(&map)?;
    part2(&map)?;

    Ok(())
}
