use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

#[cfg(test)]
const TEST_INPUT: &str = "
    47|53
    97|13
    97|61
    97|47
    75|29
    61|13
    75|53
    29|13
    97|29
    53|29
    61|53
    97|53
    61|29
    47|13
    75|47
    97|75
    47|61
    75|61
    47|29
    75|13
    53|13

    75,47,61,53,29
    97,61,53,29,13
    75,29,13
    75,97,47,61,53
    61,13,29
    97,13,75,29,47
";

fn parse(input: &str) -> (HashMap<usize, HashSet<usize>>, Vec<Vec<usize>>) {
    let (rules, updates) = input.trim().split_once("\n\n").unwrap();
    let rules = rules.lines().fold(
        HashMap::<usize, HashSet<usize>>::default(),
        |mut map, rule| {
            let (key, val) = rule.trim().split_once("|").unwrap();
            let key = key.parse::<usize>().unwrap();
            let val = val.parse::<usize>().unwrap();
            map.entry(key).or_default().insert(val);
            map
        },
    );
    let updates: Vec<Vec<_>> = updates
        .lines()
        .map(|line| {
            line.trim()
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    (rules, updates)
}

fn process1(input: &str) -> usize {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .filter(|update| {
            let mut prev_set = HashSet::<usize>::default();
            for v in update.iter() {
                if let Some(set) = rules.get(v) {
                    if set.intersection(&prev_set).count() == 0 {
                        prev_set.insert(*v);
                    } else {
                        return false;
                    }
                } else {
                    prev_set.insert(*v);
                }
            }
            true
        })
        .map(|update| update[(update.len() - 1) / 2])
        .sum()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 143)
}

fn process2(input: &str) -> usize {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .filter(|update| {
            let mut prev_set = HashSet::<usize>::default();
            for v in update.iter() {
                if let Some(set) = rules.get(v) {
                    if set.intersection(&prev_set).count() == 0 {
                        prev_set.insert(*v);
                    } else {
                        return true;
                    }
                } else {
                    prev_set.insert(*v);
                }
            }
            false
        })
        .map(|update| {
            let mut prev_set = HashSet::<usize>::default();
            let mut new_updates = vec![];
            for v in update.iter() {
                if let Some(set) = rules.get(v) {
                    let intersection = set
                        .intersection(&prev_set)
                        .copied()
                        .collect::<HashSet<usize>>();
                    if intersection.is_empty() {
                        prev_set.insert(*v);
                        new_updates.push(*v);
                    } else {
                        let i = intersection
                            .iter()
                            .map(|v| {
                                new_updates
                                    .iter()
                                    .enumerate()
                                    .find(|(_, vv)| v == *vv)
                                    .unwrap()
                                    .0
                            })
                            .min()
                            .unwrap();
                        new_updates.insert(i, *v);
                        prev_set.insert(*v);
                    }
                } else {
                    prev_set.insert(*v);
                    new_updates.push(*v);
                }
            }
            new_updates
        })
        .map(|update| update[(update.len() - 1) / 2])
        .sum()
}

#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT), 123)
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(concat!("data/", env!("CARGO_PKG_NAME"), ".dat"));
    let input = std::fs::read_to_string(path).unwrap();
    let result = process1(&input);
    println!("Result part 1: {result}");
    let result = process2(&input);
    println!("Result part 2: {result}");
}
