use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const FILENAME: &str = "input";
// If "invert" is set to false, get legal updates. If true, get illegal updates
fn get_legal_updates(updates_vec: &Vec<Vec<i32>>, rules_table: &HashMap<i32, Vec<i32>>, invert: bool) -> Vec<Vec<i32>> {
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

    // Find the legal updates
    let legal_updates: Vec<Vec<i32>> = get_legal_updates(&updates_vec, &rules_table, false);
    let illegal_updates: Vec<Vec<i32>> = get_legal_updates(&updates_vec, &rules_table, true);

    // Print legal updates, find middle values, and sum them
    println!("Legal updates:");
    let mut sum: i32 = 0;
    for update in legal_updates {
        // Print
        for value in &update {
            print!("{}, ", value);
        }

        // Find middle value and add to sum
        let middle_value: i32 = update[(update.len()) / 2];
        sum += middle_value;
        println!(" | {}", middle_value);
    }
    println!("\nSum: {}", sum);
}
