use std::cmp::min;

struct BitsSnarfer<'a> {
    data: &'a [u8],
    byte: usize,
    bit: usize,
}

#[inline]
fn getbits(value: u8, start: usize, length: usize) -> u8 {
    // Counting from most significant to least significant
    // 76543210
    //   ^  ^
    //   |  +- end
    //   +- start
    // println!("    getbits({:b}, {}, {})", value, start, length);
    let trim_right = 8 - start - length;
    (value >> trim_right) & (0xff >> (8 - length))
}

impl<'a> BitsSnarfer<'a> {
    fn new(data: &[u8]) -> BitsSnarfer {
        BitsSnarfer{data, byte: 0, bit: 0}
    }

    fn snarf(&mut self, mut bits_left: usize) -> usize {
        let mut value = 0usize;
        while bits_left > 0 {
            let bits_to_take = min(bits_left, 8 - self.bit);
            // println!("  SNARF [{}.{}] {}  take {}  --> {}", self.byte, self.bit, bits_left, bits_to_take, getbits(self.data[self.byte], self.bit, bits_to_take));
            value =
                (value << bits_to_take) +
                getbits(self.data[self.byte], self.bit, bits_to_take) as usize;

            bits_left -= bits_to_take;
            self.bit += bits_to_take;
            if self.bit == 8 {
                self.bit = 0;
                self.byte += 1;
            }
        }
        value
    }

    fn loc(&self) -> (usize, usize) {
        (self.byte, self.bit)
    }

    fn bits_since(&self, loc: &(usize, usize)) -> usize {
        (self.byte - loc.0) * 8 + self.bit - loc.1
    }
}

struct PacketResult {
    value: usize,
    sum_versions: usize,
}

fn parse_packet(sn: &mut BitsSnarfer) -> PacketResult {
    let version = sn.snarf(3);
    let type_id = sn.snarf(3);
    // println!("version: {}, type_id: {}", version, type_id);

    // let mut result = PacketResult{sum_versions: version, bits_parsed: 0};

    if type_id == 4 {
        let mut value = 0usize;
        loop {
            let segment = sn.snarf(5);
            value = (value << 4) + (segment & 0b1111);
            if (segment >> 4) == 0 {
                break;
            }
        }
        PacketResult{ value, sum_versions: version }
    } else { // Operator
        let length_type_id = sn.snarf(1);
        let (subpacket_bits, subpacket_count) =
            if length_type_id == 0 {
                (sn.snarf(15), usize::MAX)
            } else {
                (usize::MAX, sn.snarf(11))
            };

        let mut sum_versions = version;
        let mut subvalues = Vec::<usize>::with_capacity(8);
        let subpackets_start_loc = sn.loc();
        while subvalues.len() < subpacket_count && sn.bits_since(&subpackets_start_loc) < subpacket_bits {
            let subpacket = parse_packet(sn);
            sum_versions += subpacket.sum_versions;
            subvalues.push(subpacket.value);
        }

        let value: usize = match type_id {
            0 => subvalues.iter().sum(),
            1 => subvalues.iter().product(),
            2 => *subvalues.iter().min().unwrap(),
            3 => *subvalues.iter().max().unwrap(),
            5 => if subvalues[0] > subvalues[1] { 1 } else { 0 },
            6 => if subvalues[0] < subvalues[1] { 1 } else { 0 },
            7 => if subvalues[0] == subvalues[1] { 1 } else { 0 },
            _ => unimplemented!(),
        };

        PacketResult{ value, sum_versions }
    }
}

pub fn day16(test_mode: bool) {
    const INPUT: &str = "inputs/input16.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        "9C0141080250320F1802104A08"
    } else {
        &file_str.trim_end()
    };

    let data: Vec<u8> = input_str.as_bytes()
        .chunks(2)
        .map(|pair| u8::from_str_radix(&String::from_utf8_lossy(pair), 16).unwrap())
        .collect();

    let mut sn = BitsSnarfer::new(&data);
    let parsed = parse_packet(&mut sn);

    let part1 = parsed.sum_versions;
    // println!("Part 1: {}", part1);
    assert_eq!(part1, if test_mode { 20 } else { 925 });


    let part2 = parsed.value;
    // println!("Part 2: {}", part2);
    assert_eq!(part2, if test_mode { 1 } else { 342997120375 });
}
