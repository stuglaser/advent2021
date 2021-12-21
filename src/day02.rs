use std::io::{BufRead, BufReader};
use std::fs::File;


pub fn day02(_test_mode: bool, print: bool) {
    let reader = BufReader::new(File::open("inputs/input02.txt").expect("Cannot open file"));
    enum Dir {
        Forward, Up, Down
    }
    let mut inputs = Vec::<(Dir, i32)>::with_capacity(100);
    for line in reader.lines() {
        let saved_line = line.unwrap();
        let (dir_str, dist_str) = saved_line.split_once(" ").unwrap();
        let dir = match dir_str.chars().next().unwrap() {
            'f' => Dir::Forward,
            'u' => Dir::Up,
            'd' => Dir::Down,
            _ => panic!("Bad direction"),
        };
        let dist: i32 = dist_str.parse().unwrap();
        inputs.push((dir, dist));
    }

    let mut x = 0;
    let mut depth = 0; 
    for (dir, dist) in &inputs {
        match dir {
            Dir::Forward => x += dist,
            Dir::Up => depth -= dist,
            Dir::Down => depth += dist,
        }
    }

    if print {
        println!("Part 1: {}", x * depth);
    }
    assert_eq!(x * depth, 1670340);

    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (dir, dist) in inputs {
        match dir {
            Dir::Forward => { x += dist; depth += dist * aim; }
            Dir::Up => aim -= dist,
            Dir::Down => aim += dist,
        }
    }
    if print {
        println!("Part 2: {}", x * depth);
    }
    assert_eq!(x * depth, 1954293920);
}
