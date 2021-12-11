use crate::utils::*;


fn lookup_canonical_and_collapse(collapsing: &mut [i32], idx: i32) -> i32 {
    if idx < 0 {
        idx
    } else if collapsing[idx as usize] == idx {
        idx
    } else {
        let base = lookup_canonical_and_collapse(collapsing, collapsing[idx as usize]);
        collapsing[idx as usize] = base;
        base
    }
}

pub fn day09(test_mode: bool) {
    const INPUT: &str = "inputs/input09.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        "2199943210
3987894921
9856789892
8767896789
9899965678"
    } else {
        file_str.trim_end()
    };
    
    let mut grid_data = Vec::<u8>::with_capacity(50 * 50);
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

    let grid = Grid{rows: grid_rows, cols: grid_cols, data: grid_data};

    let mut part1: usize = 0;
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            let value = grid[(r, c)];
            let is_min =
                (r == 0 || grid[(r - 1, c)] > value) &&
                (c == 0 || grid[(r, c - 1)] > value) &&
                (r + 1 == grid.rows || grid[(r + 1, c)] > value) &&
                (c + 1 == grid.cols || grid[(r, c + 1)] > value);
            if is_min {
                part1 += (value + 1) as usize;
            }
        }
    }
    // println!("Part 1: {}", part1);
    assert_eq!(part1, if test_mode { 15 } else { 548 });

    // Really 9's are just the boundaries, and every other value is the same.

    let mut basin_sizes = Vec::<usize>::with_capacity(2 * grid.rows);
    let mut basin_collapses_to = Vec::<i32>::with_capacity(2 * grid.rows);

    let mut last_basins: Vec<i32> = vec![-1; grid.cols];
    let mut next_basin_id = 0i32;

    for r in 0..grid.rows {
        let mut current_basins = vec![-1; grid.cols];
        for c in 0..grid.cols {
            if grid[(r, c)] == 9 { continue; }
            let basin_above = lookup_canonical_and_collapse(&mut basin_collapses_to, last_basins[c]);

            if c > 0 && current_basins[c - 1] >= 0 {
                // Continues the horizontal basin.
                let basin = current_basins[c - 1];
                basin_sizes[basin as usize] += 1;
                current_basins[c] = basin;

                // Checks the vertical basin.
                if basin_above >= 0 && basin_above != basin {
                    // Collapse
                    basin_collapses_to[basin_above as usize] = basin;
                    basin_sizes[basin as usize] += basin_sizes[basin_above as usize];
                }
            } else if basin_above >= 0 {
                // Continues the vertical basin.
                basin_sizes[basin_above as usize] += 1;
                current_basins[c] = basin_above;
            } else {
                assert_eq!(basin_sizes.len(), next_basin_id as usize); assert_eq!(basin_collapses_to.len(), next_basin_id as usize);
                // New basin!
                basin_sizes.push(1);
                basin_collapses_to.push(next_basin_id);
                current_basins[c] = next_basin_id;
                next_basin_id += 1;
            }
        }

        last_basins = current_basins;
    }

    let mut top3 = vec![0; 4];
    for basin in 0..basin_sizes.len() {
        if basin_collapses_to[basin] == basin as i32 {
            top3[0] = basin_sizes[basin];
            top3.sort();
        }
    }
    let part2 = top3[1] * top3[2] * top3[3];
    // println!("Part 2: {}", part2);
    assert_eq!(part2, if test_mode { 1134 } else { 786048 });
}
