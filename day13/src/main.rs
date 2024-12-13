use std::path::PathBuf;


#[cfg(test)]
const TEST_INPUT: &str = "
    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176

    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450

    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279
";

struct Vector {
    x: i64,
    y: i64,
}

struct Game {
    button_a: Vector,
    button_b: Vector,
    prize: Vector,
}

impl Game {
    fn inc_prize(self, n: i64) -> Game {
        Game {
            button_a: self.button_a,
            button_b: self.button_b,
            prize: Vector { x: self.prize.x+n, y: self.prize.y+n },
        }
    }

    fn solve(&self) -> Option<Vector> {
        let Game { button_a, button_b, prize } = self;
        let i = button_b.y * prize.x - button_b.x * prize.y;
        let j = -button_a.y * prize.x + button_a.x * prize.y;
        let det = (button_a.x * button_b.y) - (button_a.y * button_b.x);
        if det == 0 || i % det != 0 || j % det != 0 {
            None
        } else {
            Some(Vector{x: i/det, y: j/det})
        }
    }
}

fn parse(input: &str) -> impl Iterator<Item =Game> + '_ {
    input.trim().split("\n\n").map(|group| {
        let mut lines = group.lines();
        let button_a = lines.next().unwrap();
        let button_b = lines.next().unwrap();
        let prize = lines.next().unwrap();
        let button_a = button_a.split_once(':').unwrap().1.split(", ").map(|s| s.split_once('+').unwrap().1.parse().unwrap()).collect::<Vec<_>>();
        let button_b = button_b.split_once(':').unwrap().1.split(", ").map(|s| s.split_once('+').unwrap().1.parse().unwrap()).collect::<Vec<_>>();
        let prize = prize.split_once(':').unwrap().1.split(", ").map(|s| s.split_once('=').unwrap().1.parse().unwrap()).collect::<Vec<_>>();
        Game {
            button_a: Vector {x: button_a[0], y: button_a[1]},
            button_b: Vector { x: button_b[0], y: button_b[1]},
            prize: Vector { x: prize[0], y: prize[1] }
        }
    })
}

fn process1(input: &str) -> i64 {
    parse(input)
        .filter_map(|game| game.solve().map(|r| (r.x*3+r.y)))
        .sum()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 480)
}


fn process2(input: &str) ->i64 {
    parse(input)
        .filter_map(|game| game.inc_prize(10000000000000).solve().map(|r| (r.x*3+r.y)))
        .sum()
}


fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(concat!("data/", env!("CARGO_PKG_NAME"), ".dat"));
    let input = std::fs::read_to_string(path).unwrap();
    let start = std::time::Instant::now();
    let result = process1(&input);
    println!("Result part 1: {result} in {:?}",start.elapsed());
    let start = std::time::Instant::now();
    let result = process2(&input);
    println!("Result part 2: {result} in {:?}",start.elapsed());
}
