use std::collections::{HashMap, VecDeque};

use anyhow::{anyhow, bail, Result};

#[derive(Debug, Clone)]
enum ModuleState {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcast,
    Sink,
}

#[derive(Debug, Clone)]
struct Module {
    state: ModuleState,
    outputs: Vec<String>,
}

impl Module {
    fn process_signal(&mut self, source: &str, signal: bool) -> Option<bool> {
        match &mut self.state {
            ModuleState::FlipFlop(state) => {
                if !signal {
                    *state = !*state;
                    Some(*state)
                } else {
                    None
                }
            }
            ModuleState::Conjunction(state) => {
                state.insert(source.to_string(), signal);
                Some(!state.values().all(|v| *v))
            }
            ModuleState::Broadcast => Some(signal),
            ModuleState::Sink => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct System {
    modules: HashMap<String, Module>,
}

impl System {
    fn parse(path: &str) -> Result<Self> {
        let lines: Vec<String> = aoc::io::read_lines(path)?;

        // first pass: extract modules with outputs, tracking inputs
        let mut modules = HashMap::new();
        let mut inputs_for_module: HashMap<String, Vec<String>> = HashMap::new();
        for line in &lines {
            if line.trim().is_empty() {
                continue;
            }
            let (source, outputs) = line
                .split_once(" -> ")
                .ok_or_else(|| anyhow!("Bad line: '{}'", line))?;

            let outputs = outputs.split(",").map(|s| s.trim().to_string()).collect();

            let (name, state) = if source == "broadcaster" {
                let name = source.to_string();
                let state = ModuleState::Broadcast;
                (name, state)
            } else if source.starts_with("%") {
                let name = source.trim_start_matches("%").to_owned();
                let state = ModuleState::FlipFlop(false);
                (name, state)
            } else if source.starts_with("&") {
                let name = source.trim_start_matches("&").to_owned();
                let state = ModuleState::Conjunction(HashMap::new());
                (name, state)
            } else {
                bail!("Bad source: '{}'", source);
            };

            let module = Module { state, outputs };

            for output in &module.outputs {
                inputs_for_module
                    .entry(output.to_string())
                    .or_default()
                    .push(name.to_string());
            }

            modules.insert(name, module);
        }

        // second pass: set up inputs to conjunctions and add sink modules where needed
        for (name, inputs) in inputs_for_module {
            let module = modules.entry(name).or_insert(Module {
                state: ModuleState::Sink,
                outputs: Vec::new(),
            });

            if let ModuleState::Conjunction(input_states) = &mut module.state {
                for input in &inputs {
                    input_states.insert(input.to_string(), false);
                }
            }
        }

        Ok(Self { modules })
    }

    fn step(&mut self, monitor_inputs: Option<&str>) -> Result<(usize, usize, Vec<String>)> {
        let mut n_low = 0;
        let mut n_high = 0;
        let mut monitored_events = Vec::new();

        let mut queue: VecDeque<(String, String, bool)> = VecDeque::new();
        queue.push_back(("button".to_string(), "broadcaster".to_string(), false));

        while let Some((source, dest, signal)) = queue.pop_front() {
            // println!("{} -- {} -> {}", source, signal, dest);

            if signal {
                n_high += 1;
            } else {
                n_low += 1;
            }

            let module = self
                .modules
                .get_mut(&dest)
                .ok_or_else(|| anyhow!("Cannot process '{}'", dest))?;

            if let Some(output) = module.process_signal(&source, signal) {
                for new_dest in &module.outputs {
                    queue.push_back((dest.clone(), new_dest.to_string(), output));
                }
            }

            if let Some(mi) = monitor_inputs {
                if dest == mi && signal {
                    monitored_events.push(source);
                }
            }
        }

        Ok((n_low, n_high, monitored_events))
    }

    fn simulate(&mut self, steps: usize) -> Result<(usize, usize)> {
        let mut n_low = 0;
        let mut n_high = 0;

        for _ in 0..steps {
            let (nl, nh, _mi) = self.step(None)?;
            n_low += nl;
            n_high += nh;
        }

        Ok((n_low, n_high))
    }

    fn simulate_until_rx_triggered(&mut self) -> Result<usize> {
        // find the conjunction that sends to rx and get its inputs
        let (rx_sender, rx_sender_inputs) = self
            .modules
            .iter()
            .find_map(|(name, module)| {
                if module.outputs.iter().any(|o| o == "rx") {
                    if let ModuleState::Conjunction(state) = &module.state {
                        Some((name.to_string(), state.len()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow!("Cannot find who sends to rx!"))?;

        let mut steps_for_input: HashMap<String, usize> = HashMap::new();

        let mut steps = 0;
        loop {
            steps += 1;

            let (_, _, events) = self.step(Some(&rx_sender))?;

            for event in events {
                steps_for_input.entry(event).or_insert(steps);
            }

            if steps_for_input.len() >= rx_sender_inputs {
                return Ok(aoc::math::lcm_multiple(
                    &steps_for_input.values().cloned().collect::<Vec<_>>()[..],
                ));
            }
        }
    }
}

fn main() -> Result<()> {
    let system = System::parse("data/day20/input")?;

    let mut part1 = system.clone();
    let (n_low, n_high) = part1.simulate(1000)?;

    println!("Part 1: {}", n_low * n_high);

    let mut part2 = system.clone();

    println!("Part 2: {}", part2.simulate_until_rx_triggered()?);

    Ok(())
}
