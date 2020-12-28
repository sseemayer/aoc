use aoc2019::result::Result;
use aoc2019::util::read_to_parsed_lines;
use failure::{format_err, Error};

#[derive(Debug)]
enum Instruction {
    DealIntoNewStack,
    Cut { n: i128 },
    DealWithIncrement { n: i128 },
}

impl std::str::FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let t: Vec<_> = s.split(" ").collect();

        let instr = match (t[0], t[1]) {
            ("deal", "into") => Instruction::DealIntoNewStack,
            ("deal", "with") => Instruction::DealWithIncrement { n: t[3].parse()? },
            ("cut", n) => Instruction::Cut { n: n.parse()? },
            _ => return Err(format_err!("Unknown instruction: '{}'", s)),
        };

        Ok(instr)
    }
}

impl Instruction {
    fn into_transformation(&self, r: i128) -> Transformation {
        match self {
            // -x-1
            Instruction::DealIntoNewStack => Transformation { a: -1, b: -1, r },

            // x - n
            Instruction::Cut { n } => Transformation { a: 1, b: -*n, r },

            // a * x
            Instruction::DealWithIncrement { n } => Transformation { a: *n, b: 0, r },
        }
    }
    fn apply(&self, mut ge: Transformation) -> Transformation {
        ge *= &self.into_transformation(ge.r);

        ge
    }
}

/// a*x + b (mod r)
#[derive(Debug, Clone)]
struct Transformation {
    a: i128,
    b: i128,
    r: i128,
}

impl Transformation {
    fn new(n_cards: i128) -> Self {
        Transformation {
            a: 1,
            b: 0,
            r: n_cards,
        }
    }

    fn modulo(&self, v: i128) -> i128 {
        ((v % self.r as i128) + self.r) % self.r
    }

    fn transform(&self, card_pos: i128) -> i128 {
        self.modulo(self.a * card_pos + self.b)
    }

    fn repeat(&self, times: usize) -> Transformation {
        if times == 0 {
            Transformation {
                a: 1,
                b: 0,
                r: self.r,
            }
        } else if times == 1 {
            self.clone()
        } else {
            let odd = times % 2 == 1;
            let half = times / 2;

            let mut rep = self.repeat(half);
            let rep2 = rep.clone();
            rep *= &rep2;
            if odd {
                rep *= self;
            }
            rep
        }
    }
}

impl std::ops::MulAssign<&Transformation> for Transformation {
    fn mul_assign(&mut self, rhs: &Transformation) {
        // c*(ax + b) + d = acx + bc + d
        self.a = self.modulo(self.a * rhs.a);
        self.b = self.modulo(self.b * rhs.a + rhs.b);
    }
}

fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (gcd, x, y) = extended_gcd(b % a, a);
    (gcd, (y - (b / a) * x), x)
}

