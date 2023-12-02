use anyhow::Result;

#[derive(Debug)]
struct Address {
    supernet: String,
    hypernet: String,
}

impl std::str::FromStr for Address {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut supernet = String::new();
        let mut hypernet = String::new();

        let mut in_hypernet = false;

        for c in s.chars() {
            match (in_hypernet, c) {
                (_, '[') => {
                    in_hypernet = true;
                    hypernet.push('|') // delimiter to prevent cross-block patterns
                }
                (_, ']') => {
                    in_hypernet = false;
                    supernet.push('|') // delimiter to prevent cross-block patterns
                }
                (false, a) => supernet.push(a),
                (true, a) => hypernet.push(a),
            }
        }

        Ok(Address { supernet, hypernet })
    }
}

fn has_abba(s: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    for i in 0..(s.len() - 3) {
        let a = s[i];
        let b = s[i + 1];

        if a == b {
            continue;
        }
        if s[i + 2] != b {
            continue;
        }
        if s[i + 3] != a {
            continue;
        }
        return true;
    }
    false
}

fn find_aba(s: &str) -> Vec<(char, char)> {
    let s: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    for i in 0..(s.len() - 2) {
        let a = s[i];
        let b = s[i + 1];

        if a == b {
            continue;
        }
        if s[i + 2] != a {
            continue;
        }

        out.push((a, b));
    }

    out
}

impl Address {
    fn supports_tls(&self) -> bool {
        has_abba(&self.supernet) && !has_abba(&self.hypernet)
    }

    fn supports_ssl(&self) -> bool {
        let h: Vec<char> = self.hypernet.chars().collect();

        let abas = find_aba(&self.supernet);

        // for all found aba's, check if we can find a corresponding bab
        for (a, b) in abas {
            for i in 0..(h.len() - 2) {
                if h[i] != b {
                    continue;
                }
                if h[i + 1] != a {
                    continue;
                }
                if h[i + 2] != b {
                    continue;
                }
                return true;
            }
        }
        false
    }
}

fn main() -> Result<()> {
    let addresses: Vec<Address> = aoc::io::read_lines("data/day07/input")?;

    println!(
        "Part 1: {} support TLS",
        addresses.iter().filter(|a| a.supports_tls()).count()
    );

    println!(
        "Part 2: {} support SSL",
        addresses.iter().filter(|a| a.supports_ssl()).count()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abba() -> Result<()> {
        assert_eq!(
            "abba[mnop]qrst".parse::<Address>().unwrap().supports_tls(),
            true
        );
        assert_eq!(
            "abcd[bddb]xyyx".parse::<Address>().unwrap().supports_tls(),
            false
        );
        assert_eq!(
            "aaaa[qwer]tyui".parse::<Address>().unwrap().supports_tls(),
            false
        );
        assert_eq!(
            "ioxxoj[asdfgh]zxcvbn"
                .parse::<Address>()
                .unwrap()
                .supports_tls(),
            true
        );
        Ok(())
    }
}
