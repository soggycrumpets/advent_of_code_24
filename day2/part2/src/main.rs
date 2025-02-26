use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const DEBUG: bool = false;

fn compare_entries(
    level_prev: Option<&i32>,
    level_current: Option<&i32>,
    level_next: Option<&i32>
) -> bool {
    // If checking out of index, return safe=true
    match level_current {
        Some(_) => (),
        None => {
            return true;
        }
    }
    match level_next {
        Some(_) => (),
        None => {
            return true;
        }
    }

    let mut safe: bool = true;
    let increasing: bool = level_next.unwrap() > level_current.unwrap();
    let was_increasing: Option<bool> = match level_prev {
        Some(level_prev) => Some(level_current.unwrap() > level_prev),
        None => None,
    };
    if DEBUG {
        print!(
            "next: {}         current:  {}         ?: {}\n",
            level_next.unwrap(),
            level_current.unwrap(),
            (level_current.unwrap() - level_next.unwrap()).abs() <= 3
        );
    }
    safe = safe && (level_current.unwrap() - level_next.unwrap()).abs() <= 3; // Check if the level steps by more than 3
    safe = safe && level_current.unwrap() != level_next.unwrap(); // Check if level stayed the same
    safe = match was_increasing {
        Some(was_increasing) => safe && increasing == was_increasing,
        None => safe,
    };

    safe
}

fn check_report_safety(levels: &mut Vec<i32>) -> bool {
    let mut damper: bool = true; // The damper allows for one unsafe report to be skipped
    let mut safe = true;

    let mut i = 1; // start at current level = 0 = i-1
    while i < levels.len() {
        if
            !compare_entries(
                vec_try_get(&levels, i, -2),
                vec_try_get(&levels, i, -1),
                vec_try_get(&levels, i, 0)
            )
        {
            safe = false;
            // Check if current level can be thrown out
            let current_level_required = !compare_entries(
                vec_try_get(&levels, i, -3),
                vec_try_get(&levels, i, -2),
                vec_try_get(&levels, i, 0)
            );
            // Check if next level can be thrown out
            let next_level_required = !compare_entries(
                vec_try_get(&levels, i, -2),
                vec_try_get(&levels, i, -1),
                vec_try_get(&levels, i, 1)
            );
            // At the beginning of the list: does the list have a chance to be safe if you remove
            // the first entry?
            let confounding_first_entry: bool = (i == 2) && compare_entries(
                vec_try_get(&levels, i, -1),
                vec_try_get(&levels, i, 0),
                vec_try_get(&levels, i, 1),
                ) && compare_entries (
                    None,
                vec_try_get(&levels, i, -1),
                vec_try_get(&levels, i, 0),
                );
            if DEBUG {
                print!(
                    "next: {}\t current: {}\t current_required: {}\t next_required: {}\t\n",
                    vec_try_get(&levels, i, 0).unwrap(),
                    vec_try_get(&levels, i - 1, 0).unwrap(),
                    current_level_required,
                    next_level_required
                );
            }
            // First entry is causing the problem and should be removed
            if confounding_first_entry {
                // print!("\nfirst entry is bad!!!\n");
                levels.remove(i-2);
                i -= 1;
            }
            // No deletion will save the report
            else if current_level_required && next_level_required {
                damper = false;
            } else if next_level_required {
                // remove current level
                levels.remove(i-1);
                i -= 1;
            } else {
                // remove next level
                levels.remove(i);
                i -= 1;
            }
        }

        if !safe {
            if !damper {
                return safe;
            }
            damper = false;
            safe = true;
        }
        i += 1;
    }
    safe
}

fn vec_try_get<T>(vec: &Vec<T>, i: usize, offset: i32) -> Option<&T> {
    let mut index = i as i32;
    index += offset;
    if index < 0 || i >= vec.len() {
        None
    } else {
        vec.get(index as usize)
    }
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    // Create path for file
    let path: &Path;
    if DEBUG {
        path = Path::new("test_input");
    } else {
        path = Path::new("input");
    }

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

        let mut levels: Vec<i32> = report.collect();
        let levels_original = levels.clone();
        let safe = check_report_safety(&mut levels);

        print!("{} | {} | ", linecount, safe);
        levels_original.iter().for_each(|int| print!("{} ", int));
        print!("\t\t");
        levels.iter().for_each(|int| print!("{} ", int));
        print!("\n");

        if safe {
            safe_reports += 1;
        }
    }
    print!("{}\n", safe_reports);
}


#[cfg(test)]
fn test_report(report: &mut Vec<i32>, test_safety: bool) {
    let report_original = report.clone();
    let safe = check_report_safety(report);

    print_err(report_original, report);
    assert_eq!(safe, test_safety);
}

#[cfg(test)]
fn print_err(report_original: Vec<i32>, report_modified: &mut Vec<i32>) {
    eprint!("Original: ");
    report_original.iter().for_each(|num| eprint!("{} ", num));
    eprintln!();
    eprint!("Modified: ");
    report_modified.iter().for_each(|num| eprint!("{} ", num));
    eprintln!();
}


#[cfg(test)]
mod tests {
    use super::*;

    // Tests that return true
    #[test]
    fn delete_first_from_start() {
        test_report(&mut vec![11, 14, 13, 17], true);
    }

    #[test]
    fn delete_second_from_start() {
        test_report(&mut vec![11, 10, 14, 17], true);
    }

    #[test]
    fn delete_first_from_end() {
       test_report(&mut vec![10, 13, 16, 10], true);
    }

    #[test]
    fn delete_second_from_end() {
       test_report(&mut vec![10, 13, 10, 16], true);
    }

    #[test]
    fn delete_first() {
       test_report(&mut vec![10, 12, 13, 16], true);
    }

    #[test]
    fn delete_second() {
       test_report(&mut vec![10, 13, 12, 16], true);
    }

    // Tests that return false
    #[test]
    fn fail_at_start() {
        test_report(&mut vec![9, 9, 13, 17], false);
    }

    #[test]
    fn fail_at_end() {
       test_report(&mut vec![10, 13, 17, 17], false);
    }

    #[test]
    fn fail_in_middle() {
       test_report(&mut vec![10, 17, 17, 13], false);
    }

    // Known failure points
    #[test]
    fn failure_point_1() {
       test_report(&mut vec![62, 61, 62, 63, 65, 67, 68, 71], true);
    }

    #[test]
    fn failure_point_2() {
       test_report(&mut vec![74, 75, 79, 82, 85, 88, 91], false);
    }
    

    // Website examples
    #[test]
    fn website_examples() {
       test_report(&mut vec![7, 6, 4, 2, 1], true);
       test_report(&mut vec![1, 2, 7, 8, 9], false);
       test_report(&mut vec![9, 7, 6, 2, 1], false);
       test_report(&mut vec![1, 3, 2, 4, 5], true);
       test_report(&mut vec![1, 3, 6, 7, 9], true);
    }
}
