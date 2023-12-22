use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::Range;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Brick {
    start: [usize; 3],
    end: [usize; 3],
}

impl Brick {
    fn range_for_dim(&self, dim: usize) -> Range<usize> {
        self.start[dim]..(self.end[dim] + 1)
    }

    fn iter_xy(&self) -> impl Iterator<Item = [usize; 2]> {
        let y_range = self.range_for_dim(1);
        self.range_for_dim(0)
            .flat_map(move |x| y_range.clone().map(move |y| [x, y]))
    }

    const fn height(&self) -> usize {
        self.end[2] - self.start[2] + 1
    }
}

pub fn generator(input: &str) -> Vec<Brick> {
    let pattern = Regex::new(r#"^(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)$"#).unwrap();
    input
        .lines()
        .map(|line| {
            let m = pattern.captures(line).unwrap();
            let brick = Brick {
                start: [
                    m[1].parse().unwrap(),
                    m[2].parse().unwrap(),
                    m[3].parse().unwrap(),
                ],
                end: [
                    m[4].parse().unwrap(),
                    m[5].parse().unwrap(),
                    m[6].parse().unwrap(),
                ],
            };
            for (s, e) in brick.start.iter().zip(brick.end.iter()) {
                assert!(s <= e);
            }
            brick
        })
        .collect()
}

#[derive(Debug)]
struct Pillar {
    top_brick: usize,
    height: usize,
}

fn compute_supported_by_graph(input: &[Brick]) -> HashMap<usize, HashSet<usize>> {
    let mut indices_at_each_initial_z = BTreeMap::<usize, Vec<usize>>::new();
    for (i, brick) in input.iter().enumerate() {
        indices_at_each_initial_z
            .entry(brick.start[2])
            .or_default()
            .push(i);
    }
    let indices_at_each_initial_z = indices_at_each_initial_z;

    let mut pillars = HashMap::<[usize; 2], Pillar>::new();

    let mut supported_by_grap = HashMap::<usize, HashSet<usize>>::new();
    for (_initial_z, brick_indices) in indices_at_each_initial_z.iter() {
        for &brick_idx in brick_indices {
            let brick = &input[brick_idx];
            let land_on_height = brick
                .iter_xy()
                .filter_map(|coord| Some(pillars.get(&coord)?.height))
                .max()
                .unwrap_or(0);
            let new_height = land_on_height + brick.height();
            for coord in brick.iter_xy() {
                match pillars.entry(coord) {
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        let pillar = entry.get_mut();
                        if pillar.height == land_on_height {
                            supported_by_grap
                                .entry(brick_idx)
                                .or_default()
                                .insert(pillar.top_brick);
                        }
                        pillar.top_brick = brick_idx;
                        pillar.height = new_height;
                    }
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(Pillar {
                            top_brick: brick_idx,
                            height: new_height,
                        });
                    }
                }
            }
        }
    }
    supported_by_grap
}

fn find_sole_supporters(input: &[Brick]) -> HashSet<usize> {
    compute_supported_by_graph(input)
        .values()
        .filter_map(|supporters| supporters.iter().copied().exactly_one().ok())
        .collect::<HashSet<usize>>()
}

pub fn part_1(input: &[Brick]) -> usize {
    input.len() - find_sole_supporters(input).len()
}

pub fn part_2(input: &[Brick]) -> usize {
    let supported_by_grap = compute_supported_by_graph(input);
    let mut supporting_graph = HashMap::<usize, Vec<usize>>::new();
    for (supporter, supports) in supported_by_grap.iter() {
        for support in supports {
            supporting_graph
                .entry(*support)
                .or_default()
                .push(*supporter);
        }
    }

    let num_affected_when_disintegrating = |disintegrated_brick_idx: usize| -> usize {
        let mut to_disintegrate = vec![disintegrated_brick_idx];
        let mut disintegrated = HashSet::<usize>::new();

        while let Some(idx) = to_disintegrate.pop() {
            disintegrated.insert(idx);
            let Some(affected_list) = supporting_graph.get(&idx) else {
                continue;
            };
            for affected in affected_list {
                if supported_by_grap[affected]
                    .iter()
                    .all(|i| disintegrated.contains(i))
                {
                    to_disintegrate.push(*affected);
                }
            }
        }

        disintegrated.len() - 1
    };

    (0..input.len()).map(num_affected_when_disintegrating).sum()
}
