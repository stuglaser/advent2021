use crate::utils::*;

fn step(grid: &mut Grid<u8>) -> usize {
    let mut toflash = Vec::<(usize, usize)>::with_capacity(100);
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            grid[(r, c)] += 1;
            if grid[(r, c)] == 10 {
                toflash.push((r, c));
            }
        }
    }

    let mut i = 0;
    while i < toflash.len() {
        let (r, c) = toflash[i];

        let r_lo = if r == 0 { 0 } else { r - 1 };
        let r_hi = if r + 1 == grid.rows { r } else { r + 1 };
        let c_lo = if c == 0 { 0 } else { c - 1 };
        let c_hi = if c + 1 == grid.cols { c } else { c + 1 };
        for xr in r_lo..=r_hi {
            for xc in c_lo..=c_hi {
                if xr == r && xc == c { continue; }

                grid[(xr, xc)] += 1;
                if grid[(xr, xc)] == 10 {
                    toflash.push((xr, xc));
                }
            }
        }
        i += 1;
    }

    let flashes = toflash.len();
    for (r, c) in toflash {
        grid[(r, c)] = 0;
    }

    flashes
}

pub fn day11(test_mode: bool, print: bool) {
    const INPUT: &str = "inputs/input11.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
    } else {
        &file_str
    };

    let mut grid_data = Vec::<u8>::with_capacity(10 * 10);
    let mut grid_rows = 0usize;
    let mut grid_cols = 0usize;
    for line in input_str.lines() {
        let line = line.trim_start();
        grid_cols = line.len();
        grid_rows += 1;

        for ch in line.bytes() {
            grid_data.push(ch - b'0');
        }
    }

    let mut grid = Grid{rows: grid_rows, cols: grid_cols, data: grid_data};

    let mut flashes_100 = 0;
    let mut first_sync = 0;
    for s in 1..1000 {
        let step_flashes = step(&mut grid);
        if s <= 100 {
            flashes_100 += step_flashes;
        }

        if step_flashes == grid.rows * grid.cols {
            first_sync = s;
            break;
        }
    }

    let part1 = flashes_100;
    if print { println!("Part 1: {}", part1); }
    assert_eq!(part1, if test_mode { 1656 } else { 1617 });

    let part2 = first_sync;
    if print { println!("Part 2: {}", part2); }
    assert_eq!(part2, if test_mode { 195 } else { 258 });
}
