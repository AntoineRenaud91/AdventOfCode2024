use regex::Regex;
use std::path::PathBuf;

#[cfg(test)]
const TEST_INPUT: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn process1(input: &str) -> u64 {
    Regex::new(r"mul\(\d{1,3},\d{1,3}\)")
        .unwrap()
        .find_iter(input)
        .map(|mat| {
            let (num1_str, num2_str) = mat
                .as_str()
                .trim_start_matches("mul(")
                .trim_end_matches(")")
                .split_once(',')
                .unwrap();
            num1_str.parse::<u64>().unwrap() * num2_str.parse::<u64>().unwrap()
        })
        .sum()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 161)
}

fn process2(input: &str) -> u64 {
    Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)")
        .unwrap()
        .find_iter(input)
        .fold((0, true), |(sum, enabled), mat| {
            let mat_str = mat.as_str();
            if mat_str == "do()" {
                (sum, true)
            } else if mat_str == "don't()" {
                (sum, false)
            } else if enabled {
                let (num1_str, num2_str) = mat_str
                    .trim_start_matches("mul(")
                    .trim_end_matches(")")
                    .split_once(',')
                    .unwrap();
                (
                    sum + num1_str.parse::<u64>().unwrap() * num2_str.parse::<u64>().unwrap(),
                    true,
                )
            } else {
                (sum, false)
            }
        })
        .0
}

#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT), 48)
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(concat!("data/",env!("CARGO_PKG_NAME"),".dat"));
    let input = std::fs::read_to_string(path).unwrap();
    let result = process1(&input);
    println!("Result part 1: {result}");
    let result = process2(&input);
    println!("Result part 2: {result}");
}
