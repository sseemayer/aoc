use std::collections::HashSet;

use anyhow::Result;
use aoc::map::ParseMapTile;

type Map = aoc::map::Map<[isize; 2], Tile>;

#[derive(Debug, Clone)]
struct Tile;

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        if c == '#' {
            Some(Self)
        } else {
            None
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#")
    }
}

struct System {
    galaxies: Vec<[isize; 2]>,
    empty_cols: HashSet<isize>,
    empty_rows: HashSet<isize>,
}

impl System {
    fn from_map(map: &Map) -> Self {
        let ([imin, jmin], [imax, jmax]) = map.get_extent();

        let galaxies: Vec<[isize; 2]> = map.data.keys().cloned().collect();

        let empty_cols = (jmin..=jmax)
            .filter(|j| (imin..=imax).all(|i| map.get(&[i, *j]).is_none()))
            .collect();
        let empty_rows = (imin..=imax)
            .filter(|i| (jmin..=jmax).all(|j| map.get(&[*i, j]).is_none()))
            .collect();

        Self {
            galaxies,
            empty_cols,
            empty_rows,
        }
    }

    fn get_distance(&self, p: [isize; 2], q: [isize; 2], expansion_factor: isize) -> isize {
        let [imin, imax] = [isize::min(p[0], q[0]), isize::max(p[0], q[0])];
        let empty_row_crossings = (imin..=imax)
            .filter(|i| self.empty_rows.contains(i))
            .count();

        let [jmin, jmax] = [isize::min(p[1], q[1]), isize::max(p[1], q[1])];
        let empty_col_crossings = (jmin..=jmax)
            .filter(|j| self.empty_cols.contains(j))
            .count();

        (imax - imin)
            + (jmax - jmin)
            + expansion_factor * empty_row_crossings as isize
            + expansion_factor * empty_col_crossings as isize
    }

    fn get_all_distances(&self, expansion_factor: isize) -> isize {
        let mut sum = 0;
        for (i, p) in self.galaxies.iter().enumerate() {
            for q in self.galaxies.iter().take(i) {
                sum += self.get_distance(*p, *q, expansion_factor);
            }
        }

        sum
    }
}

fn main() -> Result<()> {
    let map: Map = aoc::io::read_all("data/day11/input")?.parse()?;

    let system = System::from_map(&map);

    println!("Part 1: {}", system.get_all_distances(1));
    println!("Part 2: {}", system.get_all_distances(999_999));

    Ok(())
}
