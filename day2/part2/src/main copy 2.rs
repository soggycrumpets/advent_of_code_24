use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn check_safety(level_prev: Option<&i32>, level_current: Option<&i32>, level_next: &i32) -> bool {
    // If checking out of index, return safe=true
    match level_current {
        Some(_) => (),
        None => {
            return true;
        }
    } //THIS IS FUCKED UP

    let mut safe: bool = true;
    let increasing: bool = level_next > level_current.unwrap();
    let was_increasing: Option<bool> = match level_prev {
        Some(level_prev) => Some(level_current.unwrap() > level_prev),
        None => None,
    };

    safe = safe && (level_current.unwrap() - level_next).abs() <= 3; // Check if the level steps by more than 3
    safe = safe && level_current.unwrap() != level_next; // Check if level stayed the same
    safe = match was_increasing {
        Some(was_increasing) => safe && increasing == was_increasing,
        None => safe,
    };

    safe
}

fn vec_try_get<T>(vec: &Vec<T>, i: usize, offset: i32) -> Option<&T> {
    let mut index: i32 = i as i32;
    index -= offset;
    if index < 0 || i >= vec.len() {
        None
    } else {
        vec.get(i)
    }
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

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
        let report = line
            .split_whitespace()
            .map(|string| string.to_string().parse::<i32>().unwrap());

        let mut damper: bool = true; // The damper allows for one unsafe report to be skipped
        let mut safe = true;
        let mut levels: Vec<i32> = report.collect();

        let mut i = 1; // start at current level = 0 = i-1
        while i < levels.len() {
            // Check if previous level is out of bounds
            // Edge case (first two levels)
            // if i < 2 {
            //     // First two don't match
            //     if !check_safety(None, levels.get(i - 1).unwrap(), levels.get(i).unwrap()) {
            //         safe = false;

            //         // First and third don't match - throw out the first element
            //         if !check_safety(None, levels.get(i - 1).unwrap(), levels.get(i+1).unwrap()) {
            //             levels.remove(i-1);
            //             i -= 1;
            //         }
            //         // Otherwise, throw out the second element
            //         else {
            //             levels.remove(i);
            //             i -= 1;
            //         }
            //     }
            // }
            // General case
            if
                !check_safety(
                    vec_try_get(&levels, i, -2),
                    vec_try_get(&levels, i, -1),
                    levels.get(i).unwrap()
                )
            {
                safe = false;
                // Check if current level needs to be thrown out
                if
                    !check_safety(
                        vec_try_get(&levels, i, -3),
                        vec_try_get(&levels, i, -2),
                        levels.get(i).unwrap()
                    )
                {
                    levels.remove(i - 1);
                    i -= 1;
                } else {
                    // Otherwise, the next level needs to be thrown out
                    levels.remove(i);
                    i -= 1;
                }
            }

            // Use up damper if unsafe pair was found. If damper is already used, the report is
            // unsafe
            if !safe {
                if !damper {
                    break;
                }
                damper = false;
                safe = true;
            }
            i += 1;
        }
        if safe {
            safe_reports += 1;
        }
    }
    print!("{}\n", safe_reports);
    // print!("{}\n", report_counter);
}
