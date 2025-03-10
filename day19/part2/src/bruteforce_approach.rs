use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input.txt";
const _EXAMPLE: &str = "example.txt";

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

fn make_design(towels: &mut HashSet<String>, largest_towel_size: usize, design: &str) -> i32 {
    if design.is_empty() {
        // We have completed our design!
        return 1;
    }
    // eprintln!("Checking pattern: {}", design);

    // We have a problem here. There are so many possibilities that there are too many to count
    // using the approach from part 1.

    // Perhaps we could "stitch" towels together and associate with them the number of towel combinations that can create them.
    //
    // Then, we could add this "stitched" towel to our towel pool. When we find a place where we can use this stitched towel
    // to complete the design, will already know how many ways there will be to complete it beforehand.
    // We will likely see many places where we can use the "stitched" towel to complete a pattern, and when we do, we will already
    // know how many ways there will be to complete the design from there, rather than rechecking every single possibility.
    // It may be necessary to start from the end of the pattern using this approach.


    let mut potential_designs = 0;
    for i in 1..largest_towel_size + 1 {
        // Eventually, some towels will be too big to fit into the pattern.
        if i > design.len() {
            break
        }

        // eprintln!("Subpattern: {}", &design[..i]);

        let pattern_to_match = &design[..i];
        if let Some(_) = towels.get(pattern_to_match) {
            // If we find a pattern that matches, add to our number of possibilities and continue searching for more that work.
            potential_designs += make_design(towels, largest_towel_size, &design[i..]);
            // eprintln!("Potential Designs: {}", potential_designs);
        }
    }

    potential_designs
}

fn main() {
    let (mut towels, largest_towel_size, designs) = load_towels_and_designs(_INPUT);

    let mut potential_designs = 0;
    for design in designs {
        potential_designs += make_design(&mut towels, largest_towel_size, &design);
        println!("{}", potential_designs);
    }

    println!("\nTotal possible designs: {}\n", potential_designs);
}

#[test]
fn test_website_example() {
    let (mut towels, largest_towel_size, designs) = load_towels_and_designs(_EXAMPLE);

    let mut potential_designs = 0;
    for design in designs {
        potential_designs += make_design(&mut towels, largest_towel_size, &design);
    }

    assert_eq!(potential_designs, 16)
}
