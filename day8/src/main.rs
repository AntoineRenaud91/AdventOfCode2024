use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use nalgebra::Vector2;

#[cfg(test)]
const TEST_INPUT: &str = "
    ............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............
";

fn parse(input: &str) -> ([i64; 2], HashMap<char, Vec<Vector2<i64>>>) {
    let nrows = input.trim().lines().count();
    let ncols = input.trim().lines().next().unwrap().trim().len();
    let mut map = HashMap::<char, Vec<Vector2<i64>>>::new();
    for (i, line) in input.trim().lines().enumerate() {
        for (j, c) in line.trim().chars().enumerate() {
            if c != '.' {
                map.entry(c)
                    .or_default()
                    .push(Vector2::new(i as i64, j as i64));
            }
        }
    }
    ([nrows as i64, ncols as i64], map)
}

fn process_single1(antena_pos: &[Vector2<i64>], shape: [i64; 2]) -> HashSet<Vector2<i64>> {
    let mut antinodes = HashSet::new();
    for (i, anten_i) in antena_pos.iter().enumerate() {
        for anten_j in antena_pos.iter().skip(i + 1) {
            let vec = anten_j - anten_i;
            let anti_ij = anten_j + vec;
            let anti_ji = anten_i - vec;
            if anti_ij.iter().all(|&x| x >= 0) && anti_ij.iter().zip(&shape).all(|(&x, &y)| x < y) {
                antinodes.insert(anti_ij);
            }
            if anti_ji.iter().all(|&x| x >= 0) && anti_ji.iter().zip(&shape).all(|(&x, &y)| x < y) {
                antinodes.insert(anti_ji);
            }
        }
    }
    antinodes
}

fn process1(input: &str) -> usize {
    let (shape, antena_map) = parse(input);
    antena_map
        .into_values()
        .fold(HashSet::new(), |acc, antena_pos| {
            acc.union(&process_single1(&antena_pos, shape))
                .copied()
                .collect()
        })
        .len()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 14)
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(10, -5), 5);
    assert_eq!(gcd(-10, 3), 1);
    assert_eq!(gcd(10, 0), 10);
    assert_eq!(gcd(0, 10), 10);
}

fn process_single2(antena_pos: &[Vector2<i64>], shape: [i64; 2]) -> HashSet<Vector2<i64>> {
    let mut antinodes = HashSet::new();
    for (i, anten_i) in antena_pos.iter().enumerate() {
        for anten_j in antena_pos.iter().skip(i + 1) {
            let mut vec = anten_j - anten_i;
            vec /= gcd(vec.x, vec.y);
            let mut anti_ij = *anten_j;
            while anti_ij.iter().all(|&x| x >= 0)
                && anti_ij.iter().zip(&shape).all(|(&x, &y)| x < y)
            {
                antinodes.insert(anti_ij);
                anti_ij += vec;
            }
            let mut anti_ji = *anten_i;
            while anti_ji.iter().all(|&x| x >= 0)
                && anti_ji.iter().zip(&shape).all(|(&x, &y)| x < y)
            {
                antinodes.insert(anti_ji);
                anti_ji -= vec;
            }
        }
    }
    antinodes
}

fn process2(input: &str) -> usize {
    let (shape, antena_map) = parse(input);
    antena_map
        .into_values()
        .fold(HashSet::new(), |acc, antena_pos| {
            acc.union(&process_single2(&antena_pos, shape))
                .copied()
                .collect()
        })
        .len()
}

#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT), 34)
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
