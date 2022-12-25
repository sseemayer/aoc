use std::{
    cell::{Ref, RefCell},
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    hash::Hash,
    io::Read,
};

use anyhow::Result;
use aoc::direction::Direction;
use colored::Colorize;
use strum::IntoEnumIterator;

type Pos = [i32; 2];
type Map = aoc::map::Map<Pos, Tile>;

#[derive(Debug, Clone)]
enum Tile {
    Player,
    Floor,
    Wall,
    Blizzard(Vec<Direction>),
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Player => write!(f, "{}", "@".green()),
            Tile::Floor => write!(f, "░"),
            Tile::Wall => write!(f, "█"),
            Tile::Blizzard(directions) => {
                if directions.len() == 1 {
                    let d = directions.first().unwrap();
                    write!(f, "{}", format!("{}", d).blue())
                } else {
                    write!(f, "{}", format!("{}", directions.len()).blue())
                }
            }
        }
    }
}

impl aoc::map::ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Floor),
            '#' => Some(Tile::Wall),
            '^' => Some(Tile::Blizzard(vec![Direction::North])),
            '>' => Some(Tile::Blizzard(vec![Direction::East])),
            'v' => Some(Tile::Blizzard(vec![Direction::South])),
            '<' => Some(Tile::Blizzard(vec![Direction::West])),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct World {
    map: Map,
    blizzards: Vec<(Pos, Direction, usize)>,
    start: Pos,
    goal: Pos,

    blizzard_cache: RefCell<HashMap<usize, HashMap<Pos, Vec<Direction>>>>,
}

impl World {
    fn parse(path: &str) -> Result<Self> {
        let mut buf = String::new();
        File::open(path)?.read_to_string(&mut buf)?;

        let mut map: Map = buf.parse()?;

        let ([imin, jmin], [imax, jmax]) = map.get_extent();

        let mut blizzards = map
            .data
            .iter()
            .filter_map(|(&[i, j], tile)| {
                if let Tile::Blizzard(direction) = tile {
                    let direction = *direction.first().unwrap();
                    let phase = match direction {
                        Direction::North | Direction::South => imax - imin - 1,
                        Direction::West | Direction::East => jmax - jmin - 1,
                    } as usize;

                    Some(([i, j], direction, phase))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        blizzards.sort_by_key(|(pos, _dir, _phase)| *pos);

        let mut positions = map
            .data
            .iter()
            .filter_map(|(pos, tile)| {
                if let Tile::Floor = tile {
                    Some(*pos)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        positions.sort();

        let start = *positions.first().expect("Start pos");
        let goal = *positions.last().expect("Goal pos");

        for tile in map.data.values_mut() {
            if let Tile::Blizzard(_) = tile {
                *tile = Tile::Floor;
            }
        }

        Ok(Self {
            map,
            blizzards,
            start,
            goal,

            blizzard_cache: Default::default(),
        })
    }

    fn blizzard_positions(&self, time: usize) -> Ref<'_, HashMap<Pos, Vec<Direction>>> {
        if self.blizzard_cache.borrow().contains_key(&time) {
            return Ref::map(self.blizzard_cache.borrow(), |r| r.get(&time).unwrap());
        }

        let ([imin, jmin], [imax, jmax]) = self.map.get_extent();
        let width = jmax - jmin - 1;
        let height = imax - imin - 1;

        let mut out: HashMap<[i32; 2], Vec<Direction>> = HashMap::new();
        for ([i0, j0], direction, phase) in &self.blizzards {
            let frame = (time % phase) as i32;
            let [di, dj] = direction.dpos();

            let pos = [
                (i0 - 1 + di * frame + height) % height + 1,
                (j0 - 1 + dj * frame + width) % width + 1,
            ];
            // println!("{:?}/{:?} @{}: {:?}", [i0, j0], direction, time, pos);

            out.entry(pos).or_default().push(*direction);
        }

        self.blizzard_cache.borrow_mut().insert(time, out);

        Ref::map(self.blizzard_cache.borrow(), |r| r.get(&time).unwrap())
    }

    fn state(&self, time: usize, pos: Pos, n_checkpoints: usize) -> State {
        State {
            world: self,
            time,
            pos,
            n_checkpoints,
        }
    }

    fn pathfind(&mut self, checkpoints: &[Pos]) -> Option<usize> {
        let ([imin, jmin], [imax, jmax]) = self.map.get_extent();
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back(self.state(0, checkpoints[0], 1));

        let mut best_n_checkpoints = 1;
        while let Some(state) = queue.pop_front() {
            let n_checkpoints = if state.pos == checkpoints[state.n_checkpoints] {
                state.n_checkpoints + 1
            } else {
                state.n_checkpoints
            };

            if n_checkpoints < best_n_checkpoints {
                continue;
            }

            best_n_checkpoints = usize::max(best_n_checkpoints, n_checkpoints);

            if n_checkpoints >= checkpoints.len() {
                return Some(state.time);
            }

            let [i, j] = state.pos;

            let blizzards_next = self.blizzard_positions(state.time + 1);

            for dir in Direction::iter() {
                let [di, dj] = dir.dpos();
                let [ni, nj] = [i + di, j + dj];

                if ni < imin || ni > imax || nj < jmin || nj > jmax {
                    continue;
                }

                if let Some(Tile::Wall) = self.map.get(&[ni, nj]) {
                    continue;
                }

                if blizzards_next.contains_key(&[ni, nj]) {
                    continue;
                }

                let new_state = self.state(state.time + 1, [ni, nj], n_checkpoints);

                if seen.insert(new_state.clone()) {
                    queue.push_back(new_state);
                }
            }

            if !blizzards_next.contains_key(&[i, j]) {
                let wait_state = self.state(state.time + 1, [i, j], n_checkpoints);
                if seen.insert(wait_state.clone()) {
                    queue.push_back(wait_state);
                }
            }
        }

        None
    }
}

#[derive(Clone)]
struct State<'a> {
    world: &'a World,
    time: usize,
    pos: Pos,
    n_checkpoints: usize,
}

impl<'a> Hash for State<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.time.hash(state);
        self.pos.hash(state);
    }
}

impl<'a> std::cmp::PartialEq for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.pos == other.pos
    }
}

impl<'a> std::cmp::Eq for State<'a> {}

impl<'a> std::fmt::Display for State<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = self.world.map.clone();
        for (pos, dirs) in self.world.blizzard_positions(self.time).iter() {
            map.set(*pos, Tile::Blizzard(dirs.clone()));
        }

        map.set(self.pos, Tile::Player);

        write!(f, "{}t={}", map, self.time)
    }
}

fn main() -> Result<()> {
    let mut world = World::parse("data/day24/input")?;

    println!(
        "Part 1: {}",
        world.pathfind(&[world.start, world.goal]).unwrap()
    );

    println!(
        "Part 2: {}",
        world
            .pathfind(&[world.start, world.goal, world.start, world.goal])
            .unwrap()
    );

    Ok(())
}
