use core::num;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input.txt";
const _EXAMPLE: &str = "example.txt";

fn load_secret_numbers(name: &str) -> Vec<u64> {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut numbers: Vec<u64> = Vec::new();
    let data_lines = data_string.lines();
    for line_str in data_lines {
        numbers.push(line_str.parse().unwrap());
    }

    numbers
}

fn mix(secret_number: u64, value: u64) -> u64 {
    value ^ secret_number
}

fn prune(number: u64) -> u64 {
    number % 16777216
}

fn pseudorandom(number: u64) -> u64 {
    let mut secret_number = number;
    secret_number = mix(secret_number, secret_number * 64);
    secret_number = prune(secret_number);

    secret_number = mix(secret_number, secret_number / 32);
    secret_number = prune(secret_number);

    secret_number = mix(secret_number, secret_number * 2048);
    secret_number = prune(secret_number);

    secret_number
}

fn pseudorandom_n(number: u64, iterations: u64) -> u64 {
    let mut secret_number = number;
    for _i in 0..iterations {
        secret_number = pseudorandom(secret_number);
    }
    secret_number
}

fn main() {
    let numbers = load_secret_numbers(_INPUT);
    let mut final_numbers: Vec<u64> = Vec::new();
    for number in numbers {
        let secret_number = pseudorandom_n(number, 2000);
        final_numbers.push(secret_number);
    }

    let mut sum = 0;
    for number in final_numbers {
        sum += number;
    }

    println!("Sum of secret numbers: {}", sum);
}

#[test]
fn test_example() {
    let numbers = load_secret_numbers(_EXAMPLE);
    let mut final_numbers: Vec<u64> = Vec::new();
    for number in numbers {
        let secret_number = pseudorandom_n(number, 2000);
        final_numbers.push(secret_number);
    }
    assert_eq!(final_numbers, vec![8685429, 4700978, 15273692, 8667524]);
}

#[test]
fn test_mix() {
    assert_eq!(mix(42, 15), 37);
}

#[test]
fn test_prune() {
    assert_eq!(prune(100000000), 16113920);
}

#[test]
fn test_pseudorandom() {
    assert_eq!(pseudorandom(123), 15887950);
}
