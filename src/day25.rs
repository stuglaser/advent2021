use crate::utils::*;

const FREE: u8 = b'.';

fn step(grid: &mut Grid<u8>) -> bool {
    let mut any_moved = false;

    // East-facing
    for r in 0..grid.rows {
        let wrap_move = grid[(r, 0)] == FREE && grid[(r, grid.cols - 1)] == b'>';

        let mut moved = false;
        for c in 0..(grid.cols - 1) {
            if !moved && grid[(r, c)] == b'>' && grid[(r, c + 1)] == FREE {
                grid[(r, c)] = FREE;
                grid[(r, c + 1)] = b'>';
                any_moved = true;
                moved = true;
            } else {
                moved = false;
            }
        }

        if wrap_move {
            let cols = grid.cols;
            grid[(r, cols - 1)] = FREE;
            grid[(r, 0)] = b'>';
            any_moved = true;
        }
    }

    // South-facing
    for c in 0..grid.cols {
        let wrap_move = grid[(0, c)] == FREE && grid[(grid.rows - 1, c)] == b'v';

        let mut moved = false;
        for r in 0..(grid.rows - 1) {
            if !moved && grid[(r, c)] == b'v' && grid[(r + 1, c)] == FREE {
                grid[(r, c)] = FREE;
                grid[(r + 1, c)] = b'v';
                moved = true;
                any_moved = true;
            } else {
                moved = false;
            }
        }

        if wrap_move {
            let rows = grid.rows;
            grid[(rows - 1, c)] = FREE;
            grid[(0, c)] = b'v';
            any_moved = true;
        }
    }
    any_moved
}

pub fn day25(test_mode: bool, print: bool) {
    const INPUT: &str = "inputs/input25.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };


    let mut grid = {
        let mut grid_data = Vec::<u8>::with_capacity(100*100);
        let mut grid_rows = 0;
        for line in input_str.lines() {
            grid_data.extend(line.as_bytes());
            grid_rows += 1;
        }
        Grid{rows: grid_rows, cols: grid_data.len() / grid_rows, data: grid_data}
    };

    let mut steps_until_fixed = 0;
    for i in 1..1000 {
        if !step(&mut grid) {
            steps_until_fixed = i;
            break;
        }
        // println!("{}:\n{}", i, grid.fmt_map());
    }

    let part1 = steps_until_fixed;
    if print { println!("Part 1: {}", part1); }
    assert_eq!(part1, if test_mode { 58 } else { 417 });
}

const TEST_EXAMPLE: &'static str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";