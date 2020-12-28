use aoc2019::intcode::{parse_program, IntCodeResult, State};
use aoc2019::result::{format_err, Error, Result};
use aoc2019::util::read_to_string;

fn encode_springscript(s: &str) -> Vec<i64> {
    let mut out = Vec::new();

    for line in s.lines() {
        let mut line_empty = true;

        for c in line.trim().chars() {
            if c == '#' {
                break;
            }

            out.push(c as i64);
            line_empty = false;
        }

        if !line_empty {
            out.push(10);
        }
    }

    out
}

fn render(out: &Vec<i64>) {
    for c in out {
        print!("{}", (*c as u8) as char);
    }
}

fn run(intcode: &Vec<i64>, springscript: &str) -> Result<()> {
    let inputs = encode_springscript(springscript.trim());
    let mut state = State::new(intcode.to_vec());
    state.inputs.extend(inputs);
    let mut output = Vec::new();
    loop {
        match state.run(&mut output)? {
            IntCodeResult::Input => panic!("Not enough input! "),
            IntCodeResult::Output => {}
            IntCodeResult::Halt => {
                break;
            }
        }
    }

    println!("Output:");
    render(&output);

    println!("{}", output[output.len() - 1]);

    Ok(())
}

fn main() -> Result<()> {
    let intcode = parse_program(&read_to_string("data/day21/input")?)?;

    //
    // @
    // #### ####     D
    //
    //
    // @
    // # ### ###     -A && D
    //
    // @
    // ### # #       -C && D
    //
    // #ABCD
    //
    //
    // .................
    // .................
    // @................
    // #####.#..########
    //  ABCD

    println!("PART ONE");
    run(
        &intcode,
        "
    # J: do we need to jump?
    NOT J J
    AND A J
    AND B J
    AND C J
    NOT J J

    # Can we land safely?
    AND D J
    
    WALK
    "
        .trim(),
    )?;

    // ..2...2....
    // .1.3.1.3...
    // @...4...4..
    // #.#.##..#.#
    //  ABCDEFGHI
    //
    // ..2.......
    // .1.3......
    // @...4.....
    // #...######
    //  ABCDEFGHI

    println!("\n\nPART TWO");
    run(
        &intcode,
        "
    # J: do we need to jump?
    NOT J J
    AND A J
    AND B J
    AND C J
    NOT J J

    # T: can we land safely? definitely need D as a landing pad, and also E or H as a next step
    OR E T
    OR H T
    AND D T

    # jump if needed & landing is safe
    AND T J
    
    RUN 
    "
        .trim(),
    )?;

    Ok(())
}
