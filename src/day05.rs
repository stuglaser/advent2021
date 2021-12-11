use regex::Regex;
//use std::collections::HashMap;
use rustc_hash::FxHashMap;

#[derive(Debug)]
struct Line {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

struct AlongIterator {
    //a: i32,
    next: i32,
    end: i32,
    step: i32,
}

impl Iterator for AlongIterator {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.step > 0 {
            if self.next <= self.end {
                self.next += self.step;
                Some(self.next - self.step)
            } else {
                None
            }
        } else {
            if self.next >= self.end {
                self.next += self.step;
                Some(self.next - self.step)
            } else {
                None
            }
        }
    }
}
fn iter_along(a: i32, b: i32) -> AlongIterator {
    AlongIterator{next: a, end: b, step: (b - a).signum()}
}

pub fn day05(_test_mode: bool) {
    const INPUT: &str = "inputs/input05.txt";
    let input_str = std::fs::read_to_string(INPUT).unwrap();

    let re_parse_line = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)\n").unwrap();

    let mut lines = Vec::<Line>::with_capacity(1000);
    for cap in re_parse_line.captures_iter(&input_str) {
        lines.push(Line{
            x0: cap[1].parse().unwrap(),
            y0: cap[2].parse().unwrap(),
            x1: cap[3].parse().unwrap(),
            y1: cap[4].parse().unwrap(),
        });
    }

    let mut horvert_locs = FxHashMap::<(i32, i32), i32>::default();
    let mut locs = FxHashMap::<(i32, i32), i32>::default();
    locs.reserve(100000);
    horvert_locs.reserve(100000);
    for line in lines {
        if line.x0 == line.x1 {
            for y in iter_along(line.y0, line.y1) {
                *horvert_locs.entry((line.x0, y)).or_insert(0) += 1;
                *locs.entry((line.x0, y)).or_insert(0) += 1;
            }
        } else if line.y0 == line.y1 {
            for x in iter_along(line.x0, line.x1) {
                *horvert_locs.entry((x, line.y0)).or_insert(0) += 1;
                *locs.entry((x, line.y0)).or_insert(0) += 1;
            }
        } else {
            for (x, y) in iter_along(line.x0, line.x1).zip(iter_along(line.y0, line.y1)) {
                *locs.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    //println!("Hash size: {}", locs.len());
    let part1 = horvert_locs.into_values().filter(|cnt| *cnt > 1).count();
    let part2 = locs.into_values().filter(|cnt| *cnt > 1).count();
    //println!("Part 1: {}", part1);
    assert_eq!(part1, 8111);
    //println!("Part 2: {}", part2);
    assert_eq!(part2, 22088);
}
