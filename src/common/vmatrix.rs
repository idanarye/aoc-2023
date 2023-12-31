use std::fmt::{Display, Write};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VMatrix<T> {
    pub cols: usize,
    pub rows: usize,
    pub values: Vec<T>,
}

impl<T> FromIterator<Option<T>> for VMatrix<T> {
    fn from_iter<I: IntoIterator<Item = Option<T>>>(iter: I) -> Self {
        let mut cols = 0;
        let mut cur_col = 0;
        let values = iter
            .into_iter()
            .filter_map(|item| {
                if item.is_none() {
                    cols = cur_col;
                    cur_col = 0;
                } else {
                    cur_col += 1;
                }
                item
            })
            .collect::<Vec<T>>();
        let rows = values.len() / cols;
        Self { cols, rows, values }
    }
}

impl<T> VMatrix<T> {
    pub fn new(rows: usize, cols: usize, mut fill: impl FnMut([usize; 2]) -> T) -> Self {
        Self {
            cols,
            rows,
            values: (0..(rows * cols))
                .map(|index| fill([index / cols, index % cols]))
                .collect(),
        }
    }

    pub fn from_chars(input: &str, mut mapper: impl FnMut([usize; 2], char) -> T) -> Self {
        let mut row = 0;
        let mut col = 0;
        input
            .chars()
            .map(move |ch| {
                if ch == '\n' {
                    row += 1;
                    col = 0;
                    None
                } else {
                    let result = mapper([row, col], ch);
                    col += 1;
                    Some(result)
                }
            })
            .collect()
    }

    pub fn map<S>(&self, mut dlg: impl FnMut(usize, &T) -> S) -> VMatrix<S> {
        VMatrix {
            cols: self.cols,
            rows: self.rows,
            values: self
                .values
                .iter()
                .enumerate()
                .map(|(i, v)| dlg(i, v))
                .collect(),
        }
    }

    pub fn to_display<
        'a,
        F: 'a + Fn(&mut std::fmt::Formatter<'_>, usize, &T) -> std::fmt::Result,
    >(
        &'a self,
        fmt: F,
    ) -> impl 'a + Display {
        struct Displayer<'a, T, F: Fn(&mut std::fmt::Formatter<'_>, usize, &T) -> std::fmt::Result> {
            target: &'a VMatrix<T>,
            formatter: F,
        }
        impl<'a, T, F: Fn(&mut std::fmt::Formatter<'_>, usize, &T) -> std::fmt::Result> Display
            for Displayer<'a, T, F>
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for (i, h) in self.target.values.iter().enumerate() {
                    if i % self.target.cols == 0 {
                        f.write_char('\n')?;
                    }
                    (self.formatter)(f, i, h)?;
                }
                Ok(())
            }
        }
        Displayer {
            target: self,
            formatter: fmt,
        }
    }

    pub fn to_display_simple<'a>(
        &'a self,
        value_formatter: impl 'a + Fn([usize; 2], &T) -> char,
    ) -> impl 'a + Display {
        self.to_display(move |fmt, idx, value| {
            fmt.write_char(value_formatter(self.index_to_coord(idx).unwrap(), value))
        })
    }

    pub fn neighbors_no_diag(&self, node: usize) -> impl '_ + Iterator<Item = usize> {
        let x = (node % self.cols) as isize;
        let y = (node / self.cols) as isize;
        [[0, -1], [1, 0], [0, 1], [-1, 0]]
            .into_iter()
            .filter_map(move |[dx, dy]: [isize; 2]| {
                let x = x + dx;
                if x < 0 || self.cols as isize <= x {
                    return None;
                }
                let y = y + dy;
                if y < 0 || self.rows as isize <= y {
                    return None;
                }
                Some(x as usize + y as usize * self.cols)
            })
    }

    pub fn coord_to_index(&self, [row, col]: [usize; 2]) -> Option<usize> {
        if row < self.rows && col < self.cols {
            Some(row * self.cols + col)
        } else {
            None
        }
    }

    pub fn index_to_coord(&self, index: usize) -> Option<[usize; 2]> {
        if index < self.values.len() {
            Some([index / self.cols, index % self.cols])
        } else {
            None
        }
    }

    pub fn get(&self, coord: [usize; 2]) -> Option<&T> {
        self.coord_to_index(coord).map(|i| &self.values[i])
    }

    pub fn get_mut(&mut self, coord: [usize; 2]) -> Option<&mut T> {
        self.coord_to_index(coord).map(|i| &mut self.values[i])
    }

    pub fn motion(&self, start: [usize; 2], vec: [isize; 2]) -> Result<[usize; 2], [usize; 2]> {
        let new_row = start[0] as isize + vec[0];
        let new_col = start[1] as isize + vec[1];

        if [new_row, new_col]
            .into_iter()
            .zip([self.rows, self.cols])
            .all(|(coord, dim)| 0 <= coord && (coord as usize) < dim)
        {
            Ok([new_row as usize, new_col as usize])
        } else {
            Err([
                new_row.rem_euclid(self.rows as isize) as usize,
                new_col.rem_euclid(self.cols as isize) as usize,
            ])
        }
    }

    pub fn motion_wrap(&self, start: [usize; 2], vec: [isize; 2]) -> [usize; 2] {
        let new_row = (start[0] as isize + vec[0]).rem_euclid(self.rows as isize);
        let new_col = (start[1] as isize + vec[1]).rem_euclid(self.cols as isize);
        [new_row as usize, new_col as usize]
    }

    pub fn motions<'a>(
        &'a self,
        start: [usize; 2],
        vecs: impl 'a + IntoIterator<Item = [isize; 2]>,
    ) -> impl 'a + Iterator<Item = [usize; 2]> {
        vecs.into_iter()
            .filter_map(move |vec| self.motion(start, vec).ok())
    }

    pub fn iter(&self) -> impl '_ + Iterator<Item = ([usize; 2], &T)> {
        self.values
            .iter()
            .enumerate()
            .map(|(i, value)| ([i / self.cols, i % self.cols], value))
    }
}

impl<T> Index<[usize; 2]> for VMatrix<T> {
    type Output = T;

    fn index(&self, coord: [usize; 2]) -> &Self::Output {
        self.get(coord).expect("Invalid coords")
    }
}

impl<T> IndexMut<[usize; 2]> for VMatrix<T> {
    fn index_mut(&mut self, coord: [usize; 2]) -> &mut Self::Output {
        self.get_mut(coord).expect("Invalid coord")
    }
}

impl<T> Display for VMatrix<T>
where
    for<'a> &'a T: Into<char>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_display_simple(|_, value| value.into()).fmt(f)
    }
}
