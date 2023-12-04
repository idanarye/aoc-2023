use std::collections::HashSet;

#[derive(Debug)]
pub struct Card {
    #[allow(unused)]
    id: usize,
    winning: Vec<usize>,
    chosen: Vec<usize>,
}

pub fn generator(input: &str) -> Vec<Card> {
    let pattern = regex::Regex::new(r#"^Card\s+(\d+): (.*?) \| (.*)$"#).unwrap();
    let number_pattern = regex::Regex::new(r#"\d+"#).unwrap();
    input
        .lines()
        .map(|line| {
            let m = pattern.captures(line).unwrap();
            let numbers = |capture_group: usize| {
                number_pattern
                    .find_iter(&m[capture_group])
                    .map(|m| m.as_str().parse().unwrap())
                    .collect()
            };
            Card {
                id: m[1].parse().unwrap(),
                winning: numbers(2),
                chosen: numbers(3),
            }
        })
        .collect()
}

impl Card {
    fn num_matches(&self) -> usize {
        let winning: HashSet<usize> = self.winning.iter().copied().collect();
        let chosen: HashSet<usize> = self.chosen.iter().copied().collect();
        winning.intersection(&chosen).count()
    }
}

pub fn part_1(input: &[Card]) -> usize {
    input
        .iter()
        .map(|card| {
            let num_matches = card.num_matches();
            if num_matches == 0 {
                0
            } else {
                2usize.pow(num_matches as u32 - 1)
            }
        })
        .sum()
}

pub fn part_2(input: &[Card]) -> usize {
    let mut nums_copies = vec![1; input.len()];
    for (i, card) in input.iter().enumerate() {
        let this_copies = nums_copies[i];
        let num_matches = card.num_matches();
        for winning_copies in nums_copies[i + 1..].iter_mut().take(num_matches) {
            *winning_copies += this_copies;
        }
    }
    nums_copies.iter().sum()
}
