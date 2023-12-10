use std::fmt::{Display, Write as _};

use crate::vmatrix::VMatrix;

#[derive(Debug)]
pub struct Input {
    start: (usize, usize),
    map: VMatrix<Pipe>,
}

#[derive(Debug, Clone, Copy)]
enum Pipe {
    Ground,
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    const ALL: [Direction; 4] = [Self::North, Self::South, Self::West, Self::East];

    const fn motion(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::East => (0, 1),
        }
    }
}

impl Pipe {
    fn route(&self, direction: Direction) -> Option<Direction> {
        match (self, direction) {
            (Pipe::Vertical, Direction::North) => Some(Direction::North),
            (Pipe::Vertical, Direction::South) => Some(Direction::South),
            (Pipe::Horizontal, Direction::West) => Some(Direction::West),
            (Pipe::Horizontal, Direction::East) => Some(Direction::East),
            (Pipe::BendNE, Direction::South) => Some(Direction::East),
            (Pipe::BendNE, Direction::West) => Some(Direction::North),
            (Pipe::BendNW, Direction::South) => Some(Direction::West),
            (Pipe::BendNW, Direction::East) => Some(Direction::North),
            (Pipe::BendSW, Direction::North) => Some(Direction::West),
            (Pipe::BendSW, Direction::East) => Some(Direction::South),
            (Pipe::BendSE, Direction::North) => Some(Direction::East),
            (Pipe::BendSE, Direction::West) => Some(Direction::South),
            _ => None,
        }
    }

    fn directions(&self) -> &[Direction] {
        match self {
            Pipe::Ground => &[],
            Pipe::Vertical => &[Direction::North, Direction::South],
            Pipe::Horizontal => &[Direction::West, Direction::East],
            Pipe::BendNE => &[Direction::North, Direction::East],
            Pipe::BendNW => &[Direction::North, Direction::West],
            Pipe::BendSW => &[Direction::South, Direction::West],
            Pipe::BendSE => &[Direction::South, Direction::East],
        }
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start_index = self.map.coord_to_index(self.start).unwrap();
        self.map
            .to_display(move |f, i, p| {
                if i == start_index {
                    f.write_char('S')
                } else {
                    f.write_char(match p {
                        Pipe::Ground => '.',
                        Pipe::Vertical => '|',
                        Pipe::Horizontal => '-',
                        Pipe::BendNE => 'L',
                        Pipe::BendNW => 'J',
                        Pipe::BendSW => '7',
                        Pipe::BendSE => 'F',
                    })
                }
            })
            .fmt(f)
    }
}

pub fn generator(input: &str) -> Input {
    let mut start = None;
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    Some(match c {
                        'S' => {
                            start = Some((row, col));
                            Pipe::Ground
                        }
                        '.' => Pipe::Ground,
                        '|' => Pipe::Vertical,
                        '-' => Pipe::Horizontal,
                        'L' => Pipe::BendNE,
                        'J' => Pipe::BendNW,
                        '7' => Pipe::BendSW,
                        'F' => Pipe::BendSE,
                        _ => panic!("Bad input {c:?}"),
                    })
                })
                .chain([None])
                .collect::<Vec<_>>()
        })
        .collect();
    Input {
        map,
        start: start.unwrap(),
    }
}

impl VMatrix<Pipe> {
    fn walk(
        &self,
        start: (usize, usize),
        direction: Direction,
    ) -> impl '_ + Iterator<Item = (usize, usize)> {
        let mut coord = start;
        let mut current_direction = Some(direction);
        std::iter::from_fn(move || {
            let direction = current_direction?;
            coord = self.motion(coord, direction.motion())?;
            let pipe = self.get(coord)?;
            current_direction = pipe.route(direction);
            Some(coord)
        })
    }
}

impl Input {
    fn main_loop(&self) -> impl '_ + Iterator<Item = (usize, usize)> {
        for direction in Direction::ALL {
            let Some(coord) = self.map.motion(self.start, direction.motion()) else {
                continue;
            };
            let pipe = self.map[coord];
            if pipe.route(direction).is_some() {
                return self.map.walk(self.start, direction);
            }
        }
        panic!("No main loop")
    }
}

pub fn part_1(input: &Input) -> usize {
    input.main_loop().count() / 2
}

#[derive(Clone, Copy, Debug)]
enum PaintStatus {
    Blank,
    MainLoop,
    Outside,
}

impl Display for VMatrix<PaintStatus> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_display(|f, _i, p| {
            f.write_char(match p {
                PaintStatus::Blank => '.',
                PaintStatus::MainLoop => '#',
                PaintStatus::Outside => 'O',
            })
        })
        .fmt(f)
    }
}

pub fn part_2(input: &Input) -> usize {
    let mut fillmap = VMatrix::new(input.map.rows * 2 + 1, input.map.cols * 2 + 1, |_| {
        PaintStatus::Blank
    });

    fn expand((row, col): (usize, usize)) -> (usize, usize) {
        (row * 2 + 1, col * 2 + 1)
    }

    for coord in input.main_loop() {
        let expanded_coord = expand(coord);
        fillmap[expanded_coord] = PaintStatus::MainLoop;
        for direction in input.map[coord].directions() {
            let Some(adjacent) = fillmap.motion(expanded_coord, direction.motion()) else {
                continue;
            };
            fillmap[adjacent] = PaintStatus::MainLoop;
        }
    }

    let mut to_paint = vec![(0, 0)];

    while let Some(coord) = to_paint.pop() {
        if !matches!(fillmap[coord], PaintStatus::Blank) {
            continue;
        }
        fillmap[coord] = PaintStatus::Outside;
        for neighbor in fillmap.motions(coord, Direction::ALL.map(|d| d.motion())) {
            to_paint.push(neighbor);
        }
    }

    fillmap
        .iter()
        .filter(|((row, col), paint_status)| {
            row % 2 == 1 && col % 2 == 1 && matches!(paint_status, PaintStatus::Blank)
        })
        .count()
}
