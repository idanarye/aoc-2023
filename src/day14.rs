use std::collections::HashMap;

use itertools::Itertools;

use crate::vmatrix::VMatrix;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Tile {
    Empty,
    Cube,
    Rounded,
}

impl From<&Tile> for char {
    fn from(val: &Tile) -> Self {
        match val {
            Tile::Empty => '.',
            Tile::Cube => '#',
            Tile::Rounded => 'O',
        }
    }
}

pub fn generator(input: &str) -> VMatrix<Tile> {
    VMatrix::from_chars(input, |ch| match ch {
        '.' => Tile::Empty,
        '#' => Tile::Cube,
        'O' => Tile::Rounded,
        _ => panic!(),
    })
}

impl VMatrix<Tile> {
    fn gen_four_ranges(&self) -> Vec<Vec<Vec<(usize, usize)>>> {
        vec![
            // North:
            (0..self.cols)
                .map(move |col| (0..self.rows).map(move |row| (row, col)).collect_vec())
                .collect_vec(),
            // West:
            (0..self.rows)
                .map(move |row| (0..self.cols).map(move |col| (row, col)).collect_vec())
                .collect_vec(),
            // South:
            (0..self.cols)
                .map(move |col| {
                    (0..self.rows)
                        .rev()
                        .map(move |row| (row, col))
                        .collect_vec()
                })
                .collect_vec(),
            // East:
            (0..self.rows)
                .map(move |row| {
                    (0..self.cols)
                        .rev()
                        .map(move |col| (row, col))
                        .collect_vec()
                })
                .collect_vec(),
        ]
    }

    fn tilt(&mut self, ranges: &[Vec<(usize, usize)>]) {
        for range in ranges.iter() {
            let mut range = range.iter().copied();
            let mut pile_from = range.clone();
            let mut num_rounded_to_pile = 0;
            loop {
                match range.next().map(|coord| (coord, self[coord])) {
                    Some((_, Tile::Empty)) => {}
                    Some((coord, Tile::Rounded)) => {
                        num_rounded_to_pile += 1;
                        self[coord] = Tile::Empty;
                    }
                    tile @ (Some((_, Tile::Cube)) | None) => {
                        for coord in pile_from.take(num_rounded_to_pile) {
                            self[coord] = Tile::Rounded;
                        }
                        if tile.is_some() {
                            // Note that this will actually put us AFTER the tile we are matching
                            // on. Which is what we want.
                            pile_from = range.clone();
                            num_rounded_to_pile = 0;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn calc_load(&self) -> usize {
        self.gen_four_ranges()[0]
            .iter()
            .flat_map(|range| {
                range
                    .iter()
                    .rev()
                    .enumerate()
                    .filter_map(|(i, coord)| matches!(self[*coord], Tile::Rounded).then_some(i + 1))
            })
            .sum()
    }
}

pub fn part_1(input: &VMatrix<Tile>) -> usize {
    let mut input = input.clone();
    input.tilt(&input.gen_four_ranges()[0]);
    input.calc_load()
}

pub fn part_2(input: &VMatrix<Tile>) -> usize {
    let mut input = input.clone();
    let ranges_four_ways = input.gen_four_ranges();

    let mut seen = HashMap::new();

    const TIMES_TO_REPEAT: usize = 1_000_000_000;

    for i in 0..TIMES_TO_REPEAT {
        seen.insert(input.clone(), i);
        for ranges in ranges_four_ways.iter() {
            input.tilt(ranges);
        }
        if let Some(cycle_starts_at) = seen.get(&input) {
            let cycle_will_be_repeated_at = i + 1; // at the next iteration
            let steps_to_do_from_cycle_start = TIMES_TO_REPEAT - cycle_starts_at;
            let cycle_length = cycle_will_be_repeated_at - cycle_starts_at;
            let actual_steps_in_cycle = steps_to_do_from_cycle_start % cycle_length;
            let use_the_one_from_step = actual_steps_in_cycle + cycle_starts_at;
            let input_to_clone = seen
                .iter()
                .find_map(|(status, step)| (*step == use_the_one_from_step).then_some(status));
            if let Some(input_to_clone) = input_to_clone {
                input = input_to_clone.clone();
                break;
            }
        }
    }
    input.calc_load()
}
