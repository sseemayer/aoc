use aoc2019::board::{Board, Direction, Position};
use aoc2019::result::Result;
use aoc2019::util::read_to_lines;
use std::collections::HashMap;

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Unknown,
    Empty,
    Bug,
    Recursion,
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
            '#' => Tile::Bug,
            _ => Tile::Unknown,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let c = match self {
            Tile::Unknown => "?".to_owned(),
            Tile::Empty => "░".to_owned(),
            Tile::Bug => "█".to_owned(),
            Tile::Recursion => "@".to_owned(),
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

fn step(board_in: &Board<Tile>, board_out: &mut Board<Tile>) -> Result<()> {
    for (p, ti) in board_in.tiles.iter() {
        let neighbors: u8 = Direction::ALL
            .iter()
            .map(|d| {
                let q = *p + d.to_ofs().into();
                if board_in.get(&q) == Tile::Bug {
                    1
                } else {
                    0
                }
            })
            .sum();

        if ti == &Tile::Empty && (neighbors == 1 || neighbors == 2) {
            board_out.set(p, Tile::Bug);
        } else if ti == &Tile::Bug && neighbors != 1 {
            board_out.set(p, Tile::Empty);
        } else {
            board_out.set(p, *ti);
        }
    }

    Ok(())
}

fn to_hashkey(board: &Board<Tile>) -> String {
    format!("{}", board)
}

fn score_biodiversity(board: &Board<Tile>) -> usize {
    let mut score = 0;
    let (i_min, i_max, j_min, j_max) = board.get_extent();

    let w = j_max - j_min + 1;

    for i in i_min..=i_max {
        for j in j_min..=j_max {
            if board.get(&Position { i, j }) == Tile::Bug {
                let pos = i * w + j;
                let pts = 1 << pos;
                //println!("Bug at i={} j={} tile #{} has {} points", i, j, pos, pts);
                score += pts;
            }
        }
    }

    score
}

fn part_one() -> Result<()> {
    let board_initial = parse_board(&read_to_lines("data/day24/input")?)?;
    let mut board_a = board_initial.clone();
    let mut board_b = board_initial.clone();

    println!("Part one");
    let mut seen: HashSet<String> = HashSet::new();
    let mut i = 0;
    loop {
        println!("\nBoard after step {}:\n{}", i, board_a);

        if seen.contains(&to_hashkey(&board_a)) {
            println!("Found repeated state!");
            break;
        }

        seen.insert(to_hashkey(&board_a));

        step(&board_a, &mut board_b)?;

        std::mem::swap(&mut board_a, &mut board_b);

        i += 1;
    }

    println!("Biodiversity score: {}", score_biodiversity(&board_a));

    Ok(())
}

struct BoardStack {
    boards: HashMap<i64, Board<Tile>>,
}

impl BoardStack {
    fn new() -> Self {
        BoardStack {
            boards: HashMap::new(),
        }
    }
    fn with_initial_state(board: &Board<Tile>) -> Self {
        let mut boards = HashMap::new();
        boards.insert(0, board.clone());

        BoardStack { boards }
    }

    fn get_extent(&self) -> (i64, i64) {
        let mut dmin = std::i64::MAX;
        let mut dmax = std::i64::MIN;

        for d in self.boards.keys() {
            if *d < dmin {
                dmin = *d
            };
            if *d > dmax {
                dmax = *d
            };
        }

        (dmin, dmax)
    }

    fn get(&self, d: i64, i: i64, j: i64) -> Tile {
        self.boards
            .get(&d)
            .map(|b| b.get(&Position { i, j }))
            .unwrap_or(Tile::Unknown)
    }

    fn count(&self) -> usize {
        let mut out = 0;
        for b in self.boards.values() {
            let cts = b.count();
            out += cts.get(&Tile::Bug).unwrap_or(&0);
        }

        out
    }
}

impl std::fmt::Display for BoardStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut keys: Vec<&i64> = self.boards.keys().collect();
        keys.sort();

        for d in keys {
            write!(f, "Board at depth {}:\n{}\n\n", d, self.boards[d])?;
        }

        write!(f, "Total bugs: {}", self.count());

        Ok(())
    }
}

