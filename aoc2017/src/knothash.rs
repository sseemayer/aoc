use colored::Colorize;

#[derive(Debug)]
pub struct KnotHash {
    pub numbers: Vec<u8>,

    pub current: usize,
    pub skip_size: usize,
}

impl std::fmt::Display for KnotHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (i, n) in self.numbers.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            self.write_number(*n, f)?;
        }

        write!(f, "] skip_size={}", self.skip_size)?;
        Ok(())
    }
}

impl KnotHash {
    pub fn new(n_numbers: usize) -> Self {
        let numbers = (0..n_numbers).map(|n| n as u8).collect();

        KnotHash {
            numbers,

            skip_size: 0,
            current: 0,
        }
    }

    pub fn from_str(s: &str) -> Self {
        let mut steps = s.as_bytes().to_vec();
        steps.extend(vec![17, 31, 73, 47, 23]);

        let mut hash = KnotHash::new(256);

        for _round in 0..64 {
            for l in &steps {
                hash.step(*l as usize);
            }
        }

        hash
    }

    pub fn step(&mut self, length: usize) {
        let n = self.numbers.len();

        for i in 0..(length / 2) {
            let a = (self.current + i) % n;
            let b = (self.current + length - i - 1) % n;
            // println!("swap {} {}", a, b);
            self.numbers.swap(a, b);
        }

        self.current = (self.current + length + self.skip_size) % n;

        // increase skip size
        self.skip_size += 1;
    }

    fn write_number(&self, n: u8, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.numbers[self.current] == n {
            write!(f, "{}", format!("{}", n).green())
        } else {
            write!(f, "{}", n)
        }
    }

    pub fn hash(&self) -> String {
        self.numbers
            .chunks(16)
            .map(|c| c.iter().fold(0, |a, b| a ^ *b))
            .map(|c| format!("{:02x}", c))
            .collect()
    }
}
