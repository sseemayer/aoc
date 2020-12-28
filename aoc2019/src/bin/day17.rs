use aoc2019::board::{Board, Direction, Position};
use aoc2019::intcode::{parse_program, IntCodeResult, State};
use aoc2019::result::{format_err, Error, Result};
use aoc2019::util::read_to_string;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Unknown,
    Empty,
    Scaffold,
    Robot { direction: Direction },
}

impl Tile {
    fn from_output(o: i64) -> Result<Self> {
        match o {
            35 => Ok(Tile::Scaffold),
            46 => Ok(Tile::Empty),
            94 => Ok(Tile::Robot {
                direction: Direction::North,
            }),

            _ => Err(format_err!("Output is not a tile: {}", o)),
        }
    }
}

impl std::default::Default for Tile {
    fn default() -> Self {
        Tile::Unknown
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let c = match self {
            Tile::Unknown => "░".to_owned(),
            Tile::Empty => " ".to_owned(),
            Tile::Scaffold => "█".to_owned(),
            Tile::Robot { direction } => format!("{:?}", direction),
        };
        write!(f, "{}", c)
    }
}

fn read_camera(state: &mut State) -> Result<Board<Tile>> {
    let mut i = 0;
    let mut j = 0;
    let mut board = Board::new();

    let mut outputs = Vec::new();

    loop {
        match state.run(&mut outputs)? {
            IntCodeResult::Input => {
                panic!("Unexpected input");
            }
            IntCodeResult::Output => {
                let o = outputs.pop().unwrap();
                match o {
                    10 => {
                        i += 1;
                        j = 0;
                    }
                    _ => {
                        let tile = Tile::from_output(o)?;
                        board.set(&Position { i, j }, tile);
                        j += 1;
                    }
                }
            }
            IntCodeResult::Halt => break,
        }
    }

    Ok(board)
}

fn find_intersections(board: &Board<Tile>) -> Vec<Position> {
    let mut out = Vec::new();
    for (pos, tile) in board.tiles.iter() {
        if tile != &Tile::Scaffold {
            continue;
        }

        let is_intersection = Direction::ALL.iter().all(|d| {
            let pos_ofs = *pos + d.to_ofs().into();
            board.get(&pos_ofs) == Tile::Scaffold
        });

        if is_intersection {
            out.push(*pos);
        }
    }

    out
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum Steering {
    Left,
    Right,
    Walk { steps: i64 },
}

impl Steering {
    fn to_instr(&self, input: &mut Vec<i64>) {
        match self {
            Steering::Left => input.push(76),
            Steering::Right => input.push(82),
            Steering::Walk { steps } => {
                let digits = format!("{}", steps);

                for d in digits.chars() {
                    input.push(d as i64);
                }
            }
        }
    }
}

impl std::fmt::Debug for Steering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let c = match self {
            Steering::Left => "⮢ ".to_owned(),
            Steering::Right => "⮣ ".to_owned(),
            Steering::Walk { steps } => format!("{}🠖 ", steps),
        };

        write!(f, "{}", c)
    }
}

fn make_path(board: &Board<Tile>) -> Vec<Steering> {
    let mut out = Vec::new();
    let mut pos = board
        .where_is(&Tile::Robot {
            direction: Direction::North,
        })
        .unwrap();

    let mut dir = Direction::North;

    loop {
        let mut step_pos = pos + dir.to_ofs().into();
        let mut steps = 0;
        while board.get(&step_pos) == Tile::Scaffold {
            pos = step_pos;
            steps += 1;
            step_pos = pos + dir.to_ofs().into();
        }

        if steps > 0 {
            out.push(Steering::Walk { steps });
        }

        let left_pos: Position = pos + dir.turn_left().to_ofs().into();
        let right_pos: Position = pos + dir.turn_right().to_ofs().into();

        if board.get(&left_pos) == Tile::Scaffold {
            dir = dir.turn_left();
            out.push(Steering::Left);
        } else if board.get(&right_pos) == Tile::Scaffold {
            dir = dir.turn_right();
            out.push(Steering::Right);
        } else {
            break;
        }
    }

    out
}

