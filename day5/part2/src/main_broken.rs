use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const FILENAME: &str = "test_input";
// If "invert" is set to false, get legal updates. If true, get illegal updates
fn get_legal_updates(
    updates_vec: &Vec<Vec<i32>>,
    rules_table: &HashMap<i32, Vec<i32>>,
    invert: bool,
) -> Vec<Vec<i32>> {
    let mut legal_updates: Vec<Vec<i32>> = Vec::new();

    for update in updates_vec {
        let mut illegal_values_table: HashMap<i32, bool> = HashMap::new();
        let mut update_is_legal: bool = true && !invert;

        for value in update {
            // Use the illegal values table to check if value is illegal
            let value_is_illegal: bool = match illegal_values_table.get(&value) {
                Some(_bool) => true,
                None => false,
            };

            // Stop reading the update if an illegal value is found
            if value_is_illegal {
                update_is_legal = false || invert;
                break;
            }

            // Use the rules table to add new illegal values to the illegal values table
            let new_illegal_values = rules_table.get(&value);
            match new_illegal_values {
                Some(illegal_values) => {
                    for illegal_value in illegal_values {
                        illegal_values_table.insert(*illegal_value, true);
                    }
                }
                None => (),
            }
        }

        if update_is_legal {
            legal_updates.push(update.clone());
        }
    }

    legal_updates
}

fn sum_update_middles(updates_vec: &Vec<Vec<i32>>) -> i32 {
    let mut sum: i32 = 0;
    for update in updates_vec {
        // print
        for value in update {
            print!("{}, ", value);
        }

        // find middle value and add to sum
        let middle_value: i32 = update[(update.len()) / 2];
        sum += middle_value;
        println!(" | {}", middle_value);
    }

    sum
}

// This function is a modified version of the get_legal_updates function.
// Rather than just checking if the update is legal, it fixes every update in the list it receives
// and returns them in a vector
fn fix_illegal_updates(
    illegal_updates: &Vec<Vec<i32>>,
    rules_table: &HashMap<i32, Vec<i32>>,
) -> Vec<Vec<i32>> {
    let mut fixed_updates: Vec<Vec<i32>> = Vec::new();

    for illegal_update in illegal_updates {
        // This copy of the update will be modified until it it properly sorted
        let mut update_varient = illegal_update.clone();

        // Keep changing the update until it is legal
        let mut update_is_illegal: bool = true;

        while update_is_illegal {
            // If the next loop finishes while this value is false, the update becomes legal and the loop breaks
            let mut value_is_illegal: bool = true;
            let mut illegal_values_table: HashMap<i32, bool> = HashMap::new();

            let mut i = 0;
            while i < update_varient.len() {
                let value = update_varient[i];

                // Use the illegal values table to check if value is illegal
                value_is_illegal = match illegal_values_table.get(&value) {
                    Some(_bool) => true,
                    None => false,
                };

                // This variation is illegal; swap the current number with the one to its left
                // Move back the search by and continue checking from there
                if value_is_illegal {

                    // The previous value is now in front of us in the list, so we need to remove its
                    // corresponding illegal values from the table.
                    let old_illegal_values = rules_table.get(&update_varient[i-1]);
                    match old_illegal_values {
                        Some(illegal_values) => {
                            for illegal_value in illegal_values {
                                illegal_values_table.remove(illegal_value);
                            }
                        }
                        None => (),
                    }

                    let buf: i32 = update_varient[i];
                    update_varient[i] = update_varient[i - 1];
                    update_varient[i - 1] = buf;

                    // Move back 1 entry in the list
                    continue;
                }

                // Use the rules table to add new illegal values to the illegal values table
                let new_illegal_values = rules_table.get(&value);
                match new_illegal_values {
                    Some(illegal_values) => {
                        for illegal_value in illegal_values {
                            illegal_values_table.insert(*illegal_value, true);
                        }
                    }
                    None => (),
                }

                // Move forward in the list
                i += 1;
            }
            // If the for loop finishes with value_is_illegal being false, the update has been corrected
            update_is_illegal = value_is_illegal;
        }
        // Push the fixed update to the fixed updates vector
        fixed_updates.push(update_varient);
    }

    fixed_updates
}

fn main() {
    // Read input into string
    let path = Path::new(FILENAME);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut data_lines = data_string.lines();

    // Separate rules into a hash table (key: after, value: before(s))
    let mut rules_vec: Vec<(i32, i32)> = Vec::new();
    let mut rules_table: HashMap<i32, Vec<i32>> = HashMap::new();
    for line in &mut data_lines {
        if line.is_empty() {
            break;
        }

        let mut split = line.split('|');
        let (before, after): (i32, i32) = (
            split.next().unwrap().parse().unwrap(),
            split.next_back().unwrap().parse().unwrap(),
        );
        rules_vec.push((before, after));

        // Check the rules table. If an "after" number already has a "before" associated with it, add the new "before" to the vector.
        // Otherwise, use it to createa  new vector
        let rule = rules_table.get_mut(&after);
        match &rule {
            Some(_before) => {
                rule.unwrap().push(before);
            }
            None => {
                rules_table.insert(after, vec![before]);
            }
        }
    }

    // Separate updates into their own vector
    let mut updates_vec: Vec<Vec<i32>> = Vec::new();
    for line in data_lines {
        let split = line.split(',');
        updates_vec.push(split.map(|string| string.parse().unwrap()).collect());
    }

    // Print rules and updates
    rules_vec
        .iter()
        .for_each(|rule| println!("{}, {}", rule.0, rule.1));
    println!();
    for updates in &updates_vec {
        for value in updates {
            print!("{}, ", value);
        }
        println!();
    }
    print!("\n\n");

    // Find the legal and illegal updates
    let legal_updates: Vec<Vec<i32>> = get_legal_updates(&updates_vec, &rules_table, false);
    let illegal_updates: Vec<Vec<i32>> = get_legal_updates(&updates_vec, &rules_table, true);
    let fixed_updates: Vec<Vec<i32>> = fix_illegal_updates(&illegal_updates, &rules_table);

    // Print legal updates, find middle values, and sum them
    print!("Legal updates:\n\n\n");
    let legal_updates_sum: i32 = sum_update_middles(&legal_updates);
    print!("Sum: {}\n\n", legal_updates_sum);

    // Print illegal updates, find middle values, and sum them
    print!("Illegal updates:\n\n\n");
    let illegal_updates_sum: i32 = sum_update_middles(&illegal_updates);
    print!("Sum: {}\n\n", illegal_updates_sum);

    // Print fixed updates, find middle values, and sum them
    print!("Fixed updates:\n\n\n");
    let fixed_updates_sum: i32 = sum_update_middles(&fixed_updates);
    print!("Sum: {}\n\n", fixed_updates_sum);
}
