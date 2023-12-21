use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use itertools::Itertools;
use num::Integer;

use crate::common::bfs::HashMapBfs;
use crate::common::direction::Direction;
use crate::common::vmatrix::VMatrix;

#[derive(Debug)]
pub struct Input {
    garden: VMatrix<Tile>,
    start: [usize; 2],
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.garden
            .to_display_simple(
                |pos, tile| {
                    if pos == self.start {
                        'S'
                    } else {
                        tile.into()
                    }
                },
            )
            .fmt(f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Plot,
    Rock,
}

impl From<&Tile> for char {
    fn from(val: &Tile) -> Self {
        match val {
            Tile::Plot => '.',
            Tile::Rock => '#',
        }
    }
}

pub fn generator(input: &str) -> Input {
    let mut start = None;
    let garden = VMatrix::from_chars(input, |pos, ch| match ch {
        'S' => {
            assert!(start.is_none());
            start = Some(pos);
            Tile::Plot
        }
        '.' => Tile::Plot,
        '#' => Tile::Rock,
        _ => panic!("Bad char {ch:?}"),
    });
    Input {
        garden,
        start: start.unwrap(),
    }
}

pub fn part_1(input: &Input) -> usize {
    let total_steps = 5000;
    // let total_steps = 26501365;
    let mut bfs = HashMapBfs::default();
    bfs.add_root(input.start, 0);
    while let Some(coord) = bfs.consider_next() {
        if total_steps <= *bfs.cost(&coord).unwrap() {
            continue;
        }
        for direction in Direction::ALL {
            if let Ok(step_to) = input.garden.motion(coord, direction.motion()) {
                if input.garden[step_to] == Tile::Plot {
                    bfs.add_edge(coord, step_to, 1);
                }
            }
        }
    }
    bfs.all_known()
        .filter(|coord| bfs.cost(coord).unwrap() % 2 == total_steps % 2)
        .count()
}

pub fn part_2(input: &Input) -> usize {
    let total_steps = 26501365;
    let mut bfs = HashMapBfs::default();
    bfs.add_root(([0, 0], input.start), 0);
    while let Some(parent) = bfs.consider_next() {
        let (instance, coord) = parent;
        for direction in Direction::ALL {
            let motion = direction.motion();
            match input.garden.motion(coord, motion) {
                Ok(step_to) => {
                    if input.garden[step_to] == Tile::Plot {
                        bfs.add_edge(parent, (instance, step_to), 1);
                    }
                }
                Err(step_to) => {
                    if input.garden[step_to] == Tile::Plot {
                        let new_instance = [instance[0] + motion[0], instance[1] + motion[1]];
                        if new_instance.iter().all(|coord| coord.abs() <= 2) {
                            bfs.add_edge(parent, (new_instance, step_to), 1);
                        }
                    }
                }
            }
        }
    }

    println!(
        "Num instances {}",
        bfs.all_known()
            .map(|(instance, _)| instance)
            .collect::<HashSet<_>>()
            .len()
    );

    let mut instance_to_map = HashMap::<[isize; 2], HashMap<[usize; 2], usize>>::new();
    for &(instance, pos) in bfs.all_known() {
        instance_to_map
            .entry(instance)
            .or_default()
            .insert(pos, *bfs.cost(&(instance, pos)).unwrap());
    }

    fn calc_uniform_step_diff(
        this: &HashMap<[usize; 2], usize>,
        that: &HashMap<[usize; 2], usize>,
    ) -> Option<usize> {
        let diffs = this
            .iter()
            .filter_map(|(pos, this)| {
                let that = that.get(pos)?;
                Some(*this as isize - *that as isize)
            })
            .collect::<HashSet<isize>>();
        Some(diffs.into_iter().exactly_one().ok()?.unsigned_abs())
    }

    let instance_distance = instance_to_map
        .iter()
        .flat_map(|(instance, instance_map)| {
            Direction::ALL.into_iter().flat_map(|direction| {
                let motion = direction.motion();
                let neighbor = [instance[0] + motion[0], instance[1] + motion[1]];
                let neighbor_map = instance_to_map.get(&neighbor)?;
                calc_uniform_step_diff(instance_map, neighbor_map)
            })
        })
        .collect::<HashSet<usize>>()
        .into_iter()
        .exactly_one()
        .expect("does not support the general input of non-rectangular gardens");
    assert!(
        instance_distance % 2 == 1,
        "does not support the general input of any parity"
    );
    dbg!(instance_distance);

    fn calc_manhatten_length([x, y]: [isize; 2]) -> usize {
        (x.abs() + y.abs()) as usize
    }

    let source_garden_positions = instance_to_map
        .keys()
        .copied()
        .filter(|pos| pos.iter().map(|p| p.abs()).max().unwrap() <= 1)
        .collect_vec();

    source_garden_positions
        .iter()
        .map(|&source_pos| {
            let instance_map = &instance_to_map[&source_pos];
            match calc_manhatten_length(source_pos) {
                0 => instance_map
                    .values()
                    .filter(|&&steps| steps % 2 == total_steps % 2 && steps <= total_steps)
                    .count(),
                1 => instance_map
                    .values()
                    .filter_map(|&steps| {
                        let remaining_steps = total_steps as isize - steps as isize;
                        let remaining_steps = if remaining_steps.is_odd() {
                            remaining_steps - instance_distance as isize
                        } else {
                            remaining_steps
                        };
                        if remaining_steps < 0 {
                            return None;
                        }
                        Some(1 + remaining_steps as usize / (instance_distance * 2))
                    })
                    .sum(),
                2 => instance_map
                    .values()
                    .filter_map(|&steps| {
                        let remaining_steps =
                            usize::try_from(total_steps as isize - steps as isize).ok()?;
                        fn gauss_summation(a: usize, d: usize, n: usize) -> usize {
                            n * (2 * a + (n - 1) * d) / 2
                        }
                        if remaining_steps.is_even() {
                            Some(gauss_summation(
                                1,
                                2,
                                1 + remaining_steps / (instance_distance * 2),
                            ))
                        } else {
                            if remaining_steps < instance_distance {
                                return None;
                            }
                            let remaining_steps = remaining_steps - instance_distance;
                            Some(gauss_summation(
                                2,
                                2,
                                1 + remaining_steps / (instance_distance * 2),
                            ))
                        }
                    })
                    .sum(),
                _ => panic!(),
            }
        })
        .sum()
}
