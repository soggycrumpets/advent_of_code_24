use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const FILENAME: &str = "test_input";

// Transpose rows into columns and vice-versa
fn transpose_matrix(arr: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Initialize an emty buffer of dimensions mxn (where input is nxm)
    let mut transpose = vec![vec!['\0'; arr.len()]; arr[0].len()];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            transpose[j][i] = arr[i][j];
        }
    }

    transpose
}

// Flip matrix HORIZONTALLY
fn flip_matrix_horiz(arr: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Initialize an empty buffer of dimensions matching the input
    let mut flip = vec![vec!['\0'; arr[0].len()]; arr.len()];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            flip[i][arr[i].len() - j - 1] = arr[i][j];
        }
    }

    flip
}

// Flip matrix VERTICALLY
fn flip_matrix_vert(arr: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Initialize an empty buffer of dimensions matching the input
    let mut flip = vec![vec!['\0'; arr[0].len()]; arr.len()];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            flip[arr.len() - i - 1][j] = arr[i][j];
        }
    }

    flip
}

fn print_matrix(arr: &Vec<Vec<char>>) {
    for row in arr {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

// Returns true when pattern is found
fn update_pattern(c: &char, word: &Vec<char>, i_word: &mut usize) -> bool {
               // Start of pattern found
            if  *c == word[0] {
                *i_word = 1;
                print!("{}", c); 
                return false
            }
            // Miss - restart search
            else if *c != word[*i_word] {
                *i_word = 0;
                print!("-");
                return false
            }
            // Word completed! Add to counter and restart search
            else if *c == word[*i_word] && word[*i_word] == *word.last().unwrap() {
                *i_word = 0;
                print!("{}", c);
                return true
            }
            // Character matches sequence - continue search for next character
            else {
                *i_word += 1;
                print!("{}", c);
                return false
            }
             
}

// Reads left-to-right across all rows and counts number of XMAS occurances
fn read_rows(arr: &Vec<Vec<char>>) -> i32 {
    let mut word_count = 0;

    let xmas = vec!['X', 'M', 'A', 'S'];

    for i in 0..arr.len() {
        let mut i_xmas: usize = 0;
        for j in 0..arr[0].len() {

            if update_pattern(&arr[i][j], &xmas, &mut i_xmas) {
                word_count += 1;
            }
        }
        println!();
    }
    // print!("\n\n");

    word_count
}

// Reads diagonally from bottom-left to top-right, starting from points going from the top-left, to bottom-left, to bottom-right of the matrix
// & counts number of XMAS occurances
fn read_diags(arr: &Vec<Vec<char>>) -> i32 {
    let mut word_count = 0;

    let xmas = vec!['X', 'M', 'A', 'S'];

    // Move down the left side of the matrix, bottom-to-top
    for anchor_row in 0..arr.len() {
        let mut i_xmas: usize = 0;
        let mut i: i32 = anchor_row as i32;
        let mut j: i32 = 0;
        while (0 <= i) && (i < arr.len() as i32) && (0 <= j) && (j < arr[0].len() as i32) {
            if update_pattern(&arr[i as usize][j as usize], &xmas, &mut i_xmas) {
                word_count += 1;
            }
            i -= 1;
            j += 1;
        }
        println!();
    }

    // Move along the bottom side of the matrix, left-to-right
    // NOTE: anchor col starts at 1. the diagonal has already been read!
    for anchor_col in 1..arr[0].len() {
        let mut i_xmas: usize = 0;
        let mut i: i32 = arr.len() as i32 - 1;
        let mut j: i32 = anchor_col as i32;
        while (0 <= i) && (i < arr.len() as i32) && (0 <= j) && (j < arr[0].len() as i32) {
            if update_pattern(&arr[i as usize][j as usize], &xmas, &mut i_xmas) {
                word_count += 1;
            }
            i -= 1;
            j += 1;
        }
        println!();
    }

    word_count
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

    let mut xmas_counter: i32 = 0;

    println!("READING ROWS:");
    println!("ORIGINAL ORIENTATION");
    print_matrix(&char_matrix);
    xmas_counter += read_rows(&char_matrix);
    print!("\n\nXMAS count: {}\n\n", xmas_counter);

    // READ ALONG ROWS
    println!("FLIPPED ORIENTATION");
    char_matrix = flip_matrix_horiz(&mut char_matrix);
    print_matrix(&char_matrix);
    xmas_counter += read_rows(&char_matrix);
    print!("\n\nXMAS count: {}\n\n", xmas_counter);

    // Restore original orientation
    char_matrix = flip_matrix_horiz(&mut char_matrix);

    // READ ALONG COLS
    println!("TRANSPOSED ORIENTATION");
    char_matrix = transpose_matrix(&mut char_matrix);
    print_matrix(&char_matrix);
    xmas_counter += read_rows(&char_matrix);
    print!("\n\nXMAS count: {}\n\n", xmas_counter);

    println!("TRANSPOSED AND FLIPPED ORIENTATION");
    char_matrix = flip_matrix_horiz(&mut char_matrix);
    print_matrix(&char_matrix);
    xmas_counter += read_rows(&char_matrix);
    print!("\n\nXMAS count: {}\n\n", xmas_counter);

    // Restore original orientation
    char_matrix = flip_matrix_horiz(&mut char_matrix);
    char_matrix = transpose_matrix(&mut char_matrix);



    // READ ALONG DIAGONALS
    println!("\n\nREADING DIAGONALS:");
    println!("ORIGINAL ORIENTATION");
    print_matrix(&char_matrix);
    xmas_counter += read_diags(&char_matrix);
    print!("\n\nXMAS count: {}\n\n", xmas_counter);

    println!("-> FLIPPED HORIZONTALLY");
    char_matrix = flip_matrix_horiz(&mut char_matrix);
    print_matrix(&char_matrix);
    xmas_counter += read_diags(&char_matrix);
    print!("\n\nXMAS count: {}\n\n", xmas_counter);

    println!("-> FLIPPED VERTICALLY");
    char_matrix = flip_matrix_vert(&mut char_matrix);
    print_matrix(&char_matrix);
    xmas_counter += read_diags(&char_matrix);
    print!("\n\nXMAS count: {}\n\n", xmas_counter);

    println!("-> FLIPPED HORIZONTALLY");
    char_matrix = flip_matrix_horiz(&mut char_matrix);
    print_matrix(&char_matrix);
    xmas_counter += read_diags(&char_matrix);
    print!("\n\nXMAS count: {}\n\n", xmas_counter);

    // Restore original orientation
    char_matrix = flip_matrix_vert(&mut char_matrix);

}