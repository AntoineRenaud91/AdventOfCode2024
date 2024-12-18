use std::path::PathBuf;

#[cfg(test)]
const TEST_INPUT_1: &str = "
    Register A: 729
    Register B: 0
    Register C: 0

    Program: 0,1,5,4,3,0
";

#[derive(Debug)]
struct Program {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    program: Vec<(Instruction, u64)>,
    program_digits: Vec<u64>,
    pointer: usize,
    outputs: Vec<u64>,
}

impl Program {
    fn set_anew(&mut self, value: u64) {
        self.register_a = value;
        self.register_b = 0;
        self.register_c = 0;
        self.pointer = 0;
        self.outputs = vec![];
    }

    fn out_str(&self) -> String {
        self.outputs
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn solve(&mut self) {
        while !self.apply_next() {}
    }

    fn apply_next(&mut self) -> bool {
        let (instr, operand) = &self.program[self.pointer];
        match instr {
            Instruction::Adv => {
                self.register_a /= 2u64.pow(self.combo(*operand) as u32);
                self.pointer += 1;
            }
            Instruction::Bxl => {
                self.register_b ^= operand;
                self.pointer += 1;
            }
            Instruction::Bst => {
                self.register_b = self.combo(*operand) % 8;
                self.pointer += 1;
            }
            Instruction::Jnz => {
                if self.register_a == 0 {
                    self.pointer += 1;
                } else {
                    self.pointer = self.combo(*operand) as usize;
                }
            }
            Instruction::Bxc => {
                self.register_b ^= self.register_c;
                self.pointer += 1;
            }
            Instruction::Out => {
                self.outputs.push(self.combo(*operand) % 8);
                self.pointer += 1;
            }
            Instruction::Bdv => {
                self.register_b = self.register_a / 2u64.pow(self.combo(*operand) as u32);
                self.pointer += 1;
            }
            Instruction::Cdv => {
                self.register_c = self.register_a / 2u64.pow(self.combo(*operand) as u32);
                self.pointer += 1;
            }
        }
        self.pointer >= self.program.len()
    }

    fn combo(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid operand"),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl TryFrom<u8> for Instruction {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Instruction::Adv),
            1 => Ok(Instruction::Bxl),
            2 => Ok(Instruction::Bst),
            3 => Ok(Instruction::Jnz),
            4 => Ok(Instruction::Bxc),
            5 => Ok(Instruction::Out),
            6 => Ok(Instruction::Bdv),
            7 => Ok(Instruction::Cdv),
            _ => Err(()),
        }
    }
}

fn parse(input: &str) -> Program {
    let mut lines = input.trim().lines();
    let register_a = lines
        .next()
        .unwrap()
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let register_b = lines
        .next()
        .unwrap()
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let register_c = lines
        .next()
        .unwrap()
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let mut program = vec![];
    let program_digits: Vec<u64> = lines
        .nth(1)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    for chunk in program_digits.chunks(2) {
        program.push((Instruction::try_from(chunk[0] as u8).unwrap(), chunk[1]));
    }
    Program {
        register_a,
        register_b,
        register_c,
        program,
        program_digits,
        pointer: 0,
        outputs: vec![],
    }
}

fn process1(input: &str) -> String {
    let mut program = parse(input);
    program.solve();
    program.out_str()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT_1), "4,6,3,5,6,3,5,2,1,0")
}

fn process2(input: &str) -> u64 {
    let mut program = parse(input);
    let program_len = program.program_digits.len();
    let mut base_8 = vec![];
    let mut value = 0;
    for n in 0..program_len {
        base_8.push(0);
        for i in 0.. {
            base_8[n] = i;
            value = (0..base_8.len())
                .map(|i| base_8[n - i] * 8u64.pow(i as u32))
                .sum();
            program.set_anew(value);
            program.solve();
            if program.outputs == program.program_digits[program_len - n - 1..] {
                break;
            }
        }
    }
    value
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
