use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

#[cfg(test)]
const TEST_INPUT: &str = "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

fn parse(input: &str) -> BTreeMap<&str, BTreeSet<&str>> {
    input.trim().lines().fold(
        BTreeMap::<&str, BTreeSet<&str>>::default(),
        |mut map, line| {
            let (a, b) = line.split_once('-').unwrap();
            map.entry(a).or_default().insert(b);
            map.entry(b).or_default().insert(a);
            map
        },
    )
}

fn process1(input: &str) -> usize {
    let adjs = parse(input);
    let mut groups = vec![];
    for (k1, set2) in adjs.iter() {
        for k2 in set2.iter().rev().take_while(|k| k > &k1) {
            for k3 in adjs.get(k2).unwrap().iter().rev().take_while(|k| k > &k2) {
                if adjs.get(k3).unwrap().contains(k1) {
                    groups.push(vec![k1, k2, k3]);
                }
            }
        }
    }
    groups
        .iter()
        .filter(|group| group.iter().any(|k| k.starts_with("t")))
        .count()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 7)
}

fn process2(input: &str) -> String {
    let adjs = parse(input);
    let mut largest = BTreeSet::default();
    'outer: for (k1, nexts) in adjs.iter() {
        if nexts.len() < largest.len() {
            continue;
        }
        let mut set = BTreeSet::from([*k1]);
        for (i, next) in nexts.iter().enumerate() {
            if set.is_subset(adjs.get(next).unwrap()) {
                set.insert(next);
            }
            if set.len() + nexts.len() - i <= largest.len() {
                continue 'outer;
            }
        }
        largest = set;
    }
    largest.iter().cloned().collect::<Vec<&str>>().join(",")
}

#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT), "co,de,ka,ta")
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(concat!("data/", env!("CARGO_PKG_NAME"), ".dat"));
    let input = std::fs::read_to_string(path).unwrap();
    let start = std::time::Instant::now();
    let result = process1(&input);
    println!("Result part 1: {result} in {:?}", start.elapsed());
    let start = std::time::Instant::now();
    let result = process2(&input);
    println!("Result part 2: {result} in {:?}", start.elapsed());
}
