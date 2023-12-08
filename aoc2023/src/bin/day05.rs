use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::{anyhow, Context, Result};

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,

    maps: HashMap<String, Map>,
}

impl Almanac {
    fn parse<R: std::io::Read>(reader: R) -> Result<Self> {
        let reader = BufReader::new(reader);

        let mut seeds = Vec::new();
        let mut maps = HashMap::new();
        let mut current_map: Option<(String, Map)> = None;
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if line.starts_with("seeds: ") {
                seeds.extend(
                    line.trim_start_matches("seeds: ")
                        .trim()
                        .split_whitespace()
                        .map(|n| n.parse().context("Parse seeds"))
                        .collect::<Result<Vec<usize>>>()?,
                )
            } else if line.ends_with("map:") {
                if let Some((source, map)) = current_map.as_mut() {
                    map.entries.sort_by_key(|e| e.source_start);
                    maps.insert(source.clone(), map.clone());
                }

                let (source, destination) = line
                    .trim_end_matches(" map:")
                    .trim()
                    .split_once("-to-")
                    .ok_or_else(|| anyhow!("Error splitting map name: '{}'", line))?;

                current_map = Some((
                    source.to_string(),
                    Map {
                        destination_type: destination.to_string(),
                        entries: Vec::new(),
                    },
                ));
            } else {
                let (_source, map) = current_map.as_mut().ok_or_else(|| {
                    anyhow!(
                        "Encountered map entries before encountering map header: '{}'",
                        line
                    )
                })?;

                let entry: MapEntry = line.parse()?;
                map.entries.push(entry);
            }
        }

        if let Some((source, map)) = current_map.as_mut() {
            map.entries.sort_by_key(|e| e.source_start);
            maps.insert(source.clone(), map.clone());
        }

        Ok(Self { seeds, maps })
    }

    fn transform_seed(&self, seed_value: usize) -> Result<usize> {
        let mut key = "seed";
        let mut value = seed_value;

        // print!("{} {}", key, value);
        while key != "location" {
            let map = self
                .maps
                .get(key)
                .ok_or_else(|| anyhow!("Cannot get '{}' map", key))?;

            value = map.transform(value);
            key = &map.destination_type;

            // print!(" -> {} {}", key, value);
        }

        // println!("");

        Ok(value)
    }

    fn transform_seed_range(
        &self,
        seed_ranges: &Vec<(usize, usize)>,
    ) -> Result<Vec<(usize, usize)>> {
        let mut key = "seed";
        let mut current = seed_ranges.clone();

        // println!("{} {:?}", key, current);

        while key != "location" {
            let map = self
                .maps
                .get(key)
                .ok_or_else(|| anyhow!("Cannot get '{}' map", key))?;

            let mut next = Vec::new();

            for value in current {
                next.extend(map.transform_range(value));
            }

            key = &map.destination_type;
            current = next;

            // println!("{} {:?}", key, current);
        }

        Ok(current)
    }
}

#[derive(Debug, Clone)]
struct Map {
    destination_type: String,
    entries: Vec<MapEntry>,
}

impl Map {
    fn transform(&self, source_value: usize) -> usize {
        self.entries
            .iter()
            .filter(|e| {
                e.source_start <= source_value && (e.source_start + e.length) > source_value
            })
            .last()
            .map(|e| e.transform(source_value))
            .unwrap_or(source_value)
    }

