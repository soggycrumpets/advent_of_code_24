use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const FILENAME: &str = "input";

fn reverse_string(mut input: String, output: &mut String) {
    loop {
        match input.pop() { 
            Some(char) => {
                // print!("{}", char);
                output.push(char);
            }
            None => {
                // println!();
                break;
            }
        }
    }    
}

fn search_for_opening (data: &mut String) -> bool {
    let opening = vec!['m', 'u', 'l', '('];
    let mut i: usize = 0;
       loop {
        match data.pop() {
            Some(char) => {
                if char == opening[i] && opening[i] == *opening.last().unwrap() {
                    return true; // Full opening found
                }
                else if char == opening[i] {
                    i += 1; // Character is a match; get ready for the next one
                } 
                else {
                    i = 0; // Character is a miss, restart search
                }
            }
            None => return false, // No more characters to process
        }
    }     
}

fn get_contents(data: &mut String) -> Option<(i32, i32)> {
    let (mut num1, mut _num2) =(0, 0);

    let mut buf = String::new();
    let mut on_first_number: bool = true;
    loop {
        match data.pop() {
            Some(char) => {
                if char.is_numeric() {
                    buf.push(char);
                }
                // First number has been read, begin reading second
                else if char == ',' && !buf.is_empty() && on_first_number {
                    num1 = buf.parse().unwrap();
                    buf.clear();
                    on_first_number = false;
                }
                // Second number is done being read, return both numbers
                else if char == ')' && !buf.is_empty() && !on_first_number {
                    _num2 = buf.parse().unwrap();
                    buf.clear();

                    return Some((num1, _num2));
                }
                // The validity of the data has broken. Push the last char back into the string (in
                // the case that it is the start of a mul(), forgoing this step would result in
                // data not being read
                else {
                    data.push(char);
                    println!("fail! on {}", char);
                    return None
                }
            }
            None => return None
        }
    }
}

fn main() {
    // Get string from input file
    let path = Path::new(FILENAME);
    let mut file = File::open(&path).unwrap();
    let mut buf= String::new();
    file.read_to_string(&mut buf).unwrap().to_string();

    let mut data: String = String::new();
    reverse_string(buf, &mut data); // Reverse string for popping

        let mut sum = 0;
        while search_for_opening(&mut data) {
            match get_contents(&mut data) {
                Some((num1, num2)) => {
                    sum += num1*num2;
                    println!("{}, {}", num1, num2)
                },
                None => ()
            }
        }
        println!("{}", sum);

    // println!("{}", data);


}
