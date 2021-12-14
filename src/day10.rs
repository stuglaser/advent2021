fn opener_of(ch: u8) -> u8{
    match ch {
        b']' => b'[',
        b')' => b'(',
        b'}' => b'{',
        b'>' => b'<',
        _ => unimplemented!(),
    }
}

fn score_of(ch: u8) -> i32 {
    match ch {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => unimplemented!(),
    }
}

fn incomplete_score_of(ch: u8) -> usize {
    match ch {
        b')' => 1,
        b']' => 2,
        b'}' => 3,
        b'>' => 4,
        b'(' => 1,
        b'[' => 2,
        b'{' => 3,
        b'<' => 4,
        _ => unimplemented!(),
    }
}

pub fn day10(test_mode: bool) {
    const INPUT: &str = "inputs/input10.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
    } else {
        &file_str
    };

    let mut part1 = 0;
    let mut incomplete_scores = Vec::<usize>::with_capacity(100);
    'byline: for line in input_str.lines() {
        let mut stack = Vec::<u8>::with_capacity(100);
        for ch in line.bytes() {
            if ch == b'[' || ch == b'(' || ch == b'{' || ch == b'<' {
                stack.push(ch)
            } else {
                if stack.last().unwrap_or(&0) == &opener_of(ch) {
                    stack.pop();
                } else {
                    // Illegal!
                    part1 += score_of(ch);
                    continue 'byline;
                }
            }
        }

        let mut incomplete_score = 0usize;
        for ch in stack.iter().rev() {
            incomplete_score = 5 * incomplete_score + incomplete_score_of(*ch);
        }
        incomplete_scores.push(incomplete_score);
    }

    // println!("Part 1: {}", part1);
    assert_eq!(part1, if test_mode { 26397 } else { 265527 });

    incomplete_scores.sort();
    let part2 = incomplete_scores[incomplete_scores.len() / 2];
    // println!("Part 2: {}", part2);
    assert_eq!(part2, if test_mode { 288957 } else { 3969823589 });
}
