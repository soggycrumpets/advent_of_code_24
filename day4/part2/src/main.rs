use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const FILENAME: &str = "input";

fn print_matrix(arr: &Vec<Vec<char>>) {
    for row in arr {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn read_x_mas (arr: &Vec<Vec<char>>) -> i32 {
    let mut x_mas_count = 0;

    // Search for A's in the matrix. Don't search the edges because those can't contain a "mas"
    for i in 1..arr.len()-1 {
        for j in 1..arr[0].len()-1 {
            match arr[i][j] {
                'A' => {
                    if 
                    (arr[i-1][j-1] == 'S' && arr[i+1][j+1] == 'M' || arr[i-1][j-1] == 'M' && arr[i+1][j+1] == 'S')
                    && 
                    (arr[i+1][j-1] == 'S' && arr[i-1][j+1] == 'M' || arr[i+1][j-1] == 'M' && arr[i-1][j+1] == 'S') {
                        x_mas_count += 1;                        
                    }
                    print!("{}", arr[i][j]);
                }
                _ => print!("-"),
            }
        }
        println!();
    }

    x_mas_count
}

fn main() {
    // Read input into string
    let path = Path::new(FILENAME);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    // Put string into a 2d char matrix
    let strings_vec: Vec<&str> = data_string.lines().collect();
    let mut char_matrix: Vec<Vec<char>> = Vec::new();
    let mut i: usize = 0;
    for string in strings_vec {
        char_matrix.push(Vec::new());
        for char in string.trim().chars() {
            char_matrix[i].push(char);
        }
        i += 1;
    }

    let mut x_mas_counter: i32 = 0;

    print_matrix(&char_matrix);
    x_mas_counter += read_x_mas(&char_matrix);
    println!("\nTotal x_mas count: {}", x_mas_counter)
}
