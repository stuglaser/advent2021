use rustc_hash::FxHashMap;

type Graph = Vec<Vec<usize>>;

fn count_paths(edges: &Graph, is_small: &[bool], seen: &mut [bool], at: usize, small_repeats_left: usize) -> usize {
    if at == 1 {  // The end
        return 1;
    }

    let is_lowercase = is_small[at];

    let is_repeat = is_lowercase && seen[at];
    let mut small_repeats_left = small_repeats_left;
    if is_repeat {
        if at == 0 || small_repeats_left == 0 {
            return 0;
        }
        small_repeats_left -= 1;
    }

    if is_lowercase && !is_repeat {
        seen[at] = true;
    }

    let mut paths = 0;
    for next in &edges[at] {
        paths += count_paths(edges, is_small, seen, *next, small_repeats_left);
    }

    if is_lowercase && !is_repeat {
        seen[at] = false;
    }

    paths
}


pub fn day12(test_mode: bool, print: bool) {
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

    let mut node_lookup = FxHashMap::<String, usize>::with_capacity_and_hasher(100, Default::default());
    node_lookup.insert("start".to_string(), 0);
    node_lookup.insert("end".to_string(), 1);

    let mut edges = Vec::<Vec<usize>>::with_capacity(100);
    edges.resize(2, Vec::with_capacity(8));

    for line in input_str.lines() {
        let mut it = line.split("-");
        let a = it.next().unwrap();
        let b = it.next().unwrap();

        let a_id = *node_lookup.entry(a.to_string()).or_insert_with(|| {
            edges.resize(edges.len() + 1, Vec::with_capacity(8));
            edges.len() - 1
        });
        let b_id = *node_lookup.entry(b.to_string()).or_insert_with(|| {
            edges.resize(edges.len() + 1, Vec::with_capacity(8));
            edges.len() - 1
        });

        edges[a_id].push(b_id);
        edges[b_id].push(a_id);
    }

    let mut is_small = vec![false; edges.len()];
    for (k, v) in node_lookup {
        is_small[v] = k.as_bytes().first().unwrap() >= &b'a';
    }

    let mut seen = vec![false; edges.len()];
    let part1 = count_paths(&edges, &is_small, &mut seen, 0, 0);
    let part2 = count_paths(&edges, &is_small, &mut seen, 0, 1);

    if print { println!("Part 1: {}", part1); }
    assert_eq!(part1, if test_mode { 226 } else { 3369 });

    if print { println!("Part 2: {}", part2); }
    assert_eq!(part2, if test_mode { 3509 } else { 85883 });
}
