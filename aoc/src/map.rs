use std::collections::HashMap;

use snafu::{ResultExt, Snafu};
use std::io::{BufRead, BufReader};

#[derive(Debug, Snafu)]
pub enum MapError {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },
}

type MapResult<T> = std::result::Result<T, MapError>;

/// Trait for a generic integer coordinate
pub trait IntCoord:
    num::PrimInt
    + num::FromPrimitive
    + std::fmt::Debug
    + std::fmt::Display
    + std::default::Default
    + std::hash::Hash
{
}

// auto-implement for types that meet requirements
impl<T> IntCoord for T where
    T: num::PrimInt
        + num::FromPrimitive
        + num::ToPrimitive
        + std::marker::Copy
        + std::fmt::Debug
        + std::fmt::Display
        + std::default::Default
        + std::hash::Hash
{
}

/// Trait for types that can correspond to map tiles
pub trait MapTile: std::fmt::Display + Sized + Clone {}

// auto-implement for types that meet requirements
impl<T> MapTile for T where T: std::fmt::Display + Sized + Clone {}

/// Trait for types that can be parsed as a map tile
pub trait ParseMapTile: MapTile {
    fn from_char(c: char) -> Option<Self>;
}

/// Trait for a generic map coordinate
pub trait MapCoordinate: Default + Eq + std::hash::Hash + std::fmt::Debug + Clone + Copy {
    type ExtentIter: Iterator<Item = Self>;

    /// Calculate the element-wise minimum of two coordinates.
    /// Used for computing the extent of the map.
    fn elementwise_min(a: Self, b: Self) -> Self;

    /// Calculate the element-wise maximum of two coordinates.
    /// Used for computing the extent of the map.
    fn elementwise_max(a: Self, b: Self) -> Self;

    fn get_extent<'a>(mut keys: impl Iterator<Item = Self>) -> (Self, Self) {
        let mut min = keys.next().unwrap_or(Default::default());
        let mut max = min.clone();

        for k in keys {
            min = MapCoordinate::elementwise_min(min, k);
            max = MapCoordinate::elementwise_max(max, k);
        }

        (min, max)
    }

    /// Get an iterator spanning all coordinates within an extent
    fn extent_iterator(min: Self, max: Self) -> Self::ExtentIter;
}

/// A tile-based map that is generic over coordinates and tiles stored within
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Map<C: MapCoordinate, T> {
    pub data: HashMap<C, T>,
    pub fixed_extent: Option<(C, C)>,
}

impl<C: MapCoordinate, T> Map<C, T> {
    pub fn new() -> Self {
        Map {
            data: HashMap::new(),
            fixed_extent: None,
        }
    }

    /// Get the tile at a coordinate
    pub fn get(&self, coord: &C) -> Option<&T> {
        self.data.get(coord)
    }

    /// Get a mutable reference to a tile at a coordinate
    pub fn get_mut(&mut self, coord: &C) -> Option<&mut T> {
        self.data.get_mut(coord)
    }

    /// Set the tile at a coordinate
    pub fn set(&mut self, coord: C, value: T) {
        self.data.insert(coord, value);
    }

    /// Clear a coordinate from tiles
    pub fn remove(&mut self, coord: &C) {
        self.data.remove(coord);
    }

    /// Get the maximum dimension for all defined tiles
    pub fn get_extent(&self) -> (C, C) {
        if let Some(e) = &self.fixed_extent {
            e.clone()
        } else {
            C::get_extent(self.data.keys().cloned())
        }
    }

    /// Find all coordinates that match a predicate
    pub fn find_all_where<P: Fn(&C, &T) -> bool>(&self, predicate: P) -> Vec<C> {
        let mut out: Vec<C> = Vec::new();
        for (coord, tile) in self.data.iter() {
            if predicate(coord, tile) {
                out.push(coord.clone());
            }
        }
        out
    }

    /// Find a coordinate that matches a predicate
    pub fn find_one_where<P: Fn(&C, &T) -> bool>(&self, predicate: P) -> Option<C> {
        for (coord, tile) in self.data.iter() {
            if predicate(coord, tile) {
                return Some(coord.clone());
            }
        }
        None
    }
}

