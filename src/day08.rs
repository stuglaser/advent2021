fn char_to_bits(ch: u8) -> u8 {
    match ch {
        b'a' => 0b0000_0001,
        b'b' => 0b0000_0010,
        b'c' => 0b0000_0100,
        b'd' => 0b0000_1000,
        b'e' => 0b0001_0000,
        b'f' => 0b0010_0000,
        b'g' => 0b0100_0000,
        _ => unimplemented!(),
    }
}

pub fn day08(test_mode: bool, print: bool) {
    const INPUT: &str = "inputs/input08.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
    } else {
        &file_str
    };
    
    let mut count_simples = 0;
    let mut part2 = 0usize;
    for line in input_str.lines() {
        let line = line.trim_start();

        let mut the1: u8 = u8::MAX;
        let mut the4: u8 = u8::MAX;
        let mut the7: u8 = u8::MAX;

        let mut counts = vec![0u8; 7];
        let mut readout = Vec::<u8>::with_capacity(4);
        for (i, part) in line.split(" ").enumerate() {
            if i < 10 {
                let mut bits =  0;
                for ch in part.bytes() {
                    counts[(ch - b'a') as usize] += 1;
                    bits |= char_to_bits(ch);
                }

                if part.len() == 2 {
                    the1 = bits;
                } else if part.len() == 3 {
                    the7 = bits;
                } else if part.len() == 4 {
                    the4 = bits;
                }
            } else if i > 10 {
                if part.len() == 2 || part.len() == 3 || part.len() == 4 || part.len() == 7 {
                    count_simples += 1;
                }

                let mut bits =  0;
                for ch in part.bytes() {
                    bits |= char_to_bits(ch);
                }
                readout.push(bits);
            }
        }

        // Wrapping up the inference

        // These wires have unique total counts.
        let mut wire_b: u8 = u8::MAX;
        let mut wire_e: u8 = u8::MAX;
        let mut wire_f: u8 = u8::MAX;
        for (i, cnt) in counts.iter().enumerate() {
            if *cnt == 4 {
                wire_e = i as u8;
            } else if *cnt == 6 {
                wire_b = i as u8;
            } else if *cnt == 9 {
                wire_f = i as u8;
            }
        }

        // These numbers are unique, and missing just one wire.
        let wire_c = (the1 & !(1 << wire_f)).trailing_zeros() as u8;
        let wire_a = (
            the7 &
            !(1 << wire_c) &
            !(1 << wire_f)).trailing_zeros() as u8;
        let wire_d = (
            the4 &
            !(1 << wire_b) &
            !(1 << wire_c) &
            !(1 << wire_f)).trailing_zeros() as u8;

        // Last wire.
        let wire_g = ((!(
            (1 << wire_a) |
            (1 << wire_b) | 
            (1 << wire_c) | 
            (1 << wire_d) | 
            (1 << wire_e) | 
            (1 << wire_f))) as u8).trailing_zeros() as u8;

        // Interprets the readout
        let mut readout_value = 0;
        for rwires in readout {
            let value = 
                if (rwires & (1 << wire_b)) > 0 {
                    if (rwires & (1 << wire_e)) > 0 {
                        if rwires == 0b0111_1111 { 8 }
                        else if (rwires & (1 << wire_c)) > 0 { 0 }
                        else { 6 }
                    } else { // No e
                        if (rwires & (1 << wire_a)) > 0 && (rwires & (1 << wire_c)) > 0 { 9 }
                        else if (rwires & (1 << wire_a)) > 0 { 5 }
                        else { 4 }
                    }
                } else { // No b
                    if (rwires & (1 << wire_g)) > 0 {
                        if (rwires & (1 << wire_e)) > 0 { 2 }
                        else { 3 }
                    } else { // No g
                        if (rwires & (1 << wire_a)) > 0 { 7 }
                        else { 1 }
                    }
                };
            readout_value = 10 * readout_value + value;
        }
        part2 += readout_value;
    }

    let part1 = count_simples;
    if print { println!("Part 1: {}", part1); }
    assert_eq!(part1, if test_mode { 26 } else { 449 });
    if print { println!("Part 2: {}", part2); }
    assert_eq!(part2, if test_mode { 61229 } else { 968175 });
}
