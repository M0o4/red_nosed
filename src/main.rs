use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut safe_count = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let levels = line
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect::<Vec<i32>>();

        if is_safe(&levels) {
            safe_count += 1;
        } else {
            if is_safe_with_one_removal(&levels) {
                safe_count += 1;
            }
        }
    }

    println!("Safe Count: {}", safe_count);
}

fn is_safe(levels: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..levels.len() - 1 {
        let diff = (levels[i + 1] - levels[i]).abs();

        if diff < 1 || diff > 3 {
            return false;
        }

        if levels[i] < levels[i + 1] {
            increasing = false;
        }

        if levels[i] > levels[i + 1] {
            decreasing = false;
        }
    }

    increasing || decreasing
}

fn is_safe_with_one_removal(levels: &[i32]) -> bool {
    for i in 0..levels.len() {
        let mut modified_levels = levels.to_vec();
        modified_levels.remove(i);

        if is_safe(&modified_levels) {
            return true;
        }
    }

    false
}
