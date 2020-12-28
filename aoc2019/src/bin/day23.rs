use aoc2019::intcode::{parse_program, IntCodeResult, State};
use aoc2019::result::{format_err, Error, Result};
use aoc2019::util::read_to_string;

use std::cell::RefCell;

struct MultiState {
    states: Vec<RefCell<State>>,
    outputs: Vec<RefCell<Vec<i64>>>,

    nat_data: Option<(i64, i64)>,
}

impl MultiState {
    fn new(program: &Vec<i64>, n_states: usize) -> Self {
        let states: Vec<RefCell<State>> = (0..n_states)
            .map(|i| {
                RefCell::new(State {
                    memory: program.to_vec(),
                    ic: 0,
                    inputs: vec![i as i64],
                    relative_base: 0,
                })
            })
            .collect();

        let outputs: Vec<RefCell<Vec<i64>>> =
            (0..n_states).map(|_| RefCell::new(Vec::new())).collect();

        let nat_data = None;

        MultiState {
            states,
            outputs,
            nat_data,
        }
    }

    fn run(&mut self) -> Result<()> {
        let mut cur_comp = 0;

        let mut idle_since = None;
        let mut last_revival_data = None;

        loop {
            let mut state = self.states[cur_comp].borrow_mut();
            let mut outputs = self.outputs[cur_comp].borrow_mut();

            match state.run(&mut outputs)? {
                IntCodeResult::Input => {
                    state.inputs.push(-1);

                    // idle detection - have we already been in an idle cycle?
                    if let Some(is) = idle_since {
                        // have we completed a full cycle?
                        if is == cur_comp {
                            println!("{} is in idle cycle", cur_comp);

                            if let Some((x, y)) = self.nat_data {
                                println!("Revive 0 with NAT data x={} y={}", x, y);

                                if let Some((lx, ly)) = last_revival_data {
                                    if x == lx && y == ly {
                                        println!("Got repeated revival data x={} y={}", x, y);
                                        break;
                                    }
                                }

                                let mut dest_mut = self.states[0].borrow_mut();
                                dest_mut.inputs.push(x);
                                dest_mut.inputs.push(y);

                                // all other computers are idle - jump 0 in next loop
                                cur_comp = self.states.len() - 1;

                                last_revival_data = self.nat_data;

                                idle_since = None;
                            }
                        }
                    } else {
                        // start idle detection cycle here
                        idle_since = Some(cur_comp);
                    }
                }
                IntCodeResult::Output => {
                    idle_since = None;
                }
                IntCodeResult::Halt => println!("Computer {} halted", cur_comp),
            }

            if outputs.len() >= 3 {
                let dest = outputs.remove(0) as usize;
                let x = outputs.remove(0);
                let y = outputs.remove(0);

                if dest == 255 {
                    if self.nat_data == None {
                        println!("Got first message to 255! x={}, y={}", x, y);
                    }
                    self.nat_data = Some((x, y));
                } else {
                    let mut dest_mut = self.states[dest].borrow_mut();

                    dest_mut.inputs.push(x);
                    dest_mut.inputs.push(y);
                }
            }

            cur_comp = (cur_comp + 1) % self.states.len();
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let program = parse_program(&read_to_string("data/day23/input")?)?;

    let mut ms = MultiState::new(&program, 50);

    ms.run()?;

    Ok(())
}
