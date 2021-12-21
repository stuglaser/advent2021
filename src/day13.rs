use crate::utils::*;

use rustc_hash::FxHashSet;


pub fn day13(test_mode: bool, print: bool) {
    const INPUT: &str = "inputs/input13.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
    } else {
        &file_str
    };

    let mut dots = Vec::<Pt>::with_capacity(1000);
    let mut foldings = Vec::<(bool, i32)>::with_capacity(10);  // (is_x, location)
    let mut folding_step = false;
    for line in input_str.lines() {

        if !folding_step {
            if line.is_empty() {
                folding_step = true;
            } else {
                let mut it = line.split(",");
                let x = it.next().unwrap().parse().unwrap();
                let y = it.next().unwrap().parse().unwrap();
                dots.push(Pt{x, y});
            }
        } else {
            let mut words = line.split(' ');
            let _ = words.next();
            let _ = words.next();
            let mut expr = words.next().unwrap().split("=");
            let var = expr.next().unwrap();
            let value = expr.next().unwrap().parse().unwrap();
            foldings.push((var == "x", value));
        }
    }

    let mut dots_showing = FxHashSet::<Pt>::with_capacity_and_hasher(dots.len(), Default::default());
    for pt in dots {
        dots_showing.insert(pt);
    }

    let mut first_fold_dots = -1;
    for (is_x, fold) in foldings {
        let mut next_dots = FxHashSet::<Pt>::with_capacity_and_hasher(dots_showing.len(), Default::default());
        for dot in dots_showing {
            if is_x {
                if dot.x < fold {
                    next_dots.insert(dot);
                } else {
                    next_dots.insert(Pt{x: 2 * fold - dot.x, y: dot.y});
                }
            } else { // folded along y
                if dot.y < fold {
                    next_dots.insert(dot);
                } else {
                    next_dots.insert(Pt{x: dot.x, y: 2 * fold - dot.y});
                }
            }
        }
        dots_showing = next_dots;

        if first_fold_dots < 0 {
            first_fold_dots = dots_showing.len() as i32;
        }
    }


    let part1 = first_fold_dots; // dots_showing.len();
    if print { println!("Part 1: {}", part1); }
    assert_eq!(part1, if test_mode { 17 } else { 720 });

    if print {
        let mut out = String::with_capacity(1000);
        for y in 0..100 {
            for x in 0..100 {
                if dots_showing.contains(&Pt{x: x, y: y}) {
                    out += "#";
                } else {
                    out += " ";
                }
            }
            out += "\n";
        }
        println!("{}", out);
    }

    let part2 = dots_showing.len();
    if print { println!("Part 2: {}", part2); }
    assert_eq!(part2, if test_mode { 16 } else { 104 });
    //AHPRPAUZ
}
