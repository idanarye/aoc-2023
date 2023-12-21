use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use itertools::Itertools;

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
    // let total_steps = 5000;
    // let total_steps = 26501365;
    let total_steps = 10;
    let mut bfs = HashMapBfs::default();
    bfs.add_root(([0, 0], input.start), 0);
    while let Some(parent) = bfs.consider_next() {
        let (instance, coord) = parent;
        if total_steps <= *bfs.cost(&(instance, coord)).unwrap() {
            // continue;
        }
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
    let instance_distance = instance_to_map.iter().flat_map(|(instance, instance_map)| {
        Direction::ALL.into_iter().flat_map(|direction| {
            let motion = direction.motion();
            let neighbor = [instance[0] + motion[0], instance[1] + motion[1]];
            let neighbor_map = instance_to_map.get(&neighbor)?;
            let diffs = instance_map
                .iter()
                .filter_map(|(pos, this)| {
                    let that = neighbor_map.get(pos)?;
                    Some(*this as isize - *that as isize)
                })
            .collect::<HashSet<isize>>();
            Some(diffs.into_iter().exactly_one().ok()?.abs() as usize)
        })
    }).collect::<HashSet<usize>>().into_iter().exactly_one().expect("does not support the general input of non-rectangular gardens");
    println!("{:?}", instance_distance);

    let source_garden_position = instance_to_map.keys().copied().filter(|pos| pos.into_iter().map(|p| p.abs()).max().unwrap() <= 1).collect_vec();

    fn count_reachable(instance_map: &HashMap<[usize; 2], usize>, num_steps: usize) -> usize {
        instance_map.values().filter(|&&steps| steps <= num_steps && (steps % 2) == (num_steps % 2)).count()
    }

    (0..).map(|manhatten_distance| {
        let extra_steps = manhatten_distance * instance_distance;
        if total_steps < extra_steps {
            return 0;
        }
        source_garden_position.iter().map(|pos| {
            let dups = match pos.into_iter().map(|p| p.abs()).sum() {
                0 => if manhatten_distance == 0 { 1 } else { 0 },
                1 => if 0 < manhatten_distance { 1 } else { 0 },
                2 => manhatten_distance.max(1) - 1,
                _ => panic!(),
            };
            if dups == 0 {
                return 0;
            }
            1 * count_reachable(&instance_to_map[pos], total_steps - extra_steps)
        }).sum::<usize>()
    }).take_while(|&reachable| 0 < reachable).sum()
}
