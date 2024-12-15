use anyhow::{anyhow, Result};
use aoc::{direction::Direction, map::ParseMapTile};
use colored::Colorize;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Floor,
    Wall,
    Box(BoxType),
    Robot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BoxType {
    Single,
    L,
    R,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Floor => " ".on_black(),
                Tile::Wall => "█".on_bright_black(),
                Tile::Box(BoxType::Single) => "🮮".blue().on_black(),
                Tile::Box(BoxType::L) => "[".blue().on_black(),
                Tile::Box(BoxType::R) => "]".blue().on_black(),
                Tile::Robot => "🯅".green().on_black(),
            }
        )
    }
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Floor),
            '#' => Some(Tile::Wall),
            'O' => Some(Tile::Box(BoxType::Single)),
            '@' => Some(Tile::Robot),
            _ => None,
        }
    }
}

type Map = aoc::map::Map<[i32; 2], Tile>;

struct World {
    map: Map,
    robot_pos: [i32; 2],
}

impl World {
    fn from_map(map: &Map, expand: bool) -> Result<Self> {
        let ([imin, jmin], [imax, jmax]) = map.get_extent();

        let x_factor = if expand { 2 } else { 1 };

        let mut map_scaled = Map::new();
        for i in imin..=imax {
            for j in jmin..((jmax + 1) * x_factor) {
                if let Some(tile) = map.get(&[i, j / x_factor]) {
                    if !expand {
                        map_scaled.set([i, j], tile.clone());
                    } else {
                        map_scaled.set(
                            [i, j],
                            match (tile, j % 2) {
                                (Tile::Robot, 0) => Tile::Robot,
                                (Tile::Wall, _) => Tile::Wall,
                                (Tile::Box(BoxType::Single), 0) => Tile::Box(BoxType::L),
                                (Tile::Box(BoxType::Single), 1) => Tile::Box(BoxType::R),
                                _ => Tile::Floor,
                            },
                        );
                    }
                }
            }
        }

        let [ri, rj] = map
            .find_one_where(|_, t| t == &Tile::Robot)
            .ok_or(anyhow!("Misplaced robot"))?;

        Ok(Self {
            map: map_scaled,
            robot_pos: [ri, rj * x_factor],
        })
    }

    fn can_push(&self, box_pos: [i32; 2], direction: Direction) -> bool {
        let [i, j] = box_pos;
        let [di, dj] = direction.dpos();

        match self.map.get(&box_pos) {
            Some(Tile::Floor) => true,
            Some(Tile::Box(BoxType::Single)) => self.can_push([i + di, j + dj], direction),
            Some(Tile::Box(box_type)) => match direction {
                Direction::East | Direction::West => self.can_push([i, j + 2 * dj], direction),
                Direction::North | Direction::South => {
                    let other_j = match box_type {
                        BoxType::L => j + 1,
                        BoxType::R => j - 1,
                        BoxType::Single => unreachable!(),
                    };

                    self.can_push([i + di, j], direction)
                        && self.can_push([i + di, other_j], direction)
                }
            },
            _ => false,
        }
    }

    fn do_push(&mut self, box_pos: [i32; 2], direction: Direction) {
        let [i, j] = box_pos;
        let [di, dj] = direction.dpos();

        match self.map.get(&box_pos).cloned() {
            Some(Tile::Box(BoxType::Single)) => {
                self.do_push([i + di, j + dj], direction);

                self.map.set([i, j], Tile::Floor);
                self.map.set([i + di, j + dj], Tile::Box(BoxType::Single));
            }
            Some(Tile::Box(box_type)) => match direction {
                Direction::East | Direction::West => {
                    self.do_push([i, j + 2 * dj], direction);

                    self.map.set([i, j], Tile::Floor);

                    match box_type {
                        BoxType::L => {
                            self.map.set([i, j + dj], Tile::Box(BoxType::L));
                            self.map.set([i, j + 2 * dj], Tile::Box(BoxType::R));
                        }
                        BoxType::R => {
                            self.map.set([i, j + dj * 2], Tile::Box(BoxType::L));
                            self.map.set([i, j + dj], Tile::Box(BoxType::R));
                        }
                        BoxType::Single => unreachable!(),
                    }
                }
                Direction::North | Direction::South => {
                    let (other_j, other_type) = match box_type {
                        BoxType::L => (j + 1, BoxType::R),
                        BoxType::R => (j - 1, BoxType::L),
                        BoxType::Single => unreachable!(),
                    };

                    self.do_push([i + di, j], direction);
                    self.do_push([i + di, other_j], direction);

                    self.map.set([i, j], Tile::Floor);
                    self.map.set([i, other_j], Tile::Floor);
                    self.map.set([i + di, j], Tile::Box(box_type));
                    self.map.set([i + di, other_j], Tile::Box(other_type));
                }
            },
            _ => {}
        }
    }

    fn step(&mut self, direction: Direction) -> Result<()> {
        let [i, j] = self.robot_pos;
        let [di, dj] = direction.dpos();

        if self.can_push([i + di, j + dj], direction) {
            self.do_push([i + di, j + dj], direction);

            self.map.set(self.robot_pos, Tile::Floor);
            self.robot_pos = [i + di, j + dj];
            self.map.set(self.robot_pos, Tile::Robot);
        }

        Ok(())
    }

    fn gps_sum(&self) -> i32 {
        self.map
            .find_all_where(|_, t| matches!(t, Tile::Box(BoxType::Single | BoxType::L)))
            .iter()
            .map(|[i, j]| 100 * i + j)
            .sum()
    }
}

fn parse(input: &str) -> Result<(Map, Vec<Direction>)> {
    let (map, directions) = input
        .split_once("\n\n")
        .ok_or(anyhow!("expected empty line delimiter"))?;

    let map: Map = map.parse()?;
    let directions = directions
        .trim()
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            'v' => Ok(Direction::South),
            '<' => Ok(Direction::West),
            _ => Err(anyhow!("Bad direction: {}", c)),
        })
        .collect::<Result<Vec<Direction>>>()?;

    Ok((map, directions))
}

fn main() -> Result<()> {
    let (map, directions) = parse(&aoc::io::read_all((2024, 15))?)?;
    //let (map, directions) = parse(&aoc::io::read_all("data/day15/example")?)?;
    //
    let mut world = World::from_map(&map, false)?;
    for &direction in &directions {
        //println!("{:?}\n{}", direction, &world.map);
        world.step(direction)?;
    }

    println!("{}", &world.map);
    println!("Part 1: {}", world.gps_sum());

    println!();

    let mut world = World::from_map(&map, true)?;
    for &direction in &directions {
        //println!("{:?}\n{}", direction, &world.map);
        world.step(direction)?;
    }

    println!("{}", &world.map);
    println!("Part 2: {}", world.gps_sum());
    Ok(())
}
