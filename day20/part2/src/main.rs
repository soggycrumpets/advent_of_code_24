use std::collections::HashMap;
use std::fmt::{self, Pointer};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input.txt";
const _EXAMPLE: &str = "example.txt";

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn west(&self, step: i32) -> Position {
        Position {
            x: self.x - step,
            y: self.y,
        }
    }
    fn east(&self, step: i32) -> Position {
        Position {
            x: self.x + step,
            y: self.y,
        }
    }
    fn north(&self, step: i32) -> Position {
        Position {
            x: self.x,
            y: self.y - step,
        }
    }
    fn south(&self, step: i32) -> Position {
        Position {
            x: self.x,
            y: self.y + step,
        }
    }
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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

fn pathfind(
    racer: &mut Position,
    track: &mut Vec<Vec<char>>,
    spaces: &mut HashMap<Position, i32>,
    steps: &mut i32,
) {
    spaces.insert(*racer, *steps);
    // Check if this space is the exit
    if found_exit(track, *racer) {
        return;
    }
    *steps += 1;
    // Mark this space on the map as visited
    mark_map(track, *racer);

    // Find the next direction to go down and continue
    if path_is_clear(track, racer.east(1)) {
        pathfind(&mut racer.east(1), track, spaces, steps);
    }
    if path_is_clear(track, racer.west(1)) {
        pathfind(&mut racer.west(1), track, spaces, steps);
    }
    if path_is_clear(track, racer.north(1)) {
        pathfind(&mut racer.north(1), track, spaces, steps);
    }
    if path_is_clear(track, racer.south(1)) {
        pathfind(&mut racer.south(1), track, spaces, steps);
    }

    return;
}

fn in_array_bounds(x: i32, y: i32, grid: &Vec<Vec<char>>) -> bool {
    x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32
}

fn mark_map(map: &mut Vec<Vec<char>>, position: Position) {
    map[position.y as usize][position.x as usize] = 'O';
}

fn path_is_clear(maze: &Vec<Vec<char>>, position: Position) -> bool {
    if !in_array_bounds(position.x, position.y, &maze) {
        return false;
    }

    let c = maze[position.y as usize][position.x as usize];
    match c {
        '#' | 'O' => false,
        _ => true,
    }
}

fn find_start_point(grid: &Vec<Vec<char>>) -> Position {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                return Position {
                    x: j as i32,
                    y: i as i32,
                };
            }
        }
    }
    panic!("Start point not found!");
}

fn found_exit(track: &Vec<Vec<char>>, position: Position) -> bool {
    if track[position.y as usize][position.x as usize] == 'E' {
        return true;
    } else {
        return false;
    }
}

fn test_cheat(
    spaces: &HashMap<Position, i32>,
    position_start: Position,
    (position_end, time): (Position, i32),
    cheating_timesaves: &mut Vec<i32>,
) {
    if let Some(new_time) = spaces.get(&position_end) {
        let timesave = new_time
            - time
            - (position_start.x - position_end.x).abs()
            - (position_start.y - position_end.y).abs();
        if timesave > 0 {
            cheating_timesaves.push(timesave);
        }
    }
}

fn test_all_cheats_for_space(
    spaces: &HashMap<Position, i32>,
    (position, time): (Position, i32),
    cheating_timesaves: &mut Vec<i32>,
    max_cheat_time: usize,
) {
    let possible_cheats = assemble_cheat_list(max_cheat_time, position);
    for cheat in possible_cheats {
        test_cheat(spaces, position, (cheat, time), cheating_timesaves);
    }
}

fn assemble_cheat_list(max_cheat_time: usize, position: Position) -> Vec<Position> {
    let mut cheatable_spaces: Vec<Position> = Vec::new();

    for i in 0..2 * max_cheat_time + 1 {
        for j in 0..2 * max_cheat_time + 1 {
            let x: i32 = j as i32 - max_cheat_time as i32;
            let y: i32 = i as i32 - max_cheat_time as i32;
            if x.abs() + y.abs() <= max_cheat_time as i32 {
                cheatable_spaces.push(Position {
                    x: position.x + x,
                    y: position.y + y,
                });
            }
        }
    }

    cheatable_spaces
}

fn main() {
    let mut track = read_file_to_array(_INPUT);
    let mut racer = find_start_point(&track);
    let mut spaces: HashMap<Position, i32> = HashMap::new();
    let mut steps = 0;
    _print_grid(&track);
    pathfind(&mut racer, &mut track, &mut spaces, &mut steps);
    println!("The race was finished in {} picoseconds", steps);

    let mut cheating_timesaves: Vec<i32> = Vec::new();
    let max_cheat_time = 20;
    for (position, time) in &spaces {
        test_all_cheats_for_space(&spaces, (*position, *time), &mut cheating_timesaves, max_cheat_time);
    }
    let mut best_timesaves: Vec<i32> = Vec::new();
    let minimum_timesave_allowed = 100;
    for timesave in cheating_timesaves {
        if timesave >= minimum_timesave_allowed {
            best_timesaves.push(timesave);
        }
    }
    for timesave in &best_timesaves {
        println!("{}", timesave);
    }
    println!(
        "\nNumber of cheats that save 100+ picoseconds: {}\n",
        best_timesaves.len()
    );
}

#[test]
fn print_website_example() {
    let mut track = read_file_to_array(_EXAMPLE);
    let mut racer = find_start_point(&track);
    let mut spaces: HashMap<Position, i32> = HashMap::new();
    let mut steps = 0;
    pathfind(&mut racer, &mut track, &mut spaces, &mut steps);
    println!("The race was finished in {} picoseconds", steps);

    let mut cheating_timesaves: Vec<i32> = Vec::new();
    let max_cheat_time = 20;
    for (position, time) in &spaces {
        test_all_cheats_for_space(
            &spaces,
            (*position, *time),
            &mut cheating_timesaves,
            max_cheat_time,
        );
    }
    let mut best_timesaves: Vec<i32> = Vec::new();
    let minimum_timesave_allowed = 50;
    for timesave in cheating_timesaves {
        if timesave >= minimum_timesave_allowed {
            best_timesaves.push(timesave);
        }
    }
    for timesave in best_timesaves {
        println!("{}", timesave);
    }
}

#[test]
fn print_test_assemble_cheat_array() {
    let cheat_list= assemble_cheat_list(20, Position { x: 0, y: 0});
    for i in cheat_list {
        println!("{}", i);
    }
    println!();
}
