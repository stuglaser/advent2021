use std::cmp::min;


fn optimize_cost(input: &[i32], cost_at: impl Fn(&[i32], i32) -> i32) -> i32 {
    let mut lo = *input.iter().min().unwrap();
    let mut hi = *input.iter().max().unwrap();

    loop {
        let mid = (lo + hi) / 2;

        let mid_cost = cost_at(input, mid);
        let mid_next_cost = cost_at(input, mid + 1);

        if mid_cost < mid_next_cost {
            hi = mid;
        } else {
            lo = mid + 1;
        }

        if lo == hi {
            return min(mid_cost, mid_next_cost);
        }
    }
}

pub fn day07(test_mode: bool, print: bool) {
    const INPUT: &str = "inputs/input07.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        "16,1,2,0,4,2,7,1,2,14"
    } else {
        file_str.trim_end()
    };

    let input = input_str
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();


    let cost_at = |input: &[i32], pt: i32| {
        let mut cost = 0;
        for x in input {
            cost += (x - pt).abs();
        }
        cost
    };

    let part1 = optimize_cost(&input, cost_at);
    if print { println!("Part 1: {}", part1); }
    assert_eq!(part1, if test_mode { 37 } else { 356922 });


    let cost_at_quadratic = |input: &[i32], pt: i32| {
        let mut cost = 0;
        for x in input { 
            let diff = (x - pt).abs();
            cost += diff * (diff + 1) / 2;
        }
        cost
    };

    let part2 = optimize_cost(&input, cost_at_quadratic);
    if print { println!("Part 2: {}", part2); }
    assert_eq!(part2, if test_mode { 168 } else { 100347031 });
}
