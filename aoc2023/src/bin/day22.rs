use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    str::FromStr,
};

use anyhow::{anyhow, bail, Context, Error, Result};
use image::{ImageBuffer, Rgb};
use lazy_static::lazy_static;

#[derive(Clone)]
struct Brick {
    min: [i32; 3],
    max: [i32; 3],
}

impl Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "C[{}..{}, {}..{}, {}..{}]",
            self.min[0], self.max[0], self.min[1], self.max[1], self.min[2], self.max[2]
        )
    }
}

impl FromStr for Brick {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (min, max) = s
            .split_once("~")
            .ok_or_else(|| anyhow!("Bad cube: '{}'", s))?;

        let min: Vec<i32> = min
            .split(",")
            .map(|n| n.parse().context("Parse min"))
            .collect::<Result<Vec<_>>>()?;

        let max: Vec<i32> = max
            .split(",")
            .map(|n| n.parse().context("Parse max"))
            .collect::<Result<Vec<_>>>()?;

        if min.len() != 3 {
            bail!("Bad number of dimensions for min");
        }

        if max.len() != 3 {
            bail!("Bad number of dimensions for max");
        }

        let min = [min[0], min[1], min[2]];
        let max = [max[0], max[1], max[2]];

        Ok(Self { min, max })
    }
}

impl Brick {
    /// Check if this brick could land on another brick. If it could, provide the minimum z value.
    fn can_land_on(&self, other: &Brick) -> Option<i32> {
        let [xa0, ya0, za0] = self.min;
        let [xb0, yb0, _zb0] = other.min;
        let [xa1, ya1, _za1] = self.max;
        let [xb1, yb1, zb1] = other.max;

        // cannot land if we are already below the other brick
        if za0 <= zb1 {
            return None;
        }

        // perform intersection in the xy plane
        //
        //   AAAAAAA
        //        BBBBB
        let x0 = i32::max(xa0, xb0);
        let x1 = i32::min(xa1, xb1);
        let y0 = i32::max(ya0, yb0);
        let y1 = i32::min(ya1, yb1);

        if x1 >= x0 && y1 >= y0 {
            Some(zb1 + 1)
        } else {
            None
        }
    }

    fn move_to_z(&mut self, z: i32) {
        let height = self.max[2] - self.min[2];
        self.min[2] = z;
        self.max[2] = z + height;
    }
}

fn settle(
    bricks: &mut Vec<Brick>,
) -> (
    HashMap<usize, HashSet<usize>>,
    HashMap<usize, HashSet<usize>>,
) {
    // brick index -> indices of supports for this brick
    let mut below: HashMap<usize, HashSet<usize>> = HashMap::new();

    loop {
        let mut finished = true;

        // sort by max z so that all possible fall targets have lower indices than the
        // currently-falling brick
        bricks.sort_by_key(|c| c.max[2]);

        // for each mobile brick i
        for i in 0..bricks.len() {
            // consider a brick j to fall on, looking for the highest landing point
            let mut new_z = 1;
            let mut supports = HashSet::new();
            for j in 0..i {
                if let Some(z) = bricks[i].can_land_on(&bricks[j]) {
                    if z > new_z {
                        new_z = z;
                        supports.clear();
                    }

                    supports.insert(j);
                }
            }

            // println!("{} lands on {:?} with z={}", i, supports, new_z);

            if bricks[i].min[2] > new_z {
                finished = false;
                bricks[i].move_to_z(new_z);
            }

            below.insert(i, supports);
        }

        if finished {
            break;
        }
    }

    // brick index -> indices of bricks supported by this brick
    let mut above: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (i, sups) in below.iter() {
        // ensure all bricks have a key in above
        above.entry(*i).or_default();
        for j in sups {
            above.entry(*j).or_default().insert(*i);
        }
    }

    (below, above)
}

