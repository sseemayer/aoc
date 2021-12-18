use anyhow::Result;
use aoc2021::io::read_lines;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Error, Debug)]
enum ParseError {
    #[error("Bad line: '{}'", .0)]
    BadLine(String),

    #[error(transparent)]
    Parse(#[from] pest::error::Error<Rule>),

    #[error(transparent)]
    Int(#[from] std::num::ParseIntError),
}

#[derive(Parser)]
#[grammar = "bin/day18.pest"]
struct SnailParser;

#[derive(PartialEq, Eq, Clone)]
enum Number {
    Literal { n: i64 },
    Pair { a: Box<Number>, b: Box<Number> },
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Literal { n } => write!(f, "{}", n),
            Number::Pair { a, b } => write!(f, "[{:?}, {:?}]", a, b),
        }
    }
}

impl std::str::FromStr for Number {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use pest::{iterators::Pair, Parser};

        fn parse(pair: Pair<Rule>) -> std::result::Result<Number, ParseError> {
            match pair.as_rule() {
                Rule::bracketed => {
                    let mut pairs: Vec<Number> =
                        pair.into_inner()
                            .into_iter()
                            .map(parse)
                            .collect::<std::result::Result<Vec<Number>, ParseError>>()?;

                    assert_eq!(pairs.len(), 2);

                    let b = Box::new(pairs.pop().unwrap());
                    let a = Box::new(pairs.pop().unwrap());
                    Ok(Number::Pair { a, b })
                }
                Rule::literal => {
                    let n: i64 = pair.as_str().parse()?;
                    Ok(Number::Literal { n })
                }
                Rule::number | Rule::WHITESPACE => unreachable!(),
            }
        }

        parse(
            SnailParser::parse(Rule::number, s)?
                .next()
                .ok_or(ParseError::BadLine(s.to_string()))?,
        )
    }
}

impl Number {
    fn reduce(self) -> Number {
        let mut num = self;
        loop {
            let (c, n) = num.explode();
            num = n;
            if c {
                continue;
            }

            let (c, n) = num.split();
            num = n;
            if c {
                continue;
            }

            return num;
        }
    }

    fn as_literal(&self) -> Option<i64> {
        match self {
            Number::Literal { n } => Some(*n),
            Number::Pair { .. } => None,
        }
    }

    fn leftmost_literal_mut(&mut self) -> Option<&mut i64> {
        match self {
            Number::Literal { ref mut n } => Some(n),
            Number::Pair { a, .. } => a.leftmost_literal_mut(),
        }
    }

    fn rightmost_literal_mut(&mut self) -> Option<&mut i64> {
        match self {
            Number::Literal { ref mut n } => Some(n),
            Number::Pair { b, .. } => b.rightmost_literal_mut(),
        }
    }

    fn explode(self) -> (bool, Number) {
        // [[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]] becomes
        //         ^  xxx      ^
        //
        // [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]
        //

        fn absorb_left(a: &mut Number, to_left: Option<i64>) -> Option<i64> {
            if let (Some(tl), Some(dest)) = (to_left, a.rightmost_literal_mut()) {
                *dest += tl;
                None
            } else {
                to_left
            }
        }

        fn absorb_right(b: &mut Number, to_right: Option<i64>) -> Option<i64> {
            if let (Some(tr), Some(dest)) = (to_right, b.leftmost_literal_mut()) {
                *dest += tr;
                None
            } else {
                to_right
            }
        }

        fn inner(num: Number, depth: usize) -> (Number, bool, Option<i64>, Option<i64>) {
            match num {
                Number::Literal { .. } => (num, false, None, None),
                Number::Pair { a, mut b } => {
                    if depth >= 4 {
                        // the current pair explodes!
                        let to_left = Some(a.as_literal().expect("explode literal"));
                        let to_right = Some(b.as_literal().expect("explode literal"));

                        // the current number explodes
                        (Number::Literal { n: 0 }, true, to_left, to_right)
                    } else {
                        // we are still good - see if a explodes
                        let (a, ae, to_left, mut to_right) = inner(*a, depth + 1);
                        let mut a = Box::new(a);

                        if ae {
                            // a exploded - see if we can absorb on this level
                            to_right = absorb_right(&mut b, to_right);

                            return (Number::Pair { a, b }, ae, to_left, to_right);
                        }

                        // a did not explode - see if b explodes
                        let (b, be, mut to_left, to_right) = inner(*b, depth + 1);
                        let b = Box::new(b);

                        if be {
                            // b exploded - see if we can absorb on this level
                            to_left = absorb_left(&mut a, to_left);

                            return (Number::Pair { a, b }, be, to_left, to_right);
                        }

                        // no children exploded
                        (Number::Pair { a, b }, false, None, None)
                    }
                }
            }
        }

        let (num, exploded, _to_left, _to_right) = inner(self, 0);

        (exploded, num)
    }

