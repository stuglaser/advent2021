use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;

use clap::Parser;


#[derive(Parser)]
struct Opts {
    #[clap(short, default_value="1")]
    repeat: i32,
    day: Option<i32>,
    #[clap(long)]
    atleast: Option<f32>,
    #[clap(long)]
    per: bool,
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
    // for (a, b) in inputs.iter().zip(inputs.iter().skip(1)) {
    //     if b > a {
    //         cnt += 1;
    //     }
    // }
    

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
    day01,
];

fn main() {
    let opts = Opts::parse();
    assert!(opts.repeat == 1 || opts.atleast == None);
    println!("Hello, world!");
    match opts.day {
        Some(day) => println!("Day {}", day),
        None => println!("All days"),
    }

    if opts.per {
        // Benchmarks per-day.
        let atleast = opts.atleast.unwrap_or(0.5);
        let mut total = 0f64;
        for i in 0..DAYS.len() {
            let started = Instant::now();
            let mut samples = 0;
            while started.elapsed().as_secs_f32() < atleast {
                DAYS[i]();
                samples += 1;
            }
            let elapsed = started.elapsed();
            total += (elapsed / samples).as_secs_f64();
            println!("Day {:2} | {:?}  ({} samples)", i + 1, elapsed / samples, samples);
        }
        println!("Theoretical total: {}", total);
    } else {  // Benchmarks the total
        // Running one day or everything?
        let runner: Box<dyn Fn()> = match opts.day {
            Some(day) => Box::new(move || {
                DAYS[day as usize - 1]();
            }),
            None => Box::new(|| {
                for run_day in DAYS {
                    run_day();
                }
            })
        };

        let started = Instant::now();

        let mut repeated = 0u32;
        match opts.atleast {
            None =>
                for _ in 0..opts.repeat {
                    runner();
                    repeated += 1;
                },
            Some(atleast) => {
                while started.elapsed().as_secs_f32() < atleast {
                    runner();
                    repeated += 1;
                }
            },
        }

        let elapsed = started.elapsed();
        println!("Took {:?}  ({} samples)", elapsed / repeated, repeated);
        // let per_run_us = elapsed.as_micros() / opts.repeat as u128;
        // println!("Took {} us  {:?}", per_run_us, elapsed / opts.repeat as u32);
    }
}