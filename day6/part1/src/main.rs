use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _FILENAME: &str = "input";
const _TEST_INPUT: &str = "test_input";

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Guard {
    x: i32,
    y: i32,
    direction: Direction,
}

fn get_guard(grid: &Vec<Vec<char>>) -> Guard {
    // Find the guard's starting position (initialize to shut up the compiler)
    let mut guard: Guard = Guard {
        x: 0,
        y: 0,
        direction: Direction::Up,
    };
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c = grid[i][j];
            match c {
                '>' | '<' | '^' | 'v' => {
                    guard.x = i as i32;
                    guard.y = j as i32;
                    match c {
                        '>' => guard.direction = Direction::Right,
                        '<' => guard.direction = Direction::Left,
                        '^' => guard.direction = Direction::Up,
                        'v' => guard.direction = Direction::Down,
                        _ => (),
                    }
                    break;
                }
                _ => (),
            }
        }
    }

    guard
}

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

// Return true is guard is still on the grid
fn move_guard(guard: &mut Guard, grid: &mut Vec<Vec<char>>) -> bool {
    let (mut _dx, mut _dy) = (0, 0);

    // Mark the guard's current spot as visited
    grid[guard.x as usize][guard.y as usize] = 'X';

    // (dx, dy) is flipped because instead of (x, y) we are indexing the matrix with (row, col), or (y, x)
    match guard.direction {
        Direction::Left => (_dy, _dx) = (-1, 0),
        Direction::Right => (_dy, _dx) = (1, 0),
        Direction::Up => (_dy, _dx) = (0, -1),
        Direction::Down => (_dy, _dx) = (0, 1),
    }

    // If the guard is about to go out of bounds, finish moving
    if (guard.x + _dx) < 0
        || (guard.x + _dx) > grid.len() as i32 - 1
        || (guard.y + _dy) < 0
        || (guard.y + _dy) > grid[0].len() as i32 - 1
    {
        return false;
    }

    // Guard hits wall - rotate 90 degrees
    // and do not move until next cycle
    if (grid[(guard.x + _dx) as usize][(guard.y + _dy) as usize]) == '#' {
        guard.direction = match guard.direction {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        };
        return true;
    }

    // Update guard position
    guard.x += _dx;
    guard.y += _dy;
    return true;
}

fn sum_visited_spaces(grid: &Vec<Vec<char>>) -> i32 {
    let mut sum: i32 = 0;
    for row in grid {
        for char in row {
            if *char == 'X' {
                sum += 1;
            }
        }
    }

    sum
}

fn main() {
    let mut grid = read_file_to_array(_FILENAME);
    let mut guard = get_guard(&grid);

    while move_guard(&mut guard, &mut grid) {}

    let sum: i32 = sum_visited_spaces(&grid);

    println!("Guard Position: ({}, {})", guard.x, guard.y);
    println!("Sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_guard() {
        let grid = read_file_to_array(_TEST_INPUT);
        let guard = get_guard(&grid);
        assert_eq!((guard.x, guard.y), (6, 4));
    }

    #[test]
    fn pass_website_example_1() {
        let mut grid = read_file_to_array(_TEST_INPUT);
        let mut guard = get_guard(&grid);
        while move_guard(&mut guard, &mut grid) {}
        let sum: i32 = sum_visited_spaces(&grid);
        assert_eq!(sum, 41);
    }
}
