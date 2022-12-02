use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
};

use anyhow::Result;
use colored::Colorize;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
enum Faction {
    Elf,
    Goblin,
}

impl std::fmt::Display for Faction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Faction::Elf => write!(f, "{}", "E".red()),
            Faction::Goblin => write!(f, "{}", "G".green()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Unit {
    atk: i32,
    hp: i32,
    faction: Faction,
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.faction, self.hp)
    }
}

#[derive(Debug, Clone)]
enum Tile {
    Ground,
    Wall,
    Unit(Unit),
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ground => write!(f, "░"),
            Tile::Wall => write!(f, "█"),
            Tile::Unit(u) => write!(f, "{}", u.faction),
        }
    }
}

impl aoc::map::ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Tile::Wall),
            '.' => Some(Tile::Ground),
            'E' => Some(Tile::Unit(Unit {
                atk: 3,
                hp: 200,
                faction: Faction::Elf,
            })),
            'G' => Some(Tile::Unit(Unit {
                atk: 3,
                hp: 200,
                faction: Faction::Goblin,
            })),
            _ => None,
        }
    }
}

type Coord = [i32; 2];
type Map = aoc::map::Map<Coord, Tile>;

const NEIGHBORS: [Coord; 4] = [[-1, 0], [0, -1], [0, 1], [1, 0]];

#[derive(Clone)]
struct State {
    map: Map,
}

impl State {
    fn parse(path: &str) -> Result<Self> {
        let mut buffer = String::new();
        File::open(path)?.read_to_string(&mut buffer)?;

        let map: Map = buffer.parse()?;

        Ok(Self { map })
    }

