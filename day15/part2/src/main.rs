use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;
use std::path::Path;

const _INPUT: &str = "input.txt";
const _TEST_INPUT: &str = "test_input.txt";
const _TEST_INPUT_2: &str = "test_input_2.txt";
const _TEST_INPUT_3: &str = "test_input_3.txt";
const _TEST_INPUT_4: &str = "test_input_4.txt";
const _TEST_INPUT_5: &str = "test_input_5.txt";

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
    let mut data_lines = data_string.lines();
    loop {
        let line = data_lines.next().unwrap();
        if !line.contains('#') {
            break;
        }

        let mut warehouse_row = Vec::new();
        for c in line.chars() {
            match c {
                '#' => {
                    warehouse_row.push('#');
                    warehouse_row.push('#');
                }
                'O' => {
                    warehouse_row.push('[');
                    warehouse_row.push(']');
                }
                '.' => {
                    warehouse_row.push('.');
                    warehouse_row.push('.');
                }
                '@' => {
                    warehouse_row.push('@');
                    warehouse_row.push('.');
                }
                _ => panic!("Attempted to put a non-warehouse character into the warehouse!"),
            }
        }

        warehouse.push(warehouse_row);
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

fn get_neighbor_position(position: Vector2d, warehouse: &Vec<Vec<char>>) -> Vector2d {
    let c = warehouse[position.y as usize][position.x as usize];
    let neighbor_x_offset;
    match c {
        '[' => neighbor_x_offset = 1,
        ']' => neighbor_x_offset = -1,
        _ => panic!(
            "Attempted to fetch a neighbor from a character that doesn't have one: {}",
            c
        ),
    }

    Vector2d {
        x: position.x + neighbor_x_offset,
        y: position.y,
    }
}

// Returns false if there is an obstruction
fn check_obstructions(
    position: Vector2d,
    direction: char,
    warehouse: &Vec<Vec<char>>,
    check_neighbor: bool,
) -> bool {
    let next_position = get_next_position(position, direction);
    let c = warehouse[position.y as usize][position.x as usize];

    match c {
        '[' | ']' => {

            // First check if this side of the box is obstructed
            if !check_obstructions(next_position, direction, warehouse, true) {
                return false;
            }

            // Then check if the other side of the box is obstructed - but only if pushing vertically.
            // If pushing horizontally, this is treated like part 1
            if check_neighbor && direction != '<' && direction != '>' {
                let neighbor = get_neighbor_position(position, warehouse);
                if !check_obstructions(neighbor, direction, warehouse, false) {
                    return false;
                }
            }

            return true;
        }
        '.' => {
            return true;
        }
        '#' => {
            return false;
        }
        _ => panic!("Attempted to move into invalid character: {}", c),
    }
}

fn do_move(
    position: Vector2d,
    direction: char,
    warehouse: &mut Vec<Vec<char>>,
) {
    let next_position = get_next_position(position, direction);
    let c = warehouse[next_position.y as usize][next_position.x as usize];
    match c {
        '[' | ']' => {
            let neighbor = get_neighbor_position(next_position, warehouse);
            do_move(next_position, direction, warehouse);
            // If moving up or down, move the other side with it
            if direction == '^' || direction == 'v' {
                do_move(neighbor, direction, warehouse);
            }
            swap_characters(position, next_position, warehouse);
        }
        '.' => {
            swap_characters(position, next_position, warehouse);
        }
        '#' => panic!("Attempted to move into a wall at: {}", position),
        _ => panic!("Attempted to move into invalid character: {}", c),
    }
}

fn move_robot(robot: &mut Vector2d, instruction: char, warehouse: &mut Vec<Vec<char>>) {
    if warehouse[robot.y as usize][robot.x as usize] != '@' {
        panic!("The robot's position and its character on the map are mismatched!");
    }

    let next_position = get_next_position(*robot, instruction);
    if check_obstructions(next_position, instruction, warehouse, true) {
        do_move(*robot, instruction, warehouse);
        *robot = next_position;
    }
}

fn sum_gps_coords(warehouse: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for i in 0..warehouse.len() {
        for j in 0..warehouse[i].len() {
            if warehouse[i][j] == '[' {
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
        move_robot(&mut robot, instruction, &mut warehouse);
    }
    _print_grid(&warehouse);
    println!("GPS Sum: {}", sum_gps_coords(&warehouse));
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    fn load_wide_warehouse(name: &str) -> (Vec<Vec<char>>, String) {
        let path = Path::new(name);
        let mut file = File::open(&path).unwrap();
        let mut data_string: String = String::new();
        file.read_to_string(&mut data_string).unwrap().to_string();

        let mut warehouse: Vec<Vec<char>> = Vec::new();
        let mut data_lines = data_string.lines();
        loop {
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

    use super::*;
    #[test]
    fn test_get_robot() {
        let (warehouse, _instructions) = load_warehouse_and_robot_instructions(_TEST_INPUT);
        let robot = get_robot(&warehouse);
        assert_eq!(robot.x, 8);
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

        assert_eq!(sum_gps_coords(&warehouse), 9021);
    }

    #[test]
    fn test_problem_instruction() {
        let (mut warehouse, instructions) = load_wide_warehouse(_TEST_INPUT_3);
        let robot = get_robot(&warehouse);
        let robot_start_position = robot;
        _print_grid(&warehouse);
        for instruction in instructions.chars() {
            println!("{}", instruction);
            assert_eq!(
                check_obstructions(
                    get_next_position(robot, instruction),
                    instruction,
                    &mut warehouse,
                    true
                ),
                false
            );
            _print_grid(&warehouse);
        }
        assert_eq!(robot, robot_start_position);
    }
    #[test]
    fn test_push_up() {
        let (mut warehouse, instructions) = load_wide_warehouse(_TEST_INPUT_4);
        let mut robot = &mut get_robot(&warehouse);
        let robot_start_position = *robot;
        _print_grid(&warehouse);
        for instruction in instructions.chars() {
            do_move(*robot, instruction, &mut warehouse);
            _print_grid(&warehouse);
        }
    }
    
      #[test]
    fn test_check_obstructions() {
        let (mut warehouse, instructions) = load_wide_warehouse(_TEST_INPUT_5);
        let robot = &mut get_robot(&warehouse);

        let mut instruction = instructions.chars();
        let mut this_instruction = instruction.next().unwrap();
        assert_eq!(check_obstructions(get_next_position(*robot, this_instruction), this_instruction, &mut warehouse, true), true);

        this_instruction = instruction.next().unwrap();
        assert_eq!(check_obstructions(get_next_position(*robot, this_instruction), this_instruction, &mut warehouse, true), false);
    }
}
