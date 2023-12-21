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
    let total_steps = 5000;
    // let total_steps = 26501365;
    let mut bfs = HashMapBfs::default();
    bfs.add_root(([0, 0], input.start), 0);
    while let Some(parent) = bfs.consider_next() {
        let (instance, coord) = parent;
        if total_steps <= *bfs.cost(&(instance, coord)).unwrap() {
            continue;
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

    #[derive(Debug, Clone)]
    struct Stats {
        min: usize,
        max: usize,
        odd: usize,
        even: usize,
    }

    impl Stats {
        fn add_steps(&self, steps: usize) -> Self {
            let (odd, even) = if steps % 2 == 0 {
                (self.odd, self.even)
            } else {
                (self.even, self.odd)
            };
            Self {
                min: self.min + steps,
                max: self.max + steps,
                odd,
                even,
            }
        }
    }

    #[derive(Debug)]
    enum InstanceStats {
        Calculated(Stats),
        Inferred {
            based_on: [isize; 2],
            steps_from: usize,
            stats: Stats,
        },
    }

    type WorldMap = HashMap<[isize; 2], InstanceStats>;

    // impl InstanceStats {
        // fn get_min(&self, world_map: &WorldMap) -> usize {
            // if let world_map
        // }
    // }

    let mut world_map = instance_to_map.iter().map(|(pos, instance)| {
        let mut min = usize::MAX;
        let mut max = 0;
        let mut odd = 0;
        let mut even = 0;
        for steps in instance.values() {
            min = min.min(*steps);
            max = max.max(*steps);
            if steps % 2 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }
        (*pos, InstanceStats::Calculated(Stats{ min, max, odd, even }))
    }).collect::<WorldMap>();

    let mut bfs = HashMapBfs::<[isize; 2], usize>::default();
    for pos in world_map.keys().copied() {
        bfs.add_root(pos, pos.into_iter().map(|k| k.abs() as usize).sum());
    }

    while let Some(parent) = bfs.consider_next() {
        let parent_stats;
        let parent_based_on;
        let parent_steps_from;
        match &world_map[&parent] {
            InstanceStats::Calculated(stats) => {
                parent_based_on = parent;
                parent_steps_from = 0;
                parent_stats = stats.clone();
            }
            InstanceStats::Inferred { based_on, steps_from, stats } => {
                parent_based_on = *based_on;
                parent_steps_from = *steps_from;
                parent_stats = stats.clone();
            }
        };

        let stats = parent_stats.add_steps(instance_distance);
        if total_steps < parent_stats.min {
            continue;
        }
        let based_on = parent_based_on;
        let steps_from = parent_steps_from + instance_distance;

        for direction in Direction::ALL {
            let motion = direction.motion();
            let neighbor_pos = [parent[0] + motion[0], parent[1] + motion[1]];
            world_map.entry(neighbor_pos).or_insert_with(|| InstanceStats::Inferred {
                based_on,
                steps_from,
                stats: stats.clone(),
            });
            bfs.add_edge(parent, neighbor_pos, 1);
        }
    }

    println!("{:?}", world_map[&[0, 0]]);
    world_map.iter().map(|(instance_pos, instance_stats)| {
        match instance_stats {
            InstanceStats::Calculated(Stats { even, odd, .. }) => {
                    if total_steps % 2 == 0 {
                        *even
                    } else {
                        *odd
                    }
            }
            InstanceStats::Inferred { based_on, steps_from, stats: Stats { even, odd, max, .. } } => {
                if *max <= total_steps {
                    if total_steps % 2 == 0 {
                        *even
                    } else {
                        *odd
                    }
                } else {
                    instance_to_map[&[0, 0]].keys().filter(|coord| {
                        let Some(min_dist) = instance_to_map.iter().filter_map(|(orig_pos, orig_map)| {
                            if orig_pos.into_iter().map(|p| p.abs() as usize).max().unwrap() < 2 {
                                return None;
                            }
                            let orig_steps = orig_map.get(*coord)?;
                            let manhattern_distance: usize = orig_pos.into_iter().zip(instance_pos).map(|(a, b)| (a - b).abs() as usize).sum();
                            Some(manhattern_distance * instance_distance + *orig_steps)
                        }).min() else {
                            return false;
                        };
                        (min_dist % 2) == (total_steps % 2) && (min_dist <= total_steps)
                    }).count()
                    // instance_to_map[based_on].values().filter(|steps| {
                        // let steps = *steps + steps_from;
                        // steps % 2 == 0 && steps <= total_steps
                    // }).count()
                }
            },
        }
    }).sum::<usize>() - 4
}
