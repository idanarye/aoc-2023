#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub const ALL: [Direction; 4] = [Self::North, Self::South, Self::West, Self::East];

    pub const fn motion(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::East => (0, 1),
        }
    }

    pub const fn clockwise(&self) -> Direction {
        match self {
            Direction::North => Self::East,
            Direction::South => Self::West,
            Direction::West => Self::North,
            Direction::East => Self::South,
        }
    }

    pub const fn counter_clockwise(&self) -> Direction {
        match self {
            Direction::North => Self::West,
            Direction::South => Self::East,
            Direction::West => Self::South,
            Direction::East => Self::North,
        }
    }
}
