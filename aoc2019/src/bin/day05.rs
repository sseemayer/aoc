use failure::{format_err, Error};
use std::fs::File;
use std::io::Read;
type Result<T> = std::result::Result<T, Error>;

fn get_parameter(data: &Vec<i64>, parmodes: &Vec<u8>, ic: usize, ofs: usize) -> Result<i64> {
    let pm = parmodes.get(ofs - 1).unwrap_or(&0);
    let pv = data[ic + ofs];

    let val = match pm {
        0 => {
            // position mode
            data[pv as usize]
        }
        1 => {
            // immediate mode
            pv
        }
        _ => return Err(format_err!("Invalid parameter mode at {}: {}", ofs, pm)),
    };

    Ok(val)
}

fn run(data: &Vec<i64>, input_val: i64) -> Result<Vec<i64>> {
    let mut data = data.clone();

    let mut ic = 0;

    loop {
        let mut v = data[ic];

        let opcode = v % 100;
        v /= 100;

        let mut parmodes = Vec::new();

        while v > 0 {
            let pm = v % 10;
            parmodes.push(pm as u8);
            v /= 10;
        }

        match opcode {
            1 => {
                // add
                let a = get_parameter(&data, &parmodes, ic, 1)?;
                let b = get_parameter(&data, &parmodes, ic, 2)?;
                let pos_store = data[ic + 3] as usize;

                data[pos_store] = a + b;
                ic += 4;
            }
            2 => {
                // mul
                let a = get_parameter(&data, &parmodes, ic, 1)?;
                let b = get_parameter(&data, &parmodes, ic, 2)?;
                let pos_store = data[ic + 3] as usize;

                data[pos_store] = a * b;
                ic += 4;
            }
            3 => {
                // input
                let pos_store = data[ic + 1] as usize;
                data[pos_store] = input_val;
                ic += 2;
            }
            4 => {
                // output
                let v = get_parameter(&data, &parmodes, ic, 1)?;
                println!("Output: {}", v);
                ic += 2;
            }
            5 => {
                // jump-if-true
                let a = get_parameter(&data, &parmodes, ic, 1)?;
                let b = get_parameter(&data, &parmodes, ic, 2)?;

                if a != 0 {
                    ic = b as usize;
                } else {
                    ic += 3;
                }
            }
            6 => {
                // jump-if false
                let a = get_parameter(&data, &parmodes, ic, 1)?;
                let b = get_parameter(&data, &parmodes, ic, 2)?;

                if a == 0 {
                    ic = b as usize;
                } else {
                    ic += 3;
                }
            }
            7 => {
                // less-than
                let a = get_parameter(&data, &parmodes, ic, 1)?;
                let b = get_parameter(&data, &parmodes, ic, 2)?;
                let pos_store = data[ic + 3] as usize;

                data[pos_store] = if a < b { 1 } else { 0 };
                ic += 4;
            }
            8 => {
                // equals
                let a = get_parameter(&data, &parmodes, ic, 1)?;
                let b = get_parameter(&data, &parmodes, ic, 2)?;
                let pos_store = data[ic + 3] as usize;

                data[pos_store] = if a == b { 1 } else { 0 };
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
    let mut f = File::open("data/day05/input")?;

    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    let buf = String::from_utf8(buf)?;

    let data: Vec<i64> = buf
        .trim()
        .split(",")
        .map(|v| v.parse().map_err(|e: std::num::ParseIntError| e.into()))
        .collect::<Result<Vec<i64>>>()?;

    println!("FIRST RUN");
    run(&data, 1)?;

    println!("SECOND RUN");
    run(&data, 5)?;

    Ok(())
}
