use anyhow::Result;
use colored::Colorize;

type Map = aoc::map::Map<[i32; 2], Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Floor,
    Paper(u8),
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Floor => write!(f, "."),
            Tile::Paper(n) => write!(
                f,
                "{}",
                match n {
                    0..4 => "@".bold().green(),
                    4..6 => "@".yellow(),
                    6..9 => "@".red(),
                    _ => "@".normal(),
                }
            ),
        }
    }
}

impl aoc::map::ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Floor),
            '@' => Some(Tile::Paper(99)),
            _ => None,
        }
    }
}

fn find_free(map: &mut Map) -> Vec<[i32; 2]> {
    let mut free = Vec::new();

    for [i, j] in map.find_all_where(|_, t| matches!(t, Tile::Paper(_))) {
        let mut count = 0;
        for di in -1..=1 {
            for dj in -1..=1 {
                if di == 0 && dj == 0 {
                    continue;
                }

                if let Some(Tile::Paper(_)) = map.get(&[i + di, j + dj]) {
                    count += 1;
                }
            }
        }

        if count < 4 {
            free.push([i, j]);
        }

        if let Some(Tile::Paper(n)) = map.get_mut(&[i, j]) {
            *n = count;
        }
    }

    free
}

fn main() -> Result<()> {
    let input = aoc::io::read_all((2025, 4))?;
    let mut map: Map = input.parse()?;

    let mut free = find_free(&mut map);
    println!("{} {}", "Part 1:".bold().green(), free.len());

    let mut removed = 0;

    while free.len() > 0 {
        //println!("{}", map);
        for pos in &free {
            map.set(*pos, Tile::Floor);
        }

        removed += free.len();

        free = find_free(&mut map);
    }

    println!("{} {}", "Part 2:".bold().green(), removed);
    Ok(())
}
