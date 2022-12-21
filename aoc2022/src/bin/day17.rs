use std::{collections::HashMap, fs::File, hash::Hash, io::Read};

use anyhow::{bail, Result};
use colored::Colorize;

type Pos = [i32; 2];
type Map = aoc::map::Map<Pos, Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Jet {
    Left,
    Right,
}

impl std::convert::TryFrom<char> for Jet {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Jet::Left),
            '>' => Ok(Jet::Right),
            _ => bail!("Bad jet: '{}'", value),
        }
    }
}

impl Jet {
    fn get_offset(&self) -> i32 {
        match self {
            Jet::Left => -1,
            Jet::Right => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Shape {
    Horizontal,
    Cross,
    Corner,
    Vertical,
    Square,
}

impl Shape {
    fn next(&self) -> Self {
        match self {
            Shape::Horizontal => Shape::Cross,
            Shape::Cross => Shape::Corner,
            Shape::Corner => Shape::Vertical,
            Shape::Vertical => Shape::Square,
            Shape::Square => Shape::Horizontal,
        }
    }

    fn to_map(&self) -> Map {
        let mut map: Map = match self {
            Shape::Horizontal => "####",
            Shape::Cross => ".#.\n###\n.#.",
            Shape::Corner => "..#\n..#\n###",
            Shape::Vertical => "#\n#\n#\n#",
            Shape::Square => "##\n##",
        }
        .parse()
        .unwrap();

        for (_, t) in map.data.iter_mut() {
            if let Tile::Rock(s) = t {
                *s = *self;
            }
        }

        map
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Rock(Shape),
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Wall => write!(f, "█"),
            Tile::Rock(s) => write!(
                f,
                "{}",
                match s {
                    Shape::Horizontal => "▒".red(),
                    Shape::Cross => "▒".green(),
                    Shape::Corner => "▒".blue(),
                    Shape::Vertical => "▒".yellow(),
                    Shape::Square => "▒".white(),
                }
            ),
        }
    }
}

impl aoc::map::ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Tile::Rock(Shape::Horizontal)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    map: Map,
    shape: Shape,

    pattern: Vec<Jet>,
    pattern_pos: usize,

    omitted_height: usize,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}next_shape={:?} next_jet={:?} pattern_pos={} omitted={}\n",
            self.map,
            self.shape,
            self.pattern[self.pattern_pos],
            self.pattern_pos,
            self.omitted_height
        )
    }
}

impl State {
    fn new(pattern: Vec<Jet>, chamber_width: i32) -> Self {
        let mut map = Map::new();
        for j in 0..chamber_width {
            map.set([0, j], Tile::Wall);
        }

        Self {
            map,
            shape: Shape::Horizontal,
            pattern,
            pattern_pos: 0,
            omitted_height: 0,
        }
    }

    /// check if there would be a collision if we had shape at position [i, j]
    fn collision_check(&self, shape: &Map, [i, j]: Pos) -> bool {
        let ([_imin, jmin], [_imax, jmax]) = self.map.get_extent();

        shape.data.iter().any(|([si, sj], _)| {
            sj + j > jmax || sj + j < jmin || self.map.get(&[si + i, sj + j]).is_some()
        })
    }

    fn blit(&mut self, shape: &Map, [i, j]: Pos) {
        for ([si, sj], t) in shape.data.iter() {
            self.map.set([si + i, sj + j], *t);
        }
    }

    fn step(&mut self) {
        let ([imin, _jmin], [_imax, _jmax]) = self.map.get_extent();

        // get current shape and advance shape
        let shape = self.shape.to_map();
        self.shape = self.shape.next();

        let ([simin, _sjmin], [simax, _sjmax]) = shape.get_extent();

        let mut i = imin - (simax - simin) - 4;
        let mut j = 2;

        loop {
            // get current wind conditions and advance wind
            let jet = self.pattern[self.pattern_pos];
            let move_j = j + jet.get_offset();
            self.pattern_pos = (self.pattern_pos + 1) % self.pattern.len();

            // try to move with the wind
            if !self.collision_check(&shape, [i, move_j]) {
                j = move_j;
                // println!(" {:5} i={} j={}", format!("{:?}", jet), i, j);
            } else {
                // println!("!{:5} i={} j={}", format!("{:?}", jet), i, j);
            }

            // try to move down
            if !self.collision_check(&shape, [i + 1, j]) {
                // we can keep going
                i += 1;
                // println!(" Down  i={} j={}", i, j);
            } else {
                // we hit something -- make it permanent
                self.blit(&shape, [i, j]);
                // println!("!Down  i={} j={}", i, j);
                break;
            }
        }

        self.simplify();
    }

    fn simplify(&mut self) {
        let ([imin, jmin], [imax, jmax]) = self.map.get_extent();

        for i in imin..imax {
            if (jmin..=jmax).all(|j| self.map.get(&[i, j]).is_some()) {
                self.omitted_height += (imax - i) as usize;

                let mut new_map = Map::new();
                for (&[ti, tj], t) in self.map.data.iter() {
                    if ti < i {
                        new_map.set([ti - i, tj], *t);
                    }
                }

                for j in jmin..=jmax {
                    new_map.set([0, j], Tile::Wall);
                }

                self.map = new_map;

                break;
            }
        }
    }

    fn height(&self) -> usize {
        let ([imin, _], [imax, _]) = self.map.get_extent();
        (imax - imin) as usize + self.omitted_height
    }

    fn hash_key(&self) -> String {
        format!(
            "{} shape={:?} pp={}",
            self.map, self.shape, self.pattern_pos
        )
    }
}

fn parse_pattern(path: &str) -> Result<Vec<Jet>> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;

    buffer
        .trim()
        .chars()
        .map(|c| Jet::try_from(c))
        .collect::<Result<Vec<Jet>>>()
}

fn main() -> Result<()> {
    let pattern = parse_pattern("data/day17/input")?;

    let mut state = State::new(pattern, 7);
    // println!("{}", state);

    let target_steps = 1_000_000_000_000;
    let mut seen = HashMap::new();
    let mut i = 1usize;
    loop {
        state.step();

        if let Some((h, j)) = seen.insert(state.hash_key(), (state.height(), i)) {
            // println!("Seen {} before as {} with height {}:\n{}", i, j, h, state);

            let i_step = i - j;
            let h_step = state.height() - h;

            let steps = (target_steps - i) / i_step;

            state.omitted_height += steps * h_step;
            i += steps * i_step;
        }

        if i == 2022 {
            println!("Part 1: {}", state.height());
        }

        if i >= target_steps {
            println!("Part 2: {}", state.height());
            break;
        }

        i += 1;
    }

    Ok(())
}
