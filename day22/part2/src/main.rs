use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input.txt";
const _EXAMPLE: &str = "example.txt";
const _EXAMPLE2: &str = "example2.txt";

fn load_secret_numbers(name: &str) -> Vec<i64> {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut numbers: Vec<i64> = Vec::new();
    let data_lines = data_string.lines();
    for line_str in data_lines {
        numbers.push(line_str.parse().unwrap());
    }

    numbers
}

fn mix(secret_number: i64, value: i64) -> i64 {
    value ^ secret_number
}

fn prune(number: i64) -> i64 {
    number % 16777216
}

fn pseudorandom(number: i64) -> i64 {
    let mut secret_number = number;
    secret_number = mix(secret_number, secret_number * 64);
    secret_number = prune(secret_number);

    secret_number = mix(secret_number, secret_number / 32);
    secret_number = prune(secret_number);

    secret_number = mix(secret_number, secret_number * 2048);
    secret_number = prune(secret_number);

    secret_number
}

fn pseudorandom_n(number: i64, iterations: i64) -> i64 {
    let mut secret_number = number;
    for _i in 0..iterations {
        secret_number = pseudorandom(secret_number);
    }
    secret_number
}

fn log_banana_prices(number: i64, iterations: i64) -> Vec<i64> {
    let mut prices: Vec<i64> = Vec::new();
    let mut secret_number = number;
    prices.push(secret_number % 10);
    for _i in 0..iterations {
        secret_number = pseudorandom(secret_number);
        prices.push(secret_number % 10);
    }
    prices
}

fn log_price_deltas(prices: &Vec<i64>) -> Vec<i64> {
    let mut price_deltas: Vec<i64> = Vec::new();
    for i in 0..prices.len() - 1 {
        price_deltas.push(prices[i + 1] - prices[i]);
    }
    price_deltas
}

fn log_banana_yields(number: i64, iterations: i64, banana_yields: &mut HashMap<Vec<i64>, i64>) {
    let prices = log_banana_prices(number, iterations);
    let price_deltas = log_price_deltas(&prices);

    // The monkey will sell the first time it sees a sequence and move on to the next market.
    // If it sees the same sequence twice, we should ignore it the second time.
    // This will be addressed using a hashset.
    let mut logged_sequences: HashSet<Vec<i64>> = HashSet::new();
    for i in 3..price_deltas.len() {
        let price_change_sequence = vec![
            price_deltas[i - 3],
            price_deltas[i - 2],
            price_deltas[i - 1],
            price_deltas[i - 0],
        ];
        let price = prices[i + 1];

        if let Some(_) = logged_sequences.get(&price_change_sequence) {
            continue
        }
        logged_sequences.insert(price_change_sequence.clone());
        
        // Log how many bananas we get from this market by looking for this sequence
        // & add it to the total banana yield we get for this sequence
        if let Some(banana_yield) = banana_yields.get_mut(&price_change_sequence) {
            *banana_yield += price;
        } else {
            banana_yields.insert(price_change_sequence, price);
        }
    }
}

fn main() {
     let numbers = load_secret_numbers(_INPUT);
    let mut banana_yields: HashMap<Vec<i64>, i64> = HashMap::new();
    for number in numbers {
        log_banana_yields(number, 2000, &mut banana_yields);
    }

    let mut best_profit: Option<i64> = None;
    for (_sequence, profit) in &banana_yields {
        if let Some(value) = best_profit {
            if value < *profit {
                best_profit = Some(*profit);
            }
        } else {
            best_profit = Some(*profit);
        }
    }

    println!("Highest possible profit in bananas: {}", best_profit.unwrap());

}

#[test]
fn test_example_part1() {
    let numbers = load_secret_numbers(_EXAMPLE);
    let mut final_numbers: Vec<i64> = Vec::new();
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

#[test]
fn test_example_part2() {
    let numbers = load_secret_numbers(_EXAMPLE2);
    let mut banana_yields: HashMap<Vec<i64>, i64> = HashMap::new();
    for number in numbers {
        log_banana_yields(number, 2000, &mut banana_yields);
    }

    let mut best_profit: Option<i64> = None;
    let mut best_sequence: Option<Vec<i64>> = None;
    for (sequence, profit) in &banana_yields {
        if let Some(value) = best_profit {
            if value < *profit {
                best_profit = Some(*profit);
                best_sequence = Some(sequence.clone()); 
            }
        } else {
            best_profit = Some(*profit);
            best_sequence = Some(sequence.clone());
        }
    }

    assert_eq!(best_profit.unwrap(), 23);
    assert_eq!(best_sequence.unwrap(), vec![-2,1,-1,3]);
}

#[test]
fn test_log_banana_prices() {
    let secret_number = 123;
    let prices = log_banana_prices(secret_number, 9);
    assert_eq!(prices, vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]);
}

#[test]
fn test_log_price_deltas() {
    let secret_number = 123;
    let prices = log_banana_prices(secret_number, 9);
    let price_deltas = log_price_deltas(&prices);
    assert_eq!(price_deltas, vec![-3, 6, -1, -1, 0, 2, -2, 0, -2]);
}