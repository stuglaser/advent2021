use crate::utils::*;

// fn neighbors(r: i64, c: i64) -> [(i64, i64); 9] {
//     [(r - 1, c - 1), (r - 1, c), (r - 1, c + 1),
//      (r, c - 1), (r, c), (r, c + 1),
//      (r + 1, c - 1), (r + 1, c), (r + 1, c + 1)]
// }

fn enhance(lookup: &[u8], grid: &Grid<u8>, assume_outside: u8) -> Grid<u8> {
    let mut data = Vec::<u8>::with_capacity((grid.rows + 2) * (grid.cols + 2));

    for r in 0..(grid.rows + 2) {
        for c in 0..(grid.cols + 2) {


            // Filling result[r,c] from grid[r-1, c-1]
            let mut index = 0usize;

            // Nicer, but slower by 1.5ms
            //
            // for (k, rc) in neighbors(r as i64, c as i64).iter().enumerate() {
            //     if rc.0 - 1 >= 0 && rc.0 - 1 < (grid.rows as i64) && rc.1 - 1 >= 0 && rc.1 - 1 < (grid.cols as i64) {
            //         index |= (grid[(rc.0 as usize - 1, rc.1 as usize - 1)] as usize) << (8 - k);
            //     } else {
            //         index |= (assume_outside as usize) << (8 - k);
            //     }
            // }

            // Top row
            if r > 1 {
                if c > 1 {
                    index |= (grid[(r - 2, c - 2)] as usize) << 8;
                } else {
                    index |= (assume_outside as usize) << 8;
                }
                if c > 0 && c - 1 < grid.cols {
                    index |= (grid[(r - 2, c - 1)] as usize) << 7;
                } else {
                    index |= (assume_outside as usize) << 7;
                }
                if c < grid.cols {
                    index |= (grid[(r - 2, c)] as usize) << 6;
                } else {
                    index |= (assume_outside as usize) << 6;
                }
            } else {
                index |= (assume_outside as usize) << 8;
                index |= (assume_outside as usize) << 7;
                index |= (assume_outside as usize) << 6;
            }
            // Middle row
            if r > 0 && r - 1 < grid.rows {
                if c > 1 {
                    index |= (grid[(r - 1, c - 2)] as usize) << 5;
                } else {
                    index |= (assume_outside as usize) << 5;
                }
                if c > 0 && c - 1 < grid.cols {
                    index |= (grid[(r - 1, c - 1)] as usize) << 4;
                } else {
                    index |= (assume_outside as usize) << 4;
                }
                if c < grid.cols {
                    index |= (grid[(r - 1, c)] as usize) << 3;
                } else {
                    index |= (assume_outside as usize) << 3;
                }
            } else {
                index |= (assume_outside as usize) << 5;
                index |= (assume_outside as usize) << 4;
                index |= (assume_outside as usize) << 3;
            }
            // Bottom row
            if r < grid.rows {
                if c > 1 {
                    index |= (grid[(r, c - 2)] as usize) << 2;
                } else {
                    index |= (assume_outside as usize) << 2;
                }
                if c > 0 && c - 1 < grid.cols {
                    index |= (grid[(r, c - 1)] as usize) << 1;
                } else {
                    index |= (assume_outside as usize) << 1;
                }
                if c < grid.cols {
                    index |= (grid[(r, c)] as usize) << 0;
                } else {
                    index |= (assume_outside as usize) << 0;
                }
            } else {
                index |= (assume_outside as usize) << 2;
                index |= (assume_outside as usize) << 1;
                index |= (assume_outside as usize) << 0;
            }

            data.push(lookup[index]);
        }
    }
    Grid{rows: grid.rows + 2, cols: grid.cols + 2, data}
}

#[allow(dead_code)]
fn pgrid(grid: &Grid<u8>) {
    let mut ch = Grid::<char>::filled(grid.rows, grid.cols, ' ');
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if grid[(r, c)] == 1 {
                ch[(r, c)] = '#';
            } else {
                ch[(r, c)] = '.';
            }
        }
    }
    println!("{}", ch.fmt_compact());
}

pub fn day20(test_mode: bool) {
    const INPUT: &str = "inputs/input20.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let (lookup, grid) = {
        let mut first_part = true;
        let mut lookup = Vec::new();
        let mut grid_data = Vec::<u8>::with_capacity(100*100);
        let mut grid_cols = 0;
        for line in input_str.lines() {
            if first_part {
                if line.is_empty() {
                    first_part = false;
                } else {
                    lookup = line.chars()
                        .map(|c| match c { '#' => 1, '.' => 0, _ => unimplemented!()})
                        .collect::<Vec<u8>>();
                }
            } else {
                grid_data.extend(line.chars()
                    .map(|c| match c { '#' => 1, '.' => 0, _ => unimplemented!()}));
                grid_cols += 1;
            }
        }
        (lookup, Grid{rows: grid_data.len() / grid_cols, cols: grid_cols, data: grid_data})
    };

    // Handles flipping patterns.
    let alt_outsides = if lookup[0] == 1 { 1 } else { 0 };

    let after1 = enhance(&lookup, &grid, 0);
    let after2 = enhance(&lookup, &after1, alt_outsides);
    let mut sum = 0usize;
    for x in after2.data {
        sum += x as usize;
    }

    let part1 = sum;
    // println!("Part 1: {}", part1);
    assert_eq!(part1, if test_mode { 35 } else { 5379 });

    let mut current = grid;
    for i in 0..50 {
        current = enhance(&lookup, &current, alt_outsides * (i % 2));
    }
    let mut sum = 0usize;
    for x in current.data {
        sum += x as usize;
    }

    let part2 = sum;
    // println!("Part 2: {}", part2);
    assert_eq!(part2, if test_mode { 3351 } else { 17917 });
}

const TEST_EXAMPLE: &'static str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";