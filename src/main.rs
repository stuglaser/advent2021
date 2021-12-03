use std::io::{BufReader, BufRead};
use std::fs::File;
use std::ops::Index;
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

fn day02() {
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

    //println!("Part 1: {}", x * depth);
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
    //println!("Part 2: {}", x * depth);
    assert_eq!(x * depth, 1954293920);
}


struct Grid<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {

}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, rowcol: (usize, usize)) -> &Self::Output {
        &self.data[rowcol.0 * self.cols + rowcol.1]
    }

}

fn day03() {
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

static DAYS: &'static [fn()] = &[
    day01,
    day02,
    day03,
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
        println!("Theoretical total: {} ms", total * 1000.0);
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