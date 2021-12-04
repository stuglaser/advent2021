//use std::collections::HashMap;
use rustc_hash::FxHashMap;

use crate::utils::*;


pub fn day04() {
    const INPUT: &str = "inputs/input04.txt";
    let input_str = std::fs::read_to_string(INPUT).unwrap();

    let mut parts = input_str.split("\n\n");

    let called: Vec<i32> = parts.next().unwrap().split(",")
        .map(|s| s.parse::<i32>().unwrap()).collect();

    // Parses the boards
    let mut board_lookup = FxHashMap::<i32, Vec<(usize, usize, usize)>>::default();

    let mut board_sums = Vec::<usize>::with_capacity(20);
    for (b, board_str) in parts.enumerate() {
        let mut board_sum = 0usize;
        for (r, row) in board_str.split("\n").enumerate() {
            for (c, num_str) in row.split_ascii_whitespace().enumerate() {
                let num = num_str.parse::<i32>().unwrap();
                board_sum += num as usize;
                board_lookup.entry(num)
                    .or_insert(Vec::with_capacity(8))
                    .push((b, r, c));
            }
        }
        board_sums.push(board_sum);
    }

    const K: usize = 5;

    // board_rows[board, row] = how many numbers are marked in that row
    let mut board_rows = Grid{rows: board_sums.len(), cols: K, data: vec![0; board_sums.len() * K]};
    let mut board_cols = Grid{rows: board_sums.len(), cols: K, data: vec![0; board_sums.len() * K]};

    let mut board_won = vec![false; board_sums.len()];
    let mut num_won = 0;
    let mut first_score = 0;
    let mut last_score = 0;
    for called_num in called {
        for (b, r, c) in board_lookup.get(&called_num).unwrap() {
            board_rows[(*b, *r)] += 1;
            board_cols[(*b, *c)] += 1;
            board_sums[*b] -= called_num as usize;

            if !board_won[*b] && (board_rows[(*b, *r)] == K || board_cols[(*b, *c)] == K) {
                board_won[*b] = true;
                num_won += 1;
                let score = board_sums[*b] * called_num as usize;
                if num_won == 1 {
                    first_score = score;
                }
                last_score = score;
            }
        }
    }
    //println!("Answers: {} {}", first_score, last_score);
    assert_eq!(first_score, 27027);
    assert_eq!(last_score, 36975);
}
