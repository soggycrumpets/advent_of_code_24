use std::collections::HashMap;
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

    // Split string by lines
    let strings_vec: Vec<String> = file_string.lines().map(String::from).collect();

    // Split string vec into 2 int vecs (1 for each column)
    let mut nums1_vec: Vec<i32> = Vec::new();
    let mut nums2_vec: Vec<i32> = Vec::new();
    for i in strings_vec {
        // Split_whitespace() returns iterator over 2 strings
        let mut string_separated = i.split_whitespace();

        // Calling .next() the first time inserts the first string into the first vec.
        nums1_vec.push(string_separated.next().unwrap().to_string().parse().unwrap());

        // Calling the next time inserts into second vec
        nums2_vec.push(string_separated.next().unwrap().to_string().parse().unwrap());
    }

    // Sort vectors from low to high value
    nums1_vec.sort();
    nums2_vec.sort();

    /* ----- PART 2 ----- */

    // Create hashmap
    // Hashmap keys are the numbers in list 2. Values are the number of times the key appears
    let mut nums2_duplicates_hmap = HashMap::new();
    for i in &nums2_vec {
        match nums2_duplicates_hmap.get_mut(&i) {
            Some(duplicates) => {
                *duplicates += 1;
            }
            None => {
                nums2_duplicates_hmap.insert(i, 1);
            }
        }
    }

    // Iterate through the first list and find the number of duplicates in the second list
    // Increment similarity score based on # of duplicates
    let mut similarity_score: i32 = 0;
    for i in &nums1_vec {
        similarity_score += match nums2_duplicates_hmap.get(&i) {
            Some(duplicates) => i * duplicates,
            None => 0,
        };
    }

    print!("{}\n", similarity_score);
}
