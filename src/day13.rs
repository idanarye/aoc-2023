use std::fmt::{Display, Write as _};

use crate::vmatrix::VMatrix;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Terrain {
    Ash,
    Rocks,
}

pub fn generator(input: &str) -> Vec<VMatrix<Terrain>> {
    input
        .split("\n\n")
        .map(|pat| {
            pat.lines()
                .flat_map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => Some(Terrain::Ash),
                            '#' => Some(Terrain::Rocks),
                            _ => panic!(),
                        })
                        .chain([None])
                })
                .collect()
        })
        .collect()
}

impl Display for VMatrix<Terrain> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_display(|f, _i, p| {
            f.write_char(match p {
                Terrain::Ash => '.',
                Terrain::Rocks => '#',
            })
        })
        .fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dimension {
    Rows,
    Cols,
}

impl VMatrix<Terrain> {
    fn compare_impl(
        &self,
        length: usize,
        pred: impl Fn(usize) -> bool,
        allowed_smudges: &mut usize,
    ) -> bool {
        (0..length).all(|i| {
            if pred(i) {
                true
            } else if 0 < *allowed_smudges {
                *allowed_smudges -= 1;
                true
            } else {
                false
            }
        })
    }

    fn compare(
        &self,
        dimension: Dimension,
        i: usize,
        j: usize,
        allowed_smudges: &mut usize,
    ) -> bool {
        match dimension {
            Dimension::Rows => {
                self.compare_impl(self.cols, |c| self[(i, c)] == self[(j, c)], allowed_smudges)
            }
            Dimension::Cols => {
                self.compare_impl(self.rows, |r| self[(r, i)] == self[(r, j)], allowed_smudges)
            }
        }
    }

    fn size_on(&self, dimension: Dimension) -> usize {
        match dimension {
            Dimension::Rows => self.rows,
            Dimension::Cols => self.cols,
        }
    }

    fn check_for_reflection_at(
        &self,
        dimension: Dimension,
        i: usize,
        mut allowed_smudges: usize,
    ) -> bool {
        let lower = 0..=i;
        let higher = (i + 1)..self.size_on(dimension);
        let found = lower
            .rev()
            .zip(higher)
            .all(|(i, j)| self.compare(dimension, i, j, &mut allowed_smudges));
        found && allowed_smudges == 0
    }

    fn find_reflection(&self, dimension: Dimension, allowed_smudges: usize) -> Option<usize> {
        (0..(self.size_on(dimension) - 1))
            .find(|i| self.check_for_reflection_at(dimension, *i, allowed_smudges))
    }

    fn calc(&self, allowed_smudges: usize) -> usize {
        if let Some(along_cols) = self.find_reflection(Dimension::Cols, allowed_smudges) {
            return along_cols + 1;
        }
        if let Some(along_rows) = self.find_reflection(Dimension::Rows, allowed_smudges) {
            return 100 * (along_rows + 1);
        }
        panic!()
    }
}

pub fn part_1(input: &[VMatrix<Terrain>]) -> usize {
    input.iter().map(|pat| pat.calc(0)).sum()
}

pub fn part_2(input: &[VMatrix<Terrain>]) -> usize {
    input.iter().map(|pat| pat.calc(1)).sum()
}
