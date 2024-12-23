use std::{collections::{BTreeSet, BTreeMap}, path::PathBuf};


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


fn parse(input: &str) -> BTreeMap<&str, BTreeSet<&str>>  {
    input.trim().lines().fold(BTreeMap::<&str,BTreeSet<&str>>::default(),|mut map, line| {
        let (a,b) = line.split_once('-').unwrap();
        map.entry(a).or_default().insert(b);
        map.entry(b).or_default().insert(a);
        map
    })
}

fn process1(input: &str) -> usize {
    let adjs = parse(input);
    let mut groups = BTreeSet::default();
    for (k1,set2) in adjs.iter() {
        for k2 in set2.iter().rev().take_while(|k| k > &k1) {
            for k3 in adjs.get(k2).unwrap().iter().rev().take_while(|k| k > &k2) {
                if adjs.get(k3).unwrap().contains(k1) {
                    groups.insert(BTreeSet::from_iter([k1,k2,k3]));
                }
            }
        }
    }
    groups.iter().filter(|group| {
        group.iter().any(|k| k.starts_with("t"))
    }).count()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 7)
}

fn largest_group_finder_rec<'a,'b>(
    group: BTreeSet<&'a str>, 
    adjs: &'b BTreeMap<&'a str, BTreeSet<&'a str>>, 
    largest: &'b mut BTreeSet<&'a str>
) {
    let last = group.iter().last().unwrap();
    for next in adjs.get(last).unwrap().iter().rev().filter(|k| k> &last) {
        if group.is_subset(adjs.get(next).unwrap()) {
            let mut next_group = group.clone();
            next_group.insert(*next);
            if next_group.len() > largest.len() {
                *largest = next_group.clone();
            }
            largest_group_finder_rec(next_group, adjs, largest);
        }
    }
}


fn process2(input: &str) -> String {
    let adjs = parse(input);
    let mut largest = BTreeSet::default();
    for k1 in adjs.keys().copied() {
        largest_group_finder_rec(BTreeSet::from_iter([k1]), &adjs, &mut largest);
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
    println!("Result part 1: {result} in {:?}",start.elapsed());
    let start = std::time::Instant::now();
    let result = process2(&input);
    println!("Result part 2: {result} in {:?}",start.elapsed());
}
