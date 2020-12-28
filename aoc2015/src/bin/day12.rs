use aoc2015::util::read_to_string;
use aoc2015::{format_err, Error, Result};

use serde_json::Value;

trait Sum {
    fn sum(&self, ignore_red: bool) -> i64;
}

impl Sum for Value {
    fn sum(&self, ignore_red: bool) -> i64 {
        match self {
            Value::Null => 0,
            Value::Bool(_) => 0,
            Value::Number(v) => v.as_i64().unwrap_or(0),
            Value::String(_) => 0,
            Value::Array(v) => v.iter().map(|e| e.sum(ignore_red)).sum(),
            Value::Object(m) => {
                let red_keys = m.contains_key("red");
                let red_values = m.values().any(|v| v == "red");
                if ignore_red && (red_keys || red_values) {
                    0
                } else {
                    m.values().map(|v| v.sum(ignore_red)).sum()
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let data = read_to_string("data/day12/input")?;

    let v: Value = serde_json::from_str(&data)?;

    println!("Sum: {}", v.sum(false));
    println!("Red-corrected sum: {}", v.sum(true));

    Ok(())
}
