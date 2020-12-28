use std::collections::{HashMap, HashSet};

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Number parsing error for '{}': {}", data, source))]
    ParseNumber {
        data: String,
        source: std::num::ParseIntError,
    },

    #[snafu(display("Field parsing error"))]
    ParseField,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct FieldDefinition {
    name: String,
    ranges: Vec<(usize, usize)>,
}

impl FieldDefinition {
    fn in_range(&self, v: usize) -> bool {
        for (a, b) in self.ranges.iter() {
            if v >= *a && v <= *b {
                return true;
            }
        }
        false
    }
}

impl std::str::FromStr for FieldDefinition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<&str> = s.split(":").collect();
        if tokens.len() != 2 {
            return Err(Error::ParseField);
        }

        let name = tokens[0].to_string();
        let ranges: Vec<(usize, usize)> = tokens[1]
            .split(" or ")
            .map(|t| {
                let tkns: Vec<&str> = t.trim().split("-").collect();
                if tkns.len() != 2 {
                    return Err(Error::ParseField);
                }
                let from: usize = tkns[0].parse().context(ParseNumber {
                    data: tkns[0].to_string(),
                })?;
                let to: usize = tkns[1].parse().context(ParseNumber {
                    data: tkns[1].to_string(),
                })?;
                Ok((from, to))
            })
            .collect::<Result<_>>()?;

        Ok(FieldDefinition { name, ranges })
    }
}

#[derive(Debug, Clone)]
struct Ticket {
    values: Vec<usize>,
}

impl Ticket {
    fn find_out_of_range(&self, defs: &[FieldDefinition]) -> Vec<usize> {
        self.values
            .iter()
            .filter(|v| {
                for fd in defs {
                    if fd.in_range(**v) {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect()
    }
}

impl std::str::FromStr for Ticket {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let values: Vec<usize> = s
            .split(",")
            .map(|v| {
                v.trim().parse().context(ParseNumber {
                    data: v.trim().to_string(),
                })
            })
            .collect::<Result<_>>()?;

        Ok(Ticket { values })
    }
}

#[derive(Debug)]
struct State {
    fields: Vec<FieldDefinition>,
    my_ticket: Ticket,
    other_tickets: Vec<Ticket>,
}

impl std::str::FromStr for State {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        enum ParserState {
            Fields,
            MyTicket,
            OtherTickets,
        }

        let mut state = ParserState::Fields;
        let mut fields: Vec<FieldDefinition> = Vec::new();
        let mut my_ticket: Option<Ticket> = None;
        let mut other_tickets: Vec<Ticket> = Vec::new();

        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match state {
                ParserState::Fields => {
                    if line == "your ticket:" {
                        state = ParserState::MyTicket;
                        continue;
                    }
                    fields.push(line.parse()?);
                }
                ParserState::MyTicket => {
                    if line == "nearby tickets:" {
                        state = ParserState::OtherTickets;
                        continue;
                    }
                    my_ticket = Some(line.parse()?);
                }
                ParserState::OtherTickets => {
                    other_tickets.push(line.parse()?);
                }
            }
        }

        Ok(State {
            fields,
            my_ticket: my_ticket.unwrap(),
            other_tickets,
        })
    }
}

fn solve(tickets: &[Ticket], fields: &[FieldDefinition]) -> Option<HashMap<usize, usize>> {
    let n_fields = fields.len();

    // possibility space: ticket value position -> which fields could correspond to that position
    let mut possible: HashMap<usize, HashSet<usize>> = (0..n_fields)
        .map(|i| {
            let possible_fields = (0..n_fields)
                .filter(|j| {
                    // can value i be field j?
                    for t in tickets {
                        let v = t.values[i];
                        if !fields[*j].in_range(v) {
                            return false;
                        }
                    }
                    true
                })
                .collect();

            (i, possible_fields)
        })
        .collect();

    let mut solution: HashMap<usize, usize> = HashMap::new();

    while solution.len() < n_fields {
        let mut new_solutions = Vec::new();
        for (i, possibilities) in possible.iter() {
            if possibilities.len() == 1 {
                new_solutions.push((*i, *possibilities.iter().next().unwrap()));
            }
        }

        if new_solutions.is_empty() {
            println!("Giving up! {:?}", possible);
            return None;
        }

        for (i, j) in new_solutions {
            println!("{} -> {}", i, j);
            solution.insert(i, j);
            possible.remove(&i);

            for possibilities in possible.values_mut() {
                possibilities.remove(&j);
            }
        }
    }

    Some(solution)
}

fn main() -> Result<()> {
    let state: State = std::fs::read_to_string("data/day16/input")
        .context(Io)?
        .parse()?;

    let mut scanning_error_rate = 0;
    let mut valid_tickets = Vec::new();
    valid_tickets.push(state.my_ticket.clone());
    for ticket in state.other_tickets {
        let errors = ticket.find_out_of_range(&state.fields[..]);

        if errors.is_empty() {
            valid_tickets.push(ticket);
        }

        scanning_error_rate += errors.into_iter().sum::<usize>();
    }

    println!("Part 1: scanning error rate {}", scanning_error_rate);

    if let Some(translation) = solve(&valid_tickets, &state.fields) {
        let mut factor = 1;
        for (i, v) in state.my_ticket.values.iter().enumerate() {
            let name = &state.fields[translation[&i]].name;
            println!("{}: {}", name, v);

            if name.starts_with("departure") {
                factor *= v;
            }
        }

        println!("Part 2: departure factor = {}", factor);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
