use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input";
const _TEST_INPUT: &str = "test_input";

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    None,
    Left,
    Right,
    Up,
    Down,
}

fn read_file_to_array(name: &str) -> Vec<Vec<i32>> {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut grid: Vec<Vec<i32>> = Vec::new();
    let data_lines = data_string.lines();
    for line in data_lines {
        let mut nums_line: Vec<i32> = Vec::new();
        for char in line.chars() {
            nums_line.push(char as i32 - '0' as i32);
        }
        grid.push(nums_line);
    }

    grid
}

fn search_for_trailheads(map: &Vec<Vec<i32>>) -> Vec<Position> {
    let mut trailheads: Vec<Position> = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                let position = Position {
                    x: j as i32,
                    y: i as i32,
                };
                trailheads.push(position);
            }
        }
    }

    trailheads
}

fn check_array_bounds<T>(x: i32, y: i32, array: &Vec<Vec<T>>) -> bool {
    y >= 0 && y < array.len() as i32 && x >= 0 && x < array[0].len() as i32
}

// Returns true if the scouted space is +1 from current elevation
fn compare_elevations(position1: Position, position2: Position, map: &Vec<Vec<i32>>) -> bool {
    let elevation1 = map[position1.y as usize][position1.x as usize];
    let elevation2 = map[position2.y as usize][position2.x as usize];

    if elevation2 == (elevation1 + 1) {
        return true;
    } else {
        return false;
    }
}

fn get_next_position(position: Position, direction: Direction) -> Position {
    let mut next_position = position;
    match direction {
        Direction::Up => next_position.y -= 1,
        Direction::Down => next_position.y += 1,
        Direction::Left => next_position.x -= 1,
        Direction::Right => next_position.x += 1,
        Direction::None => {
            panic!("Match arm \"None\" in function \"get_next_position\" should not be accessed!")
        }
    }

    next_position
}

// Helper function for search_for_path 
fn search_in_direction(
    position: Position,
    next_direction: Direction,
    map: &Vec<Vec<i32>>,
    mut trail_ends_found: &mut HashSet<Position>,
) -> i32 {
    let mut score = 0;
    let next_position = get_next_position(position, next_direction);
    if check_array_bounds(next_position.x, next_position.y, map) {
        if compare_elevations(position, next_position, map) {
            score += search_for_path(map, next_position, next_direction, &mut trail_ends_found);
        }
    }

    score
}

// Searches for a path, given a position and a direction that was travelled from.
// This function is recursive and will call itself until it exhaustively searches the trail.
// Returns score, which is equal to the number of trail ends connected to the trail.
fn search_for_path(
    map: &Vec<Vec<i32>>,
    position: Position,
    direction: Direction,
    trail_ends_found: &mut HashSet<Position>,
) -> i32 {
    let mut score: i32 = 0;

    // The hashset ensures that each trail ending is only counted once
    if map[position.y as usize][position.x as usize] == 9 {
        match trail_ends_found.get(&position) {
            Some(_) => return 0,
            None => {
                trail_ends_found.insert(position);
                return 1;
            }
        }
    }

    if direction != Direction::Down {
        score += search_in_direction(position, Direction::Up, map, trail_ends_found);
    }
    if direction != Direction::Up {
        score += search_in_direction(position, Direction::Down, map, trail_ends_found);
    }
    if direction != Direction::Right {
        score += search_in_direction(position, Direction::Left, map, trail_ends_found);
    }
    if direction != Direction::Left {
        score += search_in_direction(position, Direction::Right, map, trail_ends_found);
    }

    score
}

fn main() {
    let topography_map = read_file_to_array(_INPUT);

    let trailheads = search_for_trailheads(&topography_map);

    let mut score: i32 = 0;
    for trailhead in trailheads {
        let mut trail_ends_found = HashSet::new();
        score += search_for_path(&topography_map, trailhead, Direction::None, &mut trail_ends_found);
    }

    println!("{}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_trailheads() {
        let topography_map = read_file_to_array(_TEST_INPUT);
        let trailheads = search_for_trailheads(&topography_map);
        assert_eq!(trailheads[0], Position { x: 2, y: 0 });
        assert_eq!(trailheads[1], Position { x: 4, y: 0 });
        assert_eq!(trailheads[2], Position { x: 4, y: 2 });
        assert_eq!(trailheads[3], Position { x: 6, y: 4 });
        assert_eq!(trailheads[4], Position { x: 2, y: 5 });
        assert_eq!(trailheads[5], Position { x: 5, y: 5 });
        assert_eq!(trailheads[6], Position { x: 0, y: 6 });
        assert_eq!(trailheads[7], Position { x: 6, y: 6 });
        assert_eq!(trailheads[8], Position { x: 1, y: 7 });
    }

    #[test]
    fn test_compare_eleveations() {
        let topography_map = read_file_to_array(_TEST_INPUT);

        let position1 = Position { x: 2, y: 0 };
        let mut position2 = Position { x: 3, y: 0 };
        assert_eq!(
            compare_elevations(position1, position2, &topography_map),
            true
        );

        position2.x = 2;
        position2.y = 1;
        assert_eq!(
            compare_elevations(position1, position2, &topography_map),
            true
        );

        position2.x = 1;
        position2.y = 0;
        assert_eq!(
            compare_elevations(position1, position2, &topography_map),
            false
        );
    }

    #[test]
    fn test_website_example() {
        let topography_map = read_file_to_array(_TEST_INPUT);
        let trailheads = search_for_trailheads(&topography_map);

        let mut score: i32 = 0;
        for trailhead in trailheads {
            let mut trail_ends_found = HashSet::new();
            score += search_for_path(
                &topography_map,
                trailhead,
                Direction::None,
                &mut trail_ends_found,
            );
        }

        assert_eq!(score, 36);
    }
}