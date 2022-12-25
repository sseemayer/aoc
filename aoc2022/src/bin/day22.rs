use std::{collections::HashMap, fs::File, io::Read};

use anyhow::{anyhow, bail, Context, Result};
use colored::Colorize;
use lazy_static::lazy_static;
use regex::Regex;

use aoc::direction::Direction;

type Pos = [i32; 2];
type Map = aoc::map::Map<Pos, Tile>;

#[derive(Debug, Clone)]
enum Tile {
    Floor(bool),
    Wall,
    Cursor(Direction),
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Floor(v) => write!(f, "{}", if *v { "▒".green() } else { "░".white() }),
            Tile::Wall => write!(f, "█"),
            Tile::Cursor(d) => write!(f, "{}", format!("{}", d).green()),
        }
    }
}

impl aoc::map::ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Floor(false)),
            '#' => Some(Tile::Wall),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
enum Step {
    Step(usize),
    Right,
    Left,
}

lazy_static! {
    static ref RE_PATH: Regex = Regex::new(r"(\d+|L|R)").unwrap();
}

impl Step {
    fn parse_steps(s: &str) -> Result<Vec<Step>> {
        RE_PATH
            .find_iter(s)
            .map(|c| c.as_str())
            .map(|s| match s {
                "L" => Ok(Step::Left),
                "R" => Ok(Step::Right),
                _ => Ok(Step::Step(s.parse().context("Step count")?)),
            })
            .collect::<Result<Vec<Step>>>()
    }
}

#[derive(Debug, Clone)]
struct State {
    map: Map,
    path: Vec<Step>,
    i_path: usize,

    location: Pos,
    direction: Direction,

    face_size: Pos,
    pos_to_face: HashMap<Pos, usize>,
    face_to_pos: HashMap<usize, Pos>,

    cube_wrap: Option<HashMap<(usize, Direction), (usize, Direction)>>,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = self.map.clone();
        map.set(self.location, Tile::Cursor(self.direction.clone()));

        write!(f, "{}i={}", map, self.i_path)
    }
}

impl State {
    fn parse(
        path: &str,
        cube_wrap: Option<HashMap<(usize, Direction), (usize, Direction)>>,
    ) -> Result<Self> {
        let mut buf = String::new();
        File::open(path)?.read_to_string(&mut buf)?;

        let lines = buf
            .lines()
            .filter(|l| !l.trim().is_empty())
            .collect::<Vec<_>>();

        let path: Vec<Step> = Step::parse_steps(lines.last().expect("Last line"))?;
        let map: Map = lines[..lines.len() - 1].join("\n").parse()?;

        let location = *map
            .data
            .iter()
            .filter(|(_, t)| matches!(t, Tile::Floor(_)))
            .min_by_key(|(p, _)| *p)
            .expect("start pos")
            .0;

        let ([imin, jmin], [imax, jmax]) = map.get_extent();

        let width = jmax - jmin + 1;
        let height = imax - imin + 1;

        let [fh, fw] = [[4, 3], [3, 4]]
            .into_iter()
            .find(|[gh, gw]| width % gw == 0 && height % gh == 0)
            .map(|[gh, gw]| [height / gh, width / gw])
            .ok_or_else(|| {
                anyhow!(
                    "Cannot figure out grid for width={} height={}",
                    width,
                    height
                )
            })?;

        let mut face_to_pos = HashMap::new();
        let mut pos_to_face = HashMap::new();
        let mut i_face = 0;
        for i in imin..=imax {
            let fi = i / fh;
            for j in jmin..=jmax {
                let fj = j / fw;

                if map.get(&[i, j]).is_none() {
                    continue;
                }

                if !pos_to_face.contains_key(&[fi, fj]) {
                    pos_to_face.insert([fi, fj], i_face);
                    face_to_pos.insert(i_face, [fi, fj]);
                    i_face += 1;
                }
            }
        }

        if i_face != 6 {
            bail!("Expected 6 faces, got {}: {:?}", i_face, pos_to_face);
        }

        // check that cube_wrap is symmetric, if it was passed
        if let Some(cw) = &cube_wrap {
            for ((ff, fd), (tf, td)) in cw.iter() {
                if let Some((bff, bfd)) = cw.get(&(*tf, td.reverse())) {
                    if *fd != bfd.reverse() || ff != bff {
                        panic!("Unpaired {}/{:?}", ff, fd);
                    }
                } else {
                    panic!("Missing pair for {}/{:?}", tf, td.reverse());
                }
            }
        }

        Ok(Self {
            map,
            path,
            i_path: 0,
            location,
            direction: Direction::East,

            face_size: [fh, fw],
            pos_to_face,
            face_to_pos,

            cube_wrap,
        })
    }

