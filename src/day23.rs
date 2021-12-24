use crate::utils::*;

use std::cmp::{min, max};

const R_HALL: usize = 1;

#[inline]
fn can_stop(r: usize) -> bool {
    r % 2 == 0 ||
    r < 3 || r > 9
}

fn room_space(map: &Grid<u8>, room: u8) -> Option<usize> {
    let mut r = map.rows - 2;
    while r > R_HALL {
        let ch = map[(r, room as usize * 2 + 3)];
        if ch == 0 {
            return Some(r);
        }
        if ch != (room + b'A') {
            return None;
        }

        r -= 1;
    }
    None
}

// Steps to leave the room, or 0 if not leavable
fn room_leavable(map: &Grid<u8>, room:u8, r: usize) -> usize {
    let r_hall = 1;
    let col = room as usize * 2 + 3;

    // Is this already placed correctly?
    let mut correctly_placed = true;
    for row in r..(map.rows - 1) {
        if map[(row, col)] != (room + b'A') {
            correctly_placed = false;
            break;
        }
    }
    if correctly_placed {
        return 0;
    }

    if map[(r - 1, col)] == 0 {
        r - r_hall
    } else {
        0
    }
}

fn hall_spots(map: &Grid<u8>, col: usize) -> Vec<usize> {
    let mut stoppable = Vec::with_capacity(7);
    let mut cleft = col - 1;
    while cleft > 0 {
        if map[(R_HALL, cleft)] != 0 {
            break;
        }
        if can_stop(cleft) {
            stoppable.push(cleft);
        }
        cleft -= 1;
    }

    let mut cright = col + 1;
    while cright < 12 {
        if map[(R_HALL, cright)] != 0 {
            break;
        }
        if can_stop(cright) {
            stoppable.push(cright);
        }
        cright += 1;
    }
    stoppable
}

fn solve(map: &mut Grid<u8>, locs: &mut [(usize, usize)]) -> usize {
    let mut best_cost = usize::MAX;

    // println!("SOLVE: {:?}\n{}", locs, map.fmt_map());

    // Checks for being finished
    let mut finished = true;
    for room in 0..4u8 {
        for i in (R_HALL + 1)..(map.rows - 1) {
            if map[(i, room as usize * 2 + 3)] != (b'A' + room) {
                finished = false;
                break;
            }
        }
    }
    if finished { return 0; }

    // First we do placements, since placements are always good.
    for i in 0..locs.len() {
        let loc = locs[i];
        let ch = map[loc];
        let per_step_cost = 10usize.pow((ch - b'A').into());

        if loc.0 == R_HALL {
            // Hallway to room
            let goal_room = ch - b'A';
            let goal_col = (goal_room * 2 + 3) as usize;
            if let Some(space) = room_space(map, goal_room) {
                // println!("Space for {} at ({}, {})", ch as char, space, goal_col);
                let mut reachable = true;

                let mut c = goal_col;
                let dir = if loc.1 < goal_col { -1 } else { 1 as i32 };
                while c != loc.1 {
                    if map[(R_HALL, c)] != 0 {
                        reachable = false;
                        break;
                    }
                    c = (c as i32 + dir) as usize;
                }
                if reachable {
                    // println!("Placed {} -> room {}", ch as char, space);
                    map[(space, goal_col)] = ch;
                    map[loc] = 0;
                    locs[i] = (space, goal_col);
                    let hall_steps = if loc.1 > goal_col { loc.1 - goal_col } else { goal_col - loc.1 };
                    let move_cost = (space - R_HALL + hall_steps) * per_step_cost;
                    let beyond_cost = solve(map, locs);
                    locs[i] = loc;
                    map[loc] = ch;
                    map[(space, goal_col)] = 0;

                    // Literally any placement is worthwhile.
                    if beyond_cost < usize::MAX {
                        return move_cost + beyond_cost;
                    }
                }
            }
        }
    }


    // Then we do moves to the hallway.
    for i in 0..locs.len() {
        let loc = locs[i];
        let ch = map[loc];
        let per_step_cost = 10usize.pow((ch - b'A').into());

        if loc.0 == R_HALL {
        } else {
            // Room to hallway
            let room = ((loc.1 - 3) / 2) as u8;
            // println!("Room leavable? {} {:?}", room, loc);
            let steps_out = room_leavable(map, room, loc.0);
            if steps_out > 0 {
                // println!("Move {} from room {}", ch as char, room);

                // Possible hallway spots to move to
                for c in hall_spots(map, loc.1) {
                    // println!("  -> {}", c);

                    map[(R_HALL, c)] = ch;
                    map[loc] = 0;
                    locs[i] = (R_HALL, c);
                    let move_cost = (steps_out + (c as i32 - loc.1 as i32).abs() as usize) * per_step_cost;
                    let beyond_cost = solve(map, locs);
                    if beyond_cost < usize::MAX {
                        best_cost = min(best_cost, move_cost + beyond_cost);
                    }
                    locs[i] = loc;
                    map[loc] = ch;
                    map[(R_HALL, c)] = 0;
                }
            }
        }
    }

    best_cost
}