fn make_empty_board(w: i64) -> Board<Tile> {
    let mut bnew = Board::new();
    for i in 0..w {
        for j in 0..w {
            bnew.set(&Position { i, j }, Tile::Empty);
        }
    }
    bnew.set(&Position { i: 2, j: 2 }, Tile::Recursion);
    bnew
}

fn step_rec(stack_in: &BoardStack) -> Result<BoardStack> {
    let (dmin, dmax) = stack_in.get_extent();
    let (_imin, _imax, jmin, jmax) = stack_in.boards[&0].get_extent();

    let mut out = BoardStack::new();
    let w = jmax - jmin + 1;

    for d in (dmin - 1)..=(dmax + 1) {
        let board_in = stack_in
            .boards
            .get(&d)
            .map(|b| b.clone())
            .unwrap_or_else(|| make_empty_board(w));

        let mut has_bugs = false;
        let mut board_out = make_empty_board(w);

        for (p, ti) in board_in.tiles.iter() {
            let neighbors: u8 = Direction::ALL
                .iter()
                .map(|dir| {
                    let q = *p + dir.to_ofs().into();

                    if q.i == 2 && q.j == 2 {
                        // step into the next recursion depth and count the 5 neighbors at that
                        // border
                        let e = d - 1;
                        let mut inner_neighbors = 0;
                        match dir {
                            Direction::North => {
                                for j in 0..w {
                                    if stack_in.get(e, w - 1, j) == Tile::Bug {
                                        inner_neighbors += 1;
                                    }
                                }
                            }
                            Direction::East => {
                                for i in 0..w {
                                    if stack_in.get(e, i, 0) == Tile::Bug {
                                        inner_neighbors += 1;
                                    }
                                }
                            }
                            Direction::South => {
                                for j in 0..w {
                                    if stack_in.get(e, 0, j) == Tile::Bug {
                                        inner_neighbors += 1;
                                    }
                                }
                            }
                            Direction::West => {
                                for i in 0..w {
                                    if stack_in.get(e, i, w - 1) == Tile::Bug {
                                        inner_neighbors += 1;
                                    }
                                }
                            }
                        }

                        inner_neighbors
                    } else if q.i < 0 || q.i >= w || q.j < 0 || q.j >= w {
                        // step out of the next recursion depth and count the 1 neighbor
                        let e = d + 1;
                        let neighbor = match dir {
                            Direction::North => stack_in.get(e, 1, 2),
                            Direction::East => stack_in.get(e, 2, 3),
                            Direction::South => stack_in.get(e, 3, 2),
                            Direction::West => stack_in.get(e, 2, 1),
                        };

                        if neighbor == Tile::Bug {
                            1
                        } else {
                            0
                        }
                    } else {
                        if board_in.get(&q) == Tile::Bug {
                            1
                        } else {
                            0
                        }
                    }
                })
                .sum();

            if ti == &Tile::Empty && (neighbors == 1 || neighbors == 2) {
                board_out.set(p, Tile::Bug);
                has_bugs = true;
            } else if ti == &Tile::Bug && neighbors != 1 {
                board_out.set(p, Tile::Empty);
            } else {
                board_out.set(p, *ti);
            }
        }

        if has_bugs || stack_in.boards.contains_key(&d) {
            out.boards.insert(d, board_out);
        }
    }

    Ok(out)
}

fn part_two() -> Result<()> {
    println!("\n\n\nPart two\n");
    let mut board_initial = parse_board(&read_to_lines("data/day24/input")?)?;
    board_initial.set(&Position { i: 2, j: 2 }, Tile::Recursion);

    let mut stack = BoardStack::with_initial_state(&board_initial);

    for i in 0..200 {
        println!("=== STACK AFTER {} STEPS ====\n{}", i, stack);

        stack = step_rec(&stack)?;
    }

    println!("=== FINAL STACK ====\n{}", stack);

    Ok(())
}

fn main() -> Result<()> {
    part_one()?;
    part_two()?;

    Ok(())
}
