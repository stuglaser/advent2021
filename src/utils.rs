use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pt {
    pub x: i32,
    pub y: i32,
}

impl Pt {
    pub fn at(x: i32, y: i32) -> Pt {
        Pt{x, y}
    }
}


pub struct Grid<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn filled(rows: usize, cols: usize, value: T) -> Grid<T>
        where T: Clone
    {
        Grid{rows: rows, cols: cols, data: vec![value; rows * cols]}
    }

    #[allow(dead_code)]
    pub fn fmt_compact(&self) -> String
        where T: std::fmt::Display
    {
        let mut out = String::with_capacity(self.rows * (self.cols + 1));
        for (idx, value) in self.data.iter().enumerate() {
            out.push_str(&format!("{}", value));
            if idx % self.cols == self.cols - 1 {
                out.push('\n');
            }
        }
        out
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, rowcol: (usize, usize)) -> &Self::Output {
        &self.data[rowcol.0 * self.cols + rowcol.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, rowcol: (usize, usize)) -> &mut Self::Output {
        &mut self.data[rowcol.0 * self.cols + rowcol.1]
    }
}

impl<T> Index<&Pt> for Grid<T> {
    type Output = T;

    fn index(&self, pt: &Pt) -> &Self::Output {
        &self.data[pt.y as usize * self.cols + pt.x as usize]
    }
}

impl<T> IndexMut<&Pt> for Grid<T> {
    fn index_mut(&mut self, pt: &Pt) -> &mut Self::Output {
        &mut self.data[pt.y as usize * self.cols + pt.x as usize]
    }
}