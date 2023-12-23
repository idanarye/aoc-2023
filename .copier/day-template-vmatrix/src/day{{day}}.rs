use crate::common::vmatrix::VMatrix;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
}

impl From<&Tile> for char {
    fn from(tile: &Tile) -> char {
        match tile {
        }
    }
}

pub fn generator(input: &str) -> VMatrix<Tile> {
    VMatrix::from_chars(input, |pos, ch| match ch {
        _ => panic!("Invalid tile definition {ch:?}"),
    })
}

pub fn part_1(input: &VMatrix<Tile>) -> usize {
    println!("{}", input);
    0
}

pub fn part_2(input: &VMatrix<Tile>) -> usize {
    let _ = input;
    0
}
