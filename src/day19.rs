use std::cmp::max;

use rustc_hash::{FxHashMap, FxHashSet};

use crate::utils::*;

struct Scanner {
    beacons: Vec<Pt3>,
    dist_sqrs: Grid<i32>,
}

impl Scanner {
    fn len(&self) -> usize {
        self.beacons.len()
    }

    #[allow(dead_code)]
    fn fmt_table(&self) -> String {
        let mut table = Grid::<String>::filled(self.beacons.len(), self.beacons.len() + 3, "".to_string());
        for r in 0..self.beacons.len() {
            for c in 0..self.beacons.len() {
                table[(r, c)] = format!("{}", self.dist_sqrs[(r, c)]);
            }

            table[(r, self.beacons.len())] = format!("{:?}", self.beacons[r]);
        }
        tabulate(&table)
    }
}

fn build_scanner(beacons: Vec<Pt3>) -> Scanner {
    let mut dist_sqrs = Grid::filled(beacons.len(), beacons.len(), 0);

    for i in 0..beacons.len() {
        for j in 0..beacons.len() {
            dist_sqrs[(i, j)] = beacons[i].dist_to_sqr(&beacons[j]);
        }
    }

    Scanner{beacons, dist_sqrs}
}

fn try_complete_match(scanner_a: &Scanner, scanner_b: &Scanner, mut matches: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    matches.reserve(16);

    // Tracks which beacons have been matched up.
    let mut matched_a = vec![false; scanner_a.len()];
    let mut matched_b = vec![false; scanner_b.len()];
    for (ma, mb) in &matches {
        matched_a[*ma] = true;
        matched_b[*mb] = true;
    }

    // Loops through every potential match of beacon pairs.
    for a in 0..scanner_a.len() {
        if matched_a[a] { continue; }
        for b in 0..scanner_b.len() {
            if matched_b[b] { continue; }

            // Checks for compatibility with accepted matches.
            let mut valid_match = true;
            for (ma, mb) in &matches {
                if scanner_a.dist_sqrs[(a, *ma)] != scanner_b.dist_sqrs[(b, *mb)] {
                    valid_match = false;
                    break;
                }
            }

            if valid_match {
                matches.push((a, b));
                matched_a[a] = true;
                matched_b[b] = true;
            }
        }
    }

    matches
}

// u = R * v
fn rotation_for(u: &Pt3, v: &Pt3) -> Option<Grid<i32>> {
    if u.x.abs() == u.y.abs() || u.x.abs() == u.z.abs() || u.y.abs() == u.z.abs() {
        // Ambiguous. Could get confused.
        return None;
    }

    let mut rot = Grid::filled(3, 3, 0);

    if u.x == v.x {
        rot[(0, 0)] = 1;
    } else if u.x == -v.x {
        rot[(0, 0)] = -1;
    } else if u.x == v.y {
        rot[(0, 1)] = 1;
    } else if u.x == -v.y {
        rot[(0, 1)] = -1;
    } else if u.x == v.z {
        rot[(0, 2)] = 1;
    } else if u.x == -v.z {
        rot[(0, 2)] = -1;
    } else {
        panic!();
    }

    if u.y == v.x {
        rot[(1, 0)] = 1;
    } else if u.y == -v.x {
        rot[(1, 0)] = -1;
    } else if u.y == v.y {
        rot[(1, 1)] = 1;
    } else if u.y == -v.y {
        rot[(1, 1)] = -1;
    } else if u.y == v.z {
        rot[(1, 2)] = 1;
    } else if u.y == -v.z {
        rot[(1, 2)] = -1;
    } else {
        panic!();
    }

    if u.z == v.x {
        rot[(2, 0)] = 1;
    } else if u.z == -v.x {
        rot[(2, 0)] = -1;
    } else if u.z == v.y {
        rot[(2, 1)] = 1;
    } else if u.z == -v.y {
        rot[(2, 1)] = -1;
    } else if u.z == v.z {
        rot[(2, 2)] = 1;
    } else if u.z == -v.z {
        rot[(2, 2)] = -1;
    } else {
        panic!();
    }

    Some(rot)
}

#[derive(Clone)]
struct Transform {
    t: Pt3,
    rot: Grid<i32>,
}

impl Transform {
    fn ident() -> Self {
        let mut tr = Transform{t: Pt3::new(0,0,0), rot: Grid::filled(3, 3, 0)};
        tr.rot[(0,0)] = 1;
        tr.rot[(1,1)] = 1;
        tr.rot[(2,2)] = 1;
        tr
    }

    fn fwd(&self, pt: &Pt3) -> Pt3 {
        let x = self.t.x + self.rot[(0,0)] * pt.x + self.rot[(0,1)] * pt.y + self.rot[(0,2)] * pt.z;
        let y = self.t.y + self.rot[(1,0)] * pt.x + self.rot[(1,1)] * pt.y + self.rot[(1,2)] * pt.z;
        let z = self.t.z + self.rot[(2,0)] * pt.x + self.rot[(2,1)] * pt.y + self.rot[(2,2)] * pt.z;
        Pt3{x, y, z}
    }

