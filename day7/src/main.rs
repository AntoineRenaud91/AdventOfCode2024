use std::path::PathBuf;

#[cfg(test)]
const TEST_INPUT: &str = "
    190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20
";

fn parse(input: &str) -> impl Iterator<Item = (u64, Vec<u64>)> + '_ {
    input.trim().lines().map(|line| {
        let (a, b) = line.trim().split_once(":").unwrap();
        (
            a.parse().unwrap(),
            b.split_whitespace().map(|x| x.parse().unwrap()).collect(),
        )
    })
}

fn process_single1(total: u64, curr_value: u64, mut rev_nexts: Vec<u64>) -> bool {
    if curr_value > total {
        return false;
    }
    if let Some(next) = rev_nexts.pop() {
        process_single1(total, curr_value.saturating_add(next), rev_nexts.clone())
            || process_single1(total, curr_value.saturating_mul(next), rev_nexts)
    } else {
        curr_value == total
    }
}

fn process1(input: &str) -> u64 {
    parse(input)
        .filter_map(|(total, mut nexts)| {
            nexts.reverse();
            if process_single1(total, nexts.pop().unwrap(), nexts) {
                Some(total)
            } else {
                None
            }
        })
        .sum()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 3749)
}

fn concat_digits(a: u64, b: u64) -> u64 {
    let mut b_digits = 0;
    let mut temp_b = b;
    while temp_b > 0 {
        b_digits += 1;
        temp_b /= 10;
    }
    a * 10_u64.pow(b_digits) + b
}

#[test]
fn test_concat_digits() {
    assert_eq!(concat_digits(123, 456), 123456);
    assert_eq!(concat_digits(432, 1), 4321)
}

fn process_single2(total: u64, curr_value: u64, mut rev_nexts: Vec<u64>) -> bool {
    if curr_value > total {
        return false;
    }
    if let Some(next) = rev_nexts.pop() {
        process_single2(total, curr_value.saturating_add(next), rev_nexts.clone())
            || process_single2(total, curr_value.saturating_mul(next), rev_nexts.clone())
            || process_single2(total, concat_digits(curr_value, next), rev_nexts)
    } else {
        curr_value == total
    }
}

fn process2(input: &str) -> u64 {
    parse(input)
        .filter_map(|(total, mut nexts)| {
            nexts.reverse();
            if process_single2(total, nexts.pop().unwrap(), nexts) {
                Some(total)
            } else {
                None
            }
        })
        .sum()
}

#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT), 11387)
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
