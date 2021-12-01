use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;

use clap::{AppSettings, Parser};


#[derive(Parser)]
struct Opts {
    #[clap(short, default_value="1")]
    repeat: i32,
    day: Option<i32>,
}


fn day01() {
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
    //println!("Day 1.  Part 1: {}", cnt);
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
    //println!("Day 1.  Part 2: {}", cnt);
    assert_eq!(cnt, 1150);
}

static DAYS: &'static [fn()] = &[
    day01
];

fn main() {
    let opts = Opts::parse();
    println!("Hello, world!");
    match opts.day {
        Some(day) => println!("Day {}", day),
        None => println!("All days"),
    }

    let started = Instant::now();

    for _ in 0..opts.repeat {
        match opts.day {
            Some(day) => {
                DAYS[day as usize - 1]();
            },
            None => {
                for run_day in DAYS {
                    run_day();
                }
            }
        }
    }

    let elapsed = started.elapsed();
    let per_run_us = elapsed.as_micros() / opts.repeat as u128;
    println!("Took {} us", per_run_us);
}