use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;
use std::path::Path;

const _INPUT: &str = "input.txt";
const _TEST_INPUT: &str = "test_input.txt";
const _TEST_INPUT_2: &str = "test_input_2.txt";

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
struct Vector2d {
    x: i32,
    y: i32,
}
impl fmt::Display for Vector2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Add for Vector2d {
    type Output = Self;
    fn add(self, other: Self) -> Vector2d {
        Vector2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn load_warehouse_and_robot_instructions(name: &str) -> (Vec<Vec<char>>, String) {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut warehouse: Vec<Vec<char>> = Vec::new();
    let mut reading_warehouse = true;
    let mut data_lines = data_string.lines();
    while reading_warehouse {
        let line = data_lines.next().unwrap();
        if !line.contains('#') {
            break;
        }
        warehouse.push(line.chars().collect());
    }

    let mut instructions = String::new();
    for line in data_lines {
        for c in line.chars() {
            match c {
                '<' | '>' | 'v' | '^' => instructions.push(c),
                _ => (),
            }
        }
    }

    (warehouse, instructions)
}

fn get_robot(grid: &Vec<Vec<char>>) -> Vector2d {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c = grid[i][j];
            if c == '@' {
                return Vector2d {
                    x: j as i32,
                    y: i as i32,
                };
            }
        }
    }
    panic!("This grid doesn't have a robot!");
}

fn get_next_position(position: Vector2d, instruction: char) -> Vector2d {
    let mut next_position = position;
    match instruction {
        '>' => next_position.x += 1,
        'v' => next_position.y += 1,
        '<' => next_position.x -= 1,
        '^' => next_position.y -= 1,
        _ => panic!("Attempted to read invalid instruction!"),
    }

    next_position
}

fn _print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn swap_characters(position: Vector2d, next_position: Vector2d, warehouse: &mut Vec<Vec<char>>) {
    let char1 = warehouse[position.y as usize][position.x as usize];
    let char2 = warehouse[next_position.y as usize][next_position.x as usize];

    warehouse[position.y as usize][position.x as usize] = char2;
    warehouse[next_position.y as usize][next_position.x as usize] = char1;
}

fn try_move(position: Vector2d, direction: char, warehouse: &mut Vec<Vec<char>>) -> bool {
    let next_position = get_next_position(position, direction);
    let c = warehouse[next_position.y as usize][next_position.x as usize];
    match c {
        'O' => {
            if try_move(next_position, direction, warehouse) {
                swap_characters(position, next_position, warehouse);
                return true;
            } else {
                return false;
            }
        }
        '.' => {
            swap_characters(position, next_position, warehouse);
            return true;
        }
        '#' => return false,
        _ => panic!("Attempted to move into invalid character: {}", c),
    }
}

fn move_robot(robot: &mut Vector2d, instruction: char, warehouse: &mut Vec<Vec<char>>) {
    let next_position = get_next_position(*robot, instruction);
    if try_move(*robot, instruction, warehouse) {
        // swap_characters(*robot, next_position, warehouse);
        *robot = next_position;
    }
}

fn sum_GPS_coords(warehouse: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for i in 0..warehouse.len() {
        for j in 0..warehouse[i].len() {
            if warehouse[i][j] == 'O' {
                sum += 100 * i as i32 + j as i32;
            }
        }
    }

    sum
}

fn main() {
    let (mut warehouse, instructions) = load_warehouse_and_robot_instructions(_INPUT);
    let mut robot = get_robot(&warehouse);
    _print_grid(&warehouse);
    for instruction in instructions.chars() {
        // println!("{}", instruction);
        move_robot(&mut robot, instruction, &mut warehouse);
        // _print_grid(&warehouse);
    }
    _print_grid(&warehouse);
    println!("GPS Sum: {}", sum_GPS_coords(&warehouse));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_robot() {
        let (warehouse, _instructions) = load_warehouse_and_robot_instructions(_TEST_INPUT);
        let robot = get_robot(&warehouse);
        assert_eq!(robot.x, 4);
        assert_eq!(robot.y, 4);
    }

    #[test]
    fn test_website_example() {
        let (mut warehouse, instructions) = load_warehouse_and_robot_instructions(_TEST_INPUT);
        let mut robot = get_robot(&warehouse);
        _print_grid(&warehouse);
        for instruction in instructions.chars() {
            move_robot(&mut robot, instruction, &mut warehouse);
        }

        assert_eq!(sum_GPS_coords(&warehouse), 10092);
    }
}
