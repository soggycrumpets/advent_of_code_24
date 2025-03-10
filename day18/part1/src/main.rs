use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input.txt";
const _INPUT_GRIDSIZE: usize = 71;
const _INPUT_BYTES_TO_SIMULATE: usize = 1024;

const _EXAMPLE_1: &str = "example1.txt";
const _EXAMPLE_1_GRIDSIZE: usize = 7;
const _EXAMPLE_1_BYTES_TO_SIMULATE: usize = 12;

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
}
impl fmt::Display for Coordinate2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Reindeer {
    position: Coordinate2d,
    score: i32,
    min_score: Option<i32>,
}
impl Reindeer {
    fn new() -> Reindeer {
        Reindeer {
            position: Coordinate2d { x: 0, y: 0 },
            score: 0,
            min_score: None,
        }
    }
    fn move_to_position(&mut self, next_position: Coordinate2d) {
        self.position = next_position;
        self.score += 1;
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
}

fn in_array_bounds(x: i32, y: i32, grid: &Vec<Vec<char>>) -> bool {
    x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32
}

fn mark_map(map: &mut Vec<Vec<char>>, position: Coordinate2d) {
    map[position.y as usize][position.x as usize] = 'O';
}

fn path_is_clear(maze: &Vec<Vec<char>>, position: Coordinate2d) -> bool {
    if !in_array_bounds(position.x, position.y, &maze) {
        return false;
    }

    let c = maze[position.y as usize][position.x as usize];
    match c {
        '#' | 'O' => false,
        _ => true,
    }
}

fn _print_grid(grid: &Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            print!("{}", grid[i][j]);
        }
        println!();
    }
    println!();
    println!();
}

fn read_file_to_array(name: &str, gridsize: usize, bytes_to_simulate: usize) -> Vec<Vec<char>> {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; gridsize]; gridsize];

    let data_lines = data_string.lines();

    let mut bytes_fallen = 0;
    for line in data_lines {
        let mut coords = line.split(",").collect::<Vec<&str>>().into_iter();
        let x: usize = coords.next().unwrap().parse().unwrap();
        let y: usize = coords.next().unwrap().parse().unwrap();
        grid[y][x] = '#';
        bytes_fallen += 1;

        if bytes_fallen >= bytes_to_simulate {
            break;
        }
    }
    grid
}

fn found_exit(maze: &Vec<Vec<char>>, position: Coordinate2d) -> bool {
    if position.y as usize == maze.len() - 1 && position.x as usize == maze[0].len() - 1 {
        true
    } else {
        false
    }
}

// Record the minimum points accrued from getting to the end from this location
fn record_min_score(reindeer: &mut Reindeer, min_scores: &mut HashMap<Decision, i32>) {
    let decision = Decision {
        position: reindeer.position,
    };

    // Update the minimum steps required to reach this tile
        min_scores.insert(decision, reindeer.score);
}

// Returns false if another reindeer has reached this decision with a lower score
fn contemplate_decision(reindeer: &Reindeer, min_scores: &mut HashMap<Decision, i32>) -> bool {
    let decision = Decision {
        position: reindeer.position,
    };

    if let Some(min_steps_to_reach) = min_scores.get(&decision) {
        return reindeer.score < *min_steps_to_reach;
    }

    return true;
}

fn spawn_sub_reindeer(
    reindeer: &Reindeer,
    new_position: Coordinate2d,
    maze: &Vec<Vec<char>>,
    min_scores: &mut HashMap<Decision, i32>,
) -> Reindeer {
    let mut new_reindeer = reindeer.clone();
    new_reindeer.move_to_position(new_position);

    // Abort if score is exceeding the minimum score
    if let Some(min_score) = new_reindeer.min_score {
        if new_reindeer.score > min_score {
            return new_reindeer;
        }
    }
    if !contemplate_decision(reindeer, min_scores) {
        return new_reindeer;
    }

    let mut new_maze = maze.clone();
    mark_map(&mut new_maze, new_reindeer.position);

    pathfind(&mut new_reindeer, &mut new_maze, min_scores);

    new_reindeer
}

fn pathfind(
    reindeer: &mut Reindeer,
    maze: &mut Vec<Vec<char>>,
    min_scores: &mut HashMap<Decision, i32>,
) {
    // If the exit is found, this score is a potential minimum score. Merge it with the current minimum score
    if found_exit(maze, reindeer.position) {
        reindeer.merge_min_score(Some(reindeer.score));
        println!("Score: {}", reindeer.score);
        _print_grid(maze);
        return;
    }
    if !contemplate_decision(reindeer, min_scores) {
        return;
    }
    if let Some(min_score) = reindeer.min_score {
        if reindeer.score > min_score {
            return;
        }
    }

    // _print_grid(maze);

    // Mark this space on the map as visited
    mark_map(maze, reindeer.position);

    // Spawn a new reindeer to explore right. Merge this reindeer's min score with the current one.
    if path_is_clear(maze, reindeer.position.east()) {
        let new_reindeer =
            spawn_sub_reindeer(&reindeer, reindeer.position.east(), &maze, min_scores);
        reindeer.merge_min_score(new_reindeer.min_score);
    }
    // Spawn a new reindeer to explore left. Merge this reindeer's min score with the current one.
    if path_is_clear(maze, reindeer.position.west()) {
        let new_reindeer =
            spawn_sub_reindeer(&reindeer, reindeer.position.west(), &maze, min_scores);
        reindeer.merge_min_score(new_reindeer.min_score);
    }
    // Spawn a new reindeer to explore up. Merge this reindeer's min score with the current one.
    if path_is_clear(maze, reindeer.position.north()) {
        let new_reindeer =
            spawn_sub_reindeer(&reindeer, reindeer.position.north(), &maze, min_scores);
        reindeer.merge_min_score(new_reindeer.min_score);
    }
    // Spawn a new reindeer to explore down. Merge this reindeer's min score with the current one.
    if path_is_clear(maze, reindeer.position.south()) {
        let new_reindeer =
            spawn_sub_reindeer(&reindeer, reindeer.position.south(), &maze, min_scores);
        reindeer.merge_min_score(new_reindeer.min_score);
    }

    // Record the Minimum score to this decision
    record_min_score(reindeer, min_scores);

    // The reindeer has no more moves to make
    return;
}

fn main() {
    let mut maze = read_file_to_array(_INPUT, _INPUT_GRIDSIZE, _INPUT_BYTES_TO_SIMULATE);
    _print_grid(&maze);

    let mut person = Reindeer::new();
    let mut decision_costs: HashMap<Decision, i32> = HashMap::new();
    pathfind(&mut person, &mut maze, &mut decision_costs);

    if let Some(min_score) = person.min_score {
        println!("\nMinimum path length: {}\n", min_score);
    } else {
        panic!("Failed to find a path!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_website_example() {
        let mut maze = read_file_to_array(
            _EXAMPLE_1,
            _EXAMPLE_1_GRIDSIZE,
            _EXAMPLE_1_BYTES_TO_SIMULATE,
        );
        _print_grid(&maze);
        println!();

        let mut person = Reindeer::new();
        let mut decision_costs: HashMap<Decision, i32> = HashMap::new();
        pathfind(&mut person, &mut maze, &mut decision_costs);

        if let Some(min_score) = person.min_score {
            println!("\nMinimum path length: {}\n", min_score);
        } else {
            panic!("Failed to find a path!");
        }
    }
}