    fn get_units(&self) -> Vec<(Coord, Unit)> {
        let mut units = self
            .map
            .data
            .iter()
            .filter_map(|(pos, tile)| {
                if let Tile::Unit(unit) = tile {
                    Some((*pos, *unit))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        units.sort_by_key(|u| u.0);
        units
    }

    fn count_units_by_faction(&self) -> HashMap<Faction, usize> {
        let mut out = HashMap::new();
        for (_, unit) in self.get_units() {
            *out.entry(unit.faction).or_default() += 1;
        }

        out
    }

    fn find_attack_path(&self, start: &Coord, my_faction: &Faction) -> Option<VecDeque<Coord>> {
        let mut attack_squares = Vec::new();
        let mut backtrace = HashMap::new();
        let mut attack_distance = None;
        let mut queue = VecDeque::new();
        queue.push_back((*start, 0));

        while let Some(([ci, cj], distance)) = queue.pop_front() {
            if let Some(ad) = attack_distance {
                // we already have identified an attackable square
                // keep processing the queue until the distance from start
                // exceeds the distance to the attackable square
                if distance > ad {
                    break;
                }
            }

            for [di, dj] in NEIGHBORS {
                let new_pos = [ci + di, cj + dj];

                if backtrace.contains_key(&new_pos) {
                    continue;
                }

                backtrace.insert(new_pos.clone(), [ci, cj]);

                match self.map.get(&new_pos) {
                    Some(Tile::Ground) => {
                        // found movable ground
                        queue.push_back((new_pos, distance + 1));
                    }
                    Some(Tile::Unit(u)) if &u.faction != my_faction => {
                        // found attackable target
                        // println!("Found target {}@{:?}, distance {}", u, new_pos, distance);
                        attack_squares.push((u.hp, [ci, cj], new_pos));
                        attack_distance = Some(distance);
                    }
                    _ => {}
                }
            }
        }

        if attack_squares.is_empty() {
            return None;
        }

        if attack_distance.expect("attack distance") <= 1 {
            // we are adjacent to a target
            // if we have several option, choose based on lowest hp, then reading order of attack squares.
            attack_squares.sort();
        } else {
            // if we have several option, choose based on reading order of attack squares.
            attack_squares.sort_by_key(|(_hp, attack_pos, _attack_target)| *attack_pos);
        }

        let (_, attack_square, attack_target) = attack_squares.first().expect("Have a target");

        // println!(
        //     "{:?} -> {:?}, attacking {:?}",
        //     start, attack_square, attack_target
        // );

        // using backtrace, build up full path to target
        let mut current = Some(attack_square);
        let mut path = VecDeque::new();
        path.push_back(*attack_target);
        while let Some(c) = current {
            if c == start {
                break;
            }
            path.push_back(*c);
            current = backtrace.get(c);
        }

        Some(path)
    }

    fn step(&mut self) -> bool {
        for (pos, _) in self.get_units() {
            let unit = if let Some(Tile::Unit(u)) = self.map.get(&pos) {
                *u
            } else {
                // the unit died before it could move
                continue;
            };

            if !self
                .get_units()
                .into_iter()
                .any(|(_, u)| u.faction != unit.faction)
            {
                // cannot find an enemy for current unit -- end combat
                return false;
            }

            if let Some(mut path) = self.find_attack_path(&pos, &unit.faction) {
                println!("{}@{:?} chooses path: {:?}", unit, pos, path);

                if path.len() > 1 {
                    // move to next tile on path
                    let next_step = path.pop_back().expect("move target");
                    self.map.set(pos, Tile::Ground);
                    self.map.set(next_step, Tile::Unit(unit));
                }

                if path.len() == 1 {
                    // we are at an attack square
                    let attack_target = path.pop_front().expect("Always have a target");
                    let target_tile = self.map.get_mut(&attack_target).expect("Tile at target");
                    if let Tile::Unit(target) = target_tile {
                        println!("{}@{:?} attacks: {}@{:?}", unit, pos, target, attack_target);
                        target.hp -= unit.atk;

                        if target.hp <= 0 {
                            println!("{}@{:?} dies", target, pos);
                            *target_tile = Tile::Ground;
                        }
                    } else {
                        panic!("No unit at target");
                    }
                }
            }
        }

        true
    }

    fn sum_hp(&self) -> i32 {
        self.get_units().into_iter().map(|(_, u)| u.hp).sum()
    }

    fn simulate<F>(mut self, predicate: F) -> (Self, i32)
    where
        F: Fn(&State, bool) -> bool,
    {
        println!("Start:\n{}", &self);

        let mut steps = 0;
        loop {
            let completed = self.step();
            if !predicate(&self, completed) {
                break;
            }
            steps += 1;
            println!("Step {}:\n{}\n", steps, self);
        }

        println!("Final:\n{}", &self);
        (self, steps)
    }

    fn simulate_part1(self) -> (i32, i32, i32) {
        let (state, steps) = self.simulate(|_, completed| completed);
        let shp = state.sum_hp();
        println!("Part 1: {} ({} * {})", state, steps, shp);
        (steps * shp, steps, shp)
    }

    fn simulate_part2(self) -> Option<i32> {
        fn count_elves(state: &State) -> usize {
            *state
                .count_units_by_faction()
                .get(&Faction::Elf)
                .unwrap_or(&0)
        }

        let elves = *self
            .count_units_by_faction()
            .get(&Faction::Elf)
            .unwrap_or(&0);

        let (state, steps) =
            self.simulate(|state, completed| completed && count_elves(state) >= elves);

        let kept_all_elves = count_elves(&state) >= elves;

        if !kept_all_elves {
            return None;
        }

        let shp = state.sum_hp();
        Some(steps * shp)
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.map)?;

        for (pos, unit) in self.get_units() {
            write!(f, "{} @ {:?}\n", unit, pos)?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let state = State::parse("data/day15/input")?;

    state.clone().simulate_part1();

    let mut atk_power = 3;
    loop {
        let mut patched_state = state.clone();

        // patch the attack power for elves
        for (_, tile) in patched_state.map.data.iter_mut() {
            if let Tile::Unit(Unit {
                atk,
                faction: Faction::Elf,
                ..
            }) = tile
            {
                *atk = atk_power;
            }
        }

        if let Some(result) = patched_state.simulate_part2() {
            println!("Part 2: {}", result);
            break;
        }

        atk_power += 1;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_ex1() -> Result<()> {
        assert_eq!(
            State::parse("data/day15/example")?.simulate_part1(),
            (47 * 590, 47, 590,)
        );
        Ok(())
    }

    #[test]
    fn test_part1_ex2() -> Result<()> {
        assert_eq!(
            State::parse("data/day15/example2")?.simulate_part1(),
            (37 * 982, 37, 982,)
        );
        Ok(())
    }

    #[test]
    fn test_part1_ex3() -> Result<()> {
        assert_eq!(
            State::parse("data/day15/example3")?.simulate_part1(),
            (46 * 859, 46, 859,)
        );
        Ok(())
    }

    #[test]
    fn test_part1_ex4() -> Result<()> {
        assert_eq!(
            State::parse("data/day15/example4")?.simulate_part1(),
            (35 * 793, 35, 793,)
        );
        Ok(())
    }

    #[test]
    fn test_part1_ex5() -> Result<()> {
        assert_eq!(
            State::parse("data/day15/example5")?.simulate_part1(),
            (54 * 536, 54, 536,)
        );
        Ok(())
    }

    #[test]
    fn test_part1_ex6() -> Result<()> {
        assert_eq!(
            State::parse("data/day15/example6")?.simulate_part1(),
            (20 * 937, 20, 937,)
        );
        Ok(())
    }
}