    fn wrap(&self, pos: Pos, direction: Direction) -> Option<(Pos, Direction)> {
        if self.cube_wrap.is_some() {
            self.wrap_cubic(pos, direction)
        } else {
            self.wrap_planar(pos, direction)
        }
    }

    fn wrap_cubic(&self, [i, j]: Pos, direction: Direction) -> Option<(Pos, Direction)> {
        let [fh, fw] = self.face_size;

        // coords for face
        let [fi, fj] = [i / fh, j / fw];

        // on-face coords
        let [oi, oj] = [i % fh, j % fw];

        let face = *self.pos_to_face.get(&[fi, fj])?;

        if let Some(cw) = &self.cube_wrap {
            let (new_face, new_direction) = cw.get(&(face, direction))?;

            // coords on new face
            let [pi, pj] = match (direction, new_direction) {
                // ---^-  .....
                // .....  .....
                // .....  .....
                // .....  .....
                // .....  ---^-
                (Direction::North, Direction::North) => [fh - 1, oj],

                // ---^-  |....
                // .....  |....
                // .....  |....
                // .....  >....
                // .....  |....
                (Direction::North, Direction::East) => [oj, 0],

                // ---^-  -v---
                // .....  .....
                // .....  .....
                // .....  .....
                // .....  .....
                (Direction::North, Direction::South) => [0, fw - oj - 1],

                // ---^-  ....|
                // .....  ....<
                // .....  ....|
                // .....  ....|
                // .....  ....|
                (Direction::North, Direction::West) => [fh - oj - 1, fw - 1],

                // ....|  .....
                // ....|  .....
                // ....|  .....
                // ....>  .....
                // ....|  ---^-
                (Direction::East, Direction::North) => [fh - 1, oi],

                // ....|  |....
                // ....|  |....
                // ....|  |....
                // ....>  >....
                // ....|  |....
                (Direction::East, Direction::East) => [oi, 0],

                // ....|  -v---
                // ....|  .....
                // ....|  .....
                // ....>  .....
                // ....|  .....
                (Direction::East, Direction::South) => [0, fw - oi - 1],

                // ....|  ....|
                // ....|  ....<
                // ....|  ....|
                // ....>  ....|
                // ....|  ....|
                (Direction::East, Direction::West) => [fh - oi - 1, fw - 1],

                // .....  .....
                // .....  .....
                // .....  .....
                // .....  .....
                // ---v-  -^---
                (Direction::South, Direction::North) => [fh - 1, fw - oj - 1],

                // .....  |....
                // .....  <....
                // .....  |....
                // .....  |....
                // ---v-  |....
                (Direction::South, Direction::East) => [fh - oj - 1, 0],

                // .....  ---v-
                // .....  .....
                // .....  .....
                // .....  .....
                // ---v-  .....
                (Direction::South, Direction::South) => [0, oj],

                // .....  ....|
                // .....  ....|
                // .....  ....|
                // .....  ....<
                // ---v-  ....|
                (Direction::South, Direction::West) => [oj, fw - 1],

                // |....  .....
                // |....  .....
                // |....  .....
                // <....  .....
                // |....  -^---
                (Direction::West, Direction::North) => [fh - 1, fw - oj - 1],

                // |....  |....
                // |....  >....
                // |....  |....
                // <....  |....
                // |....  |....
                (Direction::West, Direction::East) => [fh - oi - 1, 0],

                // |....  ---v-
                // |....  .....
                // |....  .....
                // <....  .....
                // |....  .....
                (Direction::West, Direction::South) => [0, oi],

                // |....  ....|
                // |....  ....|
                // |....  ....|
                // <....  ....<
                // |....  ....|
                (Direction::West, Direction::West) => [oi, fw - 1],
            };

            // coords for new face
            let [gi, gj] = self.face_to_pos.get(new_face)?;

            let new_pos = [gi * fw + pi, gj * fh + pj];

            // println!(
            //     "WRAP {:?}/{:?} on {:?}: {:?} to {:?}: {:?} = {:?}/{:?}",
            //     [i, j],
            //     direction,
            //     [fi, fj],
            //     [oi, oj],
            //     [gi, gj],
            //     [pi, pj],
            //     new_pos,
            //     new_direction
            // );

            Some((new_pos, *new_direction))
        } else {
            None
        }
    }

    fn wrap_planar(&self, [i, j]: Pos, direction: Direction) -> Option<(Pos, Direction)> {
        let ([imin, jmin], [imax, jmax]) = self.map.get_extent();

        match direction {
            Direction::North => {
                for wi in (i..=imax).rev() {
                    if self.map.get(&[wi, j]).is_some() {
                        return Some(([wi, j], Direction::North));
                    }
                }
            }
            Direction::East => {
                for wj in jmin..=j {
                    if self.map.get(&[i, wj]).is_some() {
                        return Some(([i, wj], Direction::East));
                    }
                }
            }
            Direction::South => {
                for wi in imin..=i {
                    if self.map.get(&[wi, j]).is_some() {
                        return Some(([wi, j], Direction::South));
                    }
                }
            }
            Direction::West => {
                for wj in (j..=jmax).rev() {
                    if self.map.get(&[i, wj]).is_some() {
                        return Some(([i, wj], Direction::West));
                    }
                }
            }
        }

        None
    }

