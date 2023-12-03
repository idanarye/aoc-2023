use std::fmt::{Display, Write};
use std::ops::Index;

#[derive(Debug)]
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

    pub fn get(&self, index: (usize, usize)) -> Option<&T> {
        let (row, col) = index;
        if row < self.rows && col < self.cols {
            Some(&self.values[row * self.cols + col])
        } else {
            None
        }
    }
}

impl<T> Index<(usize, usize)> for VMatrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index).expect("Invalid index")
    }
}
