use std::path::PathBuf;

use itertools::Itertools;


#[cfg(test)]
const TEST_INPUT:&str = "
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
";

fn parse(input: &str) -> Vec<Vec<i64>> {
    input.trim().lines()
        .map(|line| line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect())
        .collect()
}

#[test]
fn test_parser() {
    assert_eq!(parse(TEST_INPUT),(vec![
        vec![7,6,4,2,1],
        vec![1,2,7,8,9],
        vec![9,7,6,2,1],
        vec![1,3,2,4,5],
        vec![8,6,4,4,1],
        vec![1,3,6,7,9]
    ]));
}

fn is_safe(diffs: impl Iterator<Item = i64>) -> bool {
    let mut count_up = 0;
    let mut count_down = 0;
    for diff in diffs {
        match diff {
            ..-3 | 0 | 4.. => {return false},
            ..0 => {count_down+=1},
            1.. => {count_up +=1},
        }
    }
    count_down.min(count_up) == 0
}

fn process1(input: &str) -> usize {
    let data = parse(input);
    data.into_iter().filter(|record| is_safe(record.iter().tuple_windows().map(|(i,j)| j-i))).count()
}


#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT),2)
}

fn is_almost_safe(record: &[i64]) -> bool {
    if is_safe(record.iter().tuple_windows().map(|(i,j)| j-i)) {return true};
    for i in 0..record.len() {
        if is_safe((0..record.len())
            .filter(|j| j!=&i)
            .map(|i| record[i])
            .tuple_windows()
            .map(|(i,j)| j-i)) {
                return true
            }
    };
    false
}


fn process2(input: &str) -> usize {
    let data = parse(input);
    data.into_iter().filter(|record| is_almost_safe(record)).count()
}


#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT),4)
}


fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("data/day2.dat");
    let input = std::fs::read_to_string(path).unwrap();
    let result = process1(&input);
    println!("Day2 result: {result}");
    let result = process2(&input);
    println!("Day2 result: {result}");
}
