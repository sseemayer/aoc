use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand<C: Card> {
    cards: Vec<C>,
    bid: usize,
}

impl<C: Card> FromStr for Hand<C> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (cards, bid) = s
            .trim()
            .split_once(' ')
            .ok_or_else(|| anyhow!("Bad hand: '{}'", s))?;

        let cards: Vec<C> = cards.chars().map(|c| C::from_char(c)).collect::<Vec<C>>();

        let bid: usize = bid.parse().context("Parse bid")?;

        Ok(Self { cards, bid })
    }
}

impl<C: Card> std::cmp::Ord for Hand<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_a = self.get_type();
        let type_b = other.get_type();

        match type_a.cmp(&type_b) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                    let c = C::cmp(a, b);

                    match &c {
                        Ordering::Less | Ordering::Greater => return c,
                        Ordering::Equal => {}
                    }
                }

                Ordering::Equal
            }
        }
    }
}

impl<C: Card> std::cmp::PartialOrd for Hand<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Hand::cmp(self, other))
    }
}

impl<C: Card> std::cmp::PartialEq for Hand<C> {
    fn eq(&self, other: &Self) -> bool {
        Hand::cmp(self, other) == Ordering::Equal
    }
}

impl<C: Card> std::cmp::Eq for Hand<C> {}

impl<C: Card> Hand<C> {
    fn get_type(&self) -> HandType {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for card in &self.cards {
            *counts.entry(card.get_char()).or_default() += 1;
        }

        C::get_hand_type(counts)
    }
}

trait Card {
    fn from_char(c: char) -> Self;
    fn get_char(&self) -> char;
    fn get_value(&self) -> u8;

    fn get_hand_type(counts: HashMap<char, usize>) -> HandType;

    fn cmp(&self, other: &Self) -> Ordering {
        let v1 = self.get_value();
        let v2 = other.get_value();
        u8::cmp(&v1, &v2)
    }
}

const REGULAR_CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

struct RegularCard(char);

impl Card for RegularCard {
    fn from_char(c: char) -> Self {
        Self(c)
    }

    fn get_char(&self) -> char {
        self.0
    }

    fn get_value(&self) -> u8 {
        REGULAR_CARDS
            .iter()
            .find_position(|v| **v == self.0)
            .expect("Valid card")
            .0 as u8
    }

    fn get_hand_type(counts: HashMap<char, usize>) -> HandType {
        let mut just_counts = counts.values().cloned().collect::<Vec<usize>>();
        just_counts.sort_by_key(|v| std::cmp::Reverse(*v));

        if just_counts[0] == 5 {
            HandType::FiveOfAKind
        } else if just_counts[0] == 4 {
            HandType::FourOfAKind
        } else if just_counts[0] == 3 && just_counts[1] == 2 {
            HandType::FullHouse
        } else if just_counts[0] == 3 {
            HandType::ThreeOfAKind
        } else if just_counts[0] == 2 && just_counts[1] == 2 {
            HandType::TwoPairs
        } else if just_counts[0] == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

const JOKER_CARDS: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

struct JokerCard(char);

impl Card for JokerCard {
    fn from_char(c: char) -> Self {
        Self(c)
    }

    fn get_char(&self) -> char {
        self.0
    }

    fn get_value(&self) -> u8 {
        JOKER_CARDS
            .iter()
            .find_position(|v| **v == self.0)
            .expect("Valid card")
            .0 as u8
    }

    fn get_hand_type(mut counts: HashMap<char, usize>) -> HandType {
        let joker_count = counts.remove(&'J').unwrap_or(0);

        let mut just_counts = counts.values().cloned().collect::<Vec<usize>>();
        just_counts.sort_by_key(|v| std::cmp::Reverse(*v));

        if joker_count == 5 {
            return HandType::FiveOfAKind;
        }

        if just_counts[0] + joker_count == 5 {
            return HandType::FiveOfAKind;
        }
        if just_counts[0] + joker_count == 4 {
            return HandType::FourOfAKind;
        }

        for j in 0..=joker_count {
            if just_counts[0] + j == 3 && just_counts[1] + (joker_count - j) == 2 {
                return HandType::FullHouse;
            }
        }

        if just_counts[0] + joker_count == 3 {
            return HandType::ThreeOfAKind;
        }

        for j in 0..=joker_count {
            if just_counts[0] + j == 2 && just_counts[1] + (joker_count - j) == 2 {
                return HandType::TwoPairs;
            }
        }

        if just_counts[0] + joker_count == 2 {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

#[derive(Debug)]
struct Deck<C: Card> {
    hands: Vec<Hand<C>>,
}

impl<C: Card> Deck<C> {
    fn read(filename: &str) -> Result<Self> {
        let mut hands = aoc::io::read_lines(filename)?;
        hands.sort();

        Ok(Self { hands })
    }

    fn get_winnings(&self) -> usize {
        self.hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i + 1) * h.bid)
            .sum::<usize>()
    }
}

fn main() -> Result<()> {
    let filename = "data/day07/input";

    let deck: Deck<RegularCard> = Deck::read(filename)?;
    println!("Part 1: {}", deck.get_winnings());

    let deck: Deck<JokerCard> = Deck::read(filename)?;
    println!("Part 2: {}", deck.get_winnings());

    Ok(())
}
