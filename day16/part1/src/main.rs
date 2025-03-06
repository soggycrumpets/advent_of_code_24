use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;
use std::path::Path;

const _INPUT: &str = "input.txt";
const _EXAMPLE_1: &str = "example1.txt";
const _EXAMPLE_2: &str = "example2.txt";
const _TEST_STRAIGHT_SHOT: &str = "test_straight_shot.txt";
const _TEST_AROUND_CORNER: &str = "test_around_corner.txt";

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
struct Vector2d {
    x: i32,
    y: i32,
}
impl Vector2d {
    fn left(&self) -> Vector2d {
        Vector2d {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn right(&self) -> Vector2d {
        Vector2d {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn up(&self) -> Vector2d {
        Vector2d {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn down(&self) -> Vector2d {
        Vector2d {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn ahead(&self, direction: char) -> Vector2d {
        match direction {
            '^' => self.up(),
            '>' => self.right(),
            'v' => self.down(),
            '<' => self.left(),
            _ => panic!(
                "Tried to get next position, but direction is \"{}\"",
                direction
            ),
        }
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
impl fmt::Display for Vector2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Reindeer {
    position: Vector2d,
    direction: char,
    score: i32,
    min_score: Option<i32>,
    found_exit: bool,
}
impl Reindeer {
    fn new(maze: &Vec<Vec<char>>) -> Reindeer {
        Reindeer {
            position: find_start_point(maze),
            direction: '>',
            score: 0,
            min_score: None,
            found_exit: false,
        }
    }
    fn move_straight(&mut self) {
        self.position = self.position.ahead(self.direction);
        self.score += 1;
    }
    fn left(&self) -> char {
        match self.direction {
            '^' => '<',
            '<' => 'v',
            'v' => '>',
            '>' => '^',
            _ => panic!(
                "Tried to turn left, but direction is \"{}\"",
                self.direction
            ),
        }
    }
    fn right(&self) -> char {
        match self.direction {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            _ => panic!(
                "Tried to turn right, but direction is \"{}\"",
                self.direction
            ),
        }
    }
    fn turn_left(&mut self) {
        self.direction = self.left();
        self.score += 1000;
    }
    fn turn_right(&mut self) {
        self.direction = self.right();
        self.score += 1000;
    }
    fn merge_min_score(&mut self, new_min_score: Option<i32>) {
        if let Some(min1) = self.min_score {
            if let Some(min2) = new_min_score {
                self.min_score = Some(min1.min(min2));
            }
        } else {
            self.min_score = new_min_score;
        }
    }
}

fn mark_map(map: &mut Vec<Vec<char>>, position: Vector2d, mark: char) {
    map[position.y as usize][position.x as usize] = mark;
}

fn path_is_clear(maze: &Vec<Vec<char>>, position: Vector2d) -> bool {
    match maze[position.y as usize][position.x as usize] {
        '#' | '<' | '>' | '^' | 'v' => false,
        _ => true,
    }
}

fn find_start_point(grid: &Vec<Vec<char>>) -> Vector2d {
    // Find the guard's starting position (initialize to shut up the compiler)

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                return Vector2d {
                    x: j as i32,
                    y: i as i32,
                };
            }
        }
    }
    panic!("Start point not found!");
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

fn found_exit(maze: &Vec<Vec<char>>, position: Vector2d) -> bool {
    if maze[position.y as usize][position.x as usize] == 'E' {
        true
    } else {
        false
    }
}

fn pathfind_new_instance(reindeer: &mut Reindeer, maze: &mut Vec<Vec<char>>) {
    mark_map(maze, reindeer.position, reindeer.direction);
    reindeer.move_straight();
    pathfind(reindeer, maze);
}

fn pathfind(reindeer: &mut Reindeer, maze: &mut Vec<Vec<char>>) {
    // If the exit is found, this score is a potential minimum score. Merge it with the current minimum score
    if found_exit(maze, reindeer.position) {
        reindeer.found_exit = true;
        reindeer.merge_min_score(Some(reindeer.score));
        println!("Score: {}", reindeer.score);
        _print_grid(maze);
        return;
    }

    // Mark this space on the map as visited
    mark_map(maze, reindeer.position, reindeer.direction);

    // Spawn a new reindeer to explore left. Merge this reindeer's min score with the new one.
    if path_is_clear(maze, reindeer.position.ahead(reindeer.left())) {
        let mut new_reindeer = reindeer.clone();
        new_reindeer.turn_left();
        let mut new_maze = maze.clone();
        pathfind_new_instance(&mut new_reindeer, &mut new_maze);

        reindeer.merge_min_score(new_reindeer.min_score);
    }
    // Spawn a new reindeer to explore right. Merge this reindeer's min score with the new one.
    if path_is_clear(maze, reindeer.position.ahead(reindeer.right())) {
        let mut new_reindeer = reindeer.clone();
        new_reindeer.turn_right();
        let mut new_maze = maze.clone();
        pathfind_new_instance(&mut new_reindeer, &mut new_maze);

        reindeer.merge_min_score(new_reindeer.min_score);
    }
    // Move reindeer ahead.
    if path_is_clear(maze, reindeer.position.ahead(reindeer.direction)) {
        reindeer.move_straight();
        pathfind(reindeer, maze);
    }

    // The reindeer has no more moves to make
    return;
}

fn main() {
    let mut maze = read_file_to_array(_EXAMPLE_1);
    _print_grid(&maze);
    let mut reindeer = Reindeer::new(&maze);

    pathfind(&mut reindeer, &mut maze);
    if let Some(min_score) = reindeer.min_score {
    println!("Min score: {}", min_score);
    } else {
        panic!("Failed to find a minimum score!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{find_start_point, read_file_to_array};

    #[test]
    fn test_find_start() {
        let maze = read_file_to_array(_EXAMPLE_1);
        let start_point = find_start_point(&maze);
        assert_eq!(start_point, Vector2d { x: 1, y: 13 });
    }

    #[test]
    fn test_reindeer_methods() {
        let mut reindeer = Reindeer {
            position: Vector2d { x: 5, y: 5 },
            direction: '^',
            score: 0,
            min_score: None,
            found_exit: false,
        };
        reindeer.move_straight();
        assert_eq!(reindeer.position, Vector2d { x: 5, y: 4 });
        assert_eq!(reindeer.score, 1);

        reindeer.turn_left();
        assert_eq!(reindeer.direction, '<');
        assert_eq!(reindeer.score, 1001);
        reindeer.turn_left();
        assert_eq!(reindeer.direction, 'v');
        assert_eq!(reindeer.score, 2001);
        reindeer.turn_left();
        assert_eq!(reindeer.direction, '>');
        assert_eq!(reindeer.score, 3001);
        reindeer.turn_left();
        assert_eq!(reindeer.direction, '^');
        assert_eq!(reindeer.score, 4001);

        reindeer.turn_right();
        assert_eq!(reindeer.direction, '>');
        assert_eq!(reindeer.score, 5001);
        reindeer.turn_right();
        assert_eq!(reindeer.direction, 'v');
        assert_eq!(reindeer.score, 6001);
        reindeer.turn_right();
        assert_eq!(reindeer.direction, '<');
        assert_eq!(reindeer.score, 7001);
        reindeer.turn_right();
        assert_eq!(reindeer.direction, '^');
        assert_eq!(reindeer.score, 8001);
    }

    #[test]
    fn test_obstacle_checking() {
        let maze = read_file_to_array(_EXAMPLE_1);
        _print_grid(&maze);
        let reindeer = Reindeer::new(&maze);

        assert_eq!(
            path_is_clear(&maze, reindeer.position.left()),
            false,
            "Path check to the left failed"
        );
        assert_eq!(
            path_is_clear(&maze, reindeer.position.right()),
            true,
            "Path check to the right failed"
        );
        assert_eq!(
            path_is_clear(&maze, reindeer.position.ahead(reindeer.direction)),
            true,
            "Path check ahead failed"
        );
    }

    #[test]
    fn test_straight_shot() {
        let mut maze = read_file_to_array(_TEST_STRAIGHT_SHOT);
        let mut reindeer = Reindeer::new(&maze);
        pathfind(&mut reindeer, &mut maze);
        assert_eq!(reindeer.min_score, Some(1012));
    }

    #[test]
    fn test_around_corner() {
        let mut maze = read_file_to_array(_TEST_AROUND_CORNER);
        let mut reindeer = Reindeer::new(&maze);
        pathfind(&mut reindeer, &mut maze);
        assert_eq!(reindeer.min_score, Some(2018));
    }

#[test]
    fn test_website_example() {
        let mut maze = read_file_to_array(_EXAMPLE_1);
        let mut reindeer = Reindeer::new(&maze);
        pathfind(&mut reindeer, &mut maze);
        assert_eq!(reindeer.min_score, Some(7036));
    }
}
