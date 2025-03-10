use std::collections::HashSet;
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

// For this part, the trick is that you no longer need to find the shortest path to the end. You just need to know if it's reachable.
// I simplified my search algorithm by disallowing it from visiting the same space twice. Also, it now returns true/false, indicating whether
// or not the end could be found, rather than the minimum path length. I took the list of bytes that could fall after 1024 as my search space
// and cut that space in half each time until it honed in on the exact byte that made the end unreachable.

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
struct Searcher {
    position: Coordinate2d,
    score: i32,
    found_exit: bool,
}
impl Searcher {
    fn new() -> Searcher {
        Searcher {
            position: Coordinate2d { x: 0, y: 0 },
            score: 0,
            found_exit: false,
        }
    }
    fn move_to_position(&mut self, next_position: Coordinate2d) {
        self.position = next_position;
        self.score += 1;
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
struct Decision {
    position: Coordinate2d,
}

struct Controller {
    searchspace_min: usize,
    searchspace_max: usize,
    guess: usize,
    best_guess: usize,
}
impl Controller {
    fn new(searchspace_min: usize, searchspace_max: usize) -> Controller {
        Controller {
            searchspace_min: searchspace_min,
            searchspace_max: searchspace_max,
            guess: searchspace_min + (searchspace_max - searchspace_min) / 2,
            best_guess: searchspace_max,
        }
    }

    fn get_next_guess(&mut self, output: bool) {
        // Update search space and best guess based on the results of the lats guess
        if true == output {
            self.searchspace_min = self.guess;
        } else {
            self.searchspace_max = self.guess;
            self.best_guess = self.best_guess.min(self.guess);
        }

        // Get next guess
        self.guess = self.searchspace_min + (self.searchspace_max - self.searchspace_min) / 2;
    }
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

fn spawn_sub_reindeer(
    reindeer: &Searcher,
    new_position: Coordinate2d,
    maze: &Vec<Vec<char>>,
    spaces_visited: &mut HashSet<Decision>,
) -> Searcher {
    // Do not spawn if this space has been visited before
    let mut new_reindeer = reindeer.clone();
    new_reindeer.move_to_position(new_position);

    if let Some(_space) = spaces_visited.get(&Decision {
        position: new_position,
    }) {
        return new_reindeer;
    }

    let mut new_maze = maze.clone();
    mark_map(&mut new_maze, new_reindeer.position);

    pathfind(&mut new_reindeer, &mut new_maze, spaces_visited);

    new_reindeer
}

fn pathfind(
    reindeer: &mut Searcher,
    maze: &mut Vec<Vec<char>>,
    spaces_visited: &mut HashSet<Decision>,
) {
    // If the exit has been found, the search is complete
    if true == reindeer.found_exit {
        return;
    }
    // Check if this space is the exit
    if found_exit(maze, reindeer.position) {
        reindeer.found_exit = true;
        return;
    }

    // Mark this space on the map as visited
    mark_map(maze, reindeer.position);

    // Spawn a new reindeer to explore right. Merge this reindeer's min score with the current one.
    if path_is_clear(maze, reindeer.position.east()) {
        let new_reindeer =
            spawn_sub_reindeer(&reindeer, reindeer.position.east(), &maze, spaces_visited);
        reindeer.found_exit = reindeer.found_exit || new_reindeer.found_exit;
    }
    // Spawn a new reindeer to explore left. Merge this reindeer's min score with the current one.
    if path_is_clear(maze, reindeer.position.west()) {
        let new_reindeer =
            spawn_sub_reindeer(&reindeer, reindeer.position.west(), &maze, spaces_visited);
        reindeer.found_exit = reindeer.found_exit || new_reindeer.found_exit;
    }
    // Spawn a new reindeer to explore up. Merge this reindeer's min score with the current one.
    if path_is_clear(maze, reindeer.position.north()) {
        let new_reindeer =
            spawn_sub_reindeer(&reindeer, reindeer.position.north(), &maze, spaces_visited);
        reindeer.found_exit = reindeer.found_exit || new_reindeer.found_exit;
    }
    // Spawn a new reindeer to explore down. Merge this reindeer's min score with the current one.
    if path_is_clear(maze, reindeer.position.south()) {
        let new_reindeer =
            spawn_sub_reindeer(&reindeer, reindeer.position.south(), &maze, spaces_visited);
        reindeer.found_exit = reindeer.found_exit || new_reindeer.found_exit;
    }

    // Mark this space as visited
    spaces_visited.insert(Decision {
        position: reindeer.position,
    });

    // The reindeer has no more moves to make
    return;
}

fn get_number_of_file_lines(name: &str) -> usize {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut lines_sum = 0;
    for _line in data_string.lines() {
        lines_sum += 1;
    }

    lines_sum
}

fn get_troublestome_byte(name: &str, index: usize) -> Coordinate2d {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut line_counter = 0;
    for line in data_string.lines() {
        line_counter += 1;

        if line_counter == index {
            let mut coords = line.split(",").collect::<Vec<&str>>().into_iter();
            let x: usize = coords.next().unwrap().parse().unwrap();
            let y: usize = coords.next().unwrap().parse().unwrap();
            return Coordinate2d {
                x: x as i32,
                y: y as i32,
            };
        }
    }

    panic!("Failed to fetch byte!");
}

fn main() {
    let searchspace_min = _INPUT_BYTES_TO_SIMULATE;
    let searchspace_max = get_number_of_file_lines(_INPUT);
    let mut controller = Controller::new(searchspace_min, searchspace_max);

    println!("\nSearchspace Start: byte #{}, Searchspace End: byte #{}\n", searchspace_min, searchspace_max);

    let mut last_guess = 0;
    let mut this_guess;
    while last_guess != controller.guess {
        let mut maze = read_file_to_array(_INPUT, _INPUT_GRIDSIZE, controller.guess);

        print!("Guess: byte #{} | ", controller.guess);
        let mut person = Searcher::new();
        let mut spaces_visited: HashSet<Decision> = HashSet::new();
        pathfind(&mut person, &mut maze, &mut spaces_visited);
        println!("End was reached: {}", person.found_exit);

        this_guess = controller.guess;
        controller.get_next_guess(person.found_exit);
        last_guess = this_guess;
    }

    println!(
        "\nEnd was made unreachable by byte #{}\nWhen it fell at the position: {}\n",controller.best_guess,
        get_troublestome_byte(_INPUT, controller.best_guess)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_website_example() {
        let searchspace_min = _EXAMPLE_1_BYTES_TO_SIMULATE;
        let searchspace_max = get_number_of_file_lines(_EXAMPLE_1);
        let mut controller = Controller::new(searchspace_min, searchspace_max);

        println!("min: {}, max: {}", searchspace_min, searchspace_max);

        let mut last_guess = 0;
        let mut this_guess = 0;
        while last_guess != controller.guess {
            let mut maze = read_file_to_array(_EXAMPLE_1, _EXAMPLE_1_GRIDSIZE, controller.guess);

            print!("Guess: {} | ", controller.guess);
            let mut person = Searcher::new();
            let mut spaces_visited: HashSet<Decision> = HashSet::new();
            pathfind(&mut person, &mut maze, &mut spaces_visited);
            println!("Result: {}", person.found_exit);

            this_guess = controller.guess;
            controller.get_next_guess(person.found_exit);
            last_guess = this_guess;
        }

    assert_eq!(get_troublestome_byte(_EXAMPLE_1, controller.best_guess), Coordinate2d{ x: 6, y: 1 });
    }
}