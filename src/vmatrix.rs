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
    pub fn new(rows: usize, cols: usize, mut fill: impl FnMut((usize, usize)) -> T) -> Self {
        Self {
            cols,
            rows,
            values: (0..(rows * cols))
                .map(|index| fill((index / cols, index % cols)))
                .collect(),
        }
    }

    pub fn from_chars(input: &str, mut mapper: impl FnMut(char) -> T) -> Self {
        input
            .chars()
            .map(|ch| if ch == '\n' { None } else { Some(mapper(ch)) })
            .collect()
    }

    pub fn map<S>(self, mut dlg: impl FnMut(usize, T) -> S) -> VMatrix<S> {
        VMatrix {
            cols: self.cols,
            rows: self.rows,
            values: self
                .values
                .into_iter()
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
        value_formatter: impl 'a + Fn(&T) -> char,
    ) -> impl 'a + Display {
        self.to_display(move |fmt, _, value| fmt.write_char(value_formatter(value)))
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

    pub fn coord_to_index(&self, (row, col): (usize, usize)) -> Option<usize> {
        if row < self.rows && col < self.cols {
            Some(row * self.cols + col)
        } else {
            None
        }
    }

    pub fn index_to_coord(&self, index: usize) -> Option<(usize, usize)> {
        if index < self.values.len() {
            Some((index / self.cols, index % self.cols))
        } else {
            None
        }
    }

    pub fn get(&self, coord: (usize, usize)) -> Option<&T> {
        self.coord_to_index(coord).map(|i| &self.values[i])
    }

    pub fn get_mut(&mut self, coord: (usize, usize)) -> Option<&mut T> {
        self.coord_to_index(coord).map(|i| &mut self.values[i])
    }

    pub fn motion(&self, start: (usize, usize), vec: (isize, isize)) -> Option<(usize, usize)> {
        let new_row = usize::try_from(start.0 as isize + vec.0).ok()?;
        if self.rows <= new_row {
            return None;
        }
        let new_col = usize::try_from(start.1 as isize + vec.1).ok()?;
        if self.cols <= new_col {
            return None;
        }
        Some((new_row, new_col))
    }

    pub fn motions<'a>(
        &'a self,
        start: (usize, usize),
        vecs: impl 'a + IntoIterator<Item = (isize, isize)>,
    ) -> impl 'a + Iterator<Item = (usize, usize)> {
        vecs.into_iter()
            .filter_map(move |vec| self.motion(start, vec))
    }

    pub fn iter(&self) -> impl '_ + Iterator<Item = ((usize, usize), &T)> {
        self.values
            .iter()
            .enumerate()
            .map(|(i, value)| ((i / self.cols, i % self.cols), value))
    }
}

impl<T> Index<(usize, usize)> for VMatrix<T> {
    type Output = T;

    fn index(&self, coord: (usize, usize)) -> &Self::Output {
        self.get(coord).expect("Invalid coords")
    }
}

impl<T> IndexMut<(usize, usize)> for VMatrix<T> {
    fn index_mut(&mut self, coord: (usize, usize)) -> &mut Self::Output {
        self.get_mut(coord).expect("Invalid coord")
    }
}

impl<T> Display for VMatrix<T>
where
    for<'a> &'a T: Into<char>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_display_simple(|value| value.into()).fmt(f)
    }
}
