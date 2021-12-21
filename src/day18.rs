use std::{iter::Peekable, cmp::max};

type SnailNum = Box<(Piece, Piece)>;
#[derive(Debug, PartialEq, Eq, Clone)]
enum Piece {
    Num(i32),
    Snail(SnailNum),
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Num(n) => write!(f, "{}", n),
            Piece::Snail(pair) => write!(f, "[{},{}]", pair.0, pair.1),
        }
    }
}

#[allow(dead_code)]
fn psnail(snail: &SnailNum) -> String {
    format!("[{},{}]", snail.0, snail.1)
}

fn parse_int<It>(it: &mut Peekable<It>) -> i32
where It: Iterator<Item=char>
{
    let mut num_str = String::with_capacity(8);
    loop {
        let ch = *it.peek().unwrap();
        if ch < '0' || ch > '9' {
            break;
        }

        num_str.push(it.next().unwrap());
    }

    num_str.parse().unwrap()
}

fn parse_snail_num<It>(it: &mut Peekable<It>) -> SnailNum
where It: Iterator<Item=char>
{
    assert_eq!(it.next(), Some('['));

    let left = if *it.peek().unwrap() == '[' {
            Piece::Snail(parse_snail_num(it))
        } else {
            Piece::Num(parse_int(it))
        };
    assert_eq!(it.next(), Some(','));
    let right = if *it.peek().unwrap() == '[' {
        Piece::Snail(parse_snail_num(it))
    } else {
        Piece::Num(parse_int(it))
    };
    assert_eq!(it.next(), Some(']'));
    Box::new((left, right))
}

fn parse_line(line: &str) -> SnailNum {
    let mut it = line.chars().peekable();
    parse_snail_num(&mut it)
}

fn add_leftmost(piece: &mut Piece, value: i32) {
    match piece {
        Piece::Num(n) => *n += value,
        Piece::Snail(pair) => add_leftmost(&mut pair.0, value),
    }
}

fn add_rightmost(piece: &mut Piece, value: i32) {
    match piece {
        Piece::Num(n) => *n += value,
        Piece::Snail(pair) => add_rightmost(&mut pair.1, value),
    }
}

struct Explosion {
    immediate: bool,
    num_left: i32,
    num_right: i32,
}

fn try_explode_piece(piece: &mut Piece, depth: usize) -> Option<Explosion> {
    match piece {
        Piece::Num(_) => None,
        Piece::Snail(pair) => {
            if depth == 4 {
                // Explode!
                Some(Explosion{immediate: true, num_left: force_num(&pair.0), num_right: force_num(&pair.1)})
            } else {
                try_explode_snail(pair, depth)
            }
        },
    }
}

#[inline]
fn force_num(piece: &Piece) -> i32 {
    if let Piece::Num(n) = piece { *n } else { unimplemented!() }
}

fn try_explode_snail(snail: &mut SnailNum, depth: usize) -> Option<Explosion> {
    if let Some(explosion) = try_explode_piece(&mut snail.0, depth + 1) {
        if explosion.immediate {
            snail.0 = Piece::Num(0);
        }

        if explosion.num_right != 0 {
            add_leftmost(&mut snail.1, explosion.num_right);
        }
        return Some(Explosion{
            immediate: false,
            num_left: explosion.num_left,
            num_right: 0,
        });
    }

    if let Some(explosion) = try_explode_piece(&mut snail.1, depth + 1) {
        if explosion.immediate {
            snail.1 = Piece::Num(0);
        }

        if explosion.num_left != 0 {
            add_rightmost(&mut snail.0, explosion.num_left);
        }
        return Some(Explosion{
            immediate: false,
            num_left: 0,
            num_right: explosion.num_right,
        });
    }

    None
}

fn try_explode(snail: &mut SnailNum) -> Option<Explosion> {
    try_explode_snail(snail, 0)
}

fn split_piece(n: i32) -> Piece {
    let a = n / 2;
    let b = (n + 1) / 2;
    Piece::Snail(Box::new((Piece::Num(a), Piece::Num(b))))
}

fn try_split(snail: &mut SnailNum) -> bool {
    match &mut snail.0 {
        Piece::Num(n) => {
            if *n >= 10 {
                // Split!
                snail.0 = split_piece(*n);
                return true;
            }
        }
        Piece::Snail(pair) => {
            if try_split(pair) {
                return true;
            }
        },
    }

    match &mut snail.1 {
        Piece::Num(n) => {
            if *n >= 10 {
                // Split!
                snail.1 = split_piece(*n);
                return true;
            }
        }
        Piece::Snail(pair) => {
            if try_split(pair) {
                return true;
            }
        }
    }
    return false;
}

