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
    for digit in num_to_digits(num) {
        digit_count += 1;
    }

    if digit_count % 2 == 0 {
        return true;
    }

    return false;
}

fn split_stone(stones: &mut Vec<i64>, stones_index: usize) {
    let mut stone = stones[stones_index];
    let mut stone_digits = num_to_digits(stone);
    let mut new_stone_digits: Vec<i64> = Vec::new();
    let mut new_stone: i64;

    for _i in 0..stone_digits.len() / 2 {
        new_stone_digits.push(stone_digits.pop().unwrap());
    }
    new_stone_digits.reverse();

    stone = digits_to_num(stone_digits);
    new_stone = digits_to_num(new_stone_digits);

    stones[stones_index] = new_stone;
    stones.insert(stones_index, stone);
}

fn blink(stones: &mut Vec<i64>) {

    // Because the length of the stones vector is dynamically changing throughout the loop,
    // the index has to be shifted accordingly.
    let mut index = 0;
    for _j in 0..stones.len() {
        if stones[index] == 0 {
            stones[index] = 1;
        } else if digits_are_even(stones[index]) {
            split_stone(stones, index);
            index += 1;
        } else {
            stones[index] *= 2024;
        }

        index += 1;
    }
}

fn main() {
        let mut stones = read_file_to_vec(_INPUT);
        let number_of_blinks = 25;

        for _i in 0..number_of_blinks {
            blink(&mut stones);
        }

        println!("Number of stones: {}", stones.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_website_example() {
        let mut stones = read_file_to_vec(_TEST_INPUT);
        let number_of_blinks = 25;

        for _i in 0..number_of_blinks {
            blink(&mut stones);
        }

        assert_eq!(stones.len(), 55312);
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
        let mut stones = vec![1234, 5678];
        split_stone(&mut stones, 0);
        assert_eq!(stones, vec![12, 34, 5678]);
        split_stone(&mut stones, 2);
        assert_eq!(stones, vec![12, 34, 56, 78]);
    }
}
