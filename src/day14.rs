use rustc_hash::FxHashMap;

pub fn minmax<I>(mut iterable: I) -> Option<(I::Item, I::Item)>
where
    I: Sized,
    I: Iterator,
    I::Item: Ord,
    I::Item: Clone,
{

    let first_item = iterable.next()?;
    Some(iterable.fold((first_item.clone(), first_item),
        |pair, item|
            (if item < pair.0 { item.clone() } else { pair.0 },
             if item > pair.1 { item } else { pair.1 })
    ))
}

fn solve_from_pairs(initial: &str, pairs: &FxHashMap<String, usize>) -> usize {
    let mut counts = vec![0usize; 26];
    for (pair, cnt) in pairs {
        counts[(pair.bytes().next().unwrap() - b'A') as usize] += cnt;
    }
    let last_letter = initial.bytes().last().unwrap() - b'A';
    counts[last_letter as usize] += 1;
    let (lo, hi) = minmax(counts.iter().filter(|x| **x > 0)).unwrap();
    hi - lo
}

pub fn day14(test_mode: bool) {
    const INPUT: &str = "inputs/input14.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
    } else {
        &file_str
    };

    let mut initial = String::new();
    let mut rules = FxHashMap::<String, char>::with_capacity_and_hasher(100, Default::default());
    let mut parse_phase = 0;
    for line in input_str.lines() {
        if parse_phase == 0 {
            if line.is_empty() {
                parse_phase = 1;
            } else {
                initial = line.to_string();
            }
        } else {
            let mut conversion = line.split(" -> ");
            let sources = conversion.next().unwrap();
            let result = conversion.next().unwrap();
            rules.insert(sources.to_string(), result.chars().next().unwrap());
        }
    }

    let mut pairs = FxHashMap::<String, usize>::with_capacity_and_hasher(initial.len() * 2, Default::default());
    let mut it = initial.chars();
    let mut last = it.next().unwrap();
    for ch in it {
        let mut scratch = String::with_capacity(2);
        scratch.push(last);
        scratch.push(ch);
        *pairs.entry(scratch).or_insert(0) += 1;
        last = ch;
    }


    let mut pairs10 = FxHashMap::default();
    for step in 1..=40 {
        let mut next_pairs = FxHashMap::with_capacity_and_hasher(pairs.len() * 2, Default::default());

        for (pair, cnt) in pairs {
            let insertion = rules.get(&pair).unwrap();
            let mut it = pair.chars();
            let mut left = String::with_capacity(2);
            left.push(it.next().unwrap());
            left.push(*insertion);
            let mut right = String::with_capacity(2);
            right.push(*insertion);
            right.push(it.next().unwrap());

            *next_pairs.entry(left).or_insert(0) += cnt;
            *next_pairs.entry(right).or_insert(0) += cnt;
        }

        pairs = next_pairs;
        if step == 10 {
            pairs10 = pairs.clone();
        }
    }

    let part1 = solve_from_pairs(&initial, &pairs10);
    // println!("Part 1: {}", part1);
    assert_eq!(part1, if test_mode { 1588 } else { 3284 });

    let part2 = solve_from_pairs(&initial, &pairs);
    // println!("Part 2: {}", part2);
    assert_eq!(part2, if test_mode { 2188189693529 } else { 4302675529689 });
}
