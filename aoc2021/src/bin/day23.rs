use anyhow::Result;
use aoc::map::ParseMapTile;
use colored::Colorize;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs::File,
};

type Map = aoc::map::Map<[i8; 2], Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Floor,
    Room(Amphipod),
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Wall => write!(f, "█"),
            Tile::Floor => write!(f, "░"),
            Tile::Room(amphipod) => write!(f, "{}", amphipod),
        }
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Tile::Wall),
            '.' => Some(Tile::Floor),
            'A' => Some(Tile::Room(Amphipod::A)),
            'B' => Some(Tile::Room(Amphipod::B)),
            'C' => Some(Tile::Room(Amphipod::C)),
            'D' => Some(Tile::Room(Amphipod::D)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn from_j(j: i8) -> Option<Self> {
        match j {
            3 => Some(Amphipod::A),
            5 => Some(Amphipod::B),
            7 => Some(Amphipod::C),
            9 => Some(Amphipod::D),
            _ => None,
        }
    }

    fn to_j(&self) -> i8 {
        match self {
            Amphipod::A => 3,
            Amphipod::B => 5,
            Amphipod::C => 7,
            Amphipod::D => 9,
        }
    }

    fn energy_factor(&self) -> usize {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }
}

impl std::fmt::Display for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Amphipod::A => write!(f, "{}", "A".red()),
            Amphipod::B => write!(f, "{}", "B".yellow()),
            Amphipod::C => write!(f, "{}", "C".green()),
            Amphipod::D => write!(f, "{}", "D".blue()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    energy: usize,
    positions: Vec<Vec<Amphipod>>,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.energy)?;
        for i in 1..12 {
            write!(f, "\n")?;

            if let Some(a) = Amphipod::from_j(i) {
                write!(f, "{}{:>2}: ", a, i)?;
            } else {
                write!(f, " {:>2}: --", i)?;
            }

            if let Some(stack) = self.positions.get(i as usize) {
                for s in stack {
                    write!(f, "{}", s)?;
                }
            }
        }
        write!(f, "\n")
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.energy.cmp(&other.energy)
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.energy.partial_cmp(&other.energy)
    }
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.positions.hash(state);
    }
}

impl State {
    fn from_map(map: &Map) -> Self {
        let ([imin, jmin], [imax, jmax]) = map.get_extent();
        let mut positions: Vec<Vec<Amphipod>> = Vec::with_capacity((jmax - jmin + 1) as usize);

        for _j in jmin..=jmax {
            positions.push(Vec::new());
        }

        for i in imin..=imax {
            for j in jmin..=jmax {
                if let Some(Tile::Room(a)) = map.get(&[imax - i + imin, j]) {
                    positions[j as usize].push(*a);
                }
            }
        }

        State {
            energy: 0,
            positions,
        }
    }

    fn is_solved(&self) -> bool {
        for (j, stack) in self.positions.iter().enumerate() {
            if let Some(a) = Amphipod::from_j(j as i8) {
                if !stack.iter().all(|v| *v == a) {
                    return false;
                }
            } else {
                if !stack.is_empty() {
                    return false;
                }
            }
        }
        true
    }

    fn is_room_hospitable(&self, room: Amphipod) -> bool {
        let j = room.to_j() as usize;
        self.positions[j].iter().all(|v| *v == room)
    }

    fn hallway_positions_from(&self, j: i8) -> Vec<i8> {
        let mut candidates = Vec::new();
        for k in (1..j).rev() {
            match (self.positions[k as usize].last(), Amphipod::from_j(k)) {
                (Some(_), None) => {
                    // break if we hit another occupant on the hallway
                    break;
                }

                (_, Some(_)) => {
                    // skip over room positions
                    continue;
                }

                (None, None) => {
                    // k is a valid hallway destination
                    candidates.push(k);
                }
            }
        }

        for k in j + 1..=11 {
            match (self.positions[k as usize].last(), Amphipod::from_j(k)) {
                (Some(_), None) => {
                    // break if we hit another occupant on the hallway
                    break;
                }

                (_, Some(_)) => {
                    // skip over room positions
                    continue;
                }

                (None, None) => {
                    // k is a valid hallway destination
                    candidates.push(k);
                }
            }
        }

        candidates
    }

    fn is_path_free(&self, i: i8, j: i8) -> bool {
        let range = if i < j { i + 1..=j } else { j..=i - 1 };
        for k in range {
            if Amphipod::from_j(k).is_some() {
                continue;
            }
            if !self.positions[k as usize].is_empty() {
                return false;
            }
        }
        true
    }

