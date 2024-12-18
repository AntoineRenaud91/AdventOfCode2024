use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    path::PathBuf,
};

#[cfg(test)]
const TEST_INPUT: &str = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

fn parse(input: &str) -> impl Iterator<Item = [i64; 2]> + '_ {
    input.trim().lines().map(|line| {
        let (a, b) = line.trim().split_once(',').unwrap();
        [a.parse().unwrap(), b.parse().unwrap()]
    })
}

fn process1(input: &str, n_max: i64, n_bytes: usize) -> usize {
    let corrupted = parse(input).take(n_bytes).collect::<HashSet<_>>();
    let target = [n_max - 1; 2];
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, [0; 2])));
    let mut visited = HashSet::new();
    while let Some(Reverse((cost, pos))) = queue.pop() {
        if pos == target {
            return cost;
        }
        if !visited.insert(pos) {
            continue;
        }
        let [i, j] = pos;
        for next_pos in [[i + 1, j], [i - 1, j], [i, j + 1], [i, j - 1]] {
            if next_pos[0] < 0
                || next_pos[1] < 0
                || next_pos[0] == n_max
                || next_pos[1] == n_max
                || corrupted.contains(&next_pos)
            {
                continue;
            }

            queue.push(Reverse((cost + 1, next_pos)));
        }
    }
    panic!("no path found")
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT, 7, 12), 22)
}

fn has_path(corrupted: HashSet<[i64; 2]>, n_max: i64) -> bool {
    let target = [n_max - 1; 2];
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, [0; 2])));
    let mut visited = HashSet::new();
    while let Some(Reverse((cost, pos))) = queue.pop() {
        if pos == target {
            return true;
        }
        if !visited.insert(pos) {
            continue;
        }
        let [i, j] = pos;
        for next_pos in [[i + 1, j], [i - 1, j], [i, j + 1], [i, j - 1]] {
            if next_pos[0] < 0
                || next_pos[1] < 0
                || next_pos[0] == n_max
                || next_pos[1] == n_max
                || corrupted.contains(&next_pos)
            {
                continue;
            }

            queue.push(Reverse((cost + 1, next_pos)));
        }
    }
    false
}

fn process2(input: &str, n_max: i64) -> [i64; 2] {
    let corrupted_all = parse(input).collect::<Vec<_>>();
    let mut n_b = 0;
    let mut n_u = corrupted_all.len();
    let mut n = corrupted_all.len() / 2;
    while n != n_b && n != n_u {
        let corrupted = corrupted_all
            .iter()
            .take(n)
            .copied()
            .collect::<HashSet<_>>();
        if has_path(corrupted, n_max) {
            n_b = n;
        } else {
            n_u = n;
        }
        n = (n_b + n_u) / 2;
    }
    corrupted_all[n]
}

#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT, 7), [6, 1])
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(concat!("data/", env!("CARGO_PKG_NAME"), ".dat"));
    let input = std::fs::read_to_string(path).unwrap();
    let start = std::time::Instant::now();
    let result = process1(&input, 71, 1024);
    println!("Result part 1: {result} in {:?}", start.elapsed());
    let start = std::time::Instant::now();
    let result = process2(&input, 71);
    println!("Result part 2: {result:?} in {:?}", start.elapsed());
}
