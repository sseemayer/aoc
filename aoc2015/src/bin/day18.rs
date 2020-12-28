use aoc2015::board::{Board, Direction, Position};
use aoc2015::util::read_to_string;
use aoc2015::{format_err, Error, Result};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Unknown,
    On,
    Off,
}

impl std::default::Default for Tile {
    fn default() -> Self {
        Tile::Unknown
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let c = match self {
            Tile::Unknown => "?",
            Tile::On => "o",
            Tile::Off => ".",
        };

        write!(f, "{}", c)
    }
}

impl std::convert::From<char> for Tile {
    fn from(v: char) -> Tile {
        match v {
            '.' => Tile::Off,
            '#' => Tile::On,
            _ => Tile::Unknown,
        }
    }
}

fn parse_board(s: &str) -> Result<Board<Tile>> {
    let mut board = Board::new();
    for (i, line) in s.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let t: Tile = c.into();
            board.set(
                &Position {
                    i: i as i64,
                    j: j as i64,
                },
                t,
            );
        }
    }

    Ok(board)
}

fn step(board: &Board<Tile>) -> Board<Tile> {
    let (imin, imax, jmin, jmax) = board.get_extent();

    let mut out = Board::new();

    for i in imin..=imax {
        for j in jmin..=jmax {
            let p = Position {
                i: i as i64,
                j: j as i64,
            };
            let t = board.get(&p);

            let mut n_neighbors = 0;
            for iofs in -1..=1 {
                for jofs in -1..=1 {
                    if iofs == 0 && jofs == 0 {
                        continue;
                    }

                    let q = p + (iofs, jofs).into();

                    if board.get(&q) == Tile::On {
                        n_neighbors += 1;
                    }
                }
            }

            let u = match (t, n_neighbors) {
                (_, 3) | (Tile::On, 2) => Tile::On,
                _ => Tile::Off,
            };

            out.set(&p, u);
        }
    }

    out
}

fn main() -> Result<()> {
    let mut board = parse_board(&read_to_string("data/day18/input")?)?;

    println!("PART ONE");
    for i in 0..100 {
        //println!("=== AFTER {} STEPS ====\n{}", i, board);
        board = step(&board);
    }

    println!("=== FINAL STATE ====\n{}", board);

    println!("Counts: {:?}", board.count());

    println!("PART TWO");
    let mut board = parse_board(&read_to_string("data/day18/input")?)?;
    let (imin, imax, jmin, jmax) = board.get_extent();

    for i in 0..100 {
        println!("=== AFTER {} STEPS ====\n{}", i, board);
        board.set(&Position { i: imin, j: jmin }, Tile::On);
        board.set(&Position { i: imax, j: jmin }, Tile::On);
        board.set(&Position { i: imin, j: jmax }, Tile::On);
        board.set(&Position { i: imax, j: jmax }, Tile::On);
        board = step(&board);
        board.set(&Position { i: imin, j: jmin }, Tile::On);
        board.set(&Position { i: imax, j: jmin }, Tile::On);
        board.set(&Position { i: imin, j: jmax }, Tile::On);
        board.set(&Position { i: imax, j: jmax }, Tile::On);
    }

    println!("=== FINAL STATE ====\n{}", board);
    println!("Counts: {:?}", board.count());

    Ok(())
}
