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

fn add_leftmost(piece: Piece, value: i32) -> Piece {
    match piece {
        Piece::Num(n) => Piece::Num(n + value),
        Piece::Snail(pair) => Piece::Snail(
            Box::new((add_leftmost(pair.0, value), pair.1))),
    }
}

fn add_rightmost(piece: Piece, value: i32) -> Piece {
    match piece {
        Piece::Num(n) => Piece::Num(n + value),
        Piece::Snail(pair) => Piece::Snail(
            Box::new((pair.0, add_rightmost(pair.1, value)))),
    }
}

struct PieceExplosion {
    changed: bool,
    piece: Piece,
    num_left: i32,
    num_right: i32,
}

struct SnailExplosion {
    changed: bool,
    snail: SnailNum,
    num_left: i32,
    num_right: i32,
}

impl SnailExplosion {
    fn unchanged(left: Piece, right: Piece) -> SnailExplosion {
        Self{
            changed: false,
            snail: Box::new((left, right)),
            num_left: 0,
            num_right: 0,
        }
    }
}

impl From<SnailExplosion> for PieceExplosion {
    fn from(snail: SnailExplosion) -> Self {
        PieceExplosion{
            changed: snail.changed,
            piece: Piece::Snail(snail.snail),
            num_left: snail.num_left,
            num_right: snail.num_right,
        }
    }
}

fn try_explode_piece(piece: Piece, depth: usize) -> PieceExplosion {
    match piece {
        Piece::Num(n) => PieceExplosion{changed: false, piece: Piece::Num(n), num_left: 0, num_right: 0},
        Piece::Snail(pair) => {
            if depth == 4 {
                // Explode!
                PieceExplosion{
                    changed: true,
                    piece: Piece::Num(0),
                    num_left: if let Piece::Num(n) = pair.0 { n } else { unimplemented!() },
                    num_right: if let Piece::Num(n) = pair.1 { n } else { unimplemented!() },
                }
            } else {
                try_explode_snail(pair, depth).into()
            }
        },
    }
}

fn try_explode_snail(snail: SnailNum, depth: usize) -> SnailExplosion {
    let left = try_explode_piece(snail.0, depth + 1);
    if left.changed {
        let adjusted_right = add_leftmost(snail.1, left.num_right);
        return SnailExplosion{
            changed: true,
            snail: Box::new((left.piece, adjusted_right)),
            num_left: left.num_left,
            num_right: 0,
        };
    }

    let right = try_explode_piece(snail.1, depth + 1);
    if right.changed {
        let adjusted_left = add_rightmost(left.piece, right.num_left);
        return SnailExplosion{
            changed: true,
            snail: Box::new((adjusted_left, right.piece)),
            num_left: 0,
            num_right: right.num_right,
        };
    }

    SnailExplosion::unchanged(left.piece, right.piece)
}

fn try_explode(snail: SnailNum) -> SnailExplosion {
    try_explode_snail(snail, 0)
}

fn try_split_piece(piece: Piece) -> (bool, Piece) {
    match piece {
        Piece::Num(n) =>
            if n < 10 {
                (false, Piece::Num(n))
            } else {
                // Split!
                let a = n / 2;
                let b = (n + 1) / 2;
                (true, Piece::Snail(Box::new((Piece::Num(a), Piece::Num(b)))))
            }
        Piece::Snail(pair) => {
            let (changed, snail) = try_split(pair);
            (changed, Piece::Snail(snail))
        },
    }
}

fn try_split(snail: SnailNum) -> (bool, SnailNum) {
    let left = try_split_piece(snail.0);
    if left.0 {
        return (true, Box::new((left.1, snail.1)));
    }

    let right = try_split_piece(snail.1);
    (right.0, Box::new((left.1, right.1)))
}

fn reduce(mut snail: SnailNum) -> SnailNum {
    loop {
        let exploded = try_explode(snail);
        snail = exploded.snail;
        if exploded.changed {
            continue;
        }

        let split = try_split(snail);
        snail = split.1;
        if split.0 {
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

pub fn day18(test_mode: bool) {
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
    // println!("Part 1: {}", part1);
    assert_eq!(part1, if test_mode { 4140 } else { 3987 });

    let mut best_mag = 0;
    for i in 0..snails.len() {
        for j in 0..snails.len() {
            if i == j { continue; }
            best_mag = max(best_mag, mag(&add(snails[i].clone(), snails[j].clone())));
        }
    }

    let part2 = best_mag;
    // println!("Part 2: {}", part2);
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
        let explosion = try_explode(parse_line("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"));
        let expect = parse_line("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert!(explosion.changed);
        assert_eq!(explosion.snail, expect);

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
