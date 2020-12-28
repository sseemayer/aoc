use aoc2019::board::{Board, Direction, Position};
use aoc2019::intcode::{parse_program, IntCodeResult, State};
use aoc2019::result::{format_err, Error, Result};
use aoc2019::util::read_to_string;
use itertools::Itertools;
use std::io::stdin;

fn encode(s: &str) -> Vec<i64> {
    let mut out = Vec::new();

    for mut line in s.lines() {
        if let Some(p) = line.find("#") {
            line = &line[0..p].trim();
        }

        let mut line_empty = true;

        for c in line.trim().chars() {
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
    let mut lines: Vec<&str> = springscript.trim().split("\n").collect();

    let inp = lines.remove(0).trim();
    println!(">> {} {:?}", inp, encode(inp));

    let mut state = State::new(intcode.to_vec());
    state.inputs.extend(encode(inp));
    let mut output = Vec::new();

    'outer: loop {
        loop {
            match state.run(&mut output)? {
                IntCodeResult::Input => {
                    break;
                }
                IntCodeResult::Output => {}
                IntCodeResult::Halt => {
                    render(&output);
                    break 'outer;
                }
            }
        }
        render(&output);

        output.clear();

        if lines.is_empty() {
            let mut l = String::new();
            stdin().read_line(&mut l)?;
            let inp = encode(l.trim());

            state.inputs.extend(inp);
        } else {
            let inp = lines.remove(0).trim();
            println!(">> {}", inp);
            state.inputs.extend(encode(inp));
        }
    }

    Ok(())
}

fn make_combinations(items: &[&str]) -> String {
    let mut out = String::new();
    for n in 1..=items.len() {
        for combo in items.iter().combinations(n) {
            out += &format!("#combination: {:?}\n", combo);
            for i in items.iter() {
                out += &format!("drop {}\n", i);
            }

            for i in combo.iter() {
                out += &format!("take {}\n", i);
            }

            out += "south\n";
        }
    }
    out
}

fn main() -> Result<()> {
    let intcode = parse_program(&read_to_string("data/day25/input")?)?;

    //     G   3
    //     |  / \
    //   H-F 2 D X
    //     | | |
    //     E.1.C B A
    //     | | | | |
    //     I @-5-6-7-8-9
    //       |
    //       4
    //
    // 1: holodeck (fixed point)
    // 2: Navigation (weather machine)
    // 3: checkpoint
    // 4: engineering
    // 5: sick bay (whirled peas)
    // 6: gift wrapping center
    // 7: observatory (!escape pod)
    // 8: stables
    // 9: kitchen (dark matter)
    // A: corridor (!infinite loop)
    // B: hallway (prime number)
    // C: passages (coin)           d
    // D; hot chocolate fountain (!giant electromagnet)
    // E: science lab (!molten lava)
    // F: crew quarters
    // G: warp drive maintenance (!photons)
    // H: arcade (astrolabe)
    // I: storage (antenna)

    let items = [
        "dark matter",
        "prime number",
        "coin",
        "whirled peas",
        "fixed point",
        "weather machine",
        "antenna",
        "astrolabe",
    ];

    println!("PART ONE");

    let mut prog = "
    east                # 5
    east                # 6
    east                # 7
    east                # 8
    east                # 9
    take dark matter
    west                # 8
    west                # 7
    west                # 6
    north               # B
    take prime number
    south               # 6
    west                # 5
    north               # C
    take coin
    west                # E
    north               # F
    west                # H
    take astrolabe
    east                # F
    south               # E
    south               # I
    take antenna
    north               # E
    east                # C
    south               # 5
    take whirled peas
    west                # @
    north               # 1
    take fixed point
    north               # 2
    take weather machine
    east                # 3
    "
    .trim()
    .to_owned();

    let combos = make_combinations(&items);

    prog += &combos.trim();

    run(&intcode, &prog)?;

    Ok(())
}