fn part1(below: &HashMap<usize, HashSet<usize>>, above: &HashMap<usize, HashSet<usize>>) -> usize {
    // count how many bricks can be removed
    above
        .values()
        .filter(|bricks_above| {
            // a brick can be removed if all bricks above it have more than the one support
            bricks_above
                .iter()
                .all(|j| below.get(j).map(|v| v.len()).unwrap_or_default() > 1)
        })
        .count()
}

fn part2(below: &HashMap<usize, HashSet<usize>>, above: &HashMap<usize, HashSet<usize>>) -> usize {
    let mut total = 0;

    // for each disentegrating brick i
    for i in 0..below.len() {
        let mut queue: Vec<HashSet<usize>> = vec![vec![i].into_iter().collect()];
        let mut sum = 0;

        while let Some(falling) = queue.pop() {
            // find any blocks directly above currently falling blocks that
            // do not have any supports left
            let new_falling: HashSet<usize> = falling
                .iter()
                .flat_map(|i| &above[i])
                .filter(|i| (&below[i] - &falling).is_empty() & !falling.contains(i))
                .cloned()
                .collect();

            sum += new_falling.len();

            if !new_falling.is_empty() {
                queue.push(&new_falling | &falling);
            }
        }

        // println!("{} {}", i, sum);

        total += sum;
    }

    total
}

type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

lazy_static! {
    static ref BRICK_COLORS: Vec<Rgb<u8>> = vec![
        Rgb([0x1f, 0x77, 0xb4]),
        Rgb([0xff, 0x7f, 0x0e]),
        Rgb([0x2c, 0xa0, 0x2c]),
        Rgb([0xd6, 0x27, 0x28]),
        Rgb([0x94, 0x67, 0xbd]),
        Rgb([0x8c, 0x56, 0x4b]),
        Rgb([0xe3, 0x77, 0xc2]),
        Rgb([0x7f, 0x7f, 0x7f]),
        Rgb([0xbc, 0xbd, 0x22]),
        Rgb([0x17, 0xbe, 0xcf]),
    ];
}

fn draw(bricks: &Vec<Brick>) -> (Image, Image) {
    let mut min = [0, 0, 0];
    let mut max = [0, 0, 0];

    for brick in bricks {
        for i in 0..3 {
            min[i] = i32::min(min[i], brick.min[i]);
            max[i] = i32::max(max[i], brick.max[i]);
        }
    }

    let dx = (max[0] - min[0] + 1) as u32;
    let dy = (max[1] - min[1] + 1) as u32;
    let dz = (max[2] - min[2] + 1) as u32;

    let mut xz = Image::new(dx, dz);
    let mut yz = Image::new(dy, dz);
    xz.fill(255);
    yz.fill(255);

    for (i, brick) in bricks.iter().enumerate() {
        let color = BRICK_COLORS[i % BRICK_COLORS.len()];
        for bz in brick.min[2]..=brick.max[2] {
            for bx in brick.min[0]..=brick.max[0] {
                *xz.get_pixel_mut((bx - min[0]) as u32, (bz - min[2]) as u32) = color;
            }
            for by in brick.min[1]..=brick.max[1] {
                *yz.get_pixel_mut((by - min[1]) as u32, (bz - min[2]) as u32) = color;
            }
        }
    }

    (xz, yz)
}

fn main() -> Result<()> {
    let mut bricks: Vec<Brick> = aoc::io::read_lines("data/day22/input")?;

    let (xz, yz) = draw(&bricks);
    xz.save("data/day22/xz0.png")?;
    yz.save("data/day22/yz0.png")?;

    let (below, above) = settle(&mut bricks);

    //println!("i\tbrick\tbelow\tabove");
    //for i in 0..bricks.len() {
    //    println!("{}\t{:?}\t{:?}\t{:?}", i, &bricks[i], below[&i], above[&i]);
    //}

    let (xz, yz) = draw(&bricks);
    xz.save("data/day22/xz1.png")?;
    yz.save("data/day22/yz1.png")?;

    println!("Part 1: {}", part1(&below, &above));
    println!("Part 2: {}", part2(&below, &above));

    Ok(())
}
