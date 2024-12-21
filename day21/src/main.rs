use std::{cmp::Ordering::*, collections::HashMap, path::PathBuf};

#[cfg(test)]
const TEST_INPUT: &str = "
    029A
    980A
    179A
    456A
    379A
";

const NUMPAD_POSITIONS: [[i32; 2]; 11] = [
    [1, 0], // 0
    [0, 1], // 1
    [1, 1], // 2
    [2, 1], // 3
    [0, 2], // 4
    [1, 2], // 5
    [2, 2], // 6
    [0, 3], // 7
    [1, 3], // 8
    [2, 3], // 9
    [2, 0], // A
];

const NUMPAD_HOLE_Y: i32 = 0;

const DIRPAD_POSITIONS: [[i32; 2]; 5] = [
    [0, 0], // <
    [1, 0], // v
    [2, 0], // >
    [1, 1], // ^
    [2, 1], // A
];

const DIRPAD_HOLE_Y: i32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
enum DirKey {
    Left = 0,
    Down = 1,
    Right = 2,
    Up = 3,
    A = 4,
}
use DirKey::*;

impl From<usize> for DirKey {
    fn from(value: usize) -> Self {
        match value {
            0 => Left,
            1 => Down,
            2 => Right,
            3 => Up,
            4 => A,
            _ => panic!("Invalid value {value}"),
        }
    }
}

fn parse(line: &str) -> (Vec<usize>, usize) {
    let num = line.trim().strip_suffix('A').unwrap().parse().unwrap();
    let nums = line
        .trim()
        .chars()
        .map(|c| match c {
            '0'..='9' => c.to_digit(10).unwrap() as usize,
            'A' => 10,
            _ => panic!("Invalid char {c}"),
        })
        .collect();
    (nums, num)
}

fn get_all_paths<const HOLE_Y: i32>(start: [i32; 2], end: [i32; 2]) -> Vec<Vec<DirKey>> {
    let [i_start, j_start] = start;
    let [i_end, j_end] = end;
    let mut paths = vec![];
    match (i_start.cmp(&i_end), j_start.cmp(&j_end)) {
        (Equal, Equal) => paths.push(vec![]),
        (Equal, Greater) => {
            let n_down = (j_start - j_end) as usize;
            paths.push(vec![Down; n_down]);
        }
        (Equal, Less) => {
            let n_up = (j_end - j_start) as usize;
            paths.push(vec![Up; n_up]);
        }
        (Greater, Equal) => {
            let n_left = (i_start - i_end) as usize;
            paths.push(vec![Left; n_left]);
        }
        (Less, Equal) => {
            let n_right = (i_end - i_start) as usize;
            paths.push(vec![Right; n_right]);
        }
        (Greater, Greater) => {
            let n_left = (i_start - i_end) as usize;
            let n_down = (j_start - j_end) as usize;
            if !(j_start == HOLE_Y && i_end == 0) {
                paths.push({
                    let mut path = vec![Left; n_left];
                    path.extend(std::iter::repeat(Down).take(n_down));
                    path
                });
            }
            if !(i_start == 0 && j_end == HOLE_Y) {
                paths.push({
                    let mut path = vec![Down; n_down];
                    path.extend(std::iter::repeat(Left).take(n_left));
                    path
                });
            }
        }
        (Greater, Less) => {
            let n_left = (i_start - i_end) as usize;
            let n_up = (j_end - j_start) as usize;
            if !(j_start == HOLE_Y && i_end == 0) {
                paths.push({
                    let mut path = vec![Left; n_left];
                    path.extend(std::iter::repeat(Up).take(n_up));
                    path
                });
            }
            paths.push({
                let mut path = vec![Up; n_up];
                path.extend(std::iter::repeat(Left).take(n_left));
                path
            });
        }
        (Less, Greater) => {
            let n_right = (i_end - i_start) as usize;
            let n_down = (j_start - j_end) as usize;
            paths.push({
                let mut path = vec![Right; n_right];
                path.extend(std::iter::repeat(Down).take(n_down));
                path
            });
            if !(i_start == 0 && j_end == HOLE_Y) {
                paths.push({
                    let mut path = vec![Down; n_down];
                    path.extend(std::iter::repeat(Right).take(n_right));
                    path
                });
            }
        }
        (Less, Less) => {
            let n_right = (i_end - i_start) as usize;
            let n_up = (j_end - j_start) as usize;
            paths.push({
                let mut path = vec![Right; n_right];
                path.extend(std::iter::repeat(Up).take(n_up));
                path
            });
            if !(i_start == 0 && j_end == HOLE_Y) {
                paths.push({
                    let mut path = vec![Up; n_up];
                    path.extend(std::iter::repeat(Right).take(n_right));
                    path
                });
            }
        }
    };
    paths
}

