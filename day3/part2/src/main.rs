use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const FILENAME: &str = "input";

fn reverse_string(mut input: String, output: &mut String) {
    loop {
        match input.pop() {
            Some(char) => {
                output.push(char);
            }
            None => {
                break;
            }
        }
    }
}

// Returns true if a full pattern is found, false otherwise, or none if passed an empty character
fn update_pattern(c: &Option<char>, vec: &Vec<char>, i: &mut usize) -> Option<bool> {
    match c {
        Some(c) => {
            if *c != vec[*i] {
                *i = 0; // Character is a miss, restart search
                Some(false)
            } else if *c == vec[*i] && vec[*i] == *vec.last().unwrap() {
                *i = 0;
                Some(true) // Full sequence
            } else {
                *i += 1; // Character is a match; get ready for the next one
                Some(false)
            }
        }
        // No more characters to process
        None => { None }
    }
}

// Returns true when a valid opening mul( is found
fn search_for_opening(data: &mut String) -> bool {
    // strings that
    let mul_vec = vec!['m', 'u', 'l', '('];
    let do_vec = vec!['d', 'o', '(', ')'];
    let dont_vec = vec!['d', 'o', 'n', '\'', 't', '(', ')'];

    let mut dont: bool = false;

    // indices for keeping track of match progress
    let mut i_mul: usize = 0;
    let mut i_do: usize = 0;
    let mut i_dont: usize = 0;

    let mut c: Option<char>;
    loop {
        c = data.pop();

        // Check for mul(
        match update_pattern(&c, &mul_vec, &mut i_mul) {
            Some(match_found) => {
                if match_found && !dont {
                    return true;
                }
            }
            // The string is empty - stop searching
            None => {
                return false;
            }
        }

        // Check for do()
        match update_pattern(&c, &do_vec, &mut i_do) {
            Some(match_found) => {
                if match_found {
                    dont = false;
                }
            }
            // The string is empty - stop searching
            None => {
                return false;
            }
        }

        // Check for don't()
        match update_pattern(&c, &dont_vec, &mut i_dont) {
            Some(match_found) => {
                if match_found {
                    dont = true;
                }
            }
            // The string is empty - stop searching
            None => {
                return false;
            }
        }
    }
}

fn get_contents(data: &mut String) -> Option<(i32, i32)> {
    let (mut num1, mut _num2) = (0, 0);

    let mut buf = String::new();
    let mut on_first_number: bool = true;
    loop {
        match data.pop() {
            Some(char) => {
                if char.is_numeric() {
                    buf.push(char);
                } else if
                    // First number has been read, begin reading second
                    char == ',' &&
                    !buf.is_empty() &&
                    on_first_number
                {
                    num1 = buf.parse().unwrap();
                    buf.clear();
                    on_first_number = false;
                } else if
                    // Second number is done being read, return both numbers
                    char == ')' &&
                    !buf.is_empty() &&
                    !on_first_number
                {
                    _num2 = buf.parse().unwrap();
                    buf.clear();
                    return Some((num1, _num2));
                } else {
                    // The validity of the data has broken. Push the last char back into the string (in
                    // the case that it is the start of a mul(), forgoing this step would result in
                    // data not being read
                    data.push(char);
                    println!("fail! on {}", char);
                    return None;
                }
            }
            None => {
                return None;
            }
        }
    }
}

fn main() {
    // Get string from input file
    let path = Path::new(FILENAME);
    let mut file = File::open(&path).unwrap();

    let mut buf = String::new();
    let mut data: String = String::new();
    file.read_to_string(&mut buf).unwrap().to_string();
    reverse_string(buf, &mut data); // Reverse string for popping

    let mut sum = 0;
    while search_for_opening(&mut data) {
        match get_contents(&mut data) {
            Some((num1, num2)) => {
                sum += num1 * num2;
                println!("{}, {}", num1, num2);
            }
            None => (),
        }
    }
    println!("{}", sum);
}