    fn inv(&self) -> Transform {
        let mut invrot = Grid::filled(3, 3, 0);
        for r in 0..3 {
            for c in 0..3 {
                invrot[(r, c)] = self.rot[(c, r)];  // Transpose
            }
        }

        // Hacky way of inverting the translation.
        let fwdpt = self.fwd(&Pt3::new(0,0,0));
        let mut inverted = Transform{t: Pt3::new(0,0,0), rot: invrot};
        let resultpt = inverted.fwd(&fwdpt);
        inverted.t.x = -resultpt.x;
        inverted.t.y = -resultpt.y;
        inverted.t.z = -resultpt.z;
        
        inverted
    }

    fn chain(&self, rhs: &Transform) -> Transform {
        let rot = Grid::filled(3, 3, 0);
        let mut tr = Transform{t: Pt3::new(0, 0, 0), rot};

        tr.rot[(0,0)] = self.rot[(0,0)] * rhs.rot[(0,0)] + self.rot[(0,1)] * rhs.rot[(1,0)] + self.rot[(0,2)] * rhs.rot[(2,0)];
        tr.rot[(0,1)] = self.rot[(0,0)] * rhs.rot[(0,1)] + self.rot[(0,1)] * rhs.rot[(1,1)] + self.rot[(0,2)] * rhs.rot[(2,1)];
        tr.rot[(0,2)] = self.rot[(0,0)] * rhs.rot[(0,2)] + self.rot[(0,1)] * rhs.rot[(1,2)] + self.rot[(0,2)] * rhs.rot[(2,2)];

        tr.rot[(1,0)] = self.rot[(1,0)] * rhs.rot[(0,0)] + self.rot[(1,1)] * rhs.rot[(1,0)] + self.rot[(1,2)] * rhs.rot[(2,0)];
        tr.rot[(1,1)] = self.rot[(1,0)] * rhs.rot[(0,1)] + self.rot[(1,1)] * rhs.rot[(1,1)] + self.rot[(1,2)] * rhs.rot[(2,1)];
        tr.rot[(1,2)] = self.rot[(1,0)] * rhs.rot[(0,2)] + self.rot[(1,1)] * rhs.rot[(1,2)] + self.rot[(1,2)] * rhs.rot[(2,2)];

        tr.rot[(2,0)] = self.rot[(2,0)] * rhs.rot[(0,0)] + self.rot[(2,1)] * rhs.rot[(1,0)] + self.rot[(2,2)] * rhs.rot[(2,0)];
        tr.rot[(2,1)] = self.rot[(2,0)] * rhs.rot[(0,1)] + self.rot[(2,1)] * rhs.rot[(1,1)] + self.rot[(2,2)] * rhs.rot[(2,1)];
        tr.rot[(2,2)] = self.rot[(2,0)] * rhs.rot[(0,2)] + self.rot[(2,1)] * rhs.rot[(1,2)] + self.rot[(2,2)] * rhs.rot[(2,2)];

        tr.t.x = self.rot[(0,0)] * rhs.t.x + self.rot[(0,1)] * rhs.t.y + self.rot[(0,2)] * rhs.t.z + self.t.x;
        tr.t.y = self.rot[(1,0)] * rhs.t.x + self.rot[(1,1)] * rhs.t.y + self.rot[(1,2)] * rhs.t.z + self.t.y;
        tr.t.z = self.rot[(2,0)] * rhs.t.x + self.rot[(2,1)] * rhs.t.y + self.rot[(2,2)] * rhs.t.z + self.t.z;

        tr
    }
}

impl std::fmt::Debug for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.t, self.rot.fmt_table())
    }
}

// a = T * b
fn solve_scanner_match(a: &Scanner, b: &Scanner, matches: &Vec<(usize, usize)>) -> Transform {
    for j in 1..matches.len() {
        let va = &a.beacons[matches[j].0] - &a.beacons[matches[0].0];
        let vb = &b.beacons[matches[j].1] - &b.beacons[matches[0].1];

        if let Some(rot) = rotation_for(&va, &vb) {
            // Hacky way of getting the translation.
            let rot_only = Transform{t: Pt3::new(0,0,0), rot: rot.clone()};
            let t = &a.beacons[matches[0].0] - &rot_only.fwd(&b.beacons[matches[0].1]);
            return Transform{t, rot};
        }
    }

    panic!("No match found");
}

