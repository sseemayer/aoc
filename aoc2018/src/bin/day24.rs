use std::{
    cmp::Reverse,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, bail, Result};
use colored::Colorize;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Response {
    Default,
    Immune,
    Weak,
}

impl std::default::Default for Response {
    fn default() -> Self {
        Response::Default
    }
}

impl std::str::FromStr for Response {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "immune" => Ok(Response::Immune),
            "weak" => Ok(Response::Weak),
            _ => bail!("Bad response: '{}'", s),
        }
    }
}

impl Response {
    fn damage_factor(&self) -> usize {
        match self {
            Response::Default => 1,
            Response::Immune => 0,
            Response::Weak => 2,
        }
    }
}

#[derive(Debug, Clone)]
struct Group {
    units: usize,
    hp: usize,

    defenses: HashMap<String, Response>,

    damage: usize,
    damage_type: String,

    initiative: usize,
}

lazy_static! {
    static ref RE_GROUP: Regex = Regex::new(r"(\d+) units each with (\d+) hit points (?:\((.*)\) )?with an attack that does (\d+) (.*?) damage at initiative (\d+)").unwrap();
}

impl std::str::FromStr for Group {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE_GROUP
            .captures(s)
            .ok_or_else(|| anyhow!("Bad group definition: '{}'", s))?;

        let units: usize = captures.get(1).unwrap().as_str().parse()?;
        let hp: usize = captures.get(2).unwrap().as_str().parse()?;

        let mut defenses: HashMap<String, Response> = HashMap::new();
        if let Some(blocks) = captures.get(3) {
            for block in blocks.as_str().split("; ") {
                let (response, types) = block
                    .split_once(" to ")
                    .ok_or_else(|| anyhow!("Bad defense specifier: '{}'", block))?;

                let response: Response = response.parse()?;

                for t in types.split(",") {
                    defenses.insert(t.trim().to_string(), response);
                }
            }
        }

        let damage: usize = captures.get(4).unwrap().as_str().parse()?;
        let damage_type = captures.get(5).unwrap().as_str().to_string();

        let initiative: usize = captures.get(6).unwrap().as_str().parse()?;

        Ok(Self {
            units,
            hp,
            defenses,
            damage,
            damage_type,
            initiative,
        })
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "⚡{}  🪖{} ♥️{} {:?} 🔫{}({})",
            self.initiative, self.units, self.hp, self.defenses, self.damage, self.damage_type
        )
    }
}

impl Group {
    fn effective_power(&self) -> usize {
        self.units * self.damage
    }

    fn effective_damage(&self, target: &Group) -> usize {
        self.effective_power()
            * target
                .defenses
                .get(&self.damage_type)
                .copied()
                .unwrap_or_default()
                .damage_factor()
    }
}