    fn transform_range(&self, (source_start, source_end): (usize, usize)) -> Vec<(usize, usize)> {
        // 01234567890123456789
        //amen und Herren,
        //     ssssssssss
        // xx aaa bb  cccccc
        //     AABCCDDEEE

        let mut out = Vec::new();

        // ii is a special index!
        // even numbers correspond to the "empty" space before entry ii/2
        // odd numbers correspond to the entry (ii-1)/2
        for ii in 0..=(self.entries.len() * 2) {
            if ii % 2 == 0 {
                // we are in an empty space
                let (start, end) = if ii == 0 {
                    // first empty space
                    if self.entries[0].source_start == 0 {
                        continue;
                    }
                    (0, self.entries[0].source_start - 1)
                } else if ii >= self.entries.len() * 2 {
                    // last empty space after all entries
                    let prev = &self.entries[ii / 2 - 1];
                    (prev.source_start + prev.length, usize::MAX)
                } else {
                    // empty space between entries
                    let prev = &self.entries[ii / 2 - 1];
                    let next = &self.entries[ii / 2];
                    (prev.source_start + prev.length, next.source_start - 1)
                };

                let overlap_start = usize::max(start, source_start);
                let overlap_end = usize::min(end, source_end);

                if overlap_end >= overlap_start {
                    out.push((overlap_start, overlap_end));
                }
            } else {
                // we are in an entry
                let entry = &self.entries[(ii - 1) / 2];

                let overlap_start = usize::max(entry.source_start, source_start);
                let overlap_end = usize::min(entry.source_start + entry.length - 1, source_end);

                if overlap_end >= overlap_start {
                    out.push((
                        overlap_start - entry.source_start + entry.destination_start,
                        overlap_end - entry.source_start + entry.destination_start,
                    ));
                }
            }
        }

        let out_len: usize = out.iter().map(|(s, e)| e - s + 1).sum();

        assert_eq!(out_len, source_end - source_start + 1);

        out
    }
}

#[derive(Debug, Clone)]
struct MapEntry {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

impl MapEntry {
    fn transform(&self, source_value: usize) -> usize {
        source_value - self.source_start + self.destination_start
    }
}

impl FromStr for MapEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let tokens: Vec<usize> = s
            .split_whitespace()
            .map(|t| t.parse().context("Parsing map entry"))
            .collect::<Result<Vec<_>>>()?;

        if tokens.len() != 3 {
            return Err(anyhow!(
                "Expected tokens in map entry line, got {}",
                tokens.len()
            ));
        }

        Ok(Self {
            destination_start: tokens[0],
            source_start: tokens[1],
            length: tokens[2],
        })
    }
}

fn main() -> Result<()> {
    let almanac = Almanac::parse(File::open("data/day05/input")?)?;

    let min_location = almanac
        .seeds
        .iter()
        .map(|s| almanac.transform_seed(*s))
        .collect::<Result<Vec<usize>>>()?
        .into_iter()
        .min()
        .expect("All seeds transformed");

    println!("Part 1: {}", min_location);

    let seed_ranges: Vec<(usize, usize)> = (0..(almanac.seeds.len() / 2))
        .map(|i| {
            let source_start = almanac.seeds[i * 2];
            let source_len = almanac.seeds[i * 2 + 1];

            (source_start, source_start + source_len - 1)
        })
        .collect();

    let locs = almanac.transform_seed_range(&seed_ranges)?;

    let min_loc = locs
        .iter()
        .map(|(a, _b)| a)
        .min()
        .expect("At least one range");
    println!("Part 2: {}", min_loc);

    Ok(())
}

#[cfg(test)]
mod test {

    use super::{Map, MapEntry};

    #[test]
    fn test_transform() {
        // 01234567890123456789
        //
        //        ssssssssss
        //    xx aaa bb  cccccc
        //        AABCCDDEEE

        let map = Map {
            destination_type: "asdf".to_string(),
            entries: vec![
                // x
                MapEntry {
                    destination_start: 103,
                    source_start: 3,
                    length: 2,
                },
                // a
                MapEntry {
                    destination_start: 106,
                    source_start: 6,
                    length: 3,
                },
                // b
                MapEntry {
                    destination_start: 110,
                    source_start: 10,
                    length: 2,
                },
                // c
                MapEntry {
                    destination_start: 114,
                    source_start: 14,
                    length: 6,
                },
            ],
        };

        assert_eq!(
            map.transform_range((7, 16)),
            vec![(107, 108), (9, 9), (110, 111), (12, 13), (114, 116)]
        );

        assert_eq!(map.transform_range((30, 40)), vec![(30, 40)]);

        assert_eq!(map.transform_range((0, 2)), vec![(0, 2)]);
    }
}
