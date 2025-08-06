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

        let mut increasing = true;
        let mut decreasing = true;
        let mut safe = true;


        for i in 0..levels.len() - 1 {
            let diff = (levels[i + 1] - levels[i]).abs();

            if diff < 1 || diff > 3 {
                safe = false;
                break;
            }

            if levels[i] < levels[i + 1] {
                increasing = false;
            }

            if levels[i] > levels[i + 1] {
                decreasing = false;
            }
        }

        if safe && (increasing || decreasing) {
            safe_count += 1;
        }
    }

    println!("Safe Count: {}", safe_count);
}
