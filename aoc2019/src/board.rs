use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Position {
    pub i: i64,
    pub j: i64,
}

impl Position {
    pub const ZERO: Position = Position { i: 0, j: 0 };
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.j, self.i)
    }
}

impl std::ops::Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Position {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
        }
    }
}

impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.i += rhs.i;
        self.j += rhs.j;
    }
}

impl From<(i64, i64)> for Position {
    fn from(p: (i64, i64)) -> Self {
        Position { i: p.0, j: p.1 }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let c = match self {
            Direction::North => "🠉",
            Direction::South => "🠋",
            Direction::West => "🠈 ",
            Direction::East => "🠊 ",
        };
        write!(f, "{}", c)
    }
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn to_input(&self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }

    pub fn to_ofs(&self) -> (i64, i64) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::East => (0, 1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board<T> {
    pub tiles: HashMap<Position, T>,
}

impl<T: std::fmt::Display + std::default::Default> std::fmt::Display for Board<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        // find maximum drawing coords
        let (i_min, i_max, j_min, j_max) = self.get_extent();

        for i in i_min..=i_max {
            for j in j_min..=j_max {
                if let Some(t) = self.tiles.get(&Position { i, j }) {
                    write!(f, "{}", t)?;
                } else {
                    write!(f, "{}", T::default())?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl<T> Board<T> {
    pub fn new() -> Self {
        Board {
            tiles: HashMap::new(),
        }
    }

    pub fn get_extent(&self) -> (i64, i64, i64, i64) {
        let mut i_min = 0;
        let mut i_max = 0;
        let mut j_min = 0;
        let mut j_max = 0;
        for pos in self.tiles.keys() {
            if pos.i < i_min {
                i_min = pos.i;
            }
            if pos.i > i_max {
                i_max = pos.i;
            }
            if pos.j < j_min {
                j_min = pos.j;
            }
            if pos.j > j_max {
                j_max = pos.j;
            }
        }

        (i_min, i_max, j_min, j_max)
    }
}

impl<T: std::default::Default + std::marker::Copy + std::cmp::PartialEq> Board<T> {
    pub fn get(&self, pos: &Position) -> T {
        self.tiles.get(pos).map(|t| *t).unwrap_or(T::default())
    }

    pub fn set(&mut self, pos: &Position, tile: T) {
        self.tiles.insert(pos.clone(), tile);
    }
    pub fn where_is(&self, tile: &T) -> Option<Position> {
        for (pos, t) in self.tiles.iter() {
            if t == tile {
                return Some(*pos);
            }
        }

        None
    }

    pub fn where_are(&self, tile: &T) -> Vec<Position> {
        let mut out = Vec::new();
        for (pos, t) in self.tiles.iter() {
            if t == tile {
                out.push(*pos);
            }
        }
        out
    }
}

impl<
        T: std::default::Default
            + std::marker::Copy
            + std::cmp::PartialEq
            + std::cmp::Eq
            + std::hash::Hash,
    > Board<T>
{
    pub fn count(&self) -> HashMap<T, usize> {
        let mut out = HashMap::new();

        for t in self.tiles.values() {
            out.entry(*t).and_modify(|c| *c += 1).or_insert(1);
        }

        out
    }
}
