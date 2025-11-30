use std::path::Path;
use std::process::Command;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- day_<01-12>");
        std::process::exit(1);
    }

    let package_name = &args[1];

    // VALIDATION - package name must be day_<01-12>

    let day_num = package_name
        .strip_prefix("day_")
        .filter(|s| s.len() == 2)
        .and_then(|s| s.parse::<u32>().ok())
        .filter(|&n| 1 <= n && n <= 12)
        .unwrap_or_else(|| {
            eprintln!("Package name must be of the format 'day_<01-12>'");
            std::process::exit(1);
        });

    // PATH SETUP

    let target_dir = Path::new("days").join(package_name);

    if target_dir.exists() {
        eprintln!("Package for {} already exists", package_name);
        std::process::exit(0);
    }

    println!("Scaffolding new package: {}", package_name);

    // CARGO NEW

    let status = Command::new("cargo")
        .arg("new")
        .arg(&target_dir)
        .arg("--quiet")
        .status()
        .expect("Failed to execute cargo new");

    if !status.success() {
        eprintln!("Command 'cargo new' failed");
        std::process::exit(1);
    }
    println!("✓ Created package");

    // WRITE TEMPLATE - and create empty input.txt

    let template_contents = include_str!("template.txt");
    let main_rs_path = target_dir.join("src").join("main.rs");

    match fs::write(&main_rs_path, template_contents) {
        Ok(_) => println!("✓ Overwrote main.rs with template.txt"),
        Err(err) => eprintln!("Failed to write template: {}", err),
    }

    let input_path = target_dir.join("src").join("input.txt");
    match fs::File::create(&input_path) {
        Ok(_) => println!("✓ Created empty input.txt"),
        Err(err) => eprintln!("Failed to create input.txt: {}", err),
    }

    println!("✓ Scaffold complete: {}", main_rs_path.display());

    // CHECK URL - if puzzle url status 404 its probably not released

    let url = format!("https://adventofcode.com/2025/day/{}", day_num);

    match reqwest::blocking::get(&url) {
        Ok(response) => {
            if response.status() == 404 {
                println!("! Puzzle hasn't unlocked yet!")
            } else {
                println!("View puzzle: {}", url);
            }
        }
        _ => {
            eprintln!("Failed to reqwest puzzle page")
        }
    };
}
