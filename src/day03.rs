use crate::utils::*;


pub fn day03(_test_mode: bool) {
    const INPUT: &str = "inputs/input03.txt";
    let input_str = std::fs::read_to_string(INPUT).unwrap();

    // Constructs the grid manually so we can stream it in.
    let mut grid_data = Vec::<u8>::with_capacity(1001 * 12);
    let mut grid_rows = 0;
    for ch in input_str.bytes() {
        match ch {
            b'0' => grid_data.push(0),
            b'1' => grid_data.push(1),
            b'\n' => grid_rows += 1,
            _ => unreachable!(),
        }
    }

    // Slower, unfortunately. Probably because our capacity guess is better than collect's
    // let grid_data: Vec<u8> = input_str.bytes()
    //     .filter_map(|b| match b {
    //         b'0' => Some(0),
    //         b'1' => Some(1),
    //         b'\n' => { grid_rows += 1; None},
    //         _ => unreachable!(),
    //     }).collect();

    let grid = Grid::<u8>{rows: grid_rows, cols: grid_data.len() / grid_rows, data: grid_data};


    let mut cnt_ones = vec![0; grid.cols];
    for row in 0..grid.rows {
        for k in 0..grid.cols {
            if grid[(row, k)] == 1 {
                cnt_ones[k] += 1;
            }
        }
    }

    let mut epsilon = 0;
    let mut gamma = 0;
    for k in 0..cnt_ones.len() {
        gamma <<= 1;
        epsilon <<= 1;
        if cnt_ones[k] > grid.rows / 2 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    //println!("Part 1: {}  (because {:?})", gamma * epsilon, cnt_ones);
    assert_eq!(gamma * epsilon, 3959450);

    // Oxygen
    let mut keep = vec![true; grid.rows];
    let mut value: i32 = if cnt_ones[0] > grid.rows / 2 { 1 } else { 0 };
    for k in 1..cnt_ones.len() {
        let mut next_ones: usize = 0;
        let mut next_total: usize = 0;
        for row in 0..grid.rows {
            if keep[row] {
                if grid[(row, k - 1)] == (value & 0b1) as u8 {
                    // Keep keeping, and count this one
                    next_ones += grid[(row, k)] as usize;
                    next_total += 1;
                } else {
                    keep[row] = false;
                }
            }
        }

        value <<= 1;
        if next_ones * 2 >= next_total { value += 1; }
    }
    let oxygen = value;


    // CO2
    let mut keep = vec![true; grid.rows];
    let mut value: i32 = if !(cnt_ones[0] > grid.rows / 2) { 1 } else { 0 };
    for k in 1..cnt_ones.len() {
        let mut next_ones: usize = 0;
        let mut next_total: usize = 0;
        let mut kept_row = 0;
        for row in 0..grid.rows {
            if keep[row] {
                if grid[(row, k - 1)] == (value & 0b1) as u8 {
                    // Keep keeping, and count this one
                    next_ones += grid[(row, k)] as usize;
                    next_total += 1;
                    kept_row = row;
                } else {
                    keep[row] = false;
                }
            }
        }

        // Early exit if there's one value left
        if next_total == 1 {
            value = 0;
            for c in 0..grid.cols {
                value <<= 1;
                value += grid[(kept_row, c)] as i32;
            }
            break;
        }

        value <<= 1;
        if next_ones * 2 < next_total { value += 1; }
    }
    let co2 = value;

    //println!("Part 2: {}", oxygen * co2);
    assert_eq!(oxygen * co2, 7440311);
}
