use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // create path
    let path = Path::new("input");
    let display = path.display();

    // open path in RO mode
    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", display, why),
        Ok(file) => file,
    };

    // read file contents into string
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let strings_vec: Vec<String> = s.lines().map(String::from).collect();

    // Split strings
    let mut nums_vec_1: Vec<i32> = Vec::new();
    let mut nums_vec_2: Vec<i32> = Vec::new();
    for i in strings_vec {
        let mut string_separated = i.split_whitespace();
        nums_vec_1.push(string_separated.next().unwrap().to_string().parse().unwrap());
        nums_vec_2.push(string_separated.next().unwrap().to_string().parse().unwrap());
    }

    nums_vec_1.sort();
    nums_vec_2.sort();

    let mut nums2_iter = nums_vec_2.into_iter();
    let mut sum: i32 = 0;
    for i in nums_vec_1 {
        sum += (i - nums2_iter.next().unwrap()).abs();
        // print!("{}\n", sum);
    }

    print!("{}\n", sum);
}
