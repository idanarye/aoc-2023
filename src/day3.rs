use std::collections::HashMap;
use std::fmt::Write;
use std::ops::Range;

use crate::vmatrix::VMatrix;

#[derive(Debug)]
pub struct Map(VMatrix<Cell>);

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Digit(u8),
    Symbol(char),
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .to_display(|f, _i, c| {
                f.write_char(match c {
                    Cell::Empty => '.',
                    Cell::Digit(d) => (d + b'0') as char,
                    Cell::Symbol(s) => *s,
                })
            })
            .fmt(f)
    }
}

pub fn generator(input: &str) -> Map {
    Map(input
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|c| {
                    Some(match c {
                        '0'..='9' => Cell::Digit(c as u8 - b'0'),
                        '.' => Cell::Empty,
                        _ => Cell::Symbol(c),
                    })
                })
                .chain([None])
        })
        .collect())
}

impl Map {
    fn number_positions_in_row(&self, row: usize) -> impl '_ + Iterator<Item = Range<usize>> {
        let mut start = None;
        (0..=(self.0.cols)).filter_map(move |col| {
            let cell = if col == self.0.cols {
                Cell::Empty
            } else {
                self.0[(row, col)]
            };
            match (cell, start) {
                (Cell::Digit(_), None) => {
                    start = Some(col);
                    None
                }
                (Cell::Empty | Cell::Symbol(_), Some(_)) => Some(start.take().unwrap()..col),
                _ => None,
            }
        })
    }

    fn number_positions(&self) -> impl '_ + Iterator<Item = (usize, Range<usize>)> {
        (0..self.0.rows).flat_map(|row| {
            self.number_positions_in_row(row)
                .map(move |cols| (row, cols))
        })
    }

    fn locations_around(
        &self,
        row: usize,
        cols: Range<usize>,
    ) -> impl Iterator<Item = (usize, usize)> {
        let first_col = if 0 < cols.start {
            Some(cols.start - 1)
        } else {
            None
        };
        let after_col = if cols.end + 1 < self.0.cols {
            Some(cols.end + 1)
        } else {
            None
        };
        let cols = (first_col.unwrap_or(0))..(after_col.unwrap_or(self.0.cols));
        let ranges: [Option<(usize, Range<usize>)>; 4] = [
            if 0 < row {
                Some((row - 1, cols.clone()))
            } else {
                None
            },
            first_col.map(|col| (row, col..(col + 1))),
            after_col.map(|col| (row, (col - 1)..col)),
            if row + 1 < self.0.rows {
                Some((row + 1, cols))
            } else {
                None
            },
        ];
        ranges
            .into_iter()
            .flatten()
            .flat_map(|(row, cols)| cols.map(move |col| (row, col)))
    }

    fn symbols_around_number(
        &self,
        row: usize,
        cols: Range<usize>,
    ) -> impl '_ + Iterator<Item = ((usize, usize), char)> {
        self.locations_around(row, cols).filter_map(|pos| {
            if let Cell::Symbol(s) = self.0[pos] {
                Some((pos, s))
            } else {
                None
            }
        })
    }

    fn number_near_symbols(&self, row: usize, cols: Range<usize>) -> bool {
        self.symbols_around_number(row, cols).any(|_| true)
    }

    fn read_number(&self, pos: (usize, Range<usize>)) -> usize {
        let (row, cols) = pos;
        let mut result = 0;
        for col in cols {
            if let Cell::Digit(digit) = self.0[(row, col)] {
                result *= 10;
                result += digit as usize;
            } else {
                panic!()
            }
        }
        result
    }
}

pub fn part_1(input: &Map) -> usize {
    input
        .number_positions()
        .filter(|(row, cols)| input.number_near_symbols(*row, cols.clone()))
        .map(|(row, cols)| input.read_number((row, cols.clone())))
        .sum()
}

pub fn part_2(input: &Map) -> usize {
    let mut numbers_around_gears = HashMap::<(usize, usize), Vec<usize>>::default();
    for (row, cols) in input.number_positions() {
        for (symbol_pos, symbol) in input.symbols_around_number(row, cols.clone()) {
            if symbol == '*' {
                numbers_around_gears
                    .entry(symbol_pos)
                    .or_default()
                    .push(input.read_number((row, cols.clone())));
            }
        }
    }
    numbers_around_gears
        .into_iter()
        .filter_map(|(_, numbers)| {
            if numbers.len() == 2 {
                Some(numbers.into_iter().product::<usize>())
            } else {
                None
            }
        })
        .sum()
}
