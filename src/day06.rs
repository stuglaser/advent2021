//use std::collections::HashMap;
use rustc_hash::FxHashMap;

use crate::utils::*;

pub fn day06() {
    const INPUT: &str = "inputs/input06.txt";
    let input_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = input_str.trim_end();
    //let input_str = "3,4,3,1,2";

    let start: Vec<usize> = input_str.split(',').map(|s| s.parse::<usize>().unwrap()).collect();

    let mut current = vec![0usize; 9];
    for n in start {
        current[n] += 1;
    }
    let mut part1 = 0;
    for d in 1..=256 {
        let mut next = vec![0usize; 9];
        for (n, cnt) in current.iter().enumerate() {
            if n == 0 {
                next[6] += cnt;
                next[8] += cnt;
            } else {
                next[n - 1] += cnt;
            }
        }
        current = next;

        if d == 80 {
            part1 = current.iter().sum::<usize>();
        }
    }
    //println!("Part 1: {}", part1);
    assert_eq!(part1, 351188);
    let part2 = current.iter().sum::<usize>();
    //println!("Part 2: {}", part2);
    assert_eq!(part2, 1595779846729);
}