pub fn day19(test_mode: bool, print: bool) {
    const INPUT: &str = "inputs/input19.txt";
    let file_str = std::fs::read_to_string(INPUT).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let scanners = {
        let mut scanners = Vec::<Scanner>::with_capacity(20);
        let mut beacons = Vec::<Pt3>::with_capacity(30);

        for line in input_str.lines() {
            if line.is_empty() {
                // End of this beacon
                scanners.push(build_scanner(beacons));
                beacons = Vec::with_capacity(30);
            } else if line.starts_with("---") {
                // Scanner id line
            } else {
                let pos: Vec<i32> = line.split(',')
                    .map(|n| n.parse().unwrap()).collect();
                beacons.push(Pt3::new(pos[0], pos[1], pos[2]));
            }
        }
        if !beacons.is_empty() {
            scanners.push(build_scanner(beacons));
        }
        scanners
    };

    // Aggregates by beacon distances, so we can easily seed matches to try.
    let mut lookup_distsqr = FxHashMap::<i32, Vec<(usize, usize, usize)>>::with_capacity_and_hasher(1000, Default::default());

    for (scanner_id, scanner) in scanners.iter().enumerate() {
        let mut dists = Vec::with_capacity(scanner.len().pow(2));
        for i in 0..scanner.len() {
            for j in (i+1)..scanner.len() {
                let distsqr = scanner.dist_sqrs[(i, j)];
                dists.push(distsqr);
                lookup_distsqr.entry(distsqr)
                    .or_insert_with(|| Vec::with_capacity(8))
                    .push((scanner_id, i, j));
            }
        }
    }

    // (scanner A, B) -> num_matches
    let mut scanner_matches = Grid::filled(scanners.len(), scanners.len(), 0usize);
    // (A, B, [(beacon A, beacon B)], Transform(A <- B))
    let mut all_matches = Vec::<(usize, usize, Vec<(usize, usize)>, Transform)>::new();
    for starter_list in lookup_distsqr.values() {
        if starter_list.len() == 1 { continue; }

        for match_idx_a in 0..starter_list.len() {
            for match_idx_b in (match_idx_a+1)..starter_list.len() {
                let seed_a = &starter_list[match_idx_a];
                let seed_b = &starter_list[match_idx_b];

                if scanner_matches[(seed_a.0, seed_b.0)] > 0 {
                    continue; // Already matched
                }

                let accepts = vec![(seed_a.1, seed_b.1), (seed_a.2, seed_b.2)];
                let mut matches = try_complete_match(&scanners[seed_a.0], &scanners[seed_b.0], accepts);
                if matches.len() < 12 {
                    // Tries the other way.
                    let accepts = vec![(seed_a.1, seed_b.2), (seed_a.2, seed_b.1)];
                    matches = try_complete_match(&scanners[seed_a.0], &scanners[seed_b.0], accepts);
                }

                // Records a good matching of these scanners.
                if matches.len() >= 12 {
                    scanner_matches[(seed_a.0, seed_b.0)] = matches.len();
                    scanner_matches[(seed_b.0, seed_a.0)] = matches.len();
                    let tr = solve_scanner_match(&scanners[seed_a.0], &scanners[seed_b.0], &matches);

                    all_matches.push((
                        seed_a.0,
                        seed_b.0,
                        matches,
                        tr
                    ));
                }
            }
        }
    }

    // A very bad topological traversal of the scanners
    let mut solved: Vec<Option<Transform>> = vec![None; scanners.len()];
    solved[0] = Some(Transform::ident());
    'outer: loop {
        for m in &all_matches {
            if solved[m.0].is_some() && solved[m.1].is_none() {
                solved[m.1] = Some(solved[m.0].as_ref().unwrap().chain(&m.3));
            } else if solved[m.0].is_none() && solved[m.1].is_some() {
                solved[m.0] = Some(solved[m.1].as_ref().unwrap().chain(&m.3.inv()));
            }
        }

        for s in &solved {
            if s.is_none() {
                // Something is still not solved.
                continue 'outer;
            }
        }
        break;
    }

    let mut all_beacons = FxHashSet::<Pt3>::with_capacity_and_hasher(30 * scanners.len(), Default::default());
    for i in 0..scanners.len() {
        let tr = solved[i].as_ref().unwrap();

        for beacon_rel in &scanners[i].beacons {
            let beacon_abs = tr.fwd(beacon_rel);
            all_beacons.insert(beacon_abs);
        }
    }

    let part1 = all_beacons.len();
    if print { println!("Part 1: {}", part1); }
    assert_eq!(part1, if test_mode { 79 } else { 436 });

    let mut max_dist = 0;
    for i in 0..scanners.len() {
        for j in (i+1)..scanners.len() {
            max_dist = max(max_dist, solved[i].as_ref().unwrap().t.l1_to(&solved[j].as_ref().unwrap().t));
        }
    }

    let part2 = max_dist;
    if print { println!("Part 2: {}", part2); }
    assert_eq!(part2, if test_mode { 3621 } else { 10918 });
}

const TEST_EXAMPLE: &'static str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";