    fn get_steps(&self, depth: usize) -> Vec<State> {
        let mut out = Vec::new();
        for (j, stack) in self.positions.iter().enumerate() {
            if stack.is_empty() {
                continue;
            }

            let leave_dist = if let Some(dest_owner) = Amphipod::from_j(j as i8) {
                // don't leave rooms that are finished
                if stack.iter().all(|v| *v == dest_owner) {
                    continue;
                }

                let src_len = depth - stack.len();

                // we are in a room - generate valid hallway positions
                out.extend(self.hallway_positions_from(j as i8).into_iter().map(|k| {
                    let mut new_state = self.clone();

                    let distance = (j as i8 - k).abs() as usize + src_len + 1;

                    let mover = new_state.positions[j].pop().unwrap();
                    new_state.energy += distance * mover.energy_factor();

                    new_state.positions[k as usize].push(mover);

                    new_state
                }));

                src_len + 1
            } else {
                0
            };

            // see if we can get to destination

            let mover = *stack.last().unwrap();
            if !self.is_room_hospitable(mover) {
                continue;
            }

            let k = mover.to_j();

            if !self.is_path_free(j as i8, k) {
                continue;
            }

            out.clear();

            let k_len = depth - self.positions[k as usize].len();

            let distance = leave_dist + (j as i8 - k).abs() as usize + k_len;

            let mut new_state = self.clone();
            new_state.positions[j].pop();
            new_state.positions[k as usize].push(mover);
            new_state.energy += distance * mover.energy_factor();
            out.push(new_state);
            break;
        }

        out
    }
}

fn solve(state: &State, depth: usize) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    queue.push(Reverse(state.clone()));

    while let Some(state) = queue.pop() {
        let state = state.0;

        //println!("steps={} / search_space={}", state.energy, queue.len());

        seen.insert(state.clone());

        if state.is_solved() {
            return Some(state.energy);
        }

        for new_state in state.get_steps(depth) {
            if seen.contains(&new_state) {
                continue;
            }

            queue.push(Reverse(new_state));
        }
    }

    None
}

fn main() -> Result<()> {
    let map = Map::read(&mut File::open("data/day23/input")?)?;
    let state = State::from_map(&map);

    println!("{}\n{}", map, state);

    if let Some(e) = solve(&state, 2) {
        println!("Part 1: {}", e);
    }

    let map = Map::read(&mut File::open("data/day23/input_b")?)?;
    let state = State::from_map(&map);

    println!("{}\n{}", map, state);

    if let Some(e) = solve(&state, 4) {
        println!("Part 2: {}", e);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solved() -> Result<()> {
        let state_debug = State::from_map(&Map::read(&mut File::open("data/day23/debug")?)?);
        let state_solved = State::from_map(&Map::read(&mut File::open("data/day23/solved")?)?);

        assert!(!state_debug.is_solved());
        assert!(state_solved.is_solved());

        Ok(())
    }

    #[test]
    fn test_paths() -> Result<()> {
        let state = State::from_map(&Map::read(&mut File::open("data/day23/block")?)?);

        assert_eq!(state.hallway_positions_from(1), vec![2, 4]);
        assert_eq!(state.hallway_positions_from(7), vec![8, 10, 11]);
        assert_eq!(state.hallway_positions_from(10), vec![8, 11]);

        Ok(())
    }

    #[test]
    fn test_hospitable() -> Result<()> {
        let state = State::from_map(&Map::read(&mut File::open("data/day23/block")?)?);
        assert!(state.is_room_hospitable(Amphipod::A));
        assert!(state.is_room_hospitable(Amphipod::B));
        assert!(!state.is_room_hospitable(Amphipod::C));
        assert!(!state.is_room_hospitable(Amphipod::D));

        let state1 = State::from_map(&Map::read(&mut File::open("data/day23/debug1")?)?);
        assert!(!state1.is_room_hospitable(Amphipod::A));
        assert!(!state1.is_room_hospitable(Amphipod::B));
        assert!(state1.is_room_hospitable(Amphipod::C));
        assert!(!state1.is_room_hospitable(Amphipod::D));

        Ok(())
    }

    #[test]
    fn test_movie() -> Result<()> {
        let mut states = Vec::new();
        let energies = vec![0, 40, 440, 3440, 3470, 3510, 5510, 5513, 8513, 12513, 12521];

        for i in 0..=10 {
            let mut state = State::from_map(&Map::read(&mut File::open(&format!(
                "data/day23/debug{}",
                i
            ))?)?);
            state.energy = energies[i];
            states.push(state);
        }

        for i in 0..10 {
            let prev = &states[i];
            let next = &states[i + 1];

            let steps = prev.get_steps(2);
            if let Some(step) = steps.iter().find(|step| step.positions == next.positions) {
                assert_eq!(step.energy, next.energy);
            } else {
                println!("In frame {}, missing this step:\n{}", i, next);

                println!("Generated:");

                for step in steps {
                    println!("{}", step);
                }

                panic!("Missing step");
            }
        }

        Ok(())
    }
}