fn count_phrases(steer: &Vec<Steering>) -> HashMap<Vec<Steering>, usize> {
    let mut counter = HashMap::new();
    for phrase_len in 1..=steer.len() {
        for i in 0..=(steer.len() - phrase_len) {
            let phrase = steer[i..(i + phrase_len)].iter().map(|s| *s).collect();
            counter.entry(phrase).and_modify(|c| *c += 1).or_insert(1);
        }
    }
    counter
}

fn cover_by_phrasebook(path: &Vec<Steering>, phrases: &Vec<&Vec<Steering>>) -> Option<Vec<usize>> {
    let mut path = path.clone();
    let mut out = Vec::new();

    while !path.is_empty() {
        let mut found = false;

        for (i, phrase) in phrases.iter().enumerate() {
            if path.len() >= phrase.len() && &path[0..phrase.len()] == &phrase[..] {
                for _ in 0..phrase.len() {
                    path.remove(0);
                }
                found = true;
                out.push(i);
                break;
            }
        }

        if !found {
            return None;
        }
    }

    if out.len() > 51 {
        return None;
    }

    Some(out)
}

fn encode_main(main: &[usize], out: &mut Vec<i64>) {
    for (i, v) in main.iter().enumerate() {
        if i > 0 {
            out.push(44);
        }
        let instr = match v {
            0 => 65,
            1 => 66,
            2 => 67,
            _ => panic!("Unknown instruction {}", v),
        };

        out.push(instr);
    }
    out.push(10);
}

fn encode_func(f: &[Steering], mut out: &mut Vec<i64>) {
    for (i, v) in f.iter().enumerate() {
        if i > 0 {
            out.push(44);
        }

        v.to_instr(&mut out);
    }
    out.push(10);
}

fn main() -> Result<()> {
    let program = parse_program(&read_to_string("data/day17/input")?)?;

    let mut state = State::new(program.clone());
    let board = read_camera(&mut state)?;

    println!("BOARD:\n{}", board);

    let mut sum = 0;
    for pos in find_intersections(&board) {
        sum += pos.i * pos.j;
    }

    println!("Part 1 answer: {}", sum);

    let path = make_path(&board);

    println!("Path found: {:?}", path);

    let phrases = count_phrases(&path);

    let phrases: Vec<Vec<Steering>> = phrases
        .into_iter()
        .map(|(p, _)| p)
        .filter(|p| (p.len() <= 20) && (p.len() >= 3))
        .collect();

    println!(
        "Scanning for combinations of {} phrases to cover path of length {}",
        phrases.len(),
        path.len()
    );

    for phrasebook in phrases.iter().combinations(3) {
        if let Some(main) = cover_by_phrasebook(&path, &phrasebook) {
            println!(
                "FOUND PHRASEBOOK:\n0: {:?} ({})\n1: {:?} ({})\n2: {:?} ({})\nmain: {:?} ({})",
                phrasebook[0],
                phrasebook[0].len(),
                phrasebook[1],
                phrasebook[1].len(),
                phrasebook[2],
                phrasebook[2].len(),
                main,
                main.len(),
            );

            let a = phrasebook[0];
            let b = phrasebook[1];
            let c = phrasebook[2];

            let mut input = Vec::new();
            encode_main(&main, &mut input);
            encode_func(&a[..], &mut input);
            encode_func(&b[..], &mut input);
            encode_func(&c[..], &mut input);
            input.push(110); // n - no video feed;
            input.push(10);

            let inp = input.iter().map(|v| *v as u8).collect();
            let inp = String::from_utf8(inp)?;
            println!("Input is:\n{}", inp);

            let mut program2 = program.clone();
            program2[0] = 2;

            let mut state = State {
                memory: program2.clone(),
                ic: 0,
                inputs: input.clone(),
                relative_base: 0,
            };

            //let board = read_camera(&mut state)?;

            //println!("Board: {}", board);

            let mut output = Vec::new();
            loop {
                match state.run(&mut output)? {
                    IntCodeResult::Output => {}
                    IntCodeResult::Input => panic!("Insufficient input"),
                    IntCodeResult::Halt => break,
                }
            }

            let out = output.iter().map(|v| *v as u8).collect();
            let out = String::from_utf8(out)?;

            println!("Output:\n{}", out);
            println!("Final Output: {}", output[output.len() - 1]);

            break;
        }
    }

    Ok(())
}
