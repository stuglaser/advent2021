use std::collections::BinaryHeap;

use crate::utils::*;

pub fn parse_digit_grid(string: &str) -> Grid<u8> {
    let mut grid_data = Vec::<u8>::with_capacity(1024);
    let mut grid_rows = 0usize;
    let mut grid_cols = 0usize;
    for line in string.lines() {
        let line = line.trim_start();
        grid_cols = line.len();
        grid_rows += 1;

        for ch in line.bytes() {
            grid_data.push(ch - b'0');
        }
    }
    assert_eq!(grid_data.len(), grid_rows * grid_cols);

    Grid{rows: grid_rows, cols: grid_cols, data: grid_data}
}


// Ordering by the first element of a tuple (in reverse)
#[repr(transparent)]
#[derive(Debug)]
pub struct ByFirstRev<T>(pub T);

impl<A: PartialEq, B> PartialEq for ByFirstRev<(A, B)> {
    fn eq(&self, other: &Self) -> bool {
        self.0.0 == other.0.0
    }
}
impl<A: Eq, B> Eq for ByFirstRev<(A, B)> {}

impl<A: PartialOrd, B> PartialOrd for ByFirstRev<(A, B)> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.0.partial_cmp(&self.0.0)
    }
}

impl<A: Ord, B> Ord for ByFirstRev<(A, B)> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.0.cmp(&self.0.0)
    }
}


fn find_min_risk(grid: &Grid<u8>) -> usize {
    let mut visited = Grid{rows: grid.rows, cols: grid.cols, data: vec![false; grid.data.len()]};

    let mut heap = BinaryHeap::with_capacity(grid.rows + grid.cols);
    heap.push( ByFirstRev((0i32, Pt{x: 0, y: 0})));
    loop {
        let ByFirstRev((score, at)) = heap.pop().unwrap();

        if visited[&at] { continue; }
        visited[&at] = true;

        if (at.x as usize) == grid.cols - 1 && (at.y as usize) == grid.rows - 1 {
            // FINISHED
            return score as usize;
        }

        // Walks the neighbors
        if at.x > 0 {
            let n = Pt{x: at.x - 1, y: at.y};
            if !visited[&n] {
                heap.push(ByFirstRev((score + grid[&n] as i32, n)));
            }
        }
        if at.y > 0 {
            let n = Pt{x: at.x, y: at.y - 1};
            if !visited[&n] {
                heap.push(ByFirstRev((score + grid[&n] as i32, n)));
            }
        }
        if (at.x as usize) < grid.cols - 1 {
            let n = Pt{x: at.x + 1, y: at.y};
            if !visited[&n] {
                heap.push(ByFirstRev((score + grid[&n] as i32, n)));
            }
        }
        if (at.y as usize) < grid.rows - 1 {
            let n = Pt{x: at.x, y: at.y + 1};
            if !visited[&n] {
                heap.push(ByFirstRev((score + grid[&n] as i32, n)));
            }
        }
    }
}

#[inline]
fn wrap(risk: u8) -> u8 {
    (risk - 1) % 9 + 1
}

pub fn day15(test_mode: bool) {
    const INPUT: &str = "inputs/input15.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
    } else {
        &file_str
    };

    let grid = parse_digit_grid(input_str);

    let part1 = find_min_risk(&grid);
    // println!("Part 1: {}", part1);
    assert_eq!(part1, if test_mode { 40 } else { 458 });


    // Expand

    let mut big_grid = Grid::filled(grid.rows * 5, grid.cols * 5, 0u8);
    for r in 0..big_grid.rows {
        for c in 0..big_grid.cols {
            let offset = (r / grid.rows + c / grid.cols) as u8;
            big_grid[(r, c)] = wrap(grid[(r % grid.rows, c % grid.cols)] + offset);
        }
    }
    // println!("Big grid:\n\n{}", big_grid.fmt_compact());

    let part2 = find_min_risk(&big_grid);
    // println!("Part 2: {}", part2);
    assert_eq!(part2, if test_mode { 315 } else { 2800 });
}
