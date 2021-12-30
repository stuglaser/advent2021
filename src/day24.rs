use regex::Regex;

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

#[derive(Debug)]
struct Block {
    offset: i64,
    zdiv: i64,
    added: i64,
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

    // Every block has the same structure:
    //
    // read w
    // boost = z % 26 + <offset> != w
    // z /= <zdiv: 1 or 26>
    // if boost:
    //     z = z * 26 + <added> + w
    

    let re_parse_blocks = Regex::new(r"inp w
mul x 0
add x z
mod x 26
div z (\d+)
add x (-?\d+)
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y (-?\d+)
mul y x
add z y").unwrap();

    let mut blocks = Vec::<Block>::with_capacity(16);
    for cap in re_parse_blocks.captures_iter(input_str) {
        // println!("CAPTURE: {:?}", cap);
        blocks.push(Block {
            offset: cap[2].parse().unwrap(),
            zdiv: cap[1].parse().unwrap(),
            added: cap[3].parse().unwrap() });
    }
    
    // (a, b, offset) -> a = b + offset
    let mut constraints = Vec::<(usize, usize, i64)>::new();

    let mut stack = Vec::<(usize, i64)>::new();
    for (i, block) in blocks.iter().enumerate() {
        assert_eq!(block.offset < 10, block.zdiv == 26);

        if block.zdiv == 26 {
            // Reducing step. Need to prevent the increase
            let assoc = stack.pop().unwrap();
            constraints.push((i, assoc.0, assoc.1 + block.offset));
        } else {
            // Increasing step
            stack.push((i, block.added));
        }
    }
    // Maximizing
    let mut value = vec![0u8; 14];
    for c in &constraints {
        if c.2 >= 0 {
            value[c.0] = b'9';
            value[c.1] = b'9' - c.2 as u8;
        } else {
            value[c.0] = b'9' - ((-c.2) as u8);
            value[c.1] = b'9';
        }
    }

    let z= run(&program, &value, None);
    if print { println!("Part 1: ({})  {}", z, String::from_utf8_lossy(&value)); }
    assert_eq!(z, 0);
    assert_eq!(value, b"39999698799429");

    // Minimizing
    let mut value = vec![0u8; 14];
    for c in &constraints {
        if c.2 >= 0 {
            value[c.0] = b'1' + c.2 as u8;
            value[c.1] = b'1';
        } else {
            value[c.0] = b'1';
            value[c.1] = b'1' + ((-c.2) as u8);
        }
    }

    let z= run(&program, &value, None);
    if print { println!("Part 2: ({})  {}", z, String::from_utf8_lossy(&value)); }
    assert_eq!(z, 0);
    assert_eq!(value, b"18116121134117");
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