impl<C: MapCoordinate, T: Eq> Map<C, T> {
    /// Find all coordinates that contain a tile
    pub fn find_all(&self, pattern: &T) -> Vec<C> {
        self.find_all_where(|_, t| t == pattern)
    }

    /// Find one coordinate that contains a tile
    pub fn find_one(&self, pattern: &T) -> Option<C> {
        self.find_one_where(|_, t| t == pattern)
    }
}

////// Code for 2D maps

impl<I> MapCoordinate for [I; 2]
where
    I: IntCoord,
{
    type ExtentIter = Extent2DIterator<I>;

    fn elementwise_min(a: Self, b: Self) -> Self {
        [std::cmp::min(a[0], b[0]), std::cmp::min(a[1], b[1])]
    }

    fn elementwise_max(a: Self, b: Self) -> Self {
        [std::cmp::max(a[0], b[0]), std::cmp::max(a[1], b[1])]
    }

    fn extent_iterator(min: Self, max: Self) -> Self::ExtentIter {
        Extent2DIterator {
            min,
            max,
            current: Some(min.clone()),
        }
    }
}

pub struct Extent2DIterator<I>
where
    I: IntCoord,
{
    min: [I; 2],
    max: [I; 2],
    current: Option<[I; 2]>,
}

impl<I> Iterator for Extent2DIterator<I>
where
    I: IntCoord,
{
    type Item = [I; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.current {
            let mut next = c.clone();

            next[0] = next[0] + I::one();
            if next[0] > self.max[0] {
                next[0] = self.min[0];
                next[1] = next[1] + I::one();
            }

            if next[1] > self.max[1] {
                self.current = None
            } else {
                self.current = Some(next)
            }

            Some(c)
        } else {
            None
        }
    }
}

impl<T, I> Map<[I; 2], T>
where
    T: ParseMapTile,
    I: IntCoord,
{
    pub fn read<R: std::io::Read>(reader: &mut R) -> MapResult<Self> {
        let mut data: HashMap<[I; 2], T> = HashMap::new();

        let buf_reader = BufReader::new(reader);
        for (i, line) in buf_reader.lines().enumerate() {
            for (j, c) in line.context(Io)?.chars().enumerate() {
                if let Some(t) = T::from_char(c) {
                    if let (Some(i), Some(j)) = (I::from_usize(i), I::from_usize(j)) {
                        data.insert([i, j], t);
                    }
                }
            }
        }

        Ok(Map {
            data,
            fixed_extent: None,
        })
    }
}

impl<T, I> Map<[I; 2], T>
where
    T: MapTile,
    I: IntCoord,
{
    pub fn rotate_right(&self) -> Self {
        let (min, max) = self.get_extent();
        assert_eq!(min[0], I::zero());
        assert_eq!(min[1], I::zero());

        let mut out = Map::new();

        //  j 01234    0123
        // i
        // 0  abcde    kfa
        // 1  fghIj    Lgb
        // 2  kLmno    mhc
        // 3           nId
        // 4           oje

        for ([i, j], tile) in self.data.iter() {
            out.set([*j, max[0] - *i], tile.clone());
        }

        out
    }

    pub fn rotate_left(&self) -> Self {
        let (min, max) = self.get_extent();
        assert_eq!(min[0], I::zero());
        assert_eq!(min[1], I::zero());

        let mut out = Map::new();

        //  j 01234    0123
        // i
        // 0  abcde    ejo
        // 1  fghIj    dIn
        // 2  kLmno    chm
        // 3           bgL
        // 4           afk

        for ([i, j], tile) in self.data.iter() {
            out.set([max[1] - *j, *i], tile.clone());
        }

        out
    }

    pub fn flip(&self, axis: usize) -> Self {
        let (_min, max) = self.get_extent();

        let mut out = Map::new();
        for (pos, tile) in self.data.iter() {
            let mut pos = pos.clone();
            pos[axis] = max[axis] - pos[axis];

            out.set(pos, tile.clone());
        }

        out
    }

    pub fn to_vecs(&self) -> Vec<Vec<Option<T>>> {
        let (min, max) = self.get_extent();

        num::iter::range_inclusive(min[0], max[0])
            .map(|i| {
                num::iter::range_inclusive(min[1], max[1])
                    .map(|j| self.data.get(&[i, j]).cloned())
                    .collect()
            })
            .collect()
    }
}

