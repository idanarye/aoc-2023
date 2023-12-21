use std::fmt::Display;

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
    let total_steps = 64;
    let mut bfs = HashMapBfs::default();
    bfs.add_root(input.start, 0);
    while let Some(coord) = bfs.consider_next() {
        if total_steps <= *bfs.cost(&coord).unwrap() {
            continue;
        }
        for direction in Direction::ALL {
            if let Some(step_to) = input.garden.motion(coord, direction.motion()) {
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
    let _ = input;
    0
}
