use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::common::direction::Direction;
use crate::vmatrix::VMatrix;

#[derive(Debug)]
#[allow(unused)]
pub struct Instruction {
    direction: Direction,
    meters: usize,
    color: u32,
}

pub fn generator(input: &str) -> Vec<Instruction> {
    let pattern = Regex::new(r#"^([UDLR])\s*(\d+)\s*\(#([[:xdigit:]]{6})\)$"#).unwrap();
    input
        .lines()
        .map(|line| {
            let m = pattern.captures(line).unwrap();
            Instruction {
                direction: match &m[1] {
                    "U" => Direction::North,
                    "D" => Direction::South,
                    "L" => Direction::West,
                    "R" => Direction::East,
                    _ => panic!("Unknown direction {:?}", &m[1]),
                },
                meters: m[2].parse().unwrap(),
                color: u32::from_str_radix(&m[3], 16).unwrap(),
            }
        })
        .collect()
}

fn visited_points(input: &[Instruction]) -> impl '_ + Iterator<Item = [isize; 2]> {
    let mut current = [0, 0];

    [[0, 0]]
        .into_iter()
        .chain(input.iter().map(move |instruction| {
            for (pos, motion) in current.iter_mut().zip(instruction.direction.motion()) {
                *pos += motion * instruction.meters as isize;
            }
            current
        }))
}

#[derive(Debug)]
#[allow(unused)]
struct DimensionWrapper {
    mapper: HashMap<isize, usize>,
    expands: Vec<usize>,
    virtual_size: usize,
}

impl DimensionWrapper {
    fn from_points(points: impl Iterator<Item = isize>) -> Self {
        let mut points = points.collect_vec();
        points.sort();

        let mut mapper = HashMap::new();
        let mut expands = Vec::new();

        let mut points = points.into_iter();

        let first = points.next().unwrap();
        mapper.insert(first, 0);
        expands.push(1);
        let mut prev = first;

        for point in points {
            let dist = point - prev;
            if dist <= 0 {
                continue;
            }
            if 1 < dist {
                expands.push(dist as usize - 1);
            }
            prev = point;

            mapper.insert(point, expands.len());
            expands.push(1);
        }

        let virtual_size = mapper.values().max().unwrap() + 1;

        Self {
            mapper,
            expands,
            virtual_size,
        }
    }
}

#[derive(Debug)]
struct SpaceWrapper([DimensionWrapper; 2]);

impl SpaceWrapper {
    fn new(input: &[Instruction]) -> Self {
        let visited_points = visited_points(input).collect_vec();

        Self(
            [0, 1].map(|d| {
                DimensionWrapper::from_points(visited_points.iter().map(|coord| coord[d]))
            }),
        )
    }

    fn map(&self, coord: [isize; 2]) -> [usize; 2] {
        [self.0[0].mapper[&coord[0]], self.0[1].mapper[&coord[1]]]
    }

    fn calc_space(&self, coord: [usize; 2]) -> usize {
        coord
            .into_iter()
            .zip(&self.0)
            .map(|(c, w)| w.expands[c])
            .product()
    }
}

pub fn solve_for(input: &[Instruction]) -> usize {
    let space_wrapper = SpaceWrapper::new(input);

    let mut pos: [isize; 2] = [0, 0];
    let mut trenches = VMatrix::new(
        space_wrapper.0[0].virtual_size,
        space_wrapper.0[1].virtual_size,
        |_| false,
    );
    for instruction in input {
        let from = space_wrapper.map(pos);
        let motion = instruction.direction.motion();
        for (p, m) in pos.iter_mut().zip(motion) {
            *p += m * instruction.meters as isize;
        }
        let to = space_wrapper.map(pos);
        let ranges = [0, 1].map(|d| {
            let mut range: [usize; 2] = [from[d], to[d]];
            range.sort();
            range[0]..=range[1]
        });
        for row in ranges[0].clone() {
            for col in ranges[1].clone() {
                trenches[[row, col]] = true;
            }
        }
    }
    let mut outside = trenches.map(|_, _| false);
    let mut to_paint = Vec::new();
    for c in 0..outside.cols {
        to_paint.push([0, c]);
        to_paint.push([outside.rows - 1, c]);
    }
    for r in 0..outside.rows {
        to_paint.push([r, 0]);
        to_paint.push([r, outside.cols - 1]);
    }
    while let Some(coord) = to_paint.pop() {
        if outside[coord] || trenches[coord] {
            continue;
        }
        outside[coord] = true;
        for neighbor in outside.motions(coord, Direction::ALL.map(|d| d.motion())) {
            to_paint.push(neighbor);
        }
    }
    for (coord, is_outside) in outside.iter() {
        if !is_outside {
            trenches[coord] = true;
        }
    }
    trenches
        .iter()
        .filter_map(|(coord, v)| {
            if *v {
                Some(space_wrapper.calc_space(coord))
            } else {
                None
            }
        })
        .sum()
}

pub fn part_1(input: &[Instruction]) -> usize {
    solve_for(input)
}

pub fn part_2(input: &[Instruction]) -> usize {
    let new_input = input
        .iter()
        .map(|Instruction { color, .. }| Instruction {
            direction: match color % 16 {
                0 => Direction::East,
                1 => Direction::South,
                2 => Direction::West,
                3 => Direction::North,
                _ => panic!("Illegal direction"),
            },
            meters: *color as usize / 16,
            color: 0,
        })
        .collect_vec();
    solve_for(&new_input)
}
