use std::fs::File;

use anyhow::Result;
use aoc::map::ParseMapTile;
use colored::{ColoredString, Colorize};

type Map<S> = aoc::map::Map<[i16; 2], Tile<S>>;

#[derive(Debug, Clone, Default)]
struct Tile<S>
where
    S: TileState,
{
    infected: S,
    carrier: Option<Direction>,
}

trait TileState
where
    Self: Sized + Clone + Default,
{
    fn step(&self, direction: &Direction) -> (Self, Direction);

    fn clean_state() -> Self;
    fn infected_state() -> Self;

    fn is_infected(&self) -> bool;

    fn colorize(&self, s: &str) -> ColoredString;
}

#[derive(Debug, Clone)]
enum TileState1 {
    Clean,
    Infected,
}

impl Default for TileState1 {
    fn default() -> Self {
        TileState1::Clean
    }
}

impl TileState for TileState1 {
    fn step(&self, direction: &Direction) -> (Self, Direction) {
        match self {
            TileState1::Clean => (TileState1::Infected, direction.turn_left()),
            TileState1::Infected => (TileState1::Clean, direction.turn_right()),
        }
    }

    fn clean_state() -> Self {
        TileState1::Clean
    }

    fn infected_state() -> Self {
        TileState1::Infected
    }

    fn colorize(&self, s: &str) -> ColoredString {
        match self {
            TileState1::Clean => s.white(),
            TileState1::Infected => s.red(),
        }
    }

    fn is_infected(&self) -> bool {
        matches!(self, TileState1::Infected)
    }
}

#[derive(Debug, Clone)]
enum TileState2 {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl Default for TileState2 {
    fn default() -> Self {
        TileState2::Clean
    }
}

impl TileState for TileState2 {
    fn step(&self, direction: &Direction) -> (Self, Direction) {
        match self {
            TileState2::Clean => (TileState2::Weakened, direction.turn_left()),
            TileState2::Weakened => (TileState2::Infected, *direction),
            TileState2::Infected => (TileState2::Flagged, direction.turn_right()),
            TileState2::Flagged => (TileState2::Clean, direction.revert()),
        }
    }

    fn clean_state() -> Self {
        TileState2::Clean
    }

    fn infected_state() -> Self {
        TileState2::Infected
    }

    fn colorize(&self, s: &str) -> ColoredString {
        match self {
            TileState2::Clean => s.white(),
            TileState2::Weakened => s.blue(),
            TileState2::Infected => s.red(),
            TileState2::Flagged => s.yellow(),
        }
    }

    fn is_infected(&self) -> bool {
        matches!(self, TileState2::Infected)
    }
}

impl<S> ParseMapTile for Tile<S>
where
    S: TileState,
{
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Tile {
                infected: S::infected_state(),
                carrier: None,
            }),
            _ => Some(Tile {
                infected: S::clean_state(),
                carrier: None,
            }),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_str(&self) -> &'static str {
        match self {
            Direction::Up => "▲",
            Direction::Right => "▶",
            Direction::Down => "▼",
            Direction::Left => "◀",
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn revert(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    fn step(&self, pos: &[i16; 2]) -> [i16; 2] {
        match self {
            Direction::Up => [pos[0] - 1, pos[1]],
            Direction::Right => [pos[0], pos[1] + 1],
            Direction::Down => [pos[0] + 1, pos[1]],
            Direction::Left => [pos[0], pos[1] - 1],
        }
    }
}

impl<S> std::fmt::Display for Tile<S>
where
    S: TileState,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self.carrier {
                None => self.infected.colorize("."),
                Some(d) => self.infected.colorize(d.to_str()),
            }
        )
    }
}

struct State<S>
where
    S: TileState,
{
    map: Map<S>,
    pos: [i16; 2],
    direction: Direction,

    n_infections: usize,
}

impl<S> State<S>
where
    S: TileState,
{
    fn new(mut map: Map<S>) -> Self {
        let (max, min) = map.get_extent();
        let pos = [(max[0] + min[0]) / 2, (max[1] + min[1]) / 2];
        map.get_mut(&pos).expect("middle point").carrier = Some(Direction::Up);

        Self {
            map,
            pos,
            direction: Direction::Up,
            n_infections: 0,
        }
    }
    fn step(&mut self) {
        let tile_from = self.map.get_mut(&self.pos).expect("On existing tile");

        let (new_state, new_dir) = tile_from.infected.step(&self.direction);

        self.direction = new_dir;
        tile_from.infected = new_state;

        if tile_from.infected.is_infected() {
            self.n_infections += 1;
        }

        tile_from.carrier = None;

        self.pos = self.direction.step(&self.pos);

        let tile_to = self.map.data.entry(self.pos).or_insert(Default::default());
        tile_to.carrier = Some(self.direction);
    }
}

impl<S> std::fmt::Display for State<S>
where
    S: TileState,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}n_infections={}\n", self.map, self.n_infections)
    }
}

fn main() -> Result<()> {
    let mut state1: State<TileState1> =
        State::new(Map::read(&mut File::open("data/day22/input")?)?);

    for _ in 0..10_000 {
        state1.step();
    }

    println!("part 1: {}", state1);

    let mut state2: State<TileState2> =
        State::new(Map::read(&mut File::open("data/day22/input")?)?);

    for _ in 0..10_000_000 {
        state2.step();
    }

    println!("part 2: {}", state2);

    Ok(())
}
