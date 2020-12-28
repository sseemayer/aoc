use aoc2015::Result;

struct Character {
    hp: i8,
    damage: i8,
    armor: i8,
    cost: u16,
}

impl Character {
    fn buy(&mut self, item: &Item) {
        self.damage += item.damage;
        self.armor += item.armor;
        self.cost += item.cost;
    }

    fn attack(&self, target: &mut Character) {
        let damage = std::cmp::max(self.damage - target.armor, 1);
        target.hp -= damage;
    }
}

impl std::fmt::Debug for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "✚{:3}  💰{:3} ⚔{:1} 🛡{:1}",
            self.hp, self.cost, self.damage, self.armor
        )
    }
}

#[derive(PartialEq)]
struct Item {
    name: String,
    cost: u16,
    damage: i8,
    armor: i8,
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "{:10} (💰{:3} ⚔{:1} 🛡{:1})",
            self.name, self.cost, self.damage, self.armor
        )
    }
}

fn parse_shop(items: &str) -> Result<Vec<Item>> {
    /*
    0    5    10   15   20   25   30
    |    |    |    |    |    |    |
    Rings:      Cost  Damage  Armor
    Damage +1    25     1       0
    Damage +2    50     2       0
    Damage +3   100     3       0
    Defense +1   20     0       1
    Defense +2   40     0       2
    Defense +3   80     0       3
    */

    let mut out = Vec::new();
    for line in items.trim().split("\n") {
        let line = line.trim();
        let name = line[0..11].trim().to_owned();
        let cost = line[11..15].trim().parse()?;
        let damage = line[15..22].trim().parse()?;
        let armor = line[22..29].trim().parse()?;

        out.push(Item {
            name,
            cost,
            damage,
            armor,
        });
    }

    Ok(out)
}

enum BattleOutcome {
    PlayerWins,
    BossWins,
}

fn battle(mut player: Character, mut boss: Character) -> (BattleOutcome, u16) {
    let mut round = 1;
    loop {
        //println!("###  ROUND {}", round);
        player.attack(&mut boss);
        //println!("Boss:   {:?}", boss);

        if boss.hp <= 0 {
            return (BattleOutcome::PlayerWins, round);
        }

        boss.attack(&mut player);
        //println!("Player: {:?}", player);
        if player.hp <= 0 {
            return (BattleOutcome::BossWins, round);
        }

        round += 1;
    }
}

fn main() -> Result<()> {
    let weapons = parse_shop(
        "
    Dagger        8     4       0
    Shortsword   10     5       0
    Warhammer    25     6       0
    Longsword    40     7       0
    Greataxe     74     8       0
    ",
    )?;

    let armors = parse_shop(
        "
    Unarmored     0     0       0
    Leather      13     0       1
    Chainmail    31     0       2
    Splintmail   53     0       3
    Bandedmail   75     0       4
    Platemail   102     0       5
    ",
    )?;

    let rings = parse_shop(
        "
    No ring L     0     0       0
    No ring R     0     0       0
    Damage +1    25     1       0
    Damage +2    50     2       0
    Damage +3   100     3       0
    Defense +1   20     0       1
    Defense +2   40     0       2
    Defense +3   80     0       3 
   ",
    )?;

    let mut cheapest_loadout = None;
    let mut cheapest_price = std::u16::MAX;

    let mut expensiest_loadout = None;
    let mut expensiest_price = std::u16::MIN;

    for weapon in &weapons {
        for armor in &armors {
            for ring1 in &rings {
                for ring2 in &rings {
                    if ring1 == ring2 {
                        continue;
                    }
                    let mut player = Character {
                        hp: 100,
                        cost: 0,
                        damage: 0,
                        armor: 0,
                    };

                    let loadout = [weapon, armor, ring1, ring2];

                    for item in &loadout {
                        player.buy(item);
                    }

                    println!("\n\nA new challenger appears!\n{:?}", player);
                    println!("{:#?}", loadout);

                    let boss = Character {
                        hp: 109,
                        damage: 8,
                        armor: 2,
                        cost: 999,
                    };

                    let cost = player.cost;

                    match battle(player, boss) {
                        (BattleOutcome::PlayerWins, round) => {
                            println!("Player wins in round {}!", round);
                            if cost < cheapest_price {
                                cheapest_loadout = Some(loadout);
                                cheapest_price = cost;
                            }
                        }
                        (BattleOutcome::BossWins, round) => {
                            println!("Boss wins in round {}!", round);
                            if cost > expensiest_price {
                                expensiest_loadout = Some(loadout);
                                expensiest_price = cost;
                            }
                        }
                    }
                }
            }
        }
    }

    println!(
        "Cheapest victorious loadout at 💰{} was:\n{:#?}",
        cheapest_price,
        cheapest_loadout.unwrap()
    );

    println!(
        "Most expensive losing loadout at 💰{} was:\n{:#?}",
        expensiest_price,
        expensiest_loadout.unwrap()
    );

    Ok(())
}
