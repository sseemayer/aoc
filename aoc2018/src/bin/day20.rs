use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::Read,
};

use anyhow::{bail, Result};
use colored::Colorize;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone)]
enum Tile {
    Start,
    Floor,
    Wall,
    DoorH,
    DoorV,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Start => write!(f, "{}", "@".red()),
            Tile::Floor => write!(f, "░"),
            Tile::Wall => write!(f, "█"),
            Tile::DoorH => write!(f, "{}", "═".yellow()),
            Tile::DoorV => write!(f, "{}", "║".yellow()),
        }
    }
}

type Pos = [i32; 2];
type Map = aoc::map::Map<Pos, Tile>;

#[derive(Debug, Clone, EnumIter)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "N"),
            Direction::East => write!(f, "E"),
            Direction::South => write!(f, "S"),
            Direction::West => write!(f, "W"),
        }
    }
}

impl Direction {
    fn walk(&self, &[i, j]: &Pos, steps: i32) -> Pos {
        match self {
            Direction::North => [i - steps, j],
            Direction::East => [i, j + steps],
            Direction::South => [i + steps, j],
            Direction::West => [i, j - steps],
        }
    }
}

#[derive(Debug, Clone)]
enum Path {
    Direction(Direction),
    Composite(Vec<Path>),
    Alternatives(Vec<Path>),
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Path::Direction(dir) => write!(f, "{}", dir),
            Path::Composite(elems) => {
                for elem in elems {
                    elem.fmt(f)?;
                }
                Ok(())
            }
            Path::Alternatives(options) => {
                write!(
                    f,
                    "({})",
                    options
                        .iter()
                        .map(|o| format!("{}", o))
                        .collect::<Vec<String>>()
                        .join("|")
                )
            }
        }
    }
}

impl Path {
    fn simplify(self) -> Path {
        match self {
            Path::Composite(mut elems) if elems.len() == 1 => {
                elems.pop().expect("Single elem").simplify()
            }
            Path::Alternatives(mut alts) if alts.len() == 1 => {
                alts.pop().expect("Single alt").simplify()
            }
            _ => self,
        }
    }

    fn walk(&self, positions: &[Pos], map: &mut Map) -> Vec<Pos> {
        let positions: HashSet<Pos> = positions.iter().copied().collect();
        let positions: Vec<Pos> = positions.into_iter().collect();

        match self {
            Path::Direction(d) => positions
                .iter()
                .map(|pos| {
                    let door_pos = d.walk(pos, 1);
                    map.set(
                        door_pos,
                        match d {
                            Direction::North | Direction::South => Tile::DoorH,
                            Direction::East | Direction::West => Tile::DoorV,
                        },
                    );

                    let new_pos = d.walk(pos, 2);
                    map.set(new_pos, Tile::Floor);

                    surround_with_walls(map, &new_pos);
                    new_pos
                })
                .collect(),
            Path::Composite(steps) => {
                let mut positions = positions;
                for step in steps {
                    positions = step.walk(&positions[..], map);
                }
                positions
            }
            Path::Alternatives(alts) => {
                let mut out = Vec::new();

                for alt in alts {
                    out.extend(alt.walk(&positions[..], map))
                }
                out
            }
        }
    }
}

impl TryFrom<&[char]> for Path {
    type Error = anyhow::Error;

    fn try_from(value: &[char]) -> Result<Self, Self::Error> {
        let mut stack = Vec::new();
        let mut out = Path::Alternatives(vec![Path::Composite(Vec::new())]);

        let mut i = 0;

        while i < value.len() {
            let a = value[i];

            let head = if let Path::Alternatives(alts) = &mut out {
                if let Some(Path::Composite(comp)) = alts.last_mut() {
                    comp
                } else {
                    bail!("Inconsistent inner parser state")
                }
            } else {
                bail!("Inconsistent parser state");
            };

            if a == '(' {
                stack.push(i);
            } else if a == ')' {
                let j = stack.pop().expect("matching paren");

                if stack.is_empty() {
                    let in_paren = &value[j + 1..i];
                    head.push(Path::try_from(in_paren)?);
                }
            } else if a == '|' && stack.is_empty() {
                if let Path::Alternatives(alts) = &mut out {
                    alts.push(Path::Composite(Vec::new()))
                } else {
                    bail!("Inconsistent parser alternative state")
                }
            } else if stack.is_empty() {
                head.push(match a {
                    'N' => Path::Direction(Direction::North),
                    'E' => Path::Direction(Direction::East),
                    'S' => Path::Direction(Direction::South),
                    'W' => Path::Direction(Direction::West),
                    _ => bail!("Bad token: {}", a),
                })
            }

            i += 1;
        }

        Ok(out.simplify())
    }
}

impl std::str::FromStr for Path {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s
            .trim()
            .trim_start_matches("^")
            .trim_end_matches("$")
            .chars()
            .collect::<Vec<char>>();

        Path::try_from(&chars[..])
    }
}

fn parse(path: &str) -> Result<Path> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;

    buffer.parse()
}

const AROUND: [Pos; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
    [1, 0],
    [1, -1],
    [0, -1],
];

fn surround_with_walls(map: &mut Map, [i, j]: &Pos) {
    for [di, dj] in AROUND {
        map.data.entry([i + di, j + dj]).or_insert(Tile::Wall);
    }
}

fn walk(map: &Map, start: &Pos) -> (usize, HashMap<Pos, usize>) {
    let mut queue = VecDeque::new();
    queue.push_back((*start, 0));

    let mut distances = HashMap::new();
    distances.insert(*start, 0);

    let mut furthest = 0;

    while let Some((pos, dist)) = queue.pop_front() {
        furthest = usize::max(furthest, dist);

        for dir in Direction::iter() {
            // check we can cross through a door
            let door_check = dir.walk(&pos, 1);
            match map.get(&door_check) {
                Some(Tile::DoorH) | Some(Tile::DoorV) => {}
                _ => continue,
            }

            let next_pos = dir.walk(&pos, 2);

            if let Some(_d) = distances.get_mut(&next_pos) {
                // we have been here before
                continue;
            }

            distances.insert(next_pos, dist + 1);

            queue.push_back((next_pos, dist + 1));
        }
    }

    (furthest, distances)
}

fn main() -> Result<()> {
    let path = parse("data/day20/input")?;

    let mut map = Map::new();
    map.set([0, 0], Tile::Start);
    surround_with_walls(&mut map, &[0, 0]);

    path.walk(&vec![[0, 0]], &mut map);

    println!("{}", map);

    let (furthest, distances) = walk(&map, &[0, 0]);
    let far_rooms = distances.values().filter(|d| **d >= 1000).count();

    println!("Part 1: {}", furthest);
    println!("Part 2: {}", far_rooms);

    Ok(())
}