    fn step(&mut self) {
        let next_step = &self.path[self.i_path];
        self.i_path += 1;

        match next_step {
            Step::Step(n) => {
                // walk forward
                let [mut i, mut j] = self.location;

                for _ in 0..*n {
                    let [di, dj] = self.direction.dpos();
                    let ni = i + di;
                    let nj = j + dj;

                    match self.map.get(&[ni, nj]) {
                        Some(Tile::Floor(_)) | Some(Tile::Cursor(_)) => {
                            // allow movement
                            i = ni;
                            j = nj;
                            self.map.set([i, j], Tile::Floor(true));
                        }
                        Some(Tile::Wall) => {
                            // ignore movement
                        }
                        None => {
                            // wrap around
                            if let Some(([wi, wj], d)) = self.wrap([i, j], self.direction.clone()) {
                                if let Some(Tile::Floor(_)) = self.map.get(&[wi, wj]) {
                                    i = wi;
                                    j = wj;
                                    self.map.set([i, j], Tile::Floor(true));
                                    self.direction = d;
                                }
                            }
                        }
                    }
                }

                self.location = [i, j];
            }
            Step::Right => self.direction = self.direction.rot_right(),
            Step::Left => self.direction = self.direction.rot_left(),
        }
    }

    fn walk(&mut self) {
        while self.i_path < self.path.len() {
            self.step();
            // println!("{}", self);
        }
    }

    fn password(&self) -> i32 {
        let [i, j] = self.location;
        (i + 1) * 1000 + (j + 1) * 4 + self.direction.to_num()
    }
}

fn main() -> Result<()> {
    // let mut state = State::parse("data/day22/example", None)?;
    // state.walk();
    // println!("Part 1: {}", state.password());

    // {
    //     let mut wrap_example = HashMap::new();
    //     wrap_example.insert((0, Direction::West), (2, Direction::South));
    //     wrap_example.insert((0, Direction::North), (2, Direction::South));
    //     wrap_example.insert((0, Direction::East), (5, Direction::West));

    //     wrap_example.insert((1, Direction::West), (5, Direction::North));
    //     wrap_example.insert((1, Direction::North), (0, Direction::South));
    //     wrap_example.insert((1, Direction::South), (4, Direction::North));

    //     wrap_example.insert((2, Direction::North), (0, Direction::East));
    //     wrap_example.insert((2, Direction::South), (4, Direction::East));

    //     wrap_example.insert((3, Direction::East), (5, Direction::South));

    //     wrap_example.insert((4, Direction::West), (2, Direction::North));
    //     wrap_example.insert((4, Direction::South), (1, Direction::North));

    //     wrap_example.insert((5, Direction::North), (3, Direction::West));
    //     wrap_example.insert((5, Direction::East), (0, Direction::West));
    //     wrap_example.insert((5, Direction::South), (1, Direction::East));

    //     let mut state = State::parse("data/day22/example", Some(wrap_example))?;
    //     state.walk();
    //     println!("Part 2: {}", state.password());
    // }

    let mut state = State::parse("data/day22/input", None)?;
    state.walk();
    println!("Part 1: {}", state.password());

    {
        let mut wrap_input = HashMap::new();
        wrap_input.insert((0, Direction::West), (3, Direction::East));
        wrap_input.insert((0, Direction::North), (5, Direction::East));

        wrap_input.insert((1, Direction::North), (5, Direction::North));
        wrap_input.insert((1, Direction::East), (4, Direction::West));
        wrap_input.insert((1, Direction::South), (2, Direction::West));

        wrap_input.insert((2, Direction::West), (3, Direction::South));
        wrap_input.insert((2, Direction::East), (1, Direction::North));

        wrap_input.insert((3, Direction::West), (0, Direction::East));
        wrap_input.insert((3, Direction::North), (2, Direction::East));

        wrap_input.insert((4, Direction::East), (1, Direction::West));
        wrap_input.insert((4, Direction::South), (5, Direction::West));

        wrap_input.insert((5, Direction::West), (0, Direction::South));
        wrap_input.insert((5, Direction::East), (4, Direction::North));
        wrap_input.insert((5, Direction::South), (1, Direction::South));

        let mut state = State::parse("data/day22/input", Some(wrap_input))?;
        state.walk();

        println!("{}", state);
        println!("Part 2: {}", state.password());
    }

    Ok(())
}
