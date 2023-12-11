use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

pub fn generator(input: &str) -> Vec<[usize; 2]> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, ch)| match ch {
                    '.' => None,
                    '#' => Some([row, col]),
                    _ => panic!("Cannot handle {ch:?}"),
                })
        })
        .collect()
}

fn gen_expansion_mapping(
    values: impl Iterator<Item = usize>,
    expansion_multiplier: usize,
) -> BTreeMap<usize, usize> {
    let used: BTreeSet<usize> = values.collect();
    let mut it = used.iter().copied();
    let Some(first) = it.next() else {
        return Default::default();
    };
    let mut prev = first;
    let mut expansion = 0;
    [(first, first)]
        .into_iter()
        .chain(it.map(|value| {
            expansion += (value - prev - 1) * (expansion_multiplier - 1);
            prev = value;
            (value, value + expansion)
        }))
        .collect()
}

fn distance(this: [usize; 2], that: [usize; 2]) -> usize {
    this.into_iter()
        .zip(that)
        .map(|(a, b)| if a < b { b - a } else { a - b })
        .sum()
}

fn solve_with_expansion_multiplier(input: &[[usize; 2]], expansion_multiplier: usize) -> usize {
    let expansion_mappings = [0, 1]
        .map(|i| gen_expansion_mapping(input.iter().map(|coord| coord[i]), expansion_multiplier));
    let expanded = input
        .iter()
        .map(|coord| [0, 1].map(|i| expansion_mappings[i][&coord[i]]))
        .collect_vec();
    expanded
        .iter()
        .enumerate()
        .map(|(i, this)| {
            expanded[(i + 1)..]
                .iter()
                .map(|that| distance(*this, *that))
                .sum::<usize>()
        })
        .sum()
}

pub fn part_1(input: &[[usize; 2]]) -> usize {
    solve_with_expansion_multiplier(input, 2)
}

pub fn part_2(input: &[[usize; 2]]) -> usize {
    solve_with_expansion_multiplier(input, 1_000_000)
}