fn reduce(mut snail: SnailNum) -> SnailNum {
    loop {
        let exploded = try_explode(&mut snail);
        if exploded.is_some() {
            continue;
        }

        let split = try_split(&mut snail);
        if split {
            continue;
        }

        break;
    }
    snail
}

fn add_unreduced(left: SnailNum, right: SnailNum) -> SnailNum {
    Box::new((Piece::Snail(left), Piece::Snail(right)))
}

fn add(left: SnailNum, right: SnailNum) -> SnailNum {
    reduce(add_unreduced(left, right))
}

fn mag_piece(piece: &Piece) -> i32 {
    match piece {
        Piece::Num(n) => *n,
        Piece::Snail(snail) => mag(snail),
    }
}

fn mag(snail: &SnailNum) -> i32 {
    3 * mag_piece(&snail.0) + 2 * mag_piece(&snail.1)
}

pub fn day18(test_mode: bool, print: bool) {
    const INPUT: &str = "inputs/input18.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
    } else {
        &file_str.trim_end()
    };

    let snails: Vec<SnailNum> = input_str.lines().map(|line| parse_line(line)).collect();
    let sum = snails.clone().into_iter().reduce(|a, b| add(a, b)).unwrap();
    // println!("Sum: {}", psnail(&sum));



    let part1 = mag(&sum);
    if print { println!("Part 1: {}", part1); }
    assert_eq!(part1, if test_mode { 4140 } else { 3987 });

    let mut best_mag = 0;
    for i in 0..snails.len() {
        for j in 0..snails.len() {
            if i == j { continue; }
            best_mag = max(best_mag, mag(&add(snails[i].clone(), snails[j].clone())));
        }
    }

    let part2 = best_mag;
    if print { println!("Part 2: {}", part2); }
    assert_eq!(part2, if test_mode { 3993 } else { 4500 });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simplest_add() {
        let left = parse_line("[1,2]");
        let right = parse_line("[[3,4],5]");
        let result = add_unreduced(left, right);
        let expect = parse_line("[[1,2],[[3,4],5]]");

        assert_eq!(result, expect);
    }

    #[test]
    fn check_explodes() {
        let result = reduce(parse_line("[[[[[9,8],1],2],3],4]"));
        let expect = parse_line("[[[[0,9],2],3],4]");
        assert_eq!(result, expect);

        let result = reduce(parse_line("[7,[6,[5,[4,[3,2]]]]]"));
        let expect = parse_line("[7,[6,[5,[7,0]]]]");
        assert_eq!(result, expect);

        let result = reduce(parse_line("[[6,[5,[4,[3,2]]]],1]"));
        let expect = parse_line("[[6,[5,[7,0]]],3]");
        assert_eq!(result, expect);

        // Just the first explosion
        let mut result = parse_line("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        let explosion = try_explode(&mut result);
        let expect = parse_line("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert!(explosion.is_some());
        assert_eq!(result, expect);

        let result = reduce(parse_line("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
        let expect = parse_line("[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
        assert_eq!(result, expect);
    }

    #[test]
    fn check_splits() {
        let result = reduce(parse_line("[10,3]"));
        let expect = parse_line("[[5,5],3]");
        assert_eq!(result, expect);
        
        let result = reduce(parse_line("[11,3]"));
        let expect = parse_line("[[5,6],3]");
        assert_eq!(result, expect);
    }

    #[test]
    fn test_some_adds() {
        let result = add(
            parse_line("[[[[4,3],4],4],[7,[[8,4],9]]]"),
            parse_line("[1,1]"));
        assert_eq!(result, parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn test_list_sums() {
        let result = "[1,1]
[2,2]
[3,3]
[4,4]"
            .lines().map(|line| parse_line(line)).into_iter()
            .reduce(|a, b| add(a, b)).unwrap();
        assert_eq!(result, parse_line("[[[[1,1],[2,2]],[3,3]],[4,4]]"));

        let result = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"
            .lines().map(|line| parse_line(line)).into_iter()
            .reduce(|a, b| add(a, b)).unwrap();
        assert_eq!(result, parse_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"));     
    }

    #[test]
    fn test_mag() {
        let result = mag(&parse_line("[9,1]"));
        assert_eq!(result, 29);
        let result = mag(&parse_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"));
        assert_eq!(result, 3488);
    }
}
