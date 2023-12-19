use std::collections::HashSet;

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::common::bfs::HashMapBfs;
use crate::common::direction::Direction;
use crate::common::vmatrix::VMatrix;

#[derive(Debug)]
pub struct RowData {}

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Empty,
    Mirror(MirrorDirection),
    Splitter(SplitterDirection),
}

#[derive(Debug, Clone, Copy)]
pub enum MirrorDirection {
    // Names of mirrors are like forward and backward slashes
    Forward,
    Backward,
}

#[derive(Debug, Clone, Copy)]
pub enum SplitterDirection {
    Vertical,
    Horizontal,
}

impl From<&Tile> for char {
    fn from(val: &Tile) -> Self {
        match val {
            Tile::Empty => '.',
            Tile::Mirror(MirrorDirection::Forward) => '/',
            Tile::Mirror(MirrorDirection::Backward) => '\\',
            Tile::Splitter(SplitterDirection::Vertical) => '|',
            Tile::Splitter(SplitterDirection::Horizontal) => '-',
        }
    }
}

pub fn generator(input: &str) -> VMatrix<Tile> {
    VMatrix::from_chars(input, |ch| match ch {
        '.' => Tile::Empty,
        '/' => Tile::Mirror(MirrorDirection::Forward),
        '\\' => Tile::Mirror(MirrorDirection::Backward),
        '|' => Tile::Splitter(SplitterDirection::Vertical),
        '-' => Tile::Splitter(SplitterDirection::Horizontal),
        _ => panic!("Bad char {ch:?}"),
    })
}

impl MirrorDirection {
    fn redirect_light(&self, direction: Direction) -> Direction {
        match self {
            MirrorDirection::Forward => match direction {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::West => Direction::South,
                Direction::East => Direction::North,
            },
            MirrorDirection::Backward => match direction {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::West => Direction::North,
                Direction::East => Direction::South,
            },
        }
    }
}

impl SplitterDirection {
    fn split_light(&self, direction: Direction) -> Option<[Direction; 2]> {
        match self {
            SplitterDirection::Vertical => match direction {
                Direction::North | Direction::South => None,
                Direction::West | Direction::East => Some([Direction::North, Direction::South]),
            },
            SplitterDirection::Horizontal => match direction {
                Direction::North | Direction::South => Some([Direction::West, Direction::East]),
                Direction::West | Direction::East => None,
            },
        }
    }
}

impl Tile {
    fn process_light(&self, direction: Direction) -> impl Iterator<Item = Direction> {
        let (first, second) = match self {
            Tile::Empty => (direction, None),
            Tile::Mirror(mirror) => (mirror.redirect_light(direction), None),
            Tile::Splitter(splitter) => {
                if let Some(directions) = splitter.split_light(direction) {
                    (directions[0], Some(directions[1]))
                } else {
                    (direction, None)
                }
            }
        };
        [first].into_iter().chain(second)
    }
}

impl VMatrix<Tile> {
    fn calc(&self, start: [usize; 2], direction: Direction) -> usize {
        let mut bfs = HashMapBfs::default();
        bfs.add_root((start, direction), 0);

        while let Some((coord, direction)) = bfs.consider_next() {
            for new_direction in self[coord].process_light(direction) {
                let new_coord = self.motion(coord, new_direction.motion());
                if let Some(new_coord) = new_coord {
                    bfs.add_edge((coord, direction), (new_coord, new_direction), 1);
                }
            }
        }

        bfs.all_known()
            .map(|(coord, _)| coord)
            .collect::<HashSet<_>>()
            .len()
    }
}

pub fn part_1(input: &VMatrix<Tile>) -> usize {
    input.calc([0, 0], Direction::East)
}

pub fn part_2(input: &VMatrix<Tile>) -> usize {
    Direction::ALL
        .into_iter()
        .flat_map(|direction| {
            match direction {
                Direction::North => (0..input.cols)
                    .map(|col| [input.rows - 1, col])
                    .collect_vec(),
                Direction::South => (0..input.cols).map(|col| [0, col]).collect_vec(),
                Direction::West => (0..input.rows)
                    .map(|row| [row, input.cols - 1])
                    .collect_vec(),
                Direction::East => (0..input.rows).map(|row| [row, 0]).collect_vec(),
            }
            .into_iter()
            .zip(std::iter::repeat(direction))
        })
        .par_bridge()
        .map(|(start, direction)| input.calc(start, direction))
        .max()
        .unwrap()
}