impl<T, I> std::fmt::Display for Map<[I; 2], T>
where
    T: MapTile,
    I: IntCoord,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        if self.data.is_empty() {
            return Ok(());
        }

        let (min, max) = self.get_extent();

        for i in num::iter::range_inclusive(min[0], max[0]) {
            for j in num::iter::range_inclusive(min[1], max[1]) {
                match self.data.get(&[i, j]) {
                    Some(t) => t.fmt(f),
                    None => write!(f, " "),
                }?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl<T, I> std::str::FromStr for Map<[I; 2], T>
where
    T: ParseMapTile,
    I: IntCoord,
{
    type Err = MapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Map::read(&mut s.as_bytes())
    }
}

////// Code for 3D maps

impl<I> MapCoordinate for [I; 3]
where
    I: IntCoord,
{
    type ExtentIter = Extent3DIterator<I>;

    fn elementwise_min(a: Self, b: Self) -> Self {
        [
            std::cmp::min(a[0], b[0]),
            std::cmp::min(a[1], b[1]),
            std::cmp::min(a[2], b[2]),
        ]
    }

    fn elementwise_max(a: Self, b: Self) -> Self {
        [
            std::cmp::max(a[0], b[0]),
            std::cmp::max(a[1], b[1]),
            std::cmp::max(a[2], b[2]),
        ]
    }

    fn extent_iterator(min: Self, max: Self) -> Self::ExtentIter {
        Extent3DIterator {
            min,
            max,
            current: Some(min.clone()),
        }
    }
}

pub struct Extent3DIterator<I>
where
    I: IntCoord,
{
    min: [I; 3],
    max: [I; 3],
    current: Option<[I; 3]>,
}

impl<I> Iterator for Extent3DIterator<I>
where
    I: IntCoord,
{
    type Item = [I; 3];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.current {
            let mut next = c.clone();

            next[0] = next[0] + I::one();
            if next[0] > self.max[0] {
                next[0] = self.min[0];
                next[1] = next[1] + I::one();
            }

            if next[1] > self.max[1] {
                next[1] = self.min[1];
                next[2] = next[2] + I::one();
            }

            if next[2] > self.max[2] {
                self.current = None
            } else {
                self.current = Some(next)
            }

            Some(c)
        } else {
            None
        }
    }
}

impl<T, I> Map<[I; 3], T>
where
    T: MapTile,
    I: IntCoord,
{
    /// Convert a 2D map to a single-layered 3D map
    pub fn from_2d(map: &Map<[I; 2], T>) -> Self {
        let data: HashMap<[I; 3], T> = map
            .data
            .iter()
            .map(|(key, tile)| {
                let key = [I::zero(), key[0], key[1]];
                (key, tile.clone())
            })
            .collect();

        Map {
            data,
            fixed_extent: None,
        }
    }

    /// Slice a 3D map into a 2D map along one dimension
    pub fn slice(&self, i: I, axis: usize) -> Map<[I; 2], T> {
        let (ax0, ax1) = match axis {
            0 => (1, 2),
            1 => (0, 2),
            2 => (0, 1),
            _ => panic!("Invalid axis: {}", axis),
        };

        let data: HashMap<[I; 2], T> = self
            .data
            .iter()
            .filter_map(|(k, t)| {
                if k[axis] == i {
                    Some(([k[ax0], k[ax1]], t.clone()))
                } else {
                    None
                }
            })
            .collect();

        let fixed_extent = self
            .fixed_extent
            .map(|(min, max)| ([min[ax0], min[ax1]], [max[ax0], max[ax1]]));

        Map { data, fixed_extent }
    }

    pub fn to_vecs(&self) -> Vec<Vec<Vec<Option<T>>>> {
        let (min, max) = self.get_extent();

        num::iter::range_inclusive(min[0], max[0])
            .map(|i| {
                num::iter::range_inclusive(min[1], max[1])
                    .map(|j| {
                        num::iter::range_inclusive(min[2], max[2])
                            .map(|k| self.data.get(&[i, j, k]).cloned())
                            .collect()
                    })
                    .collect()
            })
            .collect()
    }
}

