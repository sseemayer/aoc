use aoc2019::board::{Board, Direction, Position};
use aoc2019::result::{format_err, Error, Result};
use aoc2019::util::read_to_lines;
use bit_set::BitSet;
use smallvec::SmallVec;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Wall {
    Inner,
    Outer,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Unknown,
    Empty,
    Wall,
    Letter { id: char },
    Portal { id: (char, char), wall: Wall },
}

impl Tile {
    fn can_move(&self) -> bool {
        match self {
            Tile::Unknown => false,
            Tile::Empty => true,
            Tile::Wall => false,
            Tile::Letter { .. } => false,
            Tile::Portal { .. } => true,
        }
    }
}

impl std::default::Default for Tile {
    fn default() -> Self {
        Tile::Unknown
    }
}

impl std::convert::From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'A'..='Z' => Tile::Letter { id: c },
            _ => Tile::Unknown,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let c = match self {
            Tile::Unknown => "░".to_owned(),
            Tile::Empty => " ".to_owned(),
            Tile::Wall => "█".to_owned(),
            Tile::Letter { id } => format!("{}", id),
            Tile::Portal {
                wall: Wall::Inner, ..
            } => ">".to_owned(),
            Tile::Portal {
                wall: Wall::Outer, ..
            } => "<".to_owned(),
        };
        write!(f, "{}", c)
    }
}

fn parse_board(lines: &Vec<String>) -> Result<Board<Tile>> {
    let mut board = Board::new();

    for (i, l) in lines.into_iter().enumerate() {
        for (j, t) in l.chars().enumerate() {
            let tile: Tile = t.into();
            let pos = Position {
                i: i as i64,
                j: j as i64,
            };

            board.set(&pos, tile.clone());
        }
    }

    Ok(board)
}

const LETTER_OFFSETS: [((i64, i64), (i64, i64)); 4] = [
    ((-2, 0), (-1, 0)), // above
    ((1, 0), (2, 0)),   // below
    ((0, -2), (0, -1)), // left
    ((0, 1), (0, 2)),   // right
];

fn portalize(board: &mut Board<Tile>) -> HashMap<(char, char), Vec<Position>> {
    let mut out = HashMap::new();

    let mut width = 0;
    let mut height = 0;

    for (p, t) in board.tiles.iter() {
        if p.j > width {
            width = p.j;
        }
        if p.i > height {
            height = p.i;
        }

        if t == &Tile::Empty {
            let p = *p;

            for ((i0ofs, j0ofs), (i1ofs, j1ofs)) in LETTER_OFFSETS.iter() {
                let ofs0 = (*i0ofs, *j0ofs).into();
                let ofs1 = (*i1ofs, *j1ofs).into();
                if let (Tile::Letter { id: a }, Tile::Letter { id: b }) =
                    (board.get(&(p + ofs0)), board.get(&(p + ofs1)))
                {
                    // we have found letters at two offset positions -- t really is a portal!
                    let id = (a, b);
                    out.entry(id)
                        .and_modify(|ps: &mut Vec<Position>| ps.push(p))
                        .or_insert(vec![p]);
                }
            }
        }
    }

    let ci = height / 2;
    let cj = width / 2;

    for (id, ps) in out.iter() {
        for p in ps.iter() {
            let di = (p.i - ci).abs();
            let dj = (p.j - cj).abs();

            let wall = if di > ci - 5 || dj > cj - 5 {
                Wall::Outer
            } else {
                Wall::Inner
            };

            board.set(p, Tile::Portal { id: *id, wall });
        }
    }

    out
}

fn fill_dead_end(board: &mut Board<Tile>) {
    let mut todo: Vec<Position> = board.tiles.keys().map(|p| *p).collect();

    while !todo.is_empty() {
        let p = todo.remove(0);
        let t = board.get(&p);

        if t != Tile::Empty {
            continue;
        }

        let non_walls: Vec<Position> = Direction::ALL
            .iter()
            .filter_map(|d| {
                let p2 = p + d.to_ofs().into();
                if board.get(&p2) != Tile::Wall {
                    Some(p2)
                } else {
                    None
                }
            })
            .collect();

        if non_walls.len() <= 1 {
            board.set(&p, Tile::Wall);
            todo.extend(non_walls);
        }
    }
}

fn bfs(
    board: &Board<Tile>,
    portals: &HashMap<(char, char), Vec<Position>>,
    recursive: bool,
) -> Option<usize> {
    let start_pos: Position = portals[&('A', 'A')][0];

    let mut seen: HashSet<(Position, i64)> = HashSet::new();
    let mut queue: Vec<(usize, Position, i64)> = vec![(0, start_pos, 0)];

    while !queue.is_empty() {
        let (dist, pos, depth) = queue.remove(0);
        let t = board.get(&pos);

        print!("\rdist={:6} queue={:6}", dist, queue.len());

        // are we there yet?
        if let Tile::Portal { id, wall: _ } = t {
            if id == ('Z', 'Z') && (!recursive || depth == 0) {
                println!("\n");
                return Some(dist);
            }
        }

        seen.insert((pos, depth));

        for dir in Direction::ALL.iter() {
            let p2 = pos + dir.to_ofs().into();
            let t2 = board.get(&p2);
            if !t2.can_move() {
                continue;
            }

            if seen.contains(&(p2, depth)) {
                continue;
            }

            queue.push((dist + 1, p2, depth));
        }

        if let Tile::Portal { id, wall } = t {
            // find the other portal
            if let Some(p2) = portals[&id].iter().find(|p2| **p2 != pos) {
                let new_depth = match wall {
                    Wall::Inner => depth + 1,
                    Wall::Outer => depth - 1,
                };

                if new_depth >= 0 && !seen.contains(&(*p2, new_depth)) {
                    queue.push((dist + 1, *p2, new_depth));
                }
            }
        }
    }

    None
}

fn main() -> Result<()> {
    let mut board = parse_board(&read_to_lines("data/day20/input")?)?;

    let portals = portalize(&mut board);

    fill_dead_end(&mut board);

    println!("Board:\n{}", board);

    println!("Portals:");
    for (id, ps) in portals.iter() {
        let coords: Vec<String> = ps.iter().map(|p| format!("{}", p)).collect();
        println!("{}{}: {}", id.0, id.1, coords.join(", "));
    }

    println!("PART 1");
    if let Some(dist) = bfs(&board, &portals, false) {
        println!("Found in {} steps", dist);
    }

    println!("PART 2");
    if let Some(dist) = bfs(&board, &portals, true) {
        println!("Found in {} steps", dist);
    }

    Ok(())
}