    fn split(self) -> (bool, Number) {
        match self {
            Number::Literal { n } => {
                if n >= 10 {
                    let a = n / 2;
                    let b = n - a;

                    let a = Box::new(Number::Literal { n: a });
                    let b = Box::new(Number::Literal { n: b });

                    (true, Number::Pair { a, b })
                } else {
                    (false, self)
                }
            }
            Number::Pair { a, b } => match a.split() {
                (true, a) => (true, Number::Pair { a: Box::new(a), b }),
                (false, a) => match b.split() {
                    (true, b) => (
                        true,
                        Number::Pair {
                            a: Box::new(a),
                            b: Box::new(b),
                        },
                    ),
                    (false, b) => (
                        false,
                        Number::Pair {
                            a: Box::new(a),
                            b: Box::new(b),
                        },
                    ),
                },
            },
        }
    }

    fn magnitude(&self) -> i64 {
        match self {
            Number::Literal { n } => *n,
            Number::Pair { a, b } => a.magnitude() * 3 + b.magnitude() * 2,
        }
    }

    fn accumulate(nums: &[Number]) -> Number {
        let mut accum = nums[0].clone();
        for l in &nums[1..] {
            accum = accum + l.clone();
        }
        accum
    }
}

impl std::ops::Add for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Number {
        Number::Pair {
            a: Box::new(self),
            b: Box::new(rhs),
        }
        .reduce()
    }
}

fn main() -> Result<()> {
    let lines: Vec<Number> = read_lines("data/day18/input")?;

    let accum = Number::accumulate(&lines[..]);
    println!("Part 1: {}", accum.magnitude());

    let mut max_mag = 0;
    for i in &lines {
        for j in &lines {
            if i != j {
                let mag = (i.clone() + j.clone()).magnitude();
                max_mag = max_mag.max(mag);
            }
        }
    }
    println!("Part 2: {}", max_mag);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_explode() -> Result<()> {
        assert_eq!(
            "[[[[[9,8],1],2],3],4]".parse::<Number>()?.explode().1,
            "[[[[0,9],2],3],4]".parse()?
        );

        assert_eq!(
            "[7,[6,[5,[4,[3,2]]]]]".parse::<Number>()?.explode().1,
            "[7,[6,[5,[7,0]]]]".parse()?
        );

        assert_eq!(
            "[[6,[5,[4,[3,2]]]],1]".parse::<Number>()?.explode().1,
            "[[6,[5,[7,0]]],3]".parse()?
        );

        assert_eq!(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
                .parse::<Number>()?
                .explode()
                .1,
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse()?
        );

        assert_eq!(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
                .parse::<Number>()?
                .explode()
                .1,
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse()?
        );
        Ok(())
    }

    #[test]
    fn test_split() -> Result<()> {
        assert_eq!(
            "[[[[0,7],4],[15,[0,13]]],[1,1]]"
                .parse::<Number>()?
                .split()
                .1,
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".parse()?
        );

        assert_eq!(
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"
                .parse::<Number>()?
                .split()
                .1,
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]".parse()?
        );

        Ok(())
    }

    #[test]
    fn test_magnitude() -> Result<()> {
        assert_eq!("[[1,2],[[3,4],5]]".parse::<Number>()?.magnitude(), 143);
        assert_eq!(
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
                .parse::<Number>()?
                .magnitude(),
            1384
        );
        assert_eq!(
            "[[[[1,1],[2,2]],[3,3]],[4,4]]"
                .parse::<Number>()?
                .magnitude(),
            445
        );
        assert_eq!(
            "[[[[3,0],[5,3]],[4,4]],[5,5]]"
                .parse::<Number>()?
                .magnitude(),
            791
        );
        assert_eq!(
            "[[[[5,0],[7,4]],[5,5]],[6,6]]"
                .parse::<Number>()?
                .magnitude(),
            1137
        );
        assert_eq!(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .parse::<Number>()?
                .magnitude(),
            3488
        );

        Ok(())
    }

    #[test]
    fn test_add() -> Result<()> {
        let a: Number = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse()?;
        let b: Number = "[1,1]".parse()?;

        assert_eq!(a + b, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse()?);

        Ok(())
    }

    #[test]
    fn test_add_hard() -> Result<()> {
        let a: Number = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".parse()?;
        let b: Number = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".parse()?;
        assert_eq!(
            a + b,
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]".parse()?
        );

        Ok(())
    }

    #[test]
    fn test_sums_easy() -> Result<()> {
        let nums = vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]"]
            .into_iter()
            .map(|n| Ok(n.parse()?))
            .collect::<Result<Vec<Number>>>()?;

        assert_eq!(
            Number::accumulate(&nums[..]),
            "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse()?
        );

        let nums = vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5, 5]"]
            .into_iter()
            .map(|n| Ok(n.parse()?))
            .collect::<Result<Vec<Number>>>()?;

        assert_eq!(
            Number::accumulate(&nums[..]),
            "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse()?
        );

        let nums = vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"]
            .into_iter()
            .map(|n| Ok(n.parse()?))
            .collect::<Result<Vec<Number>>>()?;

        assert_eq!(
            Number::accumulate(&nums[..]),
            "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse()?
        );

        Ok(())
    }

    #[test]
    fn test_sums_hard() -> Result<()> {
        let nums = vec![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ]
        .into_iter()
        .map(|n| Ok(n.parse()?))
        .collect::<Result<Vec<Number>>>()?;

        assert_eq!(
            Number::accumulate(&nums[..]),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse()?
        );
        Ok(())
    }
}
