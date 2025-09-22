/*
   Determine which reports are safe.
   - A report only counts as safe if both are true
       - the levels are either all increasing or all decreasing
       - any adjacent levels differ by at least one, and at more 3
   Goal: Figure out how many reports are safe

   - monotonically increasing/decreasing
   - and the diff between any element and next is >=1 and <=3
*/
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    process_input("resources/input.txt")
}

fn process_input(input_str: &str) {
    let file = File::open(input_str).expect("Failed to open file");
    let buffer = BufReader::new(file);
    let mut safe_count = 0;
    let mut safe_count_part2 = 0;
    for line in buffer.lines() {
        if let Ok(line) = line {
            let mut report = Vec::new();
            for level in line.split_whitespace() {
                report.push(level.parse::<i32>().unwrap());
            }
            let is_safe = is_safe_report(report.clone());
            safe_count += is_safe as u32;
            println!("Found report: {:?} with safe: {}", report, is_safe);
            let is_safe_part2 = is_safe_report_part2(report.clone());
            println!("Found report: {:?} with safety_part 2: {}", report, is_safe_part2);
            safe_count_part2 += is_safe_part2 as u32;
        }
    }
    println!("Safe count: {}", safe_count);
    println!("Safe count part 2: {}", safe_count_part2);

    fn is_safe_report(vec: Vec<i32>) -> bool {
        if vec.len() <= 1 {
            return true;
        }

        let mut is_increasing = true;
        let mut diff: i32 = vec[1] - vec[0];
        if diff < 0 {
            is_increasing = false;
        }
        diff = diff.abs();
        if diff < 1 || diff> 3 {
            return false;
        }
        for i in 1..vec.len() {
            diff = vec[i] - vec[i - 1];
            if diff < 0 && is_increasing {
               return false;
            }
            if diff > 0 && !is_increasing {
                return false;
            }
            diff = diff.abs();
            if diff < 1 || diff> 3 {
                return false;
            }
        }
        true
    }

    fn is_safe_report_part2(vec: Vec<i32>) -> bool {
        let mut combinations = Vec::new();
        combinations.push(vec.clone());
        for i in 0..vec.len() {
            let mut new_vec = vec.clone();
            new_vec.remove(i);
            combinations.push(new_vec);
        }
        let mut is_safe = false;
        for combination in combinations {
           is_safe = is_safe || is_safe_report(combination.clone())
        }
        is_safe
    }
}
