use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
fn main() {
    // Create path for file
    let path = Path::new("input");

    // Open path in RO mode
    let mut file = File::open(&path).unwrap();

    // Read file contents into string
    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();

    // Split string by lines and parse to ints
    let reports_vec: Vec<String> = file_string.lines().map(String::from).collect();

    let mut safe_reports: i32 = 0;
    for line in reports_vec {
        let mut report = line
            .split_whitespace()
            .map(|string| string.to_string().parse::<i32>().unwrap());

        // The first level cannot indicate danger, so record it and go to the next one
        let mut last_level: i32 = report.next().unwrap();
        let mut safe: bool = true;
        let mut increasing: bool;
        let mut was_increasing: bool = false;
        let mut first_loop: bool = true;
        for level in report {
            increasing = level > last_level;
            if first_loop {
                was_increasing = increasing;
            }

            safe = safe && (level - last_level).abs() <= 3; // Check if the level steps by more than 3
            safe = safe && (level != last_level);           // Check if level stayed the same
            safe = safe && (increasing == was_increasing);  // Check if level change direction flips
                                                                          
            if safe {
                break;
            }

            last_level = level;
            was_increasing = increasing;
            first_loop = false;
        }
        if safe {
            safe_reports += 1;
        }
    }
    print!("{}\n", safe_reports);
    // print!("{}\n", report_counter);
}
