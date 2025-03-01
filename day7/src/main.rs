use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input";
const _TEST_INPUT: &str = "test_input";

fn read_file_to_array(name: &str) -> Vec<Vec<i64>> {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut array: Vec<Vec<i64>> = Vec::new();
    let data_lines = data_string.lines();
    for line_str in data_lines {
        let mut num_vec: Vec<i64> = Vec::new();
        let line_filtered: String = line_str.chars().filter(|c| *c != ':').collect();

        for num in line_filtered.split_whitespace() {
            num_vec.push(num.parse().unwrap());
        }

        array.push(num_vec);
    }

    array
}

// Computes the result of every possible combination of '+' and '*' operations and compares to the target number
// If any combination results in the target number, return the target number
// Otherwise, return None
fn _compute_op_combinations(mut eqn: Vec<i64>) -> Option<i64> {
    // Remove the first element from the vector and use that as the target
    let target: i64 = eqn.remove(0);

    let ops_len = eqn.len() - 1;

    let mut result: Option<i64> = None;

    for i in 0..2 << ops_len - 1 {
        let mut total = eqn[0];
        // eprint!("{} ", total);
        for j in 0..ops_len {
            if (i & (1 << j)) > 0 {
                total *= eqn[j + 1];
                // eprint!("* {} ", eqn[j + 1]);
            } else {
                total += eqn[j + 1];
                // eprint!("+ {} ", eqn[j + 1]);
            }
        }
        if total == target {
            result = Some(target);
            return result
            // eprint!("| match");
        }
        // eprintln!("= {}", total);
    }

    result
}

// This function works the same was as "compute_op_combinations", except that it computes every possible
// combination of '+', '*', AND '||'.
fn compute_op_combinations_with_cat(mut eqn: Vec<i64>) -> Option<i64> {
    // Remove the first element from the vector and use that as the target
    let target: i64 = eqn.remove(0);
    let ops_len: usize = eqn.len() - 1;
    let number_of_combinations: usize = (3_u32).pow(ops_len as u32) as usize;

    let mut result: Option<i64> = None;

    for i in 0..number_of_combinations {
        let mut total = eqn[0];
        // eprint!("{} ", total);
        for j in 0..ops_len {
            let i_ternary = to_ternary(i as i32, ops_len);
            match i_ternary[i_ternary.len() - 1 - j ] {
                0 => {
                    total += eqn[j + 1];
                    // eprint!("+ {} ", eqn[j + 1])
                }
                1 => {
                    total *= eqn[j + 1];
                    // eprint!("* {} ", eqn[j + 1])
                }
                2 => {
                    total = cat(total, eqn[j+1]);
                    // eprint!("|| {} ", eqn[j + 1]);
                }
                _ => panic!("Ternary digit greater than 2! i_ternary[{}] = {}", j, i_ternary[j+1]),
            }
        }
        if total == target {
            result = Some(target);
            return result
        }
        // eprintln!("= {}", total);
    }

     result
}

fn to_ternary(decimal: i32, size: usize) -> Vec<i32> {
    let mut remainder = decimal;
    let mut ternary = vec![0; size];

    let mut i: usize = 0;
    while remainder >= 3 {
        ternary[i] = remainder % 3;
        remainder /= 3;
        i += 1;
    }
    ternary[i] = remainder;
    ternary.reverse();

    return ternary;
}

fn cat(left: i64, right: i64) -> i64 {
    let mut result =  left.to_string();
    result.push_str(&right.to_string().as_str());
    result.parse().unwrap()
}

fn main() {
    let array = read_file_to_array(_INPUT);

    let mut sum = 0;
    for eqn in array {
        let result = compute_op_combinations_with_cat(eqn);

        match result {
            Some(value) => sum += value,
            None => (),
        }
    }

    println!("Total: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_website_example_1() {
        let array = read_file_to_array(_TEST_INPUT);

        let mut sum = 0;
        for eqn in array {
            let result = _compute_op_combinations(eqn);

            match result {
                Some(value) => sum += value,
                None => (),
            }
        }

        assert_eq!(sum, 3749);
    }

    #[test]
    fn pass_website_example_2() {
        let array = read_file_to_array(_TEST_INPUT);

        let mut sum = 0;
        for eqn in array {
            let result = compute_op_combinations_with_cat(eqn);

            match result {
                Some(value) => sum += value,
                None => (),
            }
        }

        assert_eq!(sum, 11387);
    }

    #[test]
    fn test_decimal_to_ternary() {
        let num = 500;
        let ternary = to_ternary(num, 6);
        let solution = vec![2, 0, 0, 1, 1, 2];
        for i in 0..ternary.len() {
            assert_eq!(ternary[i], solution[i]);
        }
    }

    #[test]
    fn test_cat() {
        let num1: i64 = 123;
        let num2: i64 = 456;
        let solution: i64 = 123456;
        assert_eq!(cat(num1, num2), solution);
    }
}
