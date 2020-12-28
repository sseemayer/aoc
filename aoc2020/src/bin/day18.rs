use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Number parsing error for '{}': {}", data, source))]
    ParseNumber {
        data: String,
        source: std::num::ParseIntError,
    },

    #[snafu(display("Term parsing error for '{}'", data))]
    ParseTerm { data: String },
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum TermPart2 {
    Const(i64),
    Sum(Box<TermPart2>, Box<TermPart2>),
    Product(Box<TermPart2>, Box<TermPart2>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TermPart1 {
    Const(i64),
    Sum(Box<TermPart1>, Box<TermPart1>),
    Product(Box<TermPart1>, Box<TermPart1>),
}

impl std::str::FromStr for TermPart2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        #[derive(Debug, PartialEq, Eq, Clone)]
        enum Token {
            None,
            Num(String),
            Op(String),
            ParenExpr(String),
            ParenSum(Box<Token>, Box<Token>),
        }

        let s = s.replace(" ", "");
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut current = Token::None;
        let mut tokens: Vec<Token> = Vec::new();
        while i < s.len() {
            let a = &chars[i];
            match (&mut current, a) {
                (Token::Num(ref mut n), '0'..='9') => n.push(*a),
                (_, '0'..='9') => {
                    if current != Token::None {
                        tokens.push(current.clone())
                    };
                    current = Token::Num(a.to_string())
                }
                (_, '+') => {
                    if current != Token::None {
                        tokens.push(current.clone())
                    };
                    current = Token::Op(a.to_string())
                }
                (_, '*') => {
                    if current != Token::None {
                        tokens.push(current.clone())
                    };
                    current = Token::Op(a.to_string())
                }
                (_, '(') => {
                    if current != Token::None {
                        tokens.push(current.clone())
                    };
                    let mut blevel = 1;
                    let mut cbracket = i + 1;
                    while blevel > 0 && cbracket < chars.len() {
                        let c = chars[cbracket];
                        if c == '(' {
                            blevel += 1;
                        } else if c == ')' {
                            blevel -= 1;
                        }
                        cbracket += 1;
                    }

                    current = Token::ParenExpr(s[i + 1..cbracket - 1].to_string());
                    i = cbracket;
                    continue;
                }
                _ => {
                    return Err(Error::ParseTerm {
                        data: s.to_string(),
                    });
                }
            }
            i += 1;
        }

        tokens.push(current);

        while tokens.len() > 3 {
            let mut plus_pos = None;
            for (i, t) in tokens.iter().enumerate() {
                if t == &Token::Op("+".to_string()) {
                    plus_pos = Some(i);
                }
            }

            if let Some(i) = plus_pos {
                let b = tokens.remove(i + 1);
                tokens.remove(i);
                let a = tokens.remove(i - 1);

                tokens.insert(i - 1, Token::ParenSum(Box::new(a), Box::new(b)));
            } else {
                break;
            }
        }

        fn token_to_term(t: Token) -> Result<TermPart2> {
            match t {
                Token::Num(n) => Ok(TermPart2::Const(n.parse::<i64>().context(ParseNumber {
                    data: n.to_string(),
                })?)),
                Token::ParenExpr(pe) => pe.parse(),
                Token::ParenSum(a, b) => Ok(TermPart2::Sum(
                    Box::new(token_to_term(*a)?),
                    Box::new(token_to_term(*b)?),
                )),
                Token::None => unreachable!(),
                Token::Op(_) => unreachable!(),
            }
        }

        let mut out = token_to_term(tokens.remove(0))?;

        while !tokens.is_empty() {
            let op = tokens.remove(0);
            let tkn = tokens.remove(0);
            let term = Box::new(token_to_term(tkn)?);

            let prev = Box::new(out);

            out = if op == Token::Op("+".to_string()) {
                TermPart2::Sum(prev, term)
            } else {
                TermPart2::Product(prev, term)
            };
        }

        Ok(out)
    }
}

impl std::str::FromStr for TermPart1 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        #[derive(Debug, PartialEq, Eq, Clone)]
        enum Token {
            None,
            Num(String),
            Op(String),
            ParenExpr(String),
        }

        let s = s.replace(" ", "");
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut current = Token::None;
        let mut tokens: Vec<Token> = Vec::new();
        while i < s.len() {
            let a = &chars[i];
            match (&mut current, a) {
                (Token::Num(ref mut n), '0'..='9') => n.push(*a),
                (_, '0'..='9') => {
                    if current != Token::None {
                        tokens.push(current.clone())
                    };
                    current = Token::Num(a.to_string())
                }
                (_, '+') => {
                    if current != Token::None {
                        tokens.push(current.clone())
                    };
                    current = Token::Op(a.to_string())
                }
                (_, '*') => {
                    if current != Token::None {
                        tokens.push(current.clone())
                    };
                    current = Token::Op(a.to_string())
                }
                (_, '(') => {
                    if current != Token::None {
                        tokens.push(current.clone())
                    };
                    let mut blevel = 1;
                    let mut cbracket = i + 1;
                    while blevel > 0 && cbracket < chars.len() {
                        let c = chars[cbracket];
                        if c == '(' {
                            blevel += 1;
                        } else if c == ')' {
                            blevel -= 1;
                        }
                        cbracket += 1;
                    }

                    current = Token::ParenExpr(s[i + 1..cbracket - 1].to_string());
                    i = cbracket;
                    continue;
                }
                _ => {
                    return Err(Error::ParseTerm {
                        data: s.to_string(),
                    });
                }
            }
            i += 1;
        }

        tokens.push(current);

        fn token_to_term(t: Token) -> Result<TermPart1> {
            match t {
                Token::Num(n) => Ok(TermPart1::Const(n.parse::<i64>().context(ParseNumber {
                    data: n.to_string(),
                })?)),
                Token::ParenExpr(pe) => pe.parse(),
                Token::None => unreachable!(),
                Token::Op(_) => unreachable!(),
            }
        }

        let mut out = token_to_term(tokens.remove(0))?;

        while !tokens.is_empty() {
            let op = tokens.remove(0);
            let tkn = tokens.remove(0);
            let term = Box::new(token_to_term(tkn)?);

            let prev = Box::new(out);

            out = if op == Token::Op("+".to_string()) {
                TermPart1::Sum(prev, term)
            } else {
                TermPart1::Product(prev, term)
            };
        }

        Ok(out)
    }
}

