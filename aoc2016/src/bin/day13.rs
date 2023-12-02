use anyhow::Result;
use aoc::map::Map;

fn is_filled(x: usize, y: usize, input: usize) -> bool {
    let mut v = x * x + 3 * x + 2 * x * y + y + y * y + input;
    let mut ones = 0;
    while v > 0 {
        if v % 2 == 1 {
            ones += 1;
        }
        v >>= 1;
    }

    ones % 2 == 1
}

fn get_neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();

    // go up
    if y > 0 {
        out.push((x, y - 1));
    }

    // go right
    out.push((x + 1, y));

    // go down
    out.push((x, y + 1));

    // go left
    if x > 0 {
        out.push((x - 1, y));
    }

    out
}

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Visited,
    Free,
    Blocked,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Visited => "O",
                Tile::Free => ".",
                Tile::Blocked => "#",
            }
        )
    }
}

fn main() -> Result<()> {
    let mut map: Map<[usize; 2], Tile> = Map::new();
    let input = 1358;
    let start = (1, 1);

    let mut queue = vec![(0, start)];
    let mut wrote_part2 = false;

    while !queue.is_empty() {
        let (steps, pos) = queue.remove(0);

        if pos == (31, 39) {
            println!("Found solution in {} steps", steps);
            break;
        }

        if steps == 51 && !wrote_part2 {
            wrote_part2 = true;

            let mut visited = 0;
            for t in map.data.values() {
                if t == &Tile::Visited {
                    visited += 1;
                }
            }

            println!("In 50 turns, visited {} spaces", visited);
        }

        map.set([pos.1, pos.0], Tile::Visited);

        for neighbor in get_neighbors(pos.0, pos.1) {
            match map.get(&[neighbor.1, neighbor.0]) {
                Some(_) => {}
                None => {
                    let blocked = is_filled(neighbor.0, neighbor.1, input);
                    map.set(
                        [neighbor.1, neighbor.0],
                        if blocked { Tile::Blocked } else { Tile::Free },
                    );
                    if !blocked {
                        queue.push((steps + 1, neighbor));
                    }
                }
            }
        }
    }

    println!("{}", map);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        Ok(())
    }
}
