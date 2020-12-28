use aoc2015::Result;
use lazy_static::lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref POSSIBLE_MOVES: Vec<Spell> = vec![
        Spell::MagicMissile,
        Spell::Drain,
        Spell::Effect {
            effect: Effect::Shield,
        },
        Spell::Effect {
            effect: Effect::Poison,
        },
        Spell::Effect {
            effect: Effect::Recharge,
        },
    ];
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Effect { effect: Effect },
}

impl Spell {
    fn get_cost(&self) -> i64 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Effect { effect } => effect.get_cost(),
        }
    }

    fn apply_to(&self, state: &mut State) {
        state.health_state.player_mana -= self.get_cost();
        match self {
            Spell::MagicMissile => state.health_state.boss_health -= 4,
            Spell::Drain => {
                state.health_state.boss_health -= 2;
                state.health_state.player_health += 2;
            }
            Spell::Effect { effect } => {
                effect.start_applying_to(&mut state.health_state);
                state
                    .active_effects
                    .data
                    .insert(effect.clone(), effect.get_initial_duration());
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
enum Effect {
    Shield,
    Poison,
    Recharge,
    HardModeDrain,
}

impl Effect {
    fn get_cost(&self) -> i64 {
        match self {
            Effect::Shield => 113,
            Effect::Poison => 173,
            Effect::Recharge => 229,
            Effect::HardModeDrain => 0,
        }
    }

    fn get_initial_duration(&self) -> usize {
        match self {
            Effect::Shield => 6,
            Effect::Poison => 6,
            Effect::Recharge => 5,
            Effect::HardModeDrain => std::usize::MAX,
        }
    }

    fn start_applying_to(&self, state: &mut HealthState) {
        if self == &Effect::Shield {
            state.player_armor += 7;
        }
    }
    fn end_applying_to(&self, state: &mut HealthState) {
        if self == &Effect::Shield {
            state.player_armor -= 7;
        }
    }

    fn apply_to(&self, state: &mut HealthState) {
        match self {
            Effect::Shield => {}
            Effect::Poison => {
                state.boss_health -= 3;
            }
            Effect::Recharge => {
                state.player_mana += 101;
            }
            Effect::HardModeDrain => {
                state.player_health -= 1;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct ActiveEffects {
    data: HashMap<Effect, usize>,
}

impl ActiveEffects {
    fn apply_all_to(&mut self, state: &mut HealthState) {
        for (effect, duration) in self.data.iter_mut() {
            effect.apply_to(state);
            *duration -= 1;
        }

        self.data.retain(|e, d| {
            if *d <= 0 {
                e.end_applying_to(state);
                false
            } else {
                true
            }
        });
    }
}

impl std::hash::Hash for ActiveEffects {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut data: Vec<_> = self.data.iter().collect();
        data.sort();
        data.hash(state);
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    active_effects: ActiveEffects,
    health_state: HealthState,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "🧙: ❤️={} 🛡️={} ✨={}\n👹: ❤️={}\n{:#?}",
            self.health_state.player_health,
            self.health_state.player_armor,
            self.health_state.player_mana,
            self.health_state.boss_health,
            self.active_effects.data,
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct HealthState {
    player_mana: i64,
    player_health: i64,
    player_armor: i64,

    boss_health: i64,
    boss_damage: i64,
}

#[derive(Debug, PartialEq, Eq)]
enum TurnOutcome {
    PlayerWins,
    BossWins,
    Continue { state: State },
}

impl TurnOutcome {
    fn unwrap(self) -> State {
        match self {
            TurnOutcome::Continue { state } => state,
            _ => panic!("Unwrap win state"),
        }
    }
}

impl State {
    fn possible_moves(&self) -> Vec<Spell> {
        let mut out = Vec::new();
        for spell in POSSIBLE_MOVES.iter() {
            if spell.get_cost() <= self.health_state.player_mana {
                out.push(spell.clone());
            }
        }
        out
    }

    fn take_turn(&self, player_move: &Spell) -> TurnOutcome {
        let mut boss_turn_active_effects = self.active_effects.clone();
        let mut boss_turn_health_state = self.health_state.clone();
        boss_turn_active_effects.apply_all_to(&mut boss_turn_health_state);

        let mut boss_turn_state = State {
            active_effects: boss_turn_active_effects,
            health_state: boss_turn_health_state,
        };
        player_move.apply_to(&mut boss_turn_state);

        // println!(
        //     "\nAFTER PLAYER TURN OF {:?}:\n{:#?}",
        //     player_move, boss_turn_state
        // );

        if boss_turn_state.health_state.boss_health <= 0 {
            return TurnOutcome::PlayerWins;
        }

        let mut player_turn_active_effects = boss_turn_state.active_effects.clone();
        let mut player_turn_health_state = boss_turn_state.health_state.clone();
        player_turn_active_effects.apply_all_to(&mut player_turn_health_state);

        if player_turn_health_state.boss_health <= 0 {
            return TurnOutcome::PlayerWins;
        }

        player_turn_health_state.player_health -= std::cmp::max(
            1,
            boss_turn_state.health_state.boss_damage - boss_turn_state.health_state.player_armor,
        );

        let new_state = State {
            active_effects: player_turn_active_effects,
            health_state: player_turn_health_state,
        };

        // println!("\nAFTER BOSS TURN:\n{:#?}", new_state);

        if new_state.health_state.player_health <= 0 {
            return TurnOutcome::BossWins;
        }

        TurnOutcome::Continue { state: new_state }
    }
}

fn recurse(state: State, mana_spent: usize, current_min: usize) -> usize {
    let mut min_mana = current_min;

    for spell in state.possible_moves() {
        let turn_mana = mana_spent + spell.get_cost() as usize;

        // do not recursively process turns that would result in higher mana spend than the
        // current best path
        if turn_mana > current_min {
            continue;
        }

        let solution_mana = match state.take_turn(&spell) {
            TurnOutcome::BossWins => continue,
            TurnOutcome::PlayerWins => turn_mana,
            TurnOutcome::Continue { state: turn_state } => recurse(turn_state, turn_mana, min_mana),
        };

        min_mana = std::cmp::min(solution_mana, min_mana);
    }

    min_mana
}

fn main() -> Result<()> {
    let start_state = State {
        active_effects: Default::default(),
        health_state: HealthState {
            player_mana: 500,
            player_health: 50,
            player_armor: 0,

            boss_health: 58,
            boss_damage: 9,
        },
    };

    let min_mana = recurse(start_state, 0, std::usize::MAX);
    println!("Part 1: Min mana: {:?}", min_mana);

    let mut hard_mode = HashMap::new();
    hard_mode.insert(Effect::HardModeDrain, std::usize::MAX);
    let start_state = State {
        active_effects: ActiveEffects { data: hard_mode },
        health_state: HealthState {
            player_mana: 500,
            player_health: 50,
            player_armor: 0,

            boss_health: 58,
            boss_damage: 9,
        },
    };

    let min_mana = recurse(start_state, 0, std::usize::MAX);
    println!("Part 2: Min mana: {:?}", min_mana);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simulation_1() {
        let turn0 = State {
            active_effects: Default::default(),
            health_state: HealthState {
                player_mana: 250,
                player_health: 10,
                player_armor: 0,

                boss_health: 13,
                boss_damage: 8,
            },
        };

        println!("{:#?}", turn0);

        let turn1 = turn0
            .take_turn(&Spell::Effect {
                effect: Effect::Poison,
            })
            .unwrap();

        let turn2 = turn1.take_turn(&Spell::MagicMissile);

        assert_eq!(turn2, TurnOutcome::PlayerWins);
    }

    #[test]
    fn test_simulation_2() {
        let turn0 = State {
            active_effects: Default::default(),
            health_state: HealthState {
                player_mana: 250,
                player_health: 10,
                player_armor: 0,

                boss_health: 14,
                boss_damage: 8,
            },
        };

        println!("{:#?}", turn0);

        let turn1 = turn0
            .take_turn(&Spell::Effect {
                effect: Effect::Recharge,
            })
            .unwrap();

        let turn2 = turn1
            .take_turn(&Spell::Effect {
                effect: Effect::Shield,
            })
            .unwrap();

        let turn3 = turn2.take_turn(&Spell::Drain).unwrap();

        let turn4 = turn3
            .take_turn(&Spell::Effect {
                effect: Effect::Poison,
            })
            .unwrap();

        let turn5 = turn4.take_turn(&Spell::MagicMissile);

        assert_eq!(turn5, TurnOutcome::PlayerWins);
    }
}
