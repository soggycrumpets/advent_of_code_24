use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input";
const _TEST_INPUT: &str = "test_input";

fn read_file_to_array(name: &str) -> Vec<Vec<char>> {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut grid: Vec<Vec<char>> = Vec::new();
    let data_lines = data_string.lines();
    for line in data_lines {
        grid.push(line.chars().collect());
    }

    grid
}

#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn check_array_bounds<T>(x: i32, y: i32, array: &Vec<Vec<T>>) -> bool {
    y >= 0 && y < array.len() as i32 && x >= 0 && x < array[0].len() as i32
}

fn get_antenna_table(char_array: &Vec<Vec<char>>) -> HashMap<char, Vec<Position>> {
    let mut antenna_table: HashMap<char, Vec<Position>> = HashMap::new();

    for i in 0..char_array.len() {
        for j in 0..char_array[0].len() {
            let frequency = char_array[i][j];
            if frequency == '.' {
                continue;
            }

            let position: Position = Position {
                x: j as i32,
                y: i as i32,
            };

            let antenna_option = antenna_table.get_mut(&frequency);
            match antenna_option {
                Some(antenna) => {
                    antenna.push(position);
                }
                None => {
                    antenna_table.insert(frequency, vec![position]);
                }
            }
        }
    }

    antenna_table
}

// This function is modified for part 2!
fn place_antinode(char_array: &mut Vec<Vec<char>>, antenna1: &Position, antenna2: &Position) {

    let mut antinode1 = antenna1.clone();
    while check_array_bounds(antinode1.x, antinode1.y, &char_array) {
        char_array[antinode1.y as usize][antinode1.x as usize] = '#';
        antinode1.x += antenna1.x - antenna2.x;
        antinode1.y += antenna1.y - antenna2.y;
    }
}

fn place_all_antinodes(
    char_array: &mut Vec<Vec<char>>,
    antenna_table: &HashMap<char, Vec<Position>>,
) {
    for kv_pairs in antenna_table {
        let antenna_positions = kv_pairs.1;
        for i in 0..antenna_positions.len() {
            let antenna1 = &antenna_positions[i];

            for j in 0..antenna_positions.len() {
                if i == j {
                    continue
                }
                let antenna2 = &antenna_positions[j];

                place_antinode(char_array, antenna1, antenna2);
            }
        }
    }
}

fn sum_antinodes(char_array: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for row in char_array {
        for char in row {
            if *char == '#' {
                sum += 1;
            }
        }
    }

    sum
}

fn main() {
    read_file_to_array(_INPUT);
    let mut char_array = read_file_to_array(_INPUT);
    let antenna_table = get_antenna_table(&char_array);

    place_all_antinodes(&mut char_array, &antenna_table);

    println!("Total antinodes: {}", sum_antinodes(&char_array));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_place_antinodes() {
        let mut char_array = read_file_to_array(_TEST_INPUT);
        let antenna1 = Position { x: 6, y: 5 };
        let antenna2 = Position { x: 8, y: 8 };
        place_antinode(&mut char_array, &antenna1, &antenna2);
        place_antinode(&mut char_array, &antenna2, &antenna1);

        for i in &char_array {
            for j in i {
                eprint!("{}", j);
            }
            eprintln!();
        }

        assert_eq!(char_array[11][10], '#');
        assert_eq!(char_array[2][4], '#');
    }

    #[test]
    fn test_website_example_2() {
        let mut char_array = read_file_to_array(_TEST_INPUT);
        let antenna_table = get_antenna_table(&char_array);

        place_all_antinodes(&mut char_array, &antenna_table);

        for i in &char_array {
            for j in i {
                eprint!("{}", j);
            }
            eprintln!();
        }

        // Print out the antenna and frequencies for debugging
        for pairs in &antenna_table {
            let frequency = pairs.0;
            let antenna_positions = pairs.1;
            for antenna_position in antenna_positions {
                eprintln!("{}, {}", frequency, antenna_position);
            }
        }

        assert_eq!(34, sum_antinodes(&char_array));
    }
}