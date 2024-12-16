use std::path::PathBuf;

#[cfg(test)]
const TEST_INPUT: &str = "
    p=0,4 v=3,-3
    p=6,3 v=-1,-3
    p=10,3 v=-1,2
    p=2,0 v=2,-1
    p=0,0 v=1,3
    p=3,0 v=-2,-2
    p=7,6 v=-1,-3
    p=3,0 v=-1,-2
    p=9,3 v=2,3
    p=7,3 v=-1,2
    p=2,4 v=2,-3
    p=9,5 v=-3,-3
";

fn parse(input: &str) -> impl Iterator<Item = ([i64; 2], [i64; 2])> + '_ {
    input.trim().lines().map(|line| {
        let (p, v) = line.trim().split_once(" v=").unwrap();
        let p = p
            .trim_start_matches("p=")
            .trim_end_matches('>')
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();
        let v = v.split(',').map(|x| x.parse().unwrap()).collect::<Vec<_>>();
        ([p[0], p[1]], [v[0], v[1]])
    })
}

#[test]
fn test_mod() {
    assert_eq!(7i64.rem_euclid(4), 3);
    assert_eq!((-7i64).rem_euclid(4), 1);
    assert_eq!(103 / 2 + 1, 52);
}

fn process1(input: &str, nx: i64, ny: i64) -> usize {
    let quadrant = parse(input)
        .map(move |(pos, vel)| {
            [
                (pos[0] + 100 * vel[0]).rem_euclid(nx),
                (pos[1] + 100 * vel[1]).rem_euclid(ny),
            ]
        })
        .filter(|&[x, y]| x != nx / 2 && y != ny / 2)
        .fold([[0; 2]; 2], |mut acc, [x, y]| {
            let i = if x <= nx / 2 { 0 } else { 1 };
            let j = if y <= ny / 2 { 0 } else { 1 };
            acc[i][j] += 1;
            acc
        });
    quadrant[0][0] * quadrant[1][1] * quadrant[0][1] * quadrant[1][0]
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT, 11, 7), 12)
}

fn process2(input: &str, nx: i64, ny: i64, nt: i64) -> Vec<Vec<char>> {
    parse(input)
        .map(move |(pos, vel)| {
            [
                (pos[0] + nt * vel[0]).rem_euclid(nx),
                (pos[1] + nt * vel[1]).rem_euclid(ny),
            ]
        })
        .fold(
            (0..ny)
                .map(|_| (0..nx).map(|_| '.').collect::<Vec<char>>())
                .collect::<Vec<_>>(),
            |mut acc, [i, j]| {
                let char = &mut acc[j as usize][i as usize];
                if let Some(i) = char.to_digit(10) {
                    *char = std::char::from_digit(i + 1, 10).unwrap();
                } else {
                    *char = '1';
                }
                acc
            },
        )
}

#[test]
fn test_process2() {
    for line in process2(TEST_INPUT, 11, 7, 100) {
        println!("{}", line.iter().collect::<String>());
    }
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(concat!("data/", env!("CARGO_PKG_NAME"), ".dat"));
    let input = std::fs::read_to_string(path).unwrap();
    let start = std::time::Instant::now();
    let result = process1(&input, 101, 103);
    println!("Result part 1: {result} in {:?}", start.elapsed());
    let start = std::time::Instant::now();
    for i in 0.. {
        let message = process2(&input, 101, 103, i);
        let nlines_empty = message
            .iter()
            .filter(|line| line.iter().all(|&c| c == '.'))
            .count();
        let ncols_empty = (0..101)
            .filter(|i| (0..103).all(|j| message[j][*i] == '.'))
            .count();
        if nlines_empty > 10 && ncols_empty > 10 {
            for line in message {
                println!("{}", line.iter().collect::<String>());
            }
            println!("Result part 2: {i} in {:?}", start.elapsed());
            break;
        } else {
            continue;
        }
    }
}