fn parse_input(input_str: &str) -> (Grid<u8>, Vec<(usize, usize)>) {
    let mut grid = Grid::<u8>::filled(7, 13, 0);
    let mut locs = Vec::with_capacity(16);
    for (r, line) in input_str.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            let ch = ch as u8;
            grid[(r, c)] = if ch == b'.' { 0 } else { ch };
            if b'A' <= ch && ch <= b'D' {
                // let p = (ch - b'A') * 2;
                // if locs[p as usize] == (0, 0) {
                //     locs[p as usize] = (r, c);
                // } else {
                //     locs[(p + 1) as usize] = (r, c);
                // }
                locs.push((r, c));
            }
        }
    }
    (grid, locs)
}

pub fn day23(test_mode: bool, print: bool) {
    const INPUT: &str = "inputs/input23.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let mut start_grid = Grid::<u8>::filled(5, 13, 0);
    let mut start_locs = [(0usize, 0usize); 8];
    for (r, line) in input_str.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            let ch = ch as u8;
            start_grid[(r, c)] = if ch == b'.' { 0 } else { ch };
            if b'A' <= ch && ch <= b'D' {
                let p = (ch - b'A') * 2;
                if start_locs[p as usize] == (0, 0) {
                    start_locs[p as usize] = (r, c);
                } else {
                    start_locs[(p + 1) as usize] = (r, c);
                }
            }
        }
    }
    
    println!("Start locs: {:?}", start_locs);
    println!("Start grid:\n{}", start_grid.fmt_map());

    let part1 = solve(&mut start_grid, &mut start_locs);
    println!("Should show {} but show is {}", part1, print);
    if print { println!("Part 1: {}", part1); }
    assert_eq!(part1, if test_mode { 12521 } else { 14371 });

    let mut ext_input = String::with_capacity(input_str.len() + (start_grid.cols + 1) * 2);
    for (i, line) in input_str.lines().enumerate() {
        ext_input.push_str(line);
        ext_input.push('\n');
        if i == 2 {
            ext_input.push_str(EXTENSION);
        }
    }
    println!("Extended:\n{}", ext_input);

    let (mut ext_grid, mut ext_locs) = parse_input(&ext_input);
    println!("Extended: {:?}\n{}", ext_locs, ext_grid.fmt_map());


    let part2 = solve(&mut ext_grid, &mut ext_locs);
    if print { println!("Part 2: {}", part2); }
    assert_eq!(part2, if test_mode { 44169 } else { 40941 });
}

const TEST_EXAMPLE: &'static str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

const EXTENSION: &'static str ="  #D#C#B#A#
  #D#B#A#C#
";