#[test]
fn test_get_all_paths() {
    assert_eq!(get_all_paths::<NUMPAD_HOLE_Y>([1, 1], [1, 1]), vec![vec![]]);
    assert_eq!(
        get_all_paths::<NUMPAD_HOLE_Y>([2, 0], [0, 1]),
        vec![vec![Up, Left, Left]]
    );
    assert_eq!(
        get_all_paths::<NUMPAD_HOLE_Y>([0, 2], [2, 0]),
        vec![vec![Right, Right, Down, Down]]
    );
    assert_eq!(
        get_all_paths::<NUMPAD_HOLE_Y>([1, 1], [1, 2]),
        vec![vec![Up]]
    );
    assert_eq!(
        get_all_paths::<NUMPAD_HOLE_Y>([1, 1], [2, 1]),
        vec![vec![Right]]
    );
    assert_eq!(
        get_all_paths::<NUMPAD_HOLE_Y>([1, 1], [0, 1]),
        vec![vec![Left]]
    );
    assert_eq!(
        get_all_paths::<NUMPAD_HOLE_Y>([1, 1], [2, 2]),
        vec![vec![Right, Up], vec![Up, Right]]
    );
    assert_eq!(
        get_all_paths::<DIRPAD_HOLE_Y>([1, 1], [0, 0]),
        vec![vec![Down, Left]]
    );
    assert_eq!(
        get_all_paths::<DIRPAD_HOLE_Y>([0, 0], [2, 1]),
        vec![vec![Right, Right, Up]]
    );
    assert_eq!(
        get_all_paths::<DIRPAD_HOLE_Y>(
            DIRPAD_POSITIONS[A as usize],
            DIRPAD_POSITIONS[Right as usize]
        ),
        vec![vec![Down]]
    );
}

fn path_cost(
    mut path: Vec<DirKey>,
    memo: &mut HashMap<Vec<DirKey>, HashMap<usize, usize>>,
    depth: usize,
) -> usize {
    if depth == 0 {
        return path.len() + 1;
    }
    if let Some(depth_memo) = memo.get(&path) {
        if let Some(cost) = depth_memo.get(&depth) {
            return *cost;
        }
    }
    let key = path.clone();
    path.push(A);
    let mut curr = A;
    let mut cost = 0;
    for next in path {
        if curr == next {
            cost += 1;
            continue;
        }
        cost += get_all_paths::<DIRPAD_HOLE_Y>(
            DIRPAD_POSITIONS[curr as usize],
            DIRPAD_POSITIONS[next as usize],
        )
        .into_iter()
        .map(|p| path_cost(p, memo, depth - 1))
        .min()
        .unwrap();
        curr = next;
    }
    memo.entry(key).or_default().insert(depth, cost);
    cost
}

#[test]
fn test_path_cost() {
    let mut memo = HashMap::default();
    assert_eq!(path_cost(vec![Down, Left, Left], &mut memo, 0), 4); // Av<<A => 4
    assert_eq!(path_cost(vec![Down], &mut memo, 1), 6); // AvA => Av<A>^A => 6
    assert_eq!(path_cost(vec![Down, Left], &mut memo, 1), 9); // Av<A => Av<A<A>>^A => 9
    assert_eq!(path_cost(vec![Down, Left, Left], &mut memo, 1), 10); // Av<<A => Av<A<AA>^A => 10
    assert_eq!(path_cost(vec![Right], &mut memo, 2), 10); // A>A => AvA^A =>  Av<A>^A<A>A => 10
    assert_eq!(path_cost(vec![Right, Down], &mut memo, 2), 21); // A>vA => AvA<A>^A => Av<A>^Av<<A>>^AvA<^A>A => 21
}

fn process(input: &str, n_rooms: usize) -> usize {
    let mut memo = HashMap::new();
    input
        .trim()
        .lines()
        .map(|line| {
            let (nums, num) = parse(line);
            let mut cost = 0;
            let mut current = 10;
            for next in nums {
                cost += get_all_paths::<NUMPAD_HOLE_Y>(
                    NUMPAD_POSITIONS[current],
                    NUMPAD_POSITIONS[next],
                )
                .into_iter()
                .map(|p| path_cost(p, &mut memo, n_rooms))
                .min()
                .unwrap();
                current = next;
            }
            println!("{}: {cost} * {num}", line.trim());
            cost * num
        })
        .sum()
}

#[test]
fn test_process1() {
    assert_eq!(process(TEST_INPUT, 2), 126384)
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(concat!("data/", env!("CARGO_PKG_NAME"), ".dat"));
    let input = std::fs::read_to_string(path).unwrap();
    let start = std::time::Instant::now();
    let result = process(&input, 2);
    println!("Result part 1: {result} in {:?}", start.elapsed());
    let start = std::time::Instant::now();
    let result = process(&input, 25);
    println!("Result part 2: {result} in {:?}", start.elapsed());
}
