use aoc2015::util::read_to_parsed_lines;
use aoc2015::{format_err, Error, Result};

#[derive(Debug)]
struct Box {
    l: i32,
    w: i32,
    h: i32,
}

impl Box {
    fn surface_area(&self) -> i32 {
        let l = self.l;
        let w = self.w;
        let h = self.h;
        2 * l * w + 2 * w * h + 2 * l * h
    }

    fn slack(&self) -> i32 {
        let l = self.l;
        let w = self.w;

        l * w
    }

    fn wrapping(&self) -> i32 {
        self.surface_area() + self.slack()
    }

    fn ribbon_around(&self) -> i32 {
        let l = self.l;
        let w = self.w;
        2 * l + 2 * w
    }

    fn ribbon_bow(&self) -> i32 {
        let l = self.l;
        let w = self.w;
        let h = self.h;
        l * w * h
    }

    fn ribbon(&self) -> i32 {
        self.ribbon_around() + self.ribbon_bow()
    }
}

impl std::str::FromStr for Box {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut tokens: Vec<i32> = s
            .split("x")
            .map(|t| t.parse().map_err(|e: std::num::ParseIntError| e.into()))
            .collect::<Result<Vec<i32>>>()?;

        tokens.sort();

        if tokens.len() != 3 {
            return Err(format_err!("Invalid number of tokens: '{}'", s));
        }

        let l = tokens[0];
        let w = tokens[1];
        let h = tokens[2];

        Ok(Box { l, w, h })
    }
}

fn main() -> Result<()> {
    let data: Vec<Box> = read_to_parsed_lines("data/day02/input", &|l: &str| l.parse())?;

    println!("Boxes: {:#?}", data);

    let paper_needed: i32 = data.iter().map(|b| b.wrapping()).sum();
    let ribbon_needed: i32 = data.iter().map(|b| b.ribbon()).sum();

    println!("Paper needed: {} sq ft", paper_needed);
    println!("Ribbon needed: {} ft", ribbon_needed);

    Ok(())
}