fn main() -> Result<()> {
    let instrs = read_to_parsed_lines("data/day22/input", &|l: &str| l.parse::<Instruction>())?;

    println!("PART ONE");
    let mut transform = Transformation::new(10007);
    for instr in &instrs {
        transform = instr.apply(transform);
    }
    //println!("Transform is {:?}", transform);
    println!("Card 2019 is at position {}", transform.transform(2019));

    println!("\nPART TWO");
    let mut transform = Transformation::new(119315717514047);
    for instr in &instrs {
        transform = instr.apply(transform);
    }

    // println!("One-time transform is {:?}", transform);

    let full_transform = transform.repeat(101741582076661usize);

    // println!("Full transform is {:?}", full_transform);

    let lhs = full_transform.modulo(2020 - full_transform.b);

    let (_, a_inv, _) = extended_gcd(full_transform.a, full_transform.r);
    let x = full_transform.modulo(lhs * a_inv);

    println!("The card at 2020 is {}", x);

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_mod() {
        let ge = Transformation::new(10);
        assert_eq!(ge.modulo(0), 0);
        assert_eq!(ge.modulo(5), 5);
        assert_eq!(ge.modulo(10), 0);
        assert_eq!(ge.modulo(11), 1);
        assert_eq!(ge.modulo(122), 2);

        assert_eq!(ge.modulo(-0), 0);
        assert_eq!(ge.modulo(-3), 7);
        assert_eq!(ge.modulo(-10), 0);
        assert_eq!(ge.modulo(-11), 9);
        assert_eq!(ge.modulo(-122), 8);
    }

    #[test]
    fn test_cut() {
        let cut3 = Instruction::Cut { n: 3 }.into_transformation(10);
        println!("Cut3 is {:?}", cut3);
        assert_eq!(cut3.transform(0), 7);
        assert_eq!(cut3.transform(1), 8);
        assert_eq!(cut3.transform(2), 9);
        assert_eq!(cut3.transform(3), 0);
        assert_eq!(cut3.transform(4), 1);
        assert_eq!(cut3.transform(5), 2);
        assert_eq!(cut3.transform(6), 3);
        assert_eq!(cut3.transform(7), 4);
        assert_eq!(cut3.transform(8), 5);
        assert_eq!(cut3.transform(9), 6);

        let cut_neg4 = Instruction::Cut { n: -4 }.into_transformation(10);
        println!("CutNeg4 is {:?}", cut_neg4);
        assert_eq!(cut_neg4.transform(0), 4);
        assert_eq!(cut_neg4.transform(1), 5);
        assert_eq!(cut_neg4.transform(2), 6);
        assert_eq!(cut_neg4.transform(3), 7);
        assert_eq!(cut_neg4.transform(4), 8);
        assert_eq!(cut_neg4.transform(5), 9);
        assert_eq!(cut_neg4.transform(6), 0);
        assert_eq!(cut_neg4.transform(7), 1);
        assert_eq!(cut_neg4.transform(8), 2);
        assert_eq!(cut_neg4.transform(9), 3);
    }

    #[test]
    fn test_deal_new_stack() {
        let dins = Instruction::DealIntoNewStack.into_transformation(10);
        println!("DNS is {:?}", dins);
        // 0123456789
        //  V V V V V
        // 9876543210
        assert_eq!(dins.transform(0), 9);
        assert_eq!(dins.transform(1), 8);
        assert_eq!(dins.transform(2), 7);
        assert_eq!(dins.transform(3), 6);
        assert_eq!(dins.transform(4), 5);
        assert_eq!(dins.transform(5), 4);
        assert_eq!(dins.transform(6), 3);
        assert_eq!(dins.transform(7), 2);
        assert_eq!(dins.transform(8), 1);
        assert_eq!(dins.transform(9), 0);
    }

    #[test]
    fn test_deal_with_increment() {
        let dwi3 = Instruction::DealWithIncrement { n: 3 }.into_transformation(10);
        assert_eq!(dwi3.transform(0), 0); // 0
        assert_eq!(dwi3.transform(1), 3); // 7
        assert_eq!(dwi3.transform(2), 6); // 4
        assert_eq!(dwi3.transform(3), 9); // 1
        assert_eq!(dwi3.transform(4), 2); // 8
        assert_eq!(dwi3.transform(5), 5); // 5
        assert_eq!(dwi3.transform(6), 8); // 2
        assert_eq!(dwi3.transform(7), 1); // 9
        assert_eq!(dwi3.transform(8), 4); // 6
        assert_eq!(dwi3.transform(9), 7); // 3
    }

    #[test]
    fn test_combine1() {
        /*
        deal with increment 7
        deal into new stack
        deal into new stack
        Result: 0 3 6 9 2 5 8 1 4 7
        */
        let instrs = [
            Instruction::DealWithIncrement { n: 7 },
            Instruction::DealIntoNewStack,
            Instruction::DealIntoNewStack,
        ];

        let mut ge = Transformation::new(10);
        for instr in instrs.iter() {
            ge = instr.apply(ge);
        }

        println!("Combined transform is {:?}", ge);

        assert_eq!(ge.transform(0), 0);
        assert_eq!(ge.transform(3), 1);
        assert_eq!(ge.transform(6), 2);
        assert_eq!(ge.transform(9), 3);
        assert_eq!(ge.transform(2), 4);
        assert_eq!(ge.transform(5), 5);
        assert_eq!(ge.transform(8), 6);
        assert_eq!(ge.transform(1), 7);
        assert_eq!(ge.transform(4), 8);
        assert_eq!(ge.transform(7), 9);
    }

    #[test]
    fn test_combine2() {
        /*
        cut 6
        deal with increment 7
        deal into new stack
        Result: 3 0 7 4 1 8 5 2 9 6
        */
        let instrs = [
            Instruction::Cut { n: 6 },
            Instruction::DealWithIncrement { n: 7 },
            Instruction::DealIntoNewStack,
        ];

        let mut ge = Transformation::new(10);
        for instr in instrs.iter() {
            ge = instr.apply(ge);
        }

        println!("Combined transform is {:?}", ge);

        assert_eq!(ge.transform(3), 0);
        assert_eq!(ge.transform(0), 1);
        assert_eq!(ge.transform(7), 2);
        assert_eq!(ge.transform(4), 3);
        assert_eq!(ge.transform(1), 4);
        assert_eq!(ge.transform(8), 5);
        assert_eq!(ge.transform(5), 6);
        assert_eq!(ge.transform(2), 7);
        assert_eq!(ge.transform(9), 8);
        assert_eq!(ge.transform(6), 9);
    }

    #[test]
    fn test_combine3() {
        /*
        deal with increment 7
        deal with increment 9
        cut -2
        Result: 6 3 0 7 4 1 8 5 2 9
        */
        let instrs = [
            Instruction::DealWithIncrement { n: 7 },
            Instruction::DealWithIncrement { n: 9 },
            Instruction::Cut { n: -2 },
        ];

        let mut ge = Transformation::new(10);
        for instr in instrs.iter() {
            ge = instr.apply(ge);
        }

        println!("Combined transform is {:?}", ge);

        assert_eq!(ge.transform(6), 0);
        assert_eq!(ge.transform(3), 1);
        assert_eq!(ge.transform(0), 2);
        assert_eq!(ge.transform(7), 3);
        assert_eq!(ge.transform(4), 4);
        assert_eq!(ge.transform(1), 5);
        assert_eq!(ge.transform(8), 6);
        assert_eq!(ge.transform(5), 7);
        assert_eq!(ge.transform(2), 8);
        assert_eq!(ge.transform(9), 9);
    }

    #[test]
    fn test_combine4() {
        /*
        deal into new stack
        cut -2
        deal with increment 7
        cut 8
        cut -4
        deal with increment 7
        cut 3
        deal with increment 9
        deal with increment 3
        cut -1
        Result: 9 2 5 8 1 4 7 0 3 6
        */
        let instrs = [
            Instruction::DealIntoNewStack,
            Instruction::Cut { n: -2 },
            Instruction::DealWithIncrement { n: 7 },
            Instruction::Cut { n: 8 },
            Instruction::Cut { n: -4 },
            Instruction::DealWithIncrement { n: 7 },
            Instruction::Cut { n: 3 },
            Instruction::DealWithIncrement { n: 9 },
            Instruction::DealWithIncrement { n: 3 },
            Instruction::Cut { n: -1 },
        ];

        let mut ge = Transformation::new(10);
        for instr in instrs.iter() {
            ge = instr.apply(ge);
        }

        println!("Combined transform is {:?}", ge);

        assert_eq!(ge.transform(9), 0);
        assert_eq!(ge.transform(2), 1);
        assert_eq!(ge.transform(5), 2);
        assert_eq!(ge.transform(8), 3);
        assert_eq!(ge.transform(1), 4);
        assert_eq!(ge.transform(4), 5);
        assert_eq!(ge.transform(7), 6);
        assert_eq!(ge.transform(0), 7);
        assert_eq!(ge.transform(3), 8);
        assert_eq!(ge.transform(6), 9);
    }
}
