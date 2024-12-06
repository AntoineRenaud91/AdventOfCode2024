use std::{collections::HashMap, path::PathBuf};

#[cfg(test)]
const TEST_INPUT: &str = "
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
";

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .trim()
        .lines()
        .map(|line| line.trim().split_once("   ").unwrap())
        .map(|(l, r)| (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap()))
        .unzip()
}

#[test]
fn test_parser() {
    assert_eq!(
        parse(TEST_INPUT),
        (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
    );
}

fn process1(input: &str) -> i64 {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 11)
}

fn process2(input: &str) -> i64 {
    let (left, right) = parse(input);
    let right = right
        .into_iter()
        .fold(HashMap::<i64, i64>::new(), |mut map, r| {
            *map.entry(r).or_default() += 1;
            map
        });
    left.iter()
        .filter_map(|l| right.get(l).map(|r| l * r))
        .sum()
}

#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT), 31)
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/day1.dat");
    let input = std::fs::read_to_string(path).unwrap();
    let result = process1(&input);
    println!("Day1 result: {result}");
    let result = process2(&input);
    println!("Day2 result: {result}");
}
