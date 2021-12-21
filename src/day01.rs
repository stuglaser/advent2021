use std::io::{BufRead, BufReader};
use std::fs::File;


pub fn day01(_test_mode: bool, print: bool) {
    let reader = BufReader::new(File::open("inputs/input01.txt").expect("Cannot open file"));
    let mut inputs = Vec::<i32>::with_capacity(100);
    for line in reader.lines() {
        let x: i32 = line.unwrap().parse().unwrap();
        inputs.push(x);
    }

    let mut cnt = 0;
    for i in 1..inputs.len() {
        if inputs[i] > inputs[i - 1] {
            cnt += 1;
        }
    }
    // for (a, b) in inputs.iter().zip(inputs.iter().skip(1)) {
    //     if b > a {
    //         cnt += 1;
    //     }
    // }
    

    if print {
        println!("Day 1.  Part 1: {}", cnt);
    }
    assert_eq!(cnt, 1215);

    let mut cnt = 0;
    for i in 0..(inputs.len() - 2 - 1) {
        // Comparing windows
        //  i, i+1, i+2
        //     i+1, i+2, i+3
        if inputs[i + 3] > inputs[i] {
            cnt += 1;
        }
    }
    if print {
        println!("Day 1.  Part 2: {}", cnt);
    }
    assert_eq!(cnt, 1150);
}
