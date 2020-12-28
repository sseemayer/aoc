use aoc2015::util::read_to_lines;
use aoc2015::{format_err, Error, Result};

fn unescape(s: &str) -> Result<String> {
    let mut out = String::new();

    let mut iter = s.chars();

    let open_quote = iter.next().unwrap();
    if open_quote != '"' {
        return Err(format_err!("String doesnt start with \": '{}'", s));
    }

    while let Some(c) = iter.next() {
        if c == '\\' {
            let et = iter.next().unwrap();
            match et {
                '\\' => out.push('\\'),
                '"' => out.push('"'),
                'x' => {
                    let a = iter.next().unwrap();
                    let b = iter.next().unwrap();
                    let code = u8::from_str_radix(&format!("{}{}", a, b,), 16)?;
                    let c = code as char;
                    out.push(c);
                }
                _ => return Err(format_err!("Unknown escape code: '{}'", et)),
            }
        } else {
            out.push(c);
        }
    }

    if let Some(c) = out.pop() {
        if c != '"' {
            return Err(format_err!("String doesnt end with \": '{}'", s));
        }
    }

    Ok(out)
}

fn escape(s: &str) -> Result<String> {
    let mut out = String::new();
    out.push('\"');

    for c in s.chars() {
        match c {
            '"' => {
                out.push('\\');
                out.push('"');
            }
            '\\' => {
                out.push('\\');
                out.push('\\');
            }
            _ => {
                out.push(c);
            }
        }
    }

    out.push('\"');
    Ok(out)
}

fn main() -> Result<()> {
    println!("PART ONE");
    let lines: Vec<String> = read_to_lines("data/day08/input")?;

    let mut small_size = 0;
    let mut mid_size = 0;
    let mut big_size = 0;
    for l in lines.iter() {
        let s: String = unescape(l)?;
        let e: String = escape(l)?;

        mid_size += l.len();
        small_size += s.chars().count();
        big_size += e.len();
    }

    let delta1 = mid_size - small_size;
    let delta2 = big_size - mid_size;

    println!("Part one result: {}", delta1);
    println!("Part two result: {}", delta2);

    Ok(())
}
