#[derive(Debug)]
enum Var {
    Imm(i64),
    Reg(usize),
}

#[derive(Debug)]
enum Op {
    Inp(usize),
    Add(usize, Var),
    Mul(usize, Var),
    Div(usize, Var),
    Mod(usize, Var),
    Eql(usize, Var),
}

#[inline]
fn parse_reg(string: &str) -> usize {
    (string.bytes().next().unwrap() - b'w') as usize
}

#[inline]
fn parse_var(string: &str) -> Var {
    let b = string.bytes().next().unwrap();
    if b'w' <= b && b <= b'z' {
        Var::Reg((b - b'w').into())
    } else {
        Var::Imm(string.parse().unwrap())
    }
}

fn parse_instr(instr: &str) -> Op {
    let mut parts = instr.split(' ');
    let op = parts.next().unwrap();
    let a = parts.next().unwrap();
    match op {
        "inp" => Op::Inp(parse_reg(a)),
        "add" => Op::Add(parse_reg(a), parse_var(parts.next().unwrap())),
        "mul" => Op::Mul(parse_reg(a), parse_var(parts.next().unwrap())),
        "div" => Op::Div(parse_reg(a), parse_var(parts.next().unwrap())),
        "mod" => Op::Mod(parse_reg(a), parse_var(parts.next().unwrap())),
        "eql" => Op::Eql(parse_reg(a), parse_var(parts.next().unwrap())),
        _ => unimplemented!(),
    }
}

fn eval_var(v: &Var, regs: &[i64]) -> i64 {
    match v {
        Var::Imm(val) => *val,
        Var::Reg(r) => regs[*r],
    }
}

#[allow(dead_code)]
fn letters(x: i64) -> String {
    let mut s = Vec::<u8>::with_capacity(15);
    let sgn = x.signum();
    let mut n = x.abs();
    while n > 0 {
        s.push((n % 26) as u8 + b'a');
        n /= 26;
    }
    if sgn < 0 { s.push(b'-'); }
    s.reverse();
    String::from_utf8_lossy(&s).into()
}

fn run(program: &Vec<Op>, input: &[u8], mut trace_maybe: Option<&mut Vec<i64>>) -> i64 {
    let mut regs = vec![0i64; 4];
    let mut prog_input = input.iter().map(|b| (b - b'0') as i64);

    // let mut pre_input_z = Vec::<i64>::new();
    let mut first = true;
    for instr in program {
        match instr {
            Op::Inp(r) => regs[*r] = {
                if !first {
                    if let Some(ref mut trace) = trace_maybe {
                        trace.push(regs[3]);
                    }
                }
                first = false;
                prog_input.next().unwrap()
            },
            Op::Add(r, v) => regs[*r] += eval_var(&v, &regs),
            Op::Mul(r, v) => regs[*r] *= eval_var(&v, &regs),
            Op::Div(r, v) => regs[*r] /= eval_var(&v, &regs),
            Op::Mod(r, v) => regs[*r] = regs[*r] % eval_var(&v, &regs),
            Op::Eql(r, v) => regs[*r] = if regs[*r] == eval_var(&v, &regs) { 1 } else { 0 },
        }

        // println!("  {:?}  -> {:?}", instr, regs);  
    }
    if let Some(ref mut trace) = trace_maybe {
        trace.push(regs[3]);
    }

    regs[3]
}

pub fn day24(test_mode: bool, print: bool) {
    const INPUT: &str = "inputs/input24.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let mut program = Vec::<Op>::with_capacity(100);
    for line in input_str.lines() {
        let instr = parse_instr(line);
        // println!("INSTR {:?}", instr);
        program.push(instr);
    }

    let value = 39999698799429usize;
    let z= run(&program, value.to_string().as_bytes(), None);
    if print { println!("Part 1: ({})  {}", z, value); }

    let value = 18116121134117usize;
    let z= run(&program, value.to_string().as_bytes(), None);
    if print { println!("Part 2: ({})  {}", z, value); }

    // for model in (11111111111111usize..99999999999999).rev() {
    //     let prog_input_str = model.to_string();
    //     if prog_input_str.find("0").is_some() { continue; }
    //     let mut trace = Vec::<i64>::with_capacity(14);

    //     let z = run(&program, prog_input_str.as_bytes(), Some(&mut trace));

    //     // if model % 75982 == 0 { println!("Model {} -> result {:?}", model, regs); }
    //     println!("Model {} -> result {:?}   trace {:?}", model, z, trace.iter().map(|x| letters(*x)).collect::<Vec<_>>());
    //     if z == 0 {
    //         break;
    //     }
    // }

    // TODO: check DIV truncation/rounding behavior

    // let part1 = solve(&mut start_grid, &mut start_locs, &mut cnt1);
    // println!("Should show {} but show is {}", part1, print);
    // if print { println!("Part 1: {}", part1); }
    // assert_eq!(part1, if test_mode { 12521 } else { 14371 });

    // let part2 = solve(&mut ext_grid, &mut ext_locs, &mut cnt2);
    // if print { println!("Part 2: {}", part2); }
    // assert_eq!(part2, if test_mode { 44169 } else { 40941 });
}

const TEST_EXAMPLE: &'static str = "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2";
