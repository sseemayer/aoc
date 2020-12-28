use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

use lazy_static::lazy_static;
use regex::Regex;

use snafu::{ResultExt, Snafu};

use aoc2020::map::{Map, MapError, MapTile, ParseMapTile};

lazy_static! {
    static ref RE_TILE: Regex = Regex::new(r"Tile (\d+)").unwrap();
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Number parsing error for '{}': {}", data, source))]
    ParseNumber {
        data: String,
        source: std::num::ParseIntError,
    },

    #[snafu(display("Map parsing error: {}", source))]
    ParseMap { source: MapError },
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Free,
    Occupied,
    SeaMonster,
}

impl ParseMapTile for Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Tile::Occupied),
            '.' => Some(Tile::Free),
            _ => None,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Free => '.',
                Tile::Occupied => '#',
                Tile::SeaMonster => '▉',
            }
        )
    }
}

fn parse_tiles<F: Read>(f: &mut F) -> Result<HashMap<usize, Map<[usize; 2], Tile>>> {
    let mut buf = String::new();
    let mut current_tile_id = None;
    let mut tiles = HashMap::new();
    for line in BufReader::new(f).lines() {
        let line = line.context(Io)?;
        let line = line.trim();

        if line.len() == 0 {
            continue;
        }

        if let Some(caps) = RE_TILE.captures(&line) {
            if let Some(cti) = current_tile_id {
                let tile = buf.trim().parse().context(ParseMap)?;
                buf.clear();
                tiles.insert(cti, tile);
            }

            let tile_id = caps.get(1).unwrap().as_str();
            let tile_id: usize = tile_id.parse().context(ParseNumber {
                data: tile_id.to_string(),
            })?;

            current_tile_id = Some(tile_id)
        } else {
            buf.extend(line.chars());
            buf.push('\n');
        }
    }

    if let Some(cti) = current_tile_id {
        let tile = buf.parse().context(ParseMap)?;
        tiles.insert(cti, tile);
    }

    Ok(tiles)
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Border {
    tiles: Vec<Tile>,
}

impl std::fmt::Debug for Border {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for t in &self.tiles {
            write!(f, "{}", t)?;
        }

        Ok(())
    }
}

impl From<Vec<Tile>> for Border {
    fn from(tiles: Vec<Tile>) -> Self {
        Self { tiles }
    }
}

/// Transformation over vertical flips and right rotations.
/// flips are always applied before rotations.
/// Note that this is a group together with horizontal/vertical flips and right/left rotations.
#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct Transform {
    flip: bool,
    rotate: u8,
}

impl Transform {
    fn new() -> Self {
        Transform {
            flip: false,
            rotate: 0,
        }
    }
    fn apply_to<T: MapTile>(&self, map: &Map<[usize; 2], T>) -> Map<[usize; 2], T> {
        let mut out = map.clone();

        if self.flip {
            out = out.flip(0);
        }

        for _ in 0..self.rotate {
            out = out.rotate_right()
        }

        out
    }

    fn get_side(&self, side: u8) -> u8 {
        (if self.flip {
            6 - side - self.rotate
        } else {
            self.rotate + side
        }) % 4
    }
}

impl std::ops::Add for Transform {
    type Output = Transform;
    fn add(self, rhs: Self) -> Self::Output {
        let (flip, rotate) = match (self.flip, rhs.flip) {
            (false, false) => (false, self.rotate + rhs.rotate),
            (false, true) => (true, (8 - self.rotate) + rhs.rotate),
            (true, false) => (true, self.rotate + rhs.rotate),
            (true, true) => (false, (8 - self.rotate) + rhs.rotate),
        };

        Transform {
            flip,
            rotate: rotate % 4,
        }
    }
}

impl std::ops::Neg for Transform {
    type Output = Transform;

    fn neg(self) -> Self::Output {
        Transform {
            flip: self.flip,
            rotate: if self.flip {
                self.rotate
            } else {
                (4 - self.rotate) % 4
            },
        }
    }
}

impl std::ops::Sub for Transform {
    type Output = Transform;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl std::fmt::Debug for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}{}>", if self.flip { "F" } else { " " }, self.rotate)
    }
}

fn get_top_border(map: &Map<[usize; 2], Tile>) -> Border {
    let (min, max) = map.get_extent();
    let tiles: Vec<Tile> = (min[1]..=max[1])
        .map(|j| map.data[&[0, j]].clone())
        .collect();

    Border { tiles }
}

fn border_codes(map: &Map<[usize; 2], Tile>) -> HashSet<(Transform, Border)> {
    let mut out = HashSet::new();

    for flip in &[false, true] {
        for rotate in 0..4 {
            let transform = Transform {
                flip: *flip,
                rotate,
            };
            let border = get_top_border(&transform.apply_to(map));
            out.insert((transform, border));
        }
    }

    out
}

