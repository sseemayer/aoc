use failure::{format_err, Error};
use std::fs::File;
use std::io::Read;
type Result<T> = std::result::Result<T, Error>;

fn run(data: &Vec<usize>, noun: usize, verb: usize) -> Result<Vec<usize>> {
    let mut data = data.clone();
    data[1] = noun;
    data[2] = verb;

    let mut ic = 0;

    loop {
        let opcode = data[ic];

        match opcode {
            1 => {
                // add
                let pos_read_1 = data[ic + 1];
                let pos_read_2 = data[ic + 2];
                let pos_store = data[ic + 3];

                data[pos_store] = data[pos_read_1] + data[pos_read_2];
                ic += 4;
            }
            2 => {
                // mul
                let pos_read_1 = data[ic + 1];
                let pos_read_2 = data[ic + 2];
                let pos_store = data[ic + 3];

                data[pos_store] = data[pos_read_1] * data[pos_read_2];
                ic += 4;
            }
            99 => {
                // halt
                break;
            }
            _ => return Err(format_err!("Invalid opcode: {}", opcode)),
        }
    }

    return Ok(data);
}

fn main() -> Result<()> {
    let mut f = File::open("data/day02/input")?;

    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    let buf = String::from_utf8(buf)?;

    let mut data: Vec<usize> = buf
        .trim()
        .split(",")
        .map(|v| v.parse().map_err(|e: std::num::ParseIntError| e.into()))
        .collect::<Result<Vec<usize>>>()?;

    for noun in 0..100 {
        for verb in 0..100 {
            let out = run(&data, noun, verb)?[0];

            if out == 19690720 || (noun == 12 && verb == 2) {
                println!("noun={}, verb={}: out={}", noun, verb, out);
            }
        }
    }

    Ok(())
}
