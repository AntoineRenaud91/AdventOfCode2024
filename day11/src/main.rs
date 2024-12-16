use std::{collections::HashMap, path::PathBuf};

#[cfg(test)]
const TEST_INPUT: &str = "125 17";

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn try_split(n: u64) -> Option<[u64; 2]> {
    let n = n.to_string();
    if n.len() % 2 == 0 {
        Some([
            n[..n.len() / 2].parse().unwrap(),
            n[n.len() / 2..].parse().unwrap(),
        ])
    } else {
        None
    }
}

fn process(input: &str, n: usize) -> usize {
    let mut stones = parse(input)
        .into_iter()
        .map(|n| (n, 1))
        .collect::<HashMap<_, _>>();
    let mut stones_iter: std::collections::hash_map::IntoIter<u64, usize>;
    for _ in 0..n {
        (stones, stones_iter) = (HashMap::with_capacity(stones.len()), stones.into_iter());
        for (stone, count) in stones_iter {
            if stone == 0 {
                *stones.entry(1).or_default() += count;
            } else if let Some([stone_1, stone_2]) = try_split(stone) {
                *stones.entry(stone_1).or_default() += count;
                *stones.entry(stone_2).or_default() += count;
            } else {
                *stones.entry(stone * 2024).or_default() += count;
            }
        }
    }
    stones.values().sum()
}

#[test]
fn test_process() {
    assert_eq!(process(TEST_INPUT, 25), 55312)
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(concat!("data/", env!("CARGO_PKG_NAME"), ".dat"));
    let input = std::fs::read_to_string(path).unwrap();
    let start = std::time::Instant::now();
    let result = process(&input, 25);
    println!("Result part 1: {result} in {:?}", start.elapsed());
    let start = std::time::Instant::now();
    let result = process(&input, 75);
    println!("Result part 2: {result} in {:?}", start.elapsed());
}