fn solve(
    neighbors: &HashMap<usize, HashMap<u8, (usize, Border, Transform)>>,
    top_left_corner: usize,
    tiles: &HashMap<usize, Map<[usize; 2], Tile>>,
) -> HashMap<(usize, usize), (usize, Transform)> {
    let mut out = HashMap::new();

    let mut current = top_left_corner;
    let mut current_transform = Transform::new();
    let mut i = 0;
    let mut j = 0;

    loop {
        // println!("{} {}: {} {:?}", i, j, current, current_transform);
        out.insert((i, j), (current, current_transform));

        let right_side = current_transform.get_side(3);

        // can we go further right?
        if let Some((nid, border, delta)) = neighbors[&current].get(&right_side) {
            let new_transform = *delta + current_transform;
            let left_side = new_transform.get_side(1);
            // println!(
            //     "right neighbor is {} with transform {:?}. left side is {}",
            //     nid, new_transform, left_side
            // );

            let (_lid, mut left_border, _lt) = neighbors[&nid][&left_side].clone();
            if !delta.flip {
                left_border.tiles.reverse()
            }

            if border != &left_border {
                for k in &[current, *nid] {
                    let v = &neighbors[k];

                    println!("{}:", k);
                    for s in 0..4 {
                        if let Some((n, b, t)) = v.get(&s) {
                            println!("  {}: {:?} {:?} {}", s, b, t, n);
                        } else {
                            println!("  {}: -", s);
                        }
                    }
                }

                for (k, t) in &[(current, current_transform), (*nid, new_transform)] {
                    println!("{} {:?}\n{}", k, t, t.apply_to(&tiles[&k]))
                }
                panic!(
                    "Border mismatch!\n{}-{} right border: {:?}\n{}-{}  left border: {:?}",
                    current, right_side, border, nid, left_side, left_border
                );
            } else {
                // println!(
                //     "Right-Join {} and {} via border {:?}. new transform: {:?}",
                //     current, nid, border, new_transform
                // );
            }

            i += 1;
            current = *nid;
            current_transform = new_transform;

            continue;
        }

        // jump back to beginning of row
        current = out[&(0, j)].0;
        current_transform = out[&(0, j)].1;
        i = 0;
        j += 1;
        // println!("next row!");
        // println!("{} {}: {} {:?}", i, j, current, current_transform);

        let bottom_side = current_transform.get_side(2);

        // can we go down from first element of row?
        if let Some((nid, border, delta)) = neighbors[&current].get(&bottom_side) {
            let new_transform = *delta + current_transform;
            let top_side = new_transform.get_side(0);
            // println!(
            //     "bottom neighbor is {} with transform {:?}. top side is {}",
            //     nid, new_transform, top_side
            // );
            let (_tid, mut top_border, _tt) = neighbors[&nid][&top_side].clone();

            if !delta.flip {
                top_border.tiles.reverse();
            }

            if border != &top_border {
                for k in &[current, *nid] {
                    let v = &neighbors[k];

                    println!("{}:", k);
                    for s in 0..4 {
                        if let Some((n, b, t)) = v.get(&s) {
                            println!("  {}: {:?} {:?} {}", s, b, t, n);
                        } else {
                            println!("  {}: -", s);
                        }
                    }
                }

                for (k, t) in &[(current, current_transform), (*nid, new_transform)] {
                    println!("{} {:?}\n{}", k, t, t.apply_to(&tiles[&k]))
                }
                panic!(
                    "Border mismatch!\n{}-{} bottom border: {:?}\n{}-{}   top border: {:?}",
                    current, bottom_side, border, nid, top_side, top_border
                );
            } else {
                // println!(
                //     "Down-Join {} and {} via border {:?}. new transform: {:?}",
                //     current, nid, border, new_transform
                // );
            }

            current = *nid;
            current_transform = new_transform;
            continue;
        }

        // could not go right or (down from first row element) -- we must be done
        break;
    }

    out
}

fn join_all(
    tiles: &HashMap<usize, Map<[usize; 2], Tile>>,
    solution: HashMap<(usize, usize), (usize, Transform)>,
    padding: usize,
    cut: usize,
) -> Map<[usize; 2], Tile> {
    let (emin, [th, tw]) = tiles[&solution[&(0, 0)].0].get_extent();
    assert_eq!(emin[0], 0);
    assert_eq!(emin[1], 0);

    let mut out = Map::new();

    for ((x, y), (tid, transform)) in solution.iter() {
        let tile = transform.apply_to(&tiles[tid]);

        for ([ti, tj], tile) in tile.data.iter() {
            if *ti >= cut && *ti <= th - cut && *tj >= cut && *tj <= tw - cut {
                out.set(
                    [
                        y * (th + 1 + padding - 2 * cut) + ti - cut,
                        x * (tw + 1 + padding - 2 * cut) + tj - cut,
                    ],
                    tile.clone(),
                );
            }
        }
    }

    out
}

fn find_monsters(map: &mut Map<[usize; 2], Tile>, pattern: &Map<[usize; 2], Tile>) -> usize {
    let (min, max) = map.get_extent();
    let (_, pattern_dim) = pattern.get_extent();

    let mut n_found = 0;
    for iofs in min[0]..=max[0] - pattern_dim[0] {
        for jofs in min[1]..=max[1] - pattern_dim[1] {
            let mut found = true;
            for [pi, pj] in pattern.data.keys() {
                match map.get(&[iofs + pi, jofs + pj]) {
                    Some(Tile::Occupied) => {}
                    _ => {
                        found = false;
                        break;
                    }
                }
            }

            if found {
                n_found += 1;
                for [pi, pj] in pattern.data.keys() {
                    map.set([iofs + pi, jofs + pj], Tile::SeaMonster);
                }
            }
        }
    }
    n_found
}