impl<T, I> std::fmt::Display for Map<[I; 3], T>
where
    T: MapTile,
    I: IntCoord,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        if self.data.is_empty() {
            return Ok(());
        }

        let (min, max) = self.get_extent();

        for i in num::iter::range_inclusive(min[0], max[0]) {
            write!(f, "Layer {} =========\n{}\n", i, self.slice(i, 0))?;
        }

        Ok(())
    }
}

////// Code for 4D maps

impl<I> MapCoordinate for [I; 4]
where
    I: IntCoord,
{
    type ExtentIter = Extent4DIterator<I>;

    fn elementwise_min(a: Self, b: Self) -> Self {
        [
            std::cmp::min(a[0], b[0]),
            std::cmp::min(a[1], b[1]),
            std::cmp::min(a[2], b[2]),
            std::cmp::min(a[3], b[3]),
        ]
    }

    fn elementwise_max(a: Self, b: Self) -> Self {
        [
            std::cmp::max(a[0], b[0]),
            std::cmp::max(a[1], b[1]),
            std::cmp::max(a[2], b[2]),
            std::cmp::max(a[3], b[3]),
        ]
    }

    fn extent_iterator(min: Self, max: Self) -> Self::ExtentIter {
        Extent4DIterator {
            min,
            max,
            current: Some(min.clone()),
        }
    }
}

pub struct Extent4DIterator<I>
where
    I: IntCoord,
{
    min: [I; 4],
    max: [I; 4],
    current: Option<[I; 4]>,
}

impl<I> Iterator for Extent4DIterator<I>
where
    I: IntCoord,
{
    type Item = [I; 4];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.current {
            let mut next = c.clone();

            next[0] = next[0] + I::one();
            if next[0] > self.max[0] {
                next[0] = self.min[0];
                next[1] = next[1] + I::one();
            }

            if next[1] > self.max[1] {
                next[1] = self.min[1];
                next[2] = next[2] + I::one();
            }

            if next[2] > self.max[2] {
                next[2] = self.min[2];
                next[3] = next[3] + I::one();
            }

            if next[3] > self.max[3] {
                self.current = None
            } else {
                self.current = Some(next)
            }

            Some(c)
        } else {
            None
        }
    }
}

