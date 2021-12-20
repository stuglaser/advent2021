use std::{ops::{Index, IndexMut}, cmp::max};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pt3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Pt3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self{x, y, z}
    }

    pub fn dist_to(&self, p: &Pt3) -> f32 {
        (((self.x - p.x).pow(2) +
          (self.y - p.y).pow(2) +
          (self.z - p.z).pow(2)) as f32).sqrt()
    }

    pub fn dist_to_sqr(&self, p: &Pt3) -> i32 {
        (self.x - p.x).pow(2) +
        (self.y - p.y).pow(2) +
        (self.z - p.z).pow(2)
    }

    pub fn l1_to(&self, p: &Pt3) -> i32 {
        (self.x - p.x).abs() +
        (self.y - p.y).abs() +
        (self.z - p.z).abs()
    }
}

impl std::fmt::Display for Pt3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

impl std::ops::Add for &Pt3 {
    type Output = Pt3;

    fn add(self, rhs: Self) -> Self::Output {
        Pt3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for &Pt3 {
    type Output = Pt3;

    fn sub(self, rhs: Self) -> Self::Output {
        Pt3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}


#[derive(Clone)]
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

    #[allow(dead_code)]
    pub fn as_strings(&self) -> Grid<String>
        where T: std::fmt::Display
    {
        let mut table = Grid::<String>::filled(self.rows, self.cols, "".to_string());
        for r in 0..self.rows {
            for c in 0..self.cols {
                table[(r, c)] = format!("{}", self[(r, c)]);
            }
        }
        table
    }

    #[allow(dead_code)]
    pub fn fmt_table(&self) -> String
        where T: std::fmt::Display
    {
        let table = self.as_strings();
        tabulate(&table)
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

#[allow(dead_code)]
pub fn tabulate(table: &Grid<String>) -> String {
    let mut lengths = vec![0; table.cols];
    for i in 0..table.data.len() {
        let c = i % table.cols;
        lengths[c] = max(lengths[c], table.data[i].len());
        // TODO: ^^ Not correct for unicode.
    }

    let mut out = String::with_capacity(table.rows * (lengths.iter().sum::<usize>() + table.cols));
    for i in 0..table.data.len() {
        let c = i % table.cols;
        out.push_str(&str::repeat(" ", lengths[c] - table.data[i].len()));
        out.push_str(&table.data[i]);
        if c == table.cols - 1 {
            out.push('\n');
        } else {
            out.push(' ');
        }
    }
    out
}