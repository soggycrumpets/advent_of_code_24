use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input.txt";
const _EXAMPLE: &str = "example.txt";

fn load_key_lock_pairs(name: &str) -> Vec<Vec<Vec<char>>> {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut data_lines = data_string.lines();

    let mut array_vec: Vec<Vec<Vec<char>>> = Vec::new();
    let mut array: Vec<Vec<char>> = Vec::new();
    loop {
        if let Some(line) = data_lines.next() {
            if line.is_empty() {
                array_vec.push(array.clone());
                array.clear();
                continue;
            }

            array.push(line.chars().collect());
        } else {
            array_vec.push(array.clone());
            array.clear();
            break;
        }
    }

    array_vec
}

fn sort_locks_from_keys(arrays: Vec<Vec<Vec<char>>>) -> (Vec<Vec<Vec<char>>>, Vec<Vec<Vec<char>>>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for array in arrays {
        if array[0][0] == '#' {
            locks.push(array);
        } else {
            keys.push(array);
        }
    }
    (locks, keys)
}

fn log_key_heights(keys: &Vec<Vec<Vec<char>>>) -> HashMap<Vec<Vec<char>>, Vec<u32>> {
    let mut key_heights: HashMap<Vec<Vec<char>>, Vec<u32>> = HashMap::new();

    for key in keys {
        let mut pin_heights: Vec<u32> = Vec::new();
        for j in 0..key[0].len() {
            let mut pin_height: u32 = 0;
            for i in 0..key.len() {
                if key[i][j] == '#' {
                    pin_height += 1;
                }
            }
            pin_heights.push(pin_height)
        }
        key_heights.insert(key.clone(), pin_heights);
    }

    key_heights
}

fn log_lock_spaces(locks: &Vec<Vec<Vec<char>>>) -> Vec<Vec<u32>> {
    let mut key_heights: Vec<Vec<u32>> = Vec::new();

    for lock in locks {
        let mut pin_heights: Vec<u32> = Vec::new();
        for j in 0..lock[0].len() {
            let mut pin_height: u32 = 0;
            for i in 0..lock.len() {
                if lock[i][j] == '.' {
                    pin_height += 1;
                }
            }
            pin_heights.push(pin_height)
        }
        key_heights.push(pin_heights);
    }

    key_heights
}

fn main() {
    let arrays = load_key_lock_pairs(_INPUT);
    let (locks, keys) = sort_locks_from_keys(arrays);
    let key_heights = log_key_heights(&keys);
    let lock_spaces = log_lock_spaces(&locks);

    let mut pairs: u32 = 0;
    for key in keys {
        let pins = key_heights.get(&key).unwrap();
        for spaces in &lock_spaces {
            let mut key_is_valid = true;
            for i in 0..spaces.len() {
                if pins[i] > spaces[i] {
                    key_is_valid = false;
                    break;
                }
            }
            if key_is_valid {
                pairs += 1;
            }
        }
    }
    println!("Key-lock pairs: {}", pairs);
}