fn borders_to_neighbors(
    borders: &HashMap<Border, Vec<(usize, Transform)>>,
) -> HashMap<usize, HashMap<u8, (usize, Border, Transform)>> {
    let mut neighbors: HashMap<usize, HashMap<u8, (usize, Border, Transform)>> = HashMap::new();
    for (border, ns) in borders {
        for (tile1, transform1) in ns {
            for (tile2, transform2) in ns {
                if tile1 == tile2 {
                    continue;
                }

                // disambiguate by requiring that one of the transforms is not flipped
                if transform1.flip {
                    continue;
                }

                let side = if transform1.flip {
                    4 - transform1.rotate
                } else {
                    transform1.rotate
                } % 4;

                let delta = *transform2
                    + Transform {
                        flip: true,
                        rotate: 4 - side,
                    };

                // println!(
                //     "{} has {:?} on side {} with transform {:?} shared with {} with transform {:?} -> delta {:?}",
                //     tile1, border, side, transform1, tile2, transform2, delta
                // );

                let existing = neighbors
                    .entry(*tile1)
                    .or_insert(HashMap::new())
                    .insert(side, (*tile2, border.clone(), delta));

                // after disambiguation, should get every neighbor exactly once
                assert!(existing.is_none());
            }
        }
    }
    neighbors
}

fn main() -> Result<()> {
    let tiles = parse_tiles(&mut File::open("data/day20/input").context(Io)?)?;

    let mut borders: HashMap<Border, Vec<(usize, Transform)>> = HashMap::new();
    for (tid, tile) in tiles.iter() {
        for (transform, bc) in border_codes(tile) {
            borders
                .entry(bc)
                .or_insert(Vec::new())
                .push((*tid, transform));
        }
    }

    let neighbors = borders_to_neighbors(&borders);

    let mut prod = 1;
    let mut top_left_corner = None;
    for (k, v) in &neighbors {
        // println!("{}:", k);
        // for s in 0..4 {
        //     if let Some((n, b, t)) = v.get(&s) {
        //         println!("  {}: {:?} {:?} {}", s, b, t, n);
        //     } else {
        //         println!("  {}: -", s);
        //     }
        // }

        if v.len() == 2 {
            prod *= k;

            if v.contains_key(&3) && v.contains_key(&2) {
                top_left_corner = Some(k)
            }
        }
    }

    println!("Part 1: {}", prod);

    let top_left_corner = top_left_corner.expect("top left corner");

    let solution = solve(&neighbors, *top_left_corner, &tiles);
    let map = join_all(&tiles, solution, 0, 1).flip(0);

    let sea_monster: Map<[usize; 2], Tile> =
        Map::read(&mut File::open("data/day20/sea_monster").context(Io)?).context(ParseMap)?;

    let mut max_found = 0;
    let mut max_transformed = map.clone();
    for flip in &[false, true] {
        for rotate in 0..=3 {
            let transform = Transform {
                flip: *flip,
                rotate,
            };

            let mut transformed = transform.apply_to(&map);

            let n_found = find_monsters(&mut transformed, &sea_monster);

            if n_found > max_found {
                max_found = n_found;
                max_transformed = transformed;
            }
        }
    }

    println!("{}", max_transformed);

    let mut n_waves = 0;
    for t in max_transformed.data.values() {
        if t == &Tile::Occupied {
            n_waves += 1;
        }
    }

    println!("Part 2: water roughness is {}", n_waves);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct T(char);

    impl std::fmt::Display for T {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl ParseMapTile for T {
        fn from_char(c: char) -> Option<Self> {
            if c == ' ' {
                None
            } else {
                Some(T(c))
            }
        }
    }

    #[test]
    fn test_addition() {
        let map: Map<[usize; 2], T> = "abc\ndef\nghi".parse().unwrap();

        for flip_a in &[false, true] {
            for rot_a in 0..=3 {
                let a = Transform {
                    flip: *flip_a,
                    rotate: rot_a,
                };
                for flip_b in &[false, true] {
                    for rot_b in 0..=3 {
                        let b = Transform {
                            flip: *flip_b,
                            rotate: rot_b,
                        };

                        println!("{:?} {:?}", a, b);

                        assert_eq!(b.apply_to(&a.apply_to(&map)), (a + b).apply_to(&map));
                    }
                }
            }
        }
    }

    #[test]
    fn test_neutral() {
        for flip_a in &[false, true] {
            for rot_a in 0..=3 {
                let a = Transform {
                    flip: *flip_a,
                    rotate: rot_a,
                };
                println!("{:?}", a);

                assert_eq!(
                    a + (-a),
                    Transform {
                        flip: false,
                        rotate: 0
                    }
                );
            }
        }
    }
}
