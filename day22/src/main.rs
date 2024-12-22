use std::{
    collections::{HashMap, VecDeque},
    path::PathBuf,
};

#[cfg(test)]
const TEST_INPUT1: &str = "
1
10
100
2024
";

fn parse(input: &str) -> impl Iterator<Item = u64> + '_ {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse().unwrap())
}

fn next_secret(mut secret: u64) -> u64 {
    secret = (secret ^ (secret * 64)) % 16777216;
    secret = (secret ^ (secret / 32)) % 16777216;
    (secret ^ (secret * 2048)) % 16777216
}

#[test]
fn test_next_secret() {
    assert_eq!(next_secret(123), 15887950)
}

fn apply_n(mut secret: u64, n: usize) -> u64 {
    for _ in 0..n {
        secret = next_secret(secret);
    }
    secret
}

#[test]
fn test_apply_n() {
    assert_eq!(apply_n(1, 2000), 8685429);
    assert_eq!(apply_n(1, 2000), 8685429)
}

fn process1(input: &str) -> u64 {
    parse(input).map(|secret| apply_n(secret, 2000)).sum()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT1), 37327623)
}

#[cfg(test)]
const TEST_INPUT2: &str = "
    1
    2
    3
    2024
";

fn get_sequence_map(mut secret: u64, n: usize) -> HashMap<VecDeque<i64>, i64> {
    let mut map = HashMap::<VecDeque<i64>, i64>::default();
    let mut price = (secret % 10) as i64;
    let mut sequence = VecDeque::<i64>::new();
    for _ in 0..n {
        secret = next_secret(secret);
        let next_price = (secret % 10) as i64;
        sequence.push_back(next_price - price);
        if sequence.len() == 5 {
            sequence.pop_front();
            if !map.contains_key(&sequence) {
                map.insert(sequence.clone(), next_price);
            }
        }
        price = next_price;
    }
    map
}

#[test]
fn test_get_sequence_map() {
    let (seq, max) = get_sequence_map(123, 9)
        .into_iter()
        .max_by_key(|(_, b)| *b)
        .unwrap();
    assert_eq!(max, 6);
    assert_eq!(seq, [-1, -1, 0, 2]);
}

fn process2(input: &str) -> i64 {
    let mut map = HashMap::<VecDeque<i64>, i64>::default();
    for secret in parse(input) {
        for (seq, price) in get_sequence_map(secret, 2000) {
            *map.entry(seq).or_default() += price;
        }
    }
    map.into_values().max().unwrap()
}

#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT2), 23)
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
