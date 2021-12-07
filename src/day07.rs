
pub fn day07() {
    const INPUT: &str = "inputs/input07.txt";
    let input_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = input_str.trim_end();
    // let input_str = "16,1,2,0,4,2,7,1,2,14";

    let input = input_str
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let lo = *input.iter().min().unwrap();
    let hi = *input.iter().max().unwrap();
    // println!("Range {} {}", lo, hi);

    let mut best = 99999999;
    let mut best_i = 0;
    for i in lo..=hi {
        let mut diff = 0;
        for x in &input { 
            diff += (x - i).abs();
        }
        if diff < best {
            best = diff;
            best_i = i;
        }
        // println!(">>  {}  {}", i, diff);
    }
    let part1 = best;
    // println!("Part 1: {}", part1);
    assert_eq!(part1, 356922);

    let mut best = i32::MAX;
    let mut best_i = 0;
    for i in lo..=hi {
        let mut cost = 0;
        for x in &input { 
            let diff = (x - i).abs();
            cost += diff * (diff + 1) / 2;
        }
        if cost < best {
            best = cost;
            best_i = i;
        }
        // println!(">>  {}  {}", i, cost);
    }
    let part2 = best;
    // println!("Part 2: {}", part2);
    assert_eq!(part2, 100347031);
}
