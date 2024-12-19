use std::{collections::HashMap, path::PathBuf};

#[cfg(test)]
const TEST_INPUT: &str = "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (towels, design) = input.trim().split_once("\n\n").unwrap();
    let towels = towels.trim().split(", ").collect();
    let design = design.trim().lines().collect();
    (towels, design)
}

fn is_desing_doable<'a>(
    design: &'a str,
    towels: &[&str],
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    if let Some(&x) = memo.get(design) {
        return x;
    }
    if design.is_empty() {
        return true;
    }
    let mut doable = false;
    for towel in towels {
        if let Some(design) = design.strip_prefix(towel) {
            doable = is_desing_doable(design, towels, memo);
            if doable {
                break;
            }
        }
    }
    memo.insert(design, doable);
    doable
}

fn process1(input: &str) -> usize {
    let (towels, design) = parse(input);
    let mut memo = HashMap::new();
    design
        .into_iter()
        .filter(|design| is_desing_doable(design, &towels, &mut memo))
        .count()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 6)
}

fn count_patterns<'a>(
    design: &'a str,
    towels: &[&str],
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&x) = memo.get(design) {
        return x;
    }
    if design.is_empty() {
        return 1;
    }
    let mut count = 0;
    for towel in towels {
        if let Some(design) = design.strip_prefix(towel) {
            count += count_patterns(design, towels, memo);
        }
    }
    memo.insert(design, count);
    count
}

fn process2(input: &str) -> usize {
    let (towels, design) = parse(input);
    let mut memo = HashMap::new();
    design
        .into_iter()
        .map(|design| count_patterns(design, &towels, &mut memo))
        .sum()
}

#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT), 16)
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
