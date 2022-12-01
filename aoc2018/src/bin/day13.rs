use std::{fs::File, io::Read};

use anyhow::{anyhow, bail, Context, Result};
use colored::Colorize;

use aoc::map::ParseMapTile;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn step(&self, &[i, j]: &[i32; 2]) -> [i32; 2] {
        match self {
            Direction::North => [i - 1, j],
            Direction::East => [i, j + 1],
            Direction::South => [i + 1, j],
            Direction::West => [i, j - 1],
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn turn(&self, direction: &Direction) -> (Direction, Turn) {
        match self {
            Turn::Left => (direction.turn_left(), Turn::Straight),
            Turn::Straight => (direction.clone(), Turn::Right),
            Turn::Right => (direction.turn_right(), Turn::Left),
        }
    }
}

#[derive(Debug, Clone)]
struct Cart {
    direction: Direction,
    next_turn: Turn,
    removed: bool,
}

impl std::fmt::Display for Cart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.direction {
                Direction::North => "🭯",
                Direction::East => "🭬",
                Direction::South => "🭭",
                Direction::West => "🭮",
            }
        )
    }
}

#[derive(Debug, Clone)]
enum Tile {
    CornerA,
    CornerB,
    Horizontal,
    Vertical,
    Intersection,
    Cart(Cart),
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '/' => Some(Tile::CornerA),
            '\\' => Some(Tile::CornerB),
            '-' => Some(Tile::Horizontal),
            '|' => Some(Tile::Vertical),
            '+' => Some(Tile::Intersection),
            '^' => Some(Tile::Cart(Cart {
                direction: Direction::North,
                next_turn: Turn::Left,
                removed: false,
            })),
            '>' => Some(Tile::Cart(Cart {
                direction: Direction::East,
                next_turn: Turn::Left,
                removed: false,
            })),
            'v' => Some(Tile::Cart(Cart {
                direction: Direction::South,
                next_turn: Turn::Left,
                removed: false,
            })),
            '<' => Some(Tile::Cart(Cart {
                direction: Direction::West,
                next_turn: Turn::Left,
                removed: false,
            })),

            _ => None,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::CornerA => write!(f, "╱"),
            Tile::CornerB => write!(f, "╲"),
            Tile::Horizontal => write!(f, "─"),
            Tile::Vertical => write!(f, "│"),
            Tile::Intersection => write!(f, "┼"),
            Tile::Cart(c) => write!(f, "{}", format!("{}", c).red()),
        }
    }
}

type Map = aoc::map::Map<[i32; 2], Tile>;

#[derive(Debug, Clone)]
struct State {
    map: Map,
    carts: Vec<([i32; 2], Cart)>,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = self.map.clone();

        for (pos, cart) in &self.carts {
            map.set(*pos, Tile::Cart(cart.clone()));
        }

        write!(f, "{}", map)
    }
}

impl State {
    fn parse(path: &str) -> Result<Self> {
        let mut buffer = String::new();
        File::open(path)?.read_to_string(&mut buffer)?;

        let mut map: Map = buffer.parse().context("Map reading error")?;

        let mut carts = Vec::new();
        for (pos, tile) in map.data.iter_mut() {
            if let Tile::Cart(cart) = tile {
                let cart = cart.clone();

                *tile = match cart.direction {
                    Direction::North => Tile::Vertical,
                    Direction::East => Tile::Horizontal,
                    Direction::South => Tile::Vertical,
                    Direction::West => Tile::Horizontal,
                };

                carts.push((*pos, cart));
            }
        }

        Ok(Self { map, carts })
    }

    fn tick(&mut self) -> Result<Vec<[i32; 2]>> {
        let mut collisions = Vec::new();
        let mut i = 0;

        self.carts.sort_by_key(|(p, _)| *p);

        'outer: while i < self.carts.len() {
            let (pos, mut cart) = self.carts[i].clone();
            let new_pos = cart.direction.step(&pos);

            for (j, (collision_pos, _)) in self
                .carts
                .iter()
                .enumerate()
                .filter(|(j, (p, c))| !c.removed && i != *j && p == &new_pos)
            {
                collisions.push(collision_pos.clone());

                self.carts[i].1.removed = true;
                self.carts[j].1.removed = true;

                // println!(
                //     "Collision between {} and {} at {},{}",
                //     i, j, new_pos[1], new_pos[0],
                // );

                i += 1;

                continue 'outer;
            }

            let tile_under_cart = self
                .map
                .get(&new_pos)
                .ok_or_else(|| anyhow!("No tile under cart at {:?}", pos))?;

            cart.direction = match tile_under_cart {
                Tile::CornerA if cart.direction == Direction::North => Direction::East,
                Tile::CornerA if cart.direction == Direction::West => Direction::South,
                Tile::CornerA if cart.direction == Direction::South => Direction::West,
                Tile::CornerA if cart.direction == Direction::East => Direction::North,
                Tile::CornerB if cart.direction == Direction::North => Direction::West,
                Tile::CornerB if cart.direction == Direction::East => Direction::South,
                Tile::CornerB if cart.direction == Direction::South => Direction::East,
                Tile::CornerB if cart.direction == Direction::West => Direction::North,
                Tile::Horizontal => cart.direction,
                Tile::Vertical => cart.direction,
                Tile::Intersection => {
                    let (new_dir, new_turn) = cart.next_turn.turn(&cart.direction);
                    cart.next_turn = new_turn;
                    new_dir
                }
                _ => bail!(
                    "Don't know what to do with {:?} at {:?} for {:?}",
                    tile_under_cart,
                    new_pos,
                    cart
                ),
            };

            self.carts[i] = (new_pos, cart);

            i += 1;
        }

        self.carts.retain(|(_, c)| !c.removed);

        Ok(collisions)
    }
}

fn part1(mut state: State) -> Result<()> {
    loop {
        let collisions = state.tick()?;

        if !collisions.is_empty() {
            print!("Part 1: ");
            for [i, j] in collisions {
                println!("{},{}", j, i);
            }
            break;
        }
    }

    Ok(())
}

fn part2(mut state: State) -> Result<()> {
    while state.carts.len() > 1 {
        state.tick()?;
    }

    print!("Part 2: ");
    for ([i, j], _) in state.carts {
        println!("{},{}", j, i);
    }

    Ok(())
}

fn main() -> Result<()> {
    let state = State::parse("data/day13/input")?;

    part1(state.clone())?;
    part2(state.clone())?;

    Ok(())
}
