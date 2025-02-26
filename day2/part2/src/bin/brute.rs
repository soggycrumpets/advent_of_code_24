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
    let mut linecount: i32 = 0;
    for line in reports_vec {
        linecount += 1;
        let report = line
            .split_whitespace()
            .map(|string| string.to_string().parse::<i32>().unwrap());

        let mut safe_varient_exists: bool = false;

        for (skip_index, _skip_level) in report.clone().enumerate() {
            let mut report_varient = report
                .clone()
                .enumerate()
                .filter(|&(i, _)| i != skip_index)
                .map(|(_, v)| v) ;

            // Use this instead to test the first problem
            // let mut report_varient = report.clone();
            
            let mut last_level: i32 = report_varient.next().unwrap(); //report_copy.next().unwrap();
            let mut varient_is_safe = true;
            let mut safe = true;
            let mut increasing: bool;
            let mut was_increasing: bool = false;
            let mut first_loop: bool = true;

            print!("{} | {} ", linecount, last_level);
            for level in report_varient {
                print!("{} ", level);

                increasing = level > last_level;
                if first_loop {
                    was_increasing = increasing;
                }

                safe = safe && (level - last_level).abs() <= 3; // Check if the level steps by more than 3
                safe = safe && level != last_level; // Check if level stayed the same
                safe = safe && increasing == was_increasing; // Check if level change direction flips

                if !safe {
                    varient_is_safe = false;
                }

                last_level = level;
                was_increasing = increasing;
                first_loop = false;
            }
            if varient_is_safe {
                safe_varient_exists = true;
            }
            print!(" |{}\n", safe);
        }
        print!("{} | is safe: {}\n", linecount, safe_varient_exists);
        if safe_varient_exists {
            safe_reports += 1;
        }
    }
    print!("{}\n", safe_reports);
    // print!("{}\n", report_counter);
}