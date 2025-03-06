use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;
use std::path::Path;

const _INPUT: &str = "input.txt";
const _TEST_INPUT: &str = "test_input.txt";

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

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    None,
    Left,
    Right,
    Up,
    Down,
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

    (warehouse, instructions.chars().rev().collect())
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

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn main() {
    load_warehouse_and_robot_instructions(_TEST_INPUT);
}
