use std::collections::HashMap;

use pest::Parser;

#[macro_use]
extern crate pest_derive;

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error on '{}': {}", filename, source))]
    Io {
        filename: String,
        source: std::io::Error,
    },

    #[snafu(display("Passport parsing error: {}", source))]
    ParsePassport { source: pest::error::Error<Rule> },
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[grammar = "day04.pest"]
pub struct PassportParser;

#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn has_required_fields(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|f| self.fields.contains_key(&f.to_string()))
    }

    fn validate_field<F: FnOnce(&String) -> bool>(&self, field_name: &str, validator: F) -> bool {
        if let Some(v) = self.fields.get(field_name) {
            if validator(v) {
                true
            } else {
                println!("{}: Rejected {:?}", field_name, v);
                false
            }
        } else {
            println!("{}: missing", field_name);
            false
        }
    }

    fn has_correct_fields(&self) -> bool {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        if !self.validate_field("byr", |v| {
            if let Ok(x) = v.parse::<i16>() {
                x >= 1920 && x <= 2002
            } else {
                false
            }
        }) {
            return false;
        }

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        if !self.validate_field("iyr", |v| {
            if let Ok(x) = v.parse::<i16>() {
                x >= 2010 && x <= 2020
            } else {
                false
            }
        }) {
            return false;
        }

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        if !self.validate_field("eyr", |v| {
            if let Ok(x) = v.parse::<i16>() {
                x >= 2020 && x <= 2030
            } else {
                false
            }
        }) {
            return false;
        }

        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        if !self.validate_field("hgt", |v| {
            let unit = &v[v.len() - 2..];
            if let Ok(x) = v[..v.len() - 2].parse::<i16>() {
                match unit {
                    "cm" => x >= 150 && x <= 193,
                    "in" => x >= 59 && x <= 76,
                    _ => false,
                }
            } else {
                false
            }
        }) {
            return false;
        }

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        if !self.validate_field("hcl", |v| {
            if v.len() != 7 {
                return false;
            }
            if &v[0..1] != "#" {
                return false;
            }

            v[1..].chars().all(|c| char::is_ascii_hexdigit(&c))
        }) {
            return false;
        }

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        if !self.validate_field("ecl", |v| {
            vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&v[..])
        }) {
            return false;
        }

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        if !self.validate_field("pid", |v| {
            if v.len() != 9 {
                return false;
            }
            v.chars().all(|c| char::is_ascii_digit(&c))
        }) {
            return false;
        }
        true
    }
}

fn parse_passports(s: &str) -> Result<Vec<Passport>> {
    let passports = PassportParser::parse(Rule::file, s)
        .context(ParsePassport)?
        .next()
        .unwrap();

    let mut out = Vec::new();
    for passport in passports.into_inner() {
        let mut fields = HashMap::new();
        for field in passport.into_inner() {
            let mut f_iter = field.into_inner();
            let tag = f_iter.next().unwrap().as_str();
            let value = f_iter.next().unwrap().as_str();

            fields.insert(tag.to_string(), value.to_string());
        }
        out.push(Passport { fields });
    }

    Ok(out)
}

fn main() -> Result<()> {
    let filename = "data/day04/input";
    let data = std::fs::read_to_string(filename).context(Io {
        filename: filename.to_string(),
    })?;

    let passports = parse_passports(&data.trim())?;

    println!("{:#?}", passports);

    let n_valid_1 = passports.iter().filter(|p| p.has_required_fields()).count();
    let n_valid_2 = passports.iter().filter(|p| p.has_correct_fields()).count();

    println!("Part 1 Got {} valid passports", n_valid_1);
    println!("Part 2 Got {} valid passports", n_valid_2);

    Ok(())
}
