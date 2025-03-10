use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;
use std::path::Path;

const _INPUT: &str = "input.txt";
const _EXAMPLE: &str = "example.txt";

// I've determined that no two towels are exactly the same,
// so I've chosen to store the towels in a hashset over a hashmap for sake of simplicity.
fn load_towels_and_designs(name: &str) -> (HashSet<String>, usize, Vec<String>) {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut data_lines = data_string.lines();

    let mut unsorted_towels: Vec<String> = data_lines
        .next()
        .unwrap()
        .split(",")
        .map(|str| str.to_string())
        .collect();
    for towel in &mut unsorted_towels {
        towel.retain(|c| c.is_alphabetic());
    }

    // Load towels
    let mut towels = HashSet::new();
    let mut largest_towel_size = 0;
    for towel in unsorted_towels {
        largest_towel_size = largest_towel_size.max(towel.len());
        towels.insert(towel);
    }

    data_lines.next();

    // Load designs
    let mut designs: Vec<String> = Vec::new();
    for line in data_lines {
        designs.push(line.to_string())
    }

    (towels, largest_towel_size, designs)
}

fn make_design(towels: &mut HashSet<String>, largest_towel_size: usize, design: &str) -> bool {
    if design.is_empty() {
        // We have completed our design!
        return true;
    }
    // eprintln!("Checking pattern: {}", design);

    // Look up all of the possible towel patterns that could fit into the design.
    // When we find one, move to the next part of the design and repeat.
    for i in 1..largest_towel_size+1 {
        // Eventually, some towels will be too big to fit into the pattern.
        if i > design.len() {
            break;
        }

        // eprintln!("Subpattern: {}", &design[..i]);

        let pattern_to_match = &design[..i];
        if let Some(_) = towels.get(pattern_to_match) {

            let design_was_completed = make_design(towels, largest_towel_size, &design[i..]) ;
            if design_was_completed {
                // We found a towel that fits!
                return true;
            }

        }
    }

    false
}

fn main() {
    let (mut towels, largest_towel_size, designs) = load_towels_and_designs(_INPUT);

    let mut completed_designs = 0;
    for design in designs {
        if make_design(&mut towels, largest_towel_size, &design) {
            completed_designs += 1;
            println!("Success: {}", design);
        } else {
            println!("Failure: {}", design)
        }
    }

    println!("\nTotal designs completed: {}\n", completed_designs);
}

#[test]
fn test_website_example() {
    let (mut towels, largest_towel_size, designs) = load_towels_and_designs(_EXAMPLE);

    let mut completed_designs = 0;
    for design in designs {
        if make_design(&mut towels, largest_towel_size, &design) {
            completed_designs += 1;
            eprintln!("{}: Completed", design);
        } else {
            eprintln!("{}: Failed", design)
        }
    }

    assert_eq!(completed_designs, 6);
}