use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input";
const _TEST_INPUT: &str = "test_input";

fn read_file_to_vec(name: &str) -> Vec<i64> {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut num_vec: Vec<i64> = Vec::new();
    for num_string in data_string.split_whitespace() {
        num_vec.push(num_string.parse().unwrap());
    }

    num_vec
}

fn num_to_digits(num: i64) -> Vec<i64> {
    let mut remainder = num;
    let mut digits = Vec::new();

    while remainder >= 10 {
        digits.push(remainder % 10);
        remainder /= 10;
    }

    digits.push(remainder);
    digits.reverse();

    return digits;
}

fn digits_to_num(digits: Vec<i64>) -> i64 {
    let mut num: i64 = 0;
    let mut i = 0;

    for digit in digits.iter().rev() {
        num += digit * 10_i64.pow(i);
        i += 1;
    }

    num
}

fn digits_are_even(num: i64) -> bool {
    let mut digit_count = 0;

    for _digit in num_to_digits(num) {
        digit_count += 1;
    }

    if digit_count % 2 == 0 {
        return true;
    }

    return false;
}

fn split_stone(stone: i64) -> (i64, i64) {
    let mut stone_digits = num_to_digits(stone);
    let mut new_stone_digits: Vec<i64> = Vec::new();

    for _i in 0..stone_digits.len() / 2 {
        new_stone_digits.push(stone_digits.pop().unwrap());
    }
    new_stone_digits.reverse();

    (digits_to_num(stone_digits), digits_to_num(new_stone_digits))
}

fn insert_or_increment(stones: &mut HashMap<i64, i64>, stone: i64, number_of_stones: i64) {
    match stones.get_mut(&stone) {
        Some(num) => {*num += number_of_stones;}
        None => {stones.insert(stone, number_of_stones);}
    }
}

// Switched from a vector in part 1 to a hashmap, so every stone with the same number is calculated at once
fn blink(stones: &mut HashMap<i64, i64>) {

    // Clone the hashmap and clear the original. Repopulate the original using the clone
    let stones_this_blink = stones.clone();
    stones.clear();
    for (stone, number_of_stones) in stones_this_blink {
        if stone == 0 {
            insert_or_increment(stones, 1, number_of_stones);
        } else if digits_are_even(stone) {
            let (left_half, right_half) = split_stone(stone);
            insert_or_increment(stones, left_half, number_of_stones);
            insert_or_increment(stones, right_half, number_of_stones);
        } else {
            insert_or_increment(stones, stone*2024, number_of_stones);
        }
    }
}

fn main() {
    let stones_vec = read_file_to_vec(_INPUT);

    let mut stones = HashMap::new();
    for stone in stones_vec {
        insert_or_increment(&mut stones, stone, 1);        
    }

    let number_of_blinks = 75;
    for _i in 0..number_of_blinks {
        blink(&mut stones);
    }

    let mut sum_of_stones: i64 = 0;
    for (_stone, number_of_stones) in stones {
        sum_of_stones += number_of_stones;
    }

    println!("Number of stones: {}", sum_of_stones);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_website_example() {
        let number_of_blinks = 25;

        let mut stones = HashMap::new();
        stones.insert(125, 1);
        stones.insert(17, 1);

        for _i in 0..number_of_blinks {
            blink(&mut stones);
        }

        let mut sum_of_stones: i64 = 0;
        for (_stone, number_of_stones) in stones {
            sum_of_stones += number_of_stones;
            println!("({}, {})", _stone, number_of_stones);
        }

        assert_eq!(sum_of_stones, 55312);
    }

    #[test]
    fn test_num_to_digits() {
        let num = 1234;
        let digits = num_to_digits(num);
        assert_eq!(digits[0], 1);
        assert_eq!(digits[1], 2);
        assert_eq!(digits[2], 3);
        assert_eq!(digits[3], 4);
    }

    #[test]
    fn test_digits_to_num() {
        let digits = vec![1, 2, 3, 4];
        let num = digits_to_num(digits);
        assert_eq!(num, 1234);
    }

    #[test]
    fn test_digits_are_even() {
        assert_eq!(true, digits_are_even(12345678));
        assert_eq!(false, digits_are_even(123456789));
    }

    #[test]
    fn test_split_stone() {
        assert_eq!((12, 34), split_stone(1234));
    }
}
