use std::collections::HashSet;

use anyhow::Result;

use aoc2017::knothash::KnotHash;

type Map1 = aoc::map::Map<[i16; 2], Tile>;
type Map2 = aoc::map::Map<[i16; 2], Region>;

#[derive(Clone, PartialEq, Eq)]
struct Tile(bool);

#[derive(Clone, PartialEq, Eq)]
struct Region(usize);

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.0 { "█" } else { "░" })
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.0 % 16)
    }
}

fn load_map(hash_seed: &str) -> Map1 {
    let mut map = Map1::new();
    for i in 0..128 {
        let hash = KnotHash::from_str(&format!("{}-{}", hash_seed, i));
        let hex: Vec<char> = hash.hash().chars().collect();

        for j in 0..hex.len() * 4 {
            let d = (hex[j / 4].to_digit(16).expect("hex digit") >> (3 - j % 4)) % 2 == 1;
            map.set([i as i16, j as i16], Tile(d));
        }
    }

    map
}

fn find_regions(map: &Map1) -> (Map2, HashSet<usize>) {
    let ([imin, jmin], [imax, jmax]) = map.get_extent();
    let mut out = Map2::new();

    let mut seen_regions: HashSet<usize> = HashSet::new();

    for i in imin..=imax {
        for j in jmin..=jmax {
            if map.get(&[i, j]) != Some(&Tile(true)) {
                continue;
            }

            let left = out.get(&[i, j - 1]).cloned();
            let top = out.get(&[i - 1, j]).cloned();

            match (left, top) {
                (None, None) => {
                    // nothing to merge with -- start new region

                    let mut new_region = 0;
                    while seen_regions.contains(&new_region) {
                        new_region += 1;
                    }
                    seen_regions.insert(new_region);

                    out.set([i, j], Region(new_region));
                }
                (None, Some(r)) | (Some(r), None) => {
                    // Merge with single adjacent region
                    out.set([i, j], r);
                }
                (Some(r), Some(s)) if r == s => {
                    // Merge with identical adjacent regions
                    out.set([i, j], r);
                }
                (Some(r), Some(s)) => {
                    // Different adjacent regions

                    // replace all occurrences of r with s
                    for (_coord, region) in out.data.iter_mut() {
                        if region == &r {
                            *region = s.clone();
                        }
                    }

                    seen_regions.remove(&r.0);

                    out.set([i, j], s);
                }
            }
        }
    }

    (out, seen_regions)
}

fn main() -> Result<()> {
    let input = "hxtvlmkl";
    // let input = "flqrgnkx";

    let map = load_map(input);

    let part1 = map.find_all(&Tile(true)).len();

    println!("Part 1: {}", part1);

    let (map2, seen2) = find_regions(&map);

    println!("{}\n\nPart 2: {}", map2, seen2.len());

    Ok(())
}
