use std::path::PathBuf;

#[cfg(test)]
const TEST_INPUT: &str = "
    #####
    .####
    .####
    .####
    .#.#.
    .#...
    .....

    #####
    ##.##
    .#.##
    ...##
    ...#.
    ...#.
    .....

    .....
    #....
    #....
    #...#
    #.#.#
    #.###
    #####

    .....
    .....
    #.#..
    ###..
    ###.#
    ###.#
    #####

    .....
    .....
    .....
    #....
    #.#..
    #.#.#
    #####
";

fn parse(input: &str) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let mut locks = vec![];
    let mut keys = vec![];
    for pattern in input.trim().split("\n\n") {
        let mut lines = pattern.lines();
        let storage = if lines.next().unwrap().trim() == "#####" {
            &mut locks
        } else {
            &mut keys
        };
        let mut obj = [0; 5];
        for line in lines.take(5) {
            for (i, char) in line.trim().chars().enumerate() {
                if char == '#' {
                    obj[i] += 1
                }
            }
        }
        storage.push(obj);
    }
    (locks, keys)
}

#[test]
fn test_parse() {
    let (locks, keys) = parse(TEST_INPUT);
    assert_eq!(locks.len(), 2);
    assert_eq!(keys.len(), 3);
    assert_eq!(locks[0], [0, 5, 3, 4, 3]);
    assert_eq!(keys[0], [5, 0, 2, 1, 3]);
}

fn process1(input: &str) -> usize {
    let (locks, keys) = parse(input);
    locks
        .iter()
        .flat_map(|lock| {
            keys.iter()
                .filter(|key| lock.iter().zip(key.iter()).all(|(l, k)| l + k < 6))
        })
        .count()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 3)
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
}