impl<T, I> Map<[I; 4], T>
where
    T: MapTile,
    I: IntCoord,
{
    /// Convert a 3D map to a single-layered 4D map
    pub fn from_3d(map: &Map<[I; 3], T>) -> Self {
        let data: HashMap<[I; 4], T> = map
            .data
            .iter()
            .map(|(key, tile)| {
                let key = [I::zero(), key[0], key[1], key[2]];
                (key, tile.clone())
            })
            .collect();

        Map {
            data,
            fixed_extent: None,
        }
    }

    /// Slice a 4D map into a 3D map along one dimension
    pub fn slice(&self, i: I, axis: usize) -> Map<[I; 3], T> {
        let (ax0, ax1, ax2) = match axis {
            0 => (1, 2, 3),
            1 => (0, 2, 3),
            2 => (0, 1, 3),
            3 => (0, 1, 2),
            _ => panic!("Invalid axis: {}", axis),
        };

        let data: HashMap<[I; 3], T> = self
            .data
            .iter()
            .filter_map(|(k, t)| {
                if k[axis] == i {
                    Some(([k[ax0], k[ax1], k[ax2]], t.clone()))
                } else {
                    None
                }
            })
            .collect();

        let fixed_extent = self.fixed_extent.map(|(min, max)| {
            (
                [min[ax0], min[ax1], min[ax2]],
                [max[ax0], max[ax1], max[ax2]],
            )
        });

        Map { data, fixed_extent }
    }

    pub fn to_vecs(&self) -> Vec<Vec<Vec<Vec<Option<T>>>>> {
        let (min, max) = self.get_extent();

        num::iter::range_inclusive(min[0], max[0])
            .map(|i| {
                num::iter::range_inclusive(min[1], max[1])
                    .map(|j| {
                        num::iter::range_inclusive(min[2], max[2])
                            .map(|k| {
                                num::iter::range_inclusive(min[3], max[3])
                                    .map(|l| self.data.get(&[i, j, k, l]).cloned())
                                    .collect()
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect()
    }
}

impl<T, I> std::fmt::Display for Map<[I; 4], T>
where
    T: MapTile,
    I: IntCoord,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        if self.data.is_empty() {
            return Ok(());
        }

        let (min, max) = self.get_extent();

        for i in num::iter::range_inclusive(min[0], max[0]) {
            for j in num::iter::range_inclusive(min[1], max[1]) {
                write!(
                    f,
                    "Layer {}, {} =========\n{}\n",
                    i,
                    j,
                    self.slice(i, 0).slice(j, 0)
                )?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl ParseMapTile for char {
        fn from_char(c: char) -> Option<Self> {
            if c == ' ' {
                None
            } else {
                Some(c)
            }
        }
    }

    fn assert_map_eq<I, T>(a: &Map<[I; 2], T>, b: &Map<[I; 2], T>)
    where
        I: IntCoord,
        T: MapTile + PartialEq,
    {
        if a != b {
            panic!(
                "Map mismatch:\na (extent {:?}):\n{}\nb (extent {:?}):\n{}",
                a.get_extent(),
                a,
                b.get_extent(),
                b
            )
        }
    }

    #[test]
    fn test_2d_parsing() {
        let map_string = "ab \nd e";
        let map = Map::<[usize; 2], char>::read(&mut map_string.as_bytes()).unwrap();

        assert_eq!(map.get_extent(), ([0, 0], [1, 2]));

        assert_eq!(
            map.to_vecs(),
            vec![
                vec![Some('a'), Some('b'), None],
                vec![Some('d'), None, Some('e')],
            ]
        )
    }

    #[test]
    fn test_2d_editing() {
        let mut map: Map<[usize; 2], char> = Map::new();

        assert_eq!(map.get(&[1, 2]), None);
        map.set([1, 2], 'a');
        assert_eq!(map.get(&[1, 2]), Some(&'a'));

        map.set([4, 1], 'c');
        assert_eq!(map.get_extent(), ([1, 1], [4, 2]));

        map.set([8, 8], 'd');
        assert_eq!(map.get_extent(), ([1, 1], [8, 8]));

        map.remove(&[8, 8]);
        assert_eq!(map.get_extent(), ([1, 1], [4, 2]));

        assert_eq!(
            map.to_vecs(),
            vec![
                vec![None, Some('a')],
                vec![None, None],
                vec![None, None],
                vec![Some('c'), None],
            ]
        )
    }

    #[test]
    fn test_2d_display() {
        let map_string = "ab \nd e";
        let map = Map::<[usize; 2], char>::read(&mut map_string.as_bytes()).unwrap();

        assert_eq!(format!("{}", map), format!("{}\n", map_string));

        let map2: Map<[usize; 2], char> = Map::new();
        assert_eq!(format!("{}", map2), "");
    }

    #[test]
    fn test_2d_rotating_flipping() {
        //              R    L
        //  j 01234    012  012
        // i
        // 0  abcde    kfa  ejo
        // 1  fghIj    Lgb  dIn
        // 2  kLmno    mhc  chm
        // 3           nId  bgL
        // 4           oje  afk
        //
        //
        // FLIP i      FLIP j
        //
        //   kLmno     edcba
        //   fghIj     jIhgf
        //   abcde     onmLk

        let map = "abcde\nfghIj\nkLmno"
            .parse::<Map<[usize; 2], char>>()
            .unwrap();

        assert_map_eq(
            &map.rotate_right(),
            &"kfa\nLgb\nmhc\nnId\noje".parse().unwrap(),
        );

        assert_map_eq(
            &map.rotate_left(),
            &"ejo\ndIn\nchm\nbgL\nafk".parse().unwrap(),
        );

        assert_map_eq(&map.flip(0), &"kLmno\nfghIj\nabcde".parse().unwrap());
        assert_map_eq(&map.flip(1), &"edcba\njIhgf\nonmLk".parse().unwrap());
    }
}
