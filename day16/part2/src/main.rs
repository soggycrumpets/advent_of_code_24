use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::{self, Write};
use std::path::Path;

const _INPUT: &str = "input.txt";
const _EXAMPLE_1: &str = "example1.txt";
const _EXAMPLE_2: &str = "example2.txt";
const _TEST_STRAIGHT_SHOT: &str = "test_straight_shot.txt";
const _TEST_AROUND_CORNER: &str = "test_around_corner.txt";
const _TEST_BASE_CASE: &str = "test_base_case.txt";

/*            The code relies on this being input from the start to solve part 2 as of now             */
/*            This includes the tests for the website examples                                         */
const _PART1_SOLUTION: i32 = 101492;


static mut ITERATIONS: u64 = 0;

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
struct Coordinate2d {
    x: i32,
    y: i32,
}
impl Coordinate2d {
    fn west(&self) -> Coordinate2d {
        Coordinate2d {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn east(&self) -> Coordinate2d {
        Coordinate2d {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn north(&self) -> Coordinate2d {
        Coordinate2d {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn south(&self) -> Coordinate2d {
        Coordinate2d {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn ahead(&self, direction: char) -> Coordinate2d {
        match direction {
            '^' => self.north(),
            '>' => self.east(),
            'v' => self.south(),
            '<' => self.west(),
            _ => panic!(
                "Tried to get next position, but direction is \"{}\"",
                direction
            ),
        }
    }
}
impl fmt::Display for Coordinate2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Reindeer {
    position: Coordinate2d,
    direction: char,
    score: i32,
    min_score: Option<i32>,
}
impl Reindeer {
    fn new(maze: &Vec<Vec<char>>) -> Reindeer {
        Reindeer {
            position: find_start_point(maze),
            direction: '>',
            score: 0,
            min_score: None,
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
                "Tried to face left, but direction is \"{}\"",
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
                "Tried to face right, but direction is \"{}\"",
                self.direction
            ),
        }
    }
    fn turn(&mut self, direction: char) {
        self.direction = direction;
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

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
struct Decision {
    position: Coordinate2d,
    direction: char,
}

fn mark_map(map: &mut Vec<Vec<char>>, position: Coordinate2d, mark: char) {
    map[position.y as usize][position.x as usize] = mark;
}

fn path_is_clear(maze: &Vec<Vec<char>>, position: Coordinate2d) -> bool {
    let c = maze[position.y as usize][position.x as usize];
    match c {
        '#' | '<' | '>' | '^' | 'v' => {
            // print!("{}", c);
            // io::stdout().flush().unwrap();
            false
        }
        _ => true,
    }
}

fn find_start_point(grid: &Vec<Vec<char>>) -> Coordinate2d {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                return Coordinate2d {
                    x: j as i32,
                    y: i as i32,
                };
            }
        }
    }
    panic!("Start point not found!");
}

fn _print_grid(grid: &Vec<Vec<char>>, reindeer_position: Coordinate2d) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if reindeer_position
                == (Coordinate2d {
                    y: i as i32,
                    x: j as i32,
                })
            {
                let deer = emojis::get("🦌").unwrap();
                print!("{}", deer);
            } else {
                print!("{}", grid[i][j]);
            }
        }
        println!();
    }
}

fn add_potential_seats(maze: &Vec<Vec<char>>, seats: &mut Vec<Vec<char>>) {
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            match maze[i][j] {
                '>' | '<' | 'v' | '^' | 'E' | 'S' => seats[i][j] = 'X',
                _ => {}
            }
        }
    }
}

fn count_potential_seats(seats: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for row in seats {
        for c in row {
            if *c == 'X' {
                sum += 1;
            }
        }
    }

    sum
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

fn found_exit(maze: &Vec<Vec<char>>, position: Coordinate2d) -> bool {
    if maze[position.y as usize][position.x as usize] == 'E' {
        true
    } else {
        false
    }
}

// Record the minimum points accrued from getting to the end from this location
fn record_min_score(reindeer: &mut Reindeer, min_scores: &mut HashMap<Decision, i32>) {
    let decision = Decision {
        position: reindeer.position,
        direction: reindeer.direction,
    };

    // Update the min score for this tile
    if let Some(min_score) = reindeer.min_score {
        let points_to_finish = min_score - reindeer.score;
        min_scores.insert(decision, points_to_finish);
    } 
}

// Returns false if another reindeer has reached this decision with a lower score
fn contemplate_decision(
    reindeer: &Reindeer,
    min_scores: &mut HashMap<Decision, i32>,
) -> bool {
    let decision = Decision {
        position: reindeer.position,
        direction: reindeer.direction,
    };

    if let Some(points_to_finish) = min_scores.get(&decision) {
        let new_min_score = points_to_finish + reindeer.score;
        if let Some(min_score) = reindeer.min_score {
    /*                            IMPORTANT                                            */
    /*                            Must be <= instead of < for part 2.                  */
            return new_min_score <= min_score
        }
    }
    return true;
}

fn spawn_sub_reindeer(
    reindeer: &Reindeer,
    maze: &Vec<Vec<char>>,
    spawn_direction: char,
    min_scores: &mut HashMap<Decision, i32>,
    potential_seats: &mut Vec<Vec<char>>,
) -> Reindeer {
    let mut new_reindeer = reindeer.clone();

    new_reindeer.turn(spawn_direction);

    // Abort if score is exceeding the minimum score
    if let Some(min_score) = new_reindeer.min_score {
        if new_reindeer.score > min_score {
            return new_reindeer;
        }
    }

    let mut new_maze = maze.clone();
    mark_map(&mut new_maze, new_reindeer.position, new_reindeer.direction);

    new_reindeer.move_straight();

    pathfind(&mut new_reindeer, &mut new_maze, min_scores, potential_seats);

    new_reindeer
}

fn pathfind(
    reindeer: &mut Reindeer,
    maze: &mut Vec<Vec<char>>,
    min_scores: &mut HashMap<Decision, i32>,
    potential_seats: &mut Vec<Vec<char>>,
) {
    unsafe {
        if ITERATIONS % 100000 == 0 {
            // _print_grid(maze, reindeer.position);
        }
        ITERATIONS += 1;
    }

    // If the exit is found, this score is a potential minimum score. Merge it with the current minimum score
    if found_exit(maze, reindeer.position) {
        reindeer.merge_min_score(Some(reindeer.score));
        println!("Score: {}", reindeer.score);
        if reindeer.score == _PART1_SOLUTION{
            add_potential_seats(maze, potential_seats);
        }
        _print_grid(maze, reindeer.position);
        return;
    }

    // Mark this space on the map as visited
    mark_map(maze, reindeer.position, reindeer.direction);

 
    // Spawn a new reindeer to explore right. Merge this reindeer's min score with the current one.
    if path_is_clear(maze, reindeer.position.ahead(reindeer.right())) {
        let new_reindeer = spawn_sub_reindeer(&reindeer, &maze, reindeer.right(), min_scores, potential_seats);
        reindeer.merge_min_score(new_reindeer.min_score);
    }
   // Spawn a new reindeer to explore left. Merge this reindeer's min score with the current one.
    if path_is_clear(maze, reindeer.position.ahead(reindeer.left())) {
        let new_reindeer = spawn_sub_reindeer(&reindeer, &maze, reindeer.left(), min_scores, potential_seats);
        reindeer.merge_min_score(new_reindeer.min_score);
    }
    // Move reindeer ahead.
    if path_is_clear(maze, reindeer.position.ahead(reindeer.direction)) {
        reindeer.move_straight();

        if !contemplate_decision(reindeer, min_scores) {
            return
        }

        pathfind(reindeer, maze, min_scores, potential_seats);
    }

    // Record the Minimum score to this decision
    record_min_score(reindeer, min_scores);

    // The reindeer has no more moves to make
    return;
}

fn main() {
    let mut maze = read_file_to_array(_INPUT);
    // _print_grid(&maze, );
    let mut reindeer = Reindeer::new(&maze);

    let mut min_scores: HashMap<Decision, i32> = HashMap::new();
    let mut potential_seats: Vec<Vec<char>> = maze.clone();
    pathfind(&mut reindeer, &mut maze, &mut min_scores, &mut potential_seats);
    if let Some(min_score) = reindeer.min_score {
        println!("Min score: {}", min_score);
    } else {
        panic!("Failed to find a minimum score!");
    }
    println!("Potential seats: {}", count_potential_seats(&potential_seats));
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{find_start_point, read_file_to_array};

    #[test]
    fn test_find_start() {
        let maze = read_file_to_array(_EXAMPLE_1);
        let start_point = find_start_point(&maze);
        assert_eq!(start_point, Coordinate2d { x: 1, y: 13 });
    }

    #[test]
    fn test_reindeer_methods() {
        let mut reindeer = Reindeer {
            position: Coordinate2d { x: 5, y: 5 },
            direction: '^',
            score: 0,
            min_score: None,
        };
        reindeer.move_straight();
        assert_eq!(reindeer.position, Coordinate2d { x: 5, y: 4 });
        assert_eq!(reindeer.score, 1);

        reindeer.turn(reindeer.left());
        assert_eq!(reindeer.direction, '<');
        assert_eq!(reindeer.score, 1001);
        reindeer.turn(reindeer.left());
        assert_eq!(reindeer.direction, 'v');
        assert_eq!(reindeer.score, 2001);
        reindeer.turn(reindeer.left());
        assert_eq!(reindeer.direction, '>');
        assert_eq!(reindeer.score, 3001);
        reindeer.turn(reindeer.left());
        assert_eq!(reindeer.direction, '^');
        assert_eq!(reindeer.score, 4001);

        reindeer.turn(reindeer.right());
        assert_eq!(reindeer.direction, '>');
        assert_eq!(reindeer.score, 5001);
        reindeer.turn(reindeer.right());
        assert_eq!(reindeer.direction, 'v');
        assert_eq!(reindeer.score, 6001);
        reindeer.turn(reindeer.right());
        assert_eq!(reindeer.direction, '<');
        assert_eq!(reindeer.score, 7001);
        reindeer.turn(reindeer.right());
        assert_eq!(reindeer.direction, '^');
        assert_eq!(reindeer.score, 8001);
    }

    #[test]
    fn test_obstacle_checking() {
        let maze = read_file_to_array(_EXAMPLE_1);
        // _print_grid(&maze);
        let reindeer = Reindeer::new(&maze);

        assert_eq!(
            path_is_clear(&maze, reindeer.position.west()),
            false,
            "Path check to the left failed"
        );
        assert_eq!(
            path_is_clear(&maze, reindeer.position.east()),
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
        let mut potential_seats = maze.clone();
        let mut reindeer = Reindeer::new(&maze);
        let mut min_scores: HashMap<Decision, i32> = HashMap::new();

        pathfind(&mut reindeer, &mut maze, &mut min_scores, &mut potential_seats);
        assert_eq!(reindeer.min_score, Some(1012));
    }

    #[test]
    fn test_around_corner() {
        let mut maze = read_file_to_array(_TEST_AROUND_CORNER);
        let mut potential_seats = maze.clone();
        let mut reindeer = Reindeer::new(&maze);
        let mut min_scores: HashMap<Decision, i32> = HashMap::new();
        pathfind(&mut reindeer, &mut maze, &mut min_scores, &mut potential_seats);
        assert_eq!(reindeer.min_score, Some(2018));
    }

    #[test]
    fn test_website_examples() {
        {
            let mut maze = read_file_to_array(_EXAMPLE_1);
            let mut potential_seats = maze.clone();
            let mut reindeer = Reindeer::new(&maze);
            let mut min_scores: HashMap<Decision,i32> = HashMap::new();
            pathfind(&mut reindeer, &mut maze, &mut min_scores, &mut potential_seats);
            assert_eq!(reindeer.min_score, Some(7036), "Failed example 1!");
        }
        {
            let mut maze = read_file_to_array(_EXAMPLE_2);
            let mut potential_seats = maze.clone();
            let mut reindeer = Reindeer::new(&maze);
            let mut min_scores: HashMap<Decision, i32> = HashMap::new();
            pathfind(&mut reindeer, &mut maze, &mut min_scores, &mut potential_seats);
            assert_eq!(reindeer.min_score, Some(11048), "failed example 2!");
        }
        {
            let mut maze = read_file_to_array(_EXAMPLE_2);
            let mut potential_seats = maze.clone();
            let mut reindeer = Reindeer::new(&maze);
            let mut min_scores: HashMap<Decision, i32> = HashMap::new();
            pathfind(&mut reindeer, &mut maze, &mut min_scores, &mut potential_seats);
            assert_eq!(count_potential_seats(&potential_seats), 64);
        }
    }
}