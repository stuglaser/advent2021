pub fn day06(test_mode: bool) {
    const INPUT: &str = "inputs/input06.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        "3,4,3,1,2"
    } else {
        file_str.trim_end()
    };
    //let input_str = "3,4,3,1,2";

    let mut current = vec![0usize; 9];
    for num_str in input_str.split(",") {
        current[num_str.parse::<usize>().unwrap()] += 1;
    }
    
    let mut part1 = 0;
    for d in 1..=256 {
        let mut next = vec![0usize; 9];
        for (n, cnt) in current.iter().enumerate() {
            if n == 0 {
                next[6] += cnt;
                next[8] += cnt;
            } else {
                next[n - 1] += cnt;
            }
        }
        current = next;

        if d == 80 {
            part1 = current.iter().sum::<usize>();
        }
    }
    //println!("Part 1: {}", part1);
    assert_eq!(part1, if test_mode { 5934 } else { 351188 });
    let part2 = current.iter().sum::<usize>();
    //println!("Part 2: {}", part2);
    assert_eq!(part2, if test_mode { 26984457539 } else { 1595779846729 });
}
