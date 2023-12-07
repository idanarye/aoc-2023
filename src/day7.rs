use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Write};
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct RowData {
    hand: Hand,
    bid: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Card(u8);

#[derive(Clone)]
struct Hand([Card; 5]);

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Hand")
            .field(&format_args!(
                "{}{}{}{}{}",
                self.0[0], self.0[1], self.0[2], self.0[3], self.0[4]
            ))
            .finish()
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand(
            s.chars().map(Card::from).collect_vec().try_into().unwrap(),
        ))
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2'..='9' => Card(value as u8 - b'0'),
            'T' => Card(10),
            'J' => Card(11),
            'Q' => Card(12),
            'K' => Card(13),
            'A' => Card(14),
            _ => panic!("Bad card type {value:?}"),
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            1 => f.write_char('J'),
            2..=9 => write!(f, "{}", self.0),
            10 => f.write_char('T'),
            11 => f.write_char('J'),
            12 => f.write_char('Q'),
            13 => f.write_char('K'),
            14 => f.write_char('A'),
            _ => panic!("Bad card {:?}", self),
        }
    }
}

pub fn generator(input: &str) -> Vec<RowData> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            RowData {
                hand: hand.parse().unwrap(),
                bid: bid.parse().unwrap(),
            }
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut groups = BTreeMap::<Card, usize>::new();
        for card in self.0.iter() {
            *groups.entry(*card).or_default() += 1;
        }

        if let Some(joker_count) = groups.remove(&Card(1)) {
            if let Some((&join_to, _)) = groups.iter().max_by_key(|(_, v)| *v) {
                *groups.get_mut(&join_to).unwrap() += joker_count;
            } else {
                groups.insert(Card(1), joker_count);
            }
        }

        let mut cardinals = [0; 5];
        for (cardinal, count) in cardinals.iter_mut().zip(groups.values()) {
            *cardinal = *count;
        }
        cardinals.sort_by_key(|c| Reverse(*c));
        match cardinals {
            [5, ..] => HandType::FiveOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [3, 2, ..] => HandType::FullHouse,
            [3, 1, ..] => HandType::ThreeOfAKind,
            [2, 2, ..] => HandType::TwoPair,
            [2, 1, ..] => HandType::OnePair,
            [1, ..] => HandType::HighCard,
            _ => panic!("Unrecognized hand {:?}", cardinals),
        }
    }

    fn jack_to_joker(&self) -> Hand {
        Hand(self.0.map(|card| if card.0 == 11 { Card(1) } else { card }))
    }
}

fn solve(input: impl Iterator<Item = RowData>) -> usize {
    let mut with_hand_types = input
        .map(|inp| (inp.clone(), inp.hand.hand_type()))
        .collect_vec();
    with_hand_types.sort_by_key(|(inp, hand_type)| (*hand_type, inp.hand.0));
    with_hand_types
        .iter()
        .enumerate()
        .map(|(i, (inp, _))| (i + 1) * inp.bid)
        .sum()
}

pub fn part_1(input: &[RowData]) -> usize {
    solve(input.iter().cloned())
}

pub fn part_2(input: &[RowData]) -> usize {
    solve(input.iter().map(|inp| RowData {
        hand: inp.hand.jack_to_joker(),
        bid: inp.bid,
    }))
}