#[derive(Debug, Clone)]
struct State {
    armies: HashMap<String, HashMap<usize, Group>>,
    rounds: usize,
    debug: bool,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, groups) in &self.armies {
            let mut groups = groups.iter().collect::<Vec<_>>();
            groups.sort_by_key(|(i, _)| **i);

            write!(f, "{}:\n", k.bold().green())?;

            for (i, group) in groups.iter() {
                write!(f, "#{} {}\n", **i + 1, group)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl State {
    fn parse(path: &str) -> Result<Self> {
        let mut armies: HashMap<String, HashMap<usize, Group>> = HashMap::new();
        let mut current_army = String::new();
        for line in BufReader::new(File::open(path)?).lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.ends_with(":") {
                current_army = line.trim_end_matches(":").to_string();
            } else {
                let group: Group = line.parse()?;
                let groups = armies.entry(current_army.clone()).or_default();

                groups.insert(groups.len(), group);
            }
        }

        Ok(State {
            armies,
            rounds: 0,
            debug: false,
        })
    }

    fn fight(&mut self) -> bool {
        if self.armies.values().any(|a| a.is_empty()) {
            return false;
        }

        self.rounds += 1;

        type GroupId = (String, usize);

        let mut groups = self
            .armies
            .iter()
            .map(|(army, groups)| {
                groups
                    .iter()
                    .map(|(i, group)| (army.to_string(), *i, group.clone()))
            })
            .flatten()
            .collect::<Vec<_>>();

        let mut defender_to_attacker: HashMap<GroupId, GroupId> = HashMap::new();
        let mut attacker_to_defender: HashMap<GroupId, GroupId> = HashMap::new();

        // target selection: decreasing order of effective power, with higher initiative as tiebreaker
        groups.sort_by_key(|(_, _, group)| Reverse((group.effective_power(), group.initiative)));
        for (army, group_id, group) in &groups {
            // identify valid targets
            let targets = groups
                .iter()
                .filter(|(ta, ti, target)| {
                    // not attacking the same team, and not being attacked yet
                    ta != army
                        && !defender_to_attacker.contains_key(&(ta.clone(), *ti))
                        && group.effective_damage(target) > 0
                })
                .collect::<Vec<_>>();

            // choose a target
            let target = targets.iter().max_by_key(|(_, _, target)| {
                (
                    group.effective_damage(target),
                    target.effective_power(),
                    target.initiative,
                )
            });

            if let Some((target_army, target_group, _)) = target {
                defender_to_attacker.insert(
                    (target_army.to_string(), *target_group),
                    (army.to_string(), *group_id),
                );
                attacker_to_defender.insert(
                    (army.to_string(), *group_id),
                    (target_army.to_string(), *target_group),
                );
            }
        }

        // attacking: in decreasing order of initiative
        groups.sort_by_key(|(_, _, group)| Reverse(group.initiative));

        for (army, group_id, _) in &groups {
            let group = if let Some(group) = self
                .armies
                .get(army)
                .and_then(|groups| groups.get(group_id))
            {
                group.clone()
            } else {
                if self.debug {
                    println!(
                        "{}#{} was supposed to attack now but already died before it could",
                        army,
                        group_id + 1
                    );
                }
                continue;
            };

            if let Some((target_army, target_group)) =
                attacker_to_defender.get(&(army.clone(), *group_id))
            {
                let remove = if let Some(target) = self
                    .armies
                    .get_mut(target_army)
                    .and_then(|groups| groups.get_mut(target_group))
                {
                    // the target is still alive! let's change that
                    //
                    let incoming_damage = group.effective_damage(&target);
                    let units_killed = incoming_damage / target.hp;

                    if self.debug {
                        println!(
                            "{}#{} attacks {}#{} for {} damage, killing {} units",
                            army,
                            group_id + 1,
                            target_army,
                            target_group + 1,
                            incoming_damage,
                            units_killed.min(target.units)
                        );
                    }

                    if units_killed > target.units {
                        if self.debug {
                            println!("{}#{} is killed", target_army, target_group + 1);
                        }
                        true
                    } else {
                        target.units -= units_killed;
                        false
                    }
                } else {
                    false
                };

                if remove {
                    if let Some(ta) = self.armies.get_mut(target_army) {
                        ta.remove(target_group);
                    }
                }
            }
        }

        true
    }

    fn fight_to_end(&mut self) {
        let mut last_units = self.remaining_units();
        while self.fight() {
            let units = self.remaining_units();
            if units == last_units {
                // stalemate
                break;
            }
            last_units = units;

            if self.debug {
                println!("\nAfter round {}:\n{}", self.rounds, self);
            }
        }
    }

    fn remaining_units(&self) -> usize {
        self.armies
            .values()
            .map(|groups| groups.values())
            .flatten()
            .map(|group| group.units)
            .sum()
    }

    fn boost(&mut self, army: &str, amount: usize) {
        if let Some(groups) = self.armies.get_mut(army) {
            for group in groups.values_mut() {
                group.damage += amount;
            }
        }
    }
}

fn main() -> Result<()> {
    let state = State::parse("data/day24/input")?;
    println!("{}", state);

    let mut state1 = state.clone();
    state1.fight_to_end();

    println!("Part 1: {}\n\n", state1.remaining_units());

    let mut min_boost = 1;
    let mut max_boost = 100_000_000;
    loop {
        let boost = (min_boost + max_boost) / 2;

        let mut state2 = state.clone();
        state2.boost("Immune System", boost);
        state2.fight_to_end();

        let win = state2.armies.get("Infection").unwrap().is_empty();

        println!(
            "{} >= {} >= {}: {} ({} units left)",
            min_boost,
            boost,
            max_boost,
            win,
            state2.remaining_units()
        );

        if win {
            max_boost = boost - 1;
        } else {
            min_boost = boost + 1;
        }

        if min_boost > max_boost {
            println!("Part 2: {}", state2.remaining_units());
            break;
        }
    }
    Ok(())
}
