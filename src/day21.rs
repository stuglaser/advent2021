use std::cmp::{min, max};

struct DeterministicDie {
    value: i32,
    rolls: usize,
}

impl DeterministicDie {
    fn new() -> Self {
        DeterministicDie{value: 0, rolls: 0}
    }

    fn roll(&mut self) -> i32 {
        self.rolls += 1;
        self.value = self.value % 100 + 1;
        self.value
    }
}

const ROLLS3: [(i32, usize); 7] = [
    (3, 1),
    (4, 3),
    (5, 6),
    (6, 7),
    (7, 6),
    (8, 3),
    (9, 1),
];

fn sim(at1: i32, score1: i32, at2: i32, score2: i32) -> (usize, usize) {
    if score1 >= 21 { return (1, 0); }
    if score2 >= 21 { return (0, 1); }

    let mut wins1 = 0;
    let mut wins2 = 0;

    for (mv, times) in ROLLS3 {
        let at_next = (at1 + mv - 1) % 10 + 1;
        let (w2, w1) = sim(at2, score2, at_next, score1 + at_next);
        wins1 += w1 * times;
        wins2 += w2 * times;
    }
    (wins1, wins2)
}

pub fn day21(test_mode: bool) {
    const INPUT: &str = "inputs/input21.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let mut lines = input_str.lines();
    let p1_start = lines.next().unwrap().split(" ").last().unwrap().parse::<i32>().unwrap();
    let p2_start = lines.next().unwrap().split(" ").last().unwrap().parse::<i32>().unwrap();

    let mut die = DeterministicDie::new();
    let mut at1 = p1_start;
    let mut at2 = p2_start;
    let mut score1 = 0;
    let mut score2 = 0;
    loop {
        let mv = die.roll() + die.roll() + die.roll();
        at1 = (at1 + mv - 1) % 10 + 1;
        score1 += at1;
        if score1 >= 1000 {
            break;
        }

        let mv = die.roll() + die.roll() + die.roll();
        at2 = (at2 + mv - 1) % 10 + 1;
        score2 += at2;
        if score2 >= 1000 {
            break;
        }
    }

    let loss = min(score1, score2);


    let part1 = loss as usize * die.rolls;
    // println!("Part 1: {}", part1);
    assert_eq!(part1, if test_mode { 739785 } else { 920079 });

    let (wins1, wins2) = sim(p1_start, 0, p2_start, 0);
    // println!("WINS: {}, {}", wins1, wins2);


    let part2 = max(wins1, wins2);
    // println!("Part 2: {}", part2);
    assert_eq!(part2, if test_mode { 444356092776315 } else { 56852759190649 });
}

const TEST_EXAMPLE: &'static str = "Player 1 starting position: 4
Player 2 starting position: 8";