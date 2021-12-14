
use rustc_hash::{FxHashMap, FxHashSet};

type Graph = FxHashMap<String, Vec<String>>;

fn count_paths<'a>(edges: &'a Graph, seen: &mut FxHashSet<&'a str>, at: &'a str, small_repeats_left: usize) -> usize {
    if at == "end" {
        return 1;
    }

    let is_lowercase = at.as_bytes().first().unwrap() >= &b'a';

    let is_repeat = is_lowercase && seen.contains(&at);
    let mut small_repeats_left = small_repeats_left;
    if is_repeat {
        if at == "start" || small_repeats_left == 0 {
            return 0;
        }
        small_repeats_left -= 1;
    }

    if is_lowercase && !is_repeat {
        seen.insert(at);
    }

    let mut paths = 0;
    for next in &edges[at] {
        paths += count_paths(edges, seen, next, small_repeats_left);
    }

    if is_lowercase && !is_repeat {
        seen.remove(at);
    }

    paths
}


pub fn day12(test_mode: bool) {
    const INPUT: &str = "inputs/input12.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
    } else {
        &file_str
    };

    // let mut edges = FxHashMap::<String, String>::default();
    let mut edges = Graph::default();

    for line in input_str.lines() {
        let mut it = line.split("-");
        let a = it.next().unwrap();
        let b = it.next().unwrap();

        edges.entry(a.to_string()).or_insert(Vec::with_capacity(8)).push(b.to_string());
        edges.entry(b.to_string()).or_insert(Vec::with_capacity(8)).push(a.to_string());
    }

    let mut seen = FxHashSet::<&str>::default();
    let part1 = count_paths(&edges, &mut seen, "start", 0);
    let part2 = count_paths(&edges, &mut seen, "start", 1);

    // println!("Part 1: {}", part1);
    assert_eq!(part1, if test_mode { 226 } else { 3369 });

    // println!("Part 2: {}", part2);
    assert_eq!(part2, if test_mode { 3509 } else { 85883 });
}
