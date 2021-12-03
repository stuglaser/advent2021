use std::ops::Index;

pub struct Grid<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T> Grid<T> {

}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, rowcol: (usize, usize)) -> &Self::Output {
        &self.data[rowcol.0 * self.cols + rowcol.1]
    }

}
