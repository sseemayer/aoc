use std::str::FromStr;

use anyhow::{Context, Error, Result};
use colored::Colorize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(',');
        let x = parts
            .next()
            .context("Missing x coordinate")?
            .parse::<i32>()
            .context("Invalid x coordinate")?;
        let y = parts
            .next()
            .context("Missing y coordinate")?
            .parse::<i32>()
            .context("Invalid y coordinate")?;
        Ok(Point { x, y })
    }
}

/// Polygon represented by a list of points. All points are in order and all segments are
/// axis-aligned.
struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    /// Check if the line segment p1..p2 intersects with any edge of the polygon
    ///
    /// Assumes all edges and the line are axis-aligned.
    /// Colinear overlapping lines are not considered intersecting.
    fn line_intersect_polygon(&self, p1: &Point, p2: &Point) -> bool {
        let n = self.points.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let pi = &self.points[i];
            let pj = &self.points[j];

            if pi.x == pj.x && p1.x != p2.x {
                // horizontal line vs vertical edge
                //
                //      i
                //      |
                //  1...+..>2
                //      |
                //      j

                let (xmin, xmax) = if p1.x < p2.x {
                    (p1.x, p2.x)
                } else {
                    (p2.x, p1.x)
                };
                let (ymin, ymax) = if pi.y < pj.y {
                    (pi.y, pj.y)
                } else {
                    (pj.y, pi.y)
                };

                if (xmin < pi.x && xmax > pi.x) && (ymin < p1.y && ymax > p1.y) {
                    return true;
                }
            } else if pi.y == pj.y && p1.y != p2.y {
                // vertical line vs horizontal edge

                let (xmin, xmax) = if pi.x < pj.x {
                    (pi.x, pj.x)
                } else {
                    (pj.x, pi.x)
                };

                let (ymin, ymax) = if p1.y < p2.y {
                    (p1.y, p2.y)
                } else {
                    (p2.y, p1.y)
                };

                if (ymin < pi.y && ymax > pi.y) && (xmin < p1.x && xmax > p1.x) {
                    return true;
                }
            }
        }
        false
    }
}

fn main() -> Result<()> {
    //let points: Vec<Point> = aoc::io::read_lines("data/day09/example")?;
    let points: Vec<Point> = aoc::io::read_lines((2025, 9))?;

    let polygon = Polygon {
        points: points.clone(),
    };

    let mut max_area = 0;
    let mut max_area_limited = 0;
    for i in 0..points.len() {
        'tile: for j in 0..points.len() {
            let (xmin, xmax) = if points[i].x < points[j].x {
                (points[i].x, points[j].x)
            } else {
                (points[j].x, points[i].x)
            };
            let (ymin, ymax) = if points[i].y < points[j].y {
                (points[i].y, points[j].y)
            } else {
                (points[j].y, points[i].y)
            };

            let area = (xmax - xmin + 1) as usize * (ymax - ymin + 1) as usize;

            if area > max_area {
                max_area = area;
            }
            if area > max_area_limited {
                // the current rectangle could be bigger, but check validity

                // check if any line is inside of the rectangle
                for p in &polygon.points {
                    if p.x > xmin && p.x < xmax && p.y > ymin && p.y < ymax {
                        //println!("  rejected by inside point {:?}", p);
                        continue 'tile;
                    }
                }

                // check all four edges of the rectangle for intersection with polygon
                let p1 = Point { x: xmin, y: ymin };
                let p2 = Point { x: xmax, y: ymin };
                let p3 = Point { x: xmax, y: ymax };
                let p4 = Point { x: xmin, y: ymax };

                if polygon.line_intersect_polygon(&p1, &p2)
                    || polygon.line_intersect_polygon(&p2, &p3)
                    || polygon.line_intersect_polygon(&p3, &p4)
                    || polygon.line_intersect_polygon(&p4, &p1)
                {
                    //println!("  rejected by edge intersection");
                    continue 'tile;
                }

                max_area_limited = area;
            }
        }
    }

    println!("{} {}", "Part 1:".bold().green(), max_area);
    println!("{} {}", "Part 2:".bold().green(), max_area_limited);

    Ok(())
}
