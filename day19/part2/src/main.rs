use std::collections::HashMap;
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

fn make_design(
    towels: &mut HashSet<String>,
    largest_towel_size: usize,
    design: &str,
    explored_patterns: &mut HashMap<String, i64>,
) -> i64 {
    if design.is_empty() {
        // We have completed our design! That's 1 more possibility to add to the total.
        return 1;
    }

    // We have a problem here if we keep using the approach from part 1. We will often be partway through the design,
    //      and the pattern required to finish is one that we've already seen before.
    // This will happen so often that we will pretty much spend an eternity re-checking combinations for
    //      configurations that we've already seen!
    // To address this issue, I added a hashmap that keeps track of which patterns we have seen before, and how many
    //      ways there are to complete that pattern. Every time we come across a new pattern, we find all of the ways
    //      to complate it and add it to the hashmap. When we come across this pattern again, we will already know how many
    //      ways we can complete our design from here. Instead of exploring every possibility all over again, we can add
    //      the number that we've already calculated to our total and move on.

    // This is where we check if we've been here before. If we have, just grab the number that we already calculated.
    if let Some(number_of_combinations) = explored_patterns.get(design) {
        return *number_of_combinations;
    }

    let mut potential_designs: i64 = 0;
    for i in 1..largest_towel_size + 1 {
        // Eventually, some towels will be too big to fit into the pattern.
        if i > design.len() {
            break;
        }

        let pattern_to_match = &design[..i];
        if let Some(_) = towels.get(pattern_to_match) {
            potential_designs +=
                make_design(towels, largest_towel_size, &design[i..], explored_patterns);
            // Once we have fully explored this pattern, keep note of the number of combinations found
            //      so we don't have to explore this pattern again.
            explored_patterns.insert(design.to_string(), potential_designs);
        }
    }

    potential_designs
}

fn main() {
    let (mut towels, largest_towel_size, designs) = load_towels_and_designs(_INPUT);

    let mut potential_designs = 0;
    let mut explored_patterns = HashMap::new();
    for design in designs {
        potential_designs += make_design(&mut towels, largest_towel_size, &design, &mut explored_patterns);
    }

    println!("\nTotal possible designs: {}\n", potential_designs);
}

#[test]
fn test_website_example() {
    let (mut towels, largest_towel_size, designs) = load_towels_and_designs(_EXAMPLE);

    let mut potential_designs = 0;
    let mut explored_patterns = HashMap::new();
    for design in designs {
        potential_designs += make_design(&mut towels, largest_towel_size, &design, &mut explored_patterns);
    }

    assert_eq!(potential_designs, 16)
}