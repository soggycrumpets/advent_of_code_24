use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input";

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
        }
    }
}

struct Guard {
    x: i32,
    y: i32,
    direction: Direction,
    bonk_table: HashMap<(i32, i32, Direction), bool>,
    caught_in_loop: bool,
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

fn get_guard(grid: &Vec<Vec<char>>) -> Guard {
    // Find the guard's starting position (initialize to shut up the compiler)
    let mut guard: Guard = Guard {
        y: 0,
        x: 0,
        direction: Direction::Up,
        bonk_table: HashMap::new(),
        caught_in_loop: false,
    };
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c = grid[i][j];
            match c {
                '>' | '<' | '^' | 'v' => {
                    guard.y = i as i32;
                    guard.x = j as i32;
                    match c {
                        '>' => {
                            guard.direction = Direction::Right;
                        }
                        'v' => {
                            guard.direction = Direction::Down;
                        }
                        '<' => {
                            guard.direction = Direction::Left;
                        }
                        '^' => {
                            guard.direction = Direction::Up;
                        }
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

fn get_next_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
    }
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

fn get_movement_delta(direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
    }
}

fn in_array_bounds(x: i32, y: i32, grid: &Vec<Vec<char>>) -> bool {
    x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32
}

// Return true if guard is still on the grid
fn move_guard(guard: &mut Guard, grid: &mut Vec<Vec<char>>, is_ghost: bool) -> bool {
    let (dx, dy) = get_movement_delta(guard.direction);

    // Next position leaves the grid
    if !in_array_bounds(guard.y + dy, guard.x + dx, grid) {
        return false;
    }

    // Send a "ghost guard" to see if putting an obstacle here would get the guard caught in a loop
    if !is_ghost {
        send_ghost_guard(guard.x, guard.y, guard.direction, grid);
    }

    // Guard hits wall - rotate 90 degrees to the right
    // Will not more forward or change the symbol of the space it's on
    if (grid[(guard.y + dy) as usize][(guard.x + dx) as usize]) == '#' {
        // Check & update bonk table: if the bonk is in the table, the guard is caught in an infinite loop!
        match guard
            .bonk_table
            .get(&(guard.x + dx, guard.y + dy, guard.direction))
        {
            Some(_) => {
                guard.caught_in_loop = true;
            }
            None => {
                guard
                    .bonk_table
                    .insert((guard.x + dx, guard.y + dy, guard.direction), true);
            }
        }

        guard.direction = get_next_direction(guard.direction);
        return true;
    }

    // Move forward
    guard.x += dx;
    guard.y += dy;

    // Mark the guard's current spot
    if !is_ghost {
        let grid_char = &mut grid[guard.y as usize][guard.x as usize];
        *grid_char = match *grid_char {
            '|' | '-' => '+',
            'O' => 'O',
            '^' | '>' | '<' | 'v' => *grid_char,
            _ => match guard.direction {
                Direction::Up | Direction::Down => '|',
                Direction::Left | Direction::Right => '-',
            },
        };
    }

    return true;
}

// Send a "ghost" guard as if the guard has hit an object; if the ghost guard gets caught in a loop, this is an obstacle spot
fn send_ghost_guard(
    x0: i32,
    y0: i32,
    direction_initial: Direction,
    grid: &mut Vec<Vec<char>>,
) -> bool {
    // If we're starting on the edge of the grid, there is no room in front of us to place the obstacle
    let (dx, dy) = get_movement_delta(direction_initial);
    if !in_array_bounds(x0 + dx, y0 + dy, grid) {
        return false;
    }
    // The only valid spaces to check are unvisited, empty spaces
    if grid[(y0 + dy) as usize][(x0 + dx) as usize] != '.' {
        return false;
    }

    let mut ghost_guard: Guard = Guard {
        y: y0,
        x: x0,
        direction: direction_initial,
        bonk_table: HashMap::new(),
        caught_in_loop: false,
    };

    // Temporarily replace the space in front of where the ghost starts with an obstacle
    grid[(y0 + dy) as usize][(x0 + dx) as usize] = '#';

    // Move the ghost until it either gets caught in a loop or leaves the grid
    while move_guard(&mut ghost_guard, grid, true) && !ghost_guard.caught_in_loop {}

    // If the ghost got caught in a loop, mark the the obstacle location down
    if ghost_guard.caught_in_loop {
        grid[(y0 + dy) as usize][(x0 + dx) as usize] = 'O';
        return true;
    }
    // Otherwise, remove the temporary '#' to return the grid to normal
    else {
        grid[(y0 + dy) as usize][(x0 + dx) as usize] = '.';
    }
    return false;
}

fn sum_spaces(grid: &Vec<Vec<char>>) -> (i32, i32) {
    let (mut visited, mut obstacle) = (0, 0);
    for row in grid {
        for char in row {
            match *char {
                'X' | '|' | '-' | '+' | '^' | '>' | '<' | 'v' => {
                    visited += 1;
                }
                'O' => {
                    visited += 1;
                    obstacle += 1;
                }
                _ => (),
            }
        }
    }

    (visited, obstacle)
}

fn main() {
    let mut grid = read_file_to_array(_INPUT);
    let mut guard = get_guard(&grid);

    while move_guard(&mut guard, &mut grid, false) {}

    let sum = sum_spaces(&grid);

    print_grid(&grid);
    println!("Total Spaces Visited: {}", sum.0);
    println!("Total Obstacle Spots: {}", sum.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "test_input";
    const TEST_COMPLEX_LOOP: &str = "test_complex_loop";
    const TEST_FALSE_POSITIVE: &str = "test_false_positive";

    #[test]
    fn can_find_guard() {
        let grid = read_file_to_array(TEST_INPUT);
        let guard = get_guard(&grid);
        assert_eq!((guard.y, guard.x), (6, 4));
    }

    #[test]
    fn pass_website_example_part_1() {
        let mut grid = read_file_to_array(TEST_INPUT);
        let mut guard = get_guard(&grid);
        while move_guard(&mut guard, &mut grid, false) {}
        let sum: i32 = sum_spaces(&grid).0;
        // print_grid(&grid);
        assert_eq!(sum, 41);
    }

    #[test]
    fn pass_part_1() {
        let mut grid = read_file_to_array(_INPUT);
        let mut guard = get_guard(&grid);
        while move_guard(&mut guard, &mut grid, false) {}
        let sum: i32 = sum_spaces(&grid).0;
        assert_eq!(sum, 4696);
    }

    #[test]
    fn pass_website_example_part_2() {
        let mut grid = read_file_to_array(TEST_INPUT);
        let mut guard = get_guard(&grid);
        while move_guard(&mut guard, &mut grid, false) {}
        let obstacles: i32 = sum_spaces(&grid).1;
        assert_eq!(obstacles, 6);
    }

    #[test]
    fn test_ghost_guard() {
        let mut grid = read_file_to_array(TEST_INPUT);
        let detected_obstacle = send_ghost_guard(7, 8, Direction::Down, &mut grid);
        assert_eq!(detected_obstacle, true)
    }

    #[test]
    fn ghost_breaks_from_infinite_loop() {
        let mut grid = read_file_to_array(TEST_COMPLEX_LOOP);
        let guard = get_guard(&grid);
        send_ghost_guard(guard.x, guard.y, guard.direction, &mut grid);
    }

    #[test]
    fn all_obstacles_are_maintained() {
        let mut grid = read_file_to_array(_INPUT);
        let grid_copy = grid.clone();
        let mut guard = get_guard(&grid);
        while move_guard(&mut guard, &mut grid, false) {}

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '#' || grid_copy[i][j] == '#' {
                    assert_eq!(grid[i][j], grid_copy[i][j]);
                }
            }
        }
    }

    #[test]
    fn ghost_avoids_false_positive() {
        let mut grid = read_file_to_array(TEST_FALSE_POSITIVE);
        let guard = get_guard(&grid);
        let found_obstacle: bool = send_ghost_guard(guard.x, guard.y, guard.direction, &mut grid);
        print_grid(&grid);
        assert_eq!(found_obstacle, false);
    }
}
