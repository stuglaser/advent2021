use crate::utils::*;

use std::cmp::max;
use regex::Regex;


#[inline]
fn step(p: &mut Pt, v: &mut Pt) {
    p.x += v.x;
    p.y += v.y;
    v.x -= v.x.signum();
    v.y -= 1;
}

pub fn day17(test_mode: bool) {
    const INPUT: &str = "inputs/input17.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        "target area: x=20..30, y=-10..-5"
    } else {
        &file_str.trim_end()
    };

    let re_parse_input = Regex::new(r"target area: x=([-]?\d+)\.\.([-]?\d+), y=([-]?\d+)\.\.([-]?\d+)").unwrap();
    let captures = re_parse_input.captures(input_str).unwrap();
    let x0 = captures[1].parse::<i32>().unwrap();
    let x1 = captures[2].parse::<i32>().unwrap();
    let y0 = captures[3].parse::<i32>().unwrap();
    let y1 = captures[4].parse::<i32>().unwrap();

    // Pretty sure this makes the problem infeasible.
    if x0 * x1 < 0 { unimplemented!(); }
    if y0 * y1 < 0 { unimplemented!(); }

    // Sets limits on the space of possible velocities.
    let vx_abs_limit = max(x1, -x0) + 1;
    let vy_limit_lo = y0 - 1;  // Must at least hit the bottom of the box
    let vy_limit_hi = max(
        y1 + 1, // Can't skip the box top on the way up
        -y0 + 1); // Can't skip the box bottom while dropping
    // println!("Velocity sweep range: vx: [{}, {}]   vy: [{}, {}]", 1, vx_abs_limit, vy_limit_lo, vy_limit_hi);

    let mut best_y = 0;
    let mut total_hits = 0;
    for vx_abs in 1..=vx_abs_limit {
        let vx = vx_abs * x0.signum();

        for vy in vy_limit_lo..=vy_limit_hi {
            // Simulates until hit or miss
            let mut p = Pt::at(0, 0);
            let mut v = Pt::at(vx, vy);
            let mut sim_top_y = -999999;
            while v.y > 0 || p.y >= y0 {
                step(&mut p, &mut v);
                sim_top_y = max(sim_top_y, p.y);

                if x0 <= p.x && p.x <= x1 && y0 <= p.y && p.y <= y1 {
                    // Hit!
                    total_hits += 1;
                    best_y = max(best_y, sim_top_y);
                    break;
                }
            }

        }
    }
    let part1 = best_y;
    // println!("Part 1: {}", part1);
    assert_eq!(part1, if test_mode { 45 } else { 5886 });

    let part2 = total_hits;
    // println!("Part 2: {}", part2);
    assert_eq!(part2, if test_mode { 112 } else { 1806 });
}
