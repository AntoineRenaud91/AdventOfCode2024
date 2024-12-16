use std::env;
use std::fs;

const TEMPLATE: &str = r#"use std::path::PathBuf;


#[cfg(test)]
const TEST_INPUT: &str = "

";


fn parse(input: &str)  {
    todo!()
}

fn process1(input: &str) -> usize {
    todo!()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 55312)
}


fn process2(input: &str) -> usize {
    todo!()
}


#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT), 55312)
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
"#;

fn main() {
    // Get day name from command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo new-day <day-name>");
        std::process::exit(1);
    }

    let day_name = &args[1];
    let workspace_root = env::current_dir().unwrap();
    let day_path = workspace_root.join(format!("day{}", day_name));
    if day_path.exists() {
        eprintln!("Day project already exists: day{}", day_name);
        std::process::exit(1);
    }

    // Create new cargo project
    fs::create_dir_all(&day_path).unwrap();

    // Create Cargo.toml
    let cargo_toml_path = day_path.join("Cargo.toml");
    fs::write(
        cargo_toml_path,
        format!(
            r#"[package]
name = "day{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
            day_name
        ),
    )
    .unwrap();

    // Create src directory
    fs::create_dir_all(day_path.join("src")).unwrap();

    // Create main.rs with template
    let main_rs_path = day_path.join("src/main.rs");
    fs::write(main_rs_path, TEMPLATE).unwrap();

    println!("Created new day project: day{}", day_name);
}
