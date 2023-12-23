use std::collections::HashMap;

use itertools::Itertools;

use crate::common::dfs::HashMapDfs;
use crate::common::direction::Direction;
use crate::common::vmatrix::VMatrix;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Paths,
    Forset,
    Slope(Direction),
}

impl From<&Tile> for char {
    fn from(tile: &Tile) -> char {
        match tile {
            Tile::Paths => '.',
            Tile::Forset => '#',
            Tile::Slope(direction) => match direction {
                Direction::North => '^',
                Direction::South => 'v',
                Direction::West => '<',
                Direction::East => '>',
            },
        }
    }
}

pub fn generator(input: &str) -> VMatrix<Tile> {
    VMatrix::from_chars(input, |_pos, ch| match ch {
        '.' => Tile::Paths,
        '#' => Tile::Forset,
        '^' => Tile::Slope(Direction::North),
        'v' => Tile::Slope(Direction::South),
        '<' => Tile::Slope(Direction::West),
        '>' => Tile::Slope(Direction::East),
        _ => panic!("Invalid tile definition {ch:?}"),
    })
}

fn find_start_and_end_coords(input: &VMatrix<Tile>) -> ([usize; 2], [usize; 2]) {
    [0, input.rows - 1]
        .into_iter()
        .map(|row| {
            (0..input.cols)
                .map(|col| [row, col])
                .filter(|coord| input[*coord] == Tile::Paths)
                .exactly_one()
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}

fn gen_graph(input: &VMatrix<Tile>) -> HashMap<[usize; 2], HashMap<[usize; 2], usize>> {
    let (start, end) = find_start_and_end_coords(input);
    input
        .iter()
        .filter_map(|(coord, tile)| {
            if matches!(tile, Tile::Slope(_)) {
                Some(coord)
            } else {
                None
            }
        })
        .chain([start])
        .map(|start| {
            let mut edges = HashMap::new();
            let mut dfs = HashMapDfs::new(start, 0);
            while let Some(coord) = dfs.consider_next() {
                let mut add_edge = |cost: usize| match edges.entry(coord) {
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        let entry = entry.get_mut();
                        if *entry < cost {
                            *entry = cost;
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(cost);
                    }
                };

                if coord == end {
                    add_edge(dfs.cost(&coord));
                    continue;
                }
                match input[coord] {
                    Tile::Paths => {
                        for direction in Direction::ALL {
                            if let Ok(neighbor) = input.motion(coord, direction.motion()) {
                                dfs.add_edge(&coord, neighbor, 1);
                            }
                        }
                    }
                    Tile::Forset => {}
                    Tile::Slope(direction) => {
                        let neighbor = input
                            .motion(coord, direction.motion())
                            .expect("Slope lead outside map");
                        if coord == start {
                            dfs.add_edge(&coord, neighbor, 1);
                        } else if !dfs.was_in_current_path(&neighbor) {
                            add_edge(dfs.cost(&coord));
                        }
                    }
                }
            }
            (start, edges)
        })
        .collect()
}

fn solve(
    graph: &HashMap<[usize; 2], HashMap<[usize; 2], usize>>,
    start: [usize; 2],
    end: [usize; 2],
) -> usize {
    let mut dfs = HashMapDfs::new(start, 0);
    std::iter::from_fn(|| {
        let coord = dfs.consider_next()?;
        if coord == end {
            Some(dfs.cost(&coord))
        } else {
            for (neighbor, cost) in graph[&coord].iter() {
                dfs.add_edge(&coord, *neighbor, *cost);
            }
            Some(0) // can't be max
        }
    })
    .max()
    .unwrap()
}

pub fn part_1(input: &VMatrix<Tile>) -> usize {
    let (start, end) = find_start_and_end_coords(input);
    let graph = gen_graph(input);
    solve(&graph, start, end)
}

pub fn part_2(input: &VMatrix<Tile>) -> usize {
    let (start, end) = find_start_and_end_coords(input);
    let input = input.map(|_, tile| match tile {
        Tile::Paths => Tile::Paths,
        Tile::Forset => Tile::Forset,
        Tile::Slope(_) => Tile::Paths,
    });
    let graph = gen_graph(&input);
    solve(&graph, start, end)
}