impl TermPart2 {
    fn get_value(&self) -> i64 {
        match self {
            TermPart2::Const(i) => *i,
            TermPart2::Sum(a, b) => a.get_value() + b.get_value(),
            TermPart2::Product(a, b) => a.get_value() * b.get_value(),
        }
    }
}

impl TermPart1 {
    fn get_value(&self) -> i64 {
        match self {
            TermPart1::Const(i) => *i,
            TermPart1::Sum(a, b) => a.get_value() + b.get_value(),
            TermPart1::Product(a, b) => a.get_value() * b.get_value(),
        }
    }
}

fn main() -> Result<()> {
    let terms1: Vec<TermPart1> = std::fs::read_to_string("data/day18/input")
        .context(Io)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_>>()?;

    let sum1 = terms1.iter().map(|t| t.get_value()).sum::<i64>();

    println!("Part 1: sum is {}", sum1);

    let terms2: Vec<TermPart2> = std::fs::read_to_string("data/day18/input")
        .context(Io)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_>>()?;

    let sum2 = terms2.iter().map(|t| t.get_value()).sum::<i64>();
    println!("Part 2: sum is {}", sum2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_part1() -> Result<()> {
        fn sum(a: TermPart1, b: TermPart1) -> TermPart1 {
            TermPart1::Sum(Box::new(a), Box::new(b))
        }

        fn product(a: TermPart1, b: TermPart1) -> TermPart1 {
            TermPart1::Product(Box::new(a), Box::new(b))
        }

        fn c(n: i64) -> TermPart1 {
            TermPart1::Const(n)
        }

        assert_eq!(
            "(2 * 3) + 4".parse::<TermPart1>()?,
            sum(product(c(2), c(3)), c(4))
        );

        println!("FOOO -----------------");

        assert_eq!(
            "2 * 3 + (4 * 5)".parse::<TermPart1>()?,
            sum(product(c(2), c(3)), product(c(4), c(5)))
        );
        println!("BARR -----------------");

        assert_eq!(
            "5 + (8 * 3 + 9 + 3 * 4 * 3)".parse::<TermPart1>()?,
            sum(
                c(5),
                product(
                    product(sum(sum(product(c(8), c(3)), c(9)), c(3)), c(4)),
                    c(3)
                )
            )
        );

        Ok(())
    }

    #[test]
    fn test_examples_part1() -> Result<()> {
        // 2 * 3 + (4 * 5) becomes 26.
        assert_eq!("2 * 3 + (4 * 5)".parse::<TermPart1>()?.get_value(), 26);
        // 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
        assert_eq!(
            "5 + (8 * 3 + 9 + 3 * 4 * 3)"
                .parse::<TermPart1>()?
                .get_value(),
            437
        );
        // 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
        assert_eq!(
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
                .parse::<TermPart1>()?
                .get_value(),
            12240
        );
        // ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
        assert_eq!(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
                .parse::<TermPart1>()?
                .get_value(),
            13632
        );

        Ok(())
    }

    #[test]
    fn test_examples_part2() -> Result<()> {
        // 1 + (2 * 3) + (4 * (5 + 6)) still becomes 51.
        assert_eq!(
            "1 + (2 * 3) + (4 * (5 + 6))"
                .parse::<TermPart2>()?
                .get_value(),
            51
        );

        // 2 * 3 + (4 * 5) becomes 46.
        assert_eq!("2 * 3 + (4 * 5)".parse::<TermPart2>()?.get_value(), 46);

        // 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 1445.
        assert_eq!(
            "5 + (8 * 3 + 9 + 3 * 4 * 3)"
                .parse::<TermPart2>()?
                .get_value(),
            1445
        );

        // 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 669060.
        assert_eq!(
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
                .parse::<TermPart2>()?
                .get_value(),
            669060
        );

        // ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 23340.
        assert_eq!(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
                .parse::<TermPart2>()?
                .get_value(),
            23340
        );

        Ok(())
    }
}
