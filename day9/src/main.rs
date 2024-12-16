use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashMap},
    path::PathBuf,
};

#[cfg(test)]
const TEST_INPUT: &str = "2333133121414131402";

fn parse(input: &str) -> impl Iterator<Item = u64> + '_ {
    input.trim().chars().map(|c| c.to_digit(10).unwrap() as u64)
}

fn process1(input: &str) -> u64 {
    let mut pos_length_files = BinaryHeap::new();
    let mut pos_length_free = BinaryHeap::new();
    let mut pos = 0;
    let mut id = 0;
    for (i, length) in parse(input).enumerate() {
        if length == 0 {
            continue;
        }
        if i % 2 == 0 {
            pos_length_files.push((pos, length, id));
            id += 1;
        } else {
            pos_length_free.push(Reverse((pos, length)));
        }
        pos += length;
    }
    while pos_length_files.peek().unwrap().0 > pos_length_free.peek().unwrap().0 .0 {
        let Reverse((mut free_pos, mut free_length)) = pos_length_free.pop().unwrap();
        let (file_pos, mut file_length, id) = pos_length_files.pop().unwrap();
        match free_length.cmp(&file_length) {
            std::cmp::Ordering::Less => {
                file_length -= free_length;
                pos_length_files.push((free_pos, free_length, id));
                pos_length_files.push((file_pos, file_length, id));
            }
            std::cmp::Ordering::Greater => {
                pos_length_files.push((free_pos, file_length, id));
                free_length -= file_length;
                free_pos += file_length;
                pos_length_free.push(Reverse((free_pos, free_length)));
            }
            std::cmp::Ordering::Equal => {
                pos_length_files.push((free_pos, file_length, id));
            }
        }
    }
    pos_length_files
        .into_iter()
        .flat_map(|(pos, length, id)| (0..length).map(move |i| (pos + i) * id))
        .sum()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 1928)
}

fn process2(input: &str) -> u64 {
    let mut pos_length_files = HashMap::<u64, (u64, u64)>::new();
    let mut pos_length_free = BTreeMap::new();
    let mut pos = 0;
    let mut id = 0;
    for (i, length) in parse(input).enumerate() {
        if length == 0 {
            continue;
        }
        if i % 2 == 0 {
            pos_length_files.insert(id, (pos, length));
            id += 1;
        } else {
            pos_length_free.insert(pos, length);
        }
        pos += length;
    }
    for id in (0..id).rev() {
        let (pos_file, length_file) = pos_length_files.remove(&id).unwrap();
        if let Some(pos_free) = pos_length_free
            .iter()
            .take_while(|(pos_free, _)| pos_free < &&pos_file)
            .find(|(_, length_free)| length_free >= &&length_file)
            .map(|(pos_free, _)| *pos_free)
        {
            let length_free = pos_length_free.remove(&pos_free).unwrap();
            pos_length_files.insert(id, (pos_free, length_file));
            if length_free > length_file {
                pos_length_free.insert(pos_free + length_file, length_free - length_file);
            }
        } else {
            pos_length_files.insert(id, (pos_file, length_file));
        }
    }
    pos_length_files
        .into_iter()
        .flat_map(|(id, (pos, length))| (0..length).map(move |i| (pos + i) * id))
        .sum()
}

#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT), 2858)
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
