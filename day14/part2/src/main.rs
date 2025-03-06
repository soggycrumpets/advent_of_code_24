use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;
use std::path::Path;

use std::thread::sleep;
use std::time::{Duration, Instant};

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

#[derive(Clone)]
struct Robot {
    position: Vector2d,
    velocity: Vector2d,
}

fn load_robots_from_file(name: &str) -> Vec<Robot> {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut robots: Vec<Robot> = Vec::new();
    for line in data_string.lines() {
        let line_filtered = line
            .chars()
            .filter(|char| {
                char.is_numeric() || *char == ',' || char.is_whitespace() || *char == '-'
            })
            .collect::<String>();
        let mut split_line = line_filtered.split_whitespace();
        let mut position = split_line.next().unwrap().split(',');
        let position_x = position.next().unwrap().parse().unwrap();
        let position_y = position.next().unwrap().parse().unwrap();

        let mut velocity = split_line.next().unwrap().split(',');
        let velocity_x = velocity.next().unwrap().parse().unwrap();
        let velocity_y = velocity.next().unwrap().parse().unwrap();

        let robot = Robot {
            position: Vector2d {
                x: position_x,
                y: position_y,
            },
            velocity: Vector2d {
                x: velocity_x,
                y: velocity_y,
            },
        };
        robots.push(robot);
    }

    robots
}
fn update_robot_location(map: &Vec<Vec<i32>>, robot: &mut Robot, delta_t: i32) {
    robot.position.x += robot.velocity.x * delta_t as i32;
    robot.position.y += robot.velocity.y * delta_t as i32;

    // Wrap positive x
    if robot.position.x >= 0 {
        robot.position.x %= (map[0].len()) as i32;
    }
    // Wrap negative x
    else {
        robot.position.x %= -((map[0].len()) as i32);
        if robot.position.x != 0 {
            robot.position.x += map[0].len() as i32;
        }
    }

    // Wrap positive y
    if robot.position.y >= 0 {
        robot.position.y %= (map.len()) as i32;
    }
    // Wrap negative y
    else {
        robot.position.y %= -((map.len()) as i32);
        if robot.position.y != 0 {
            robot.position.y += map.len() as i32;
        }
    }
}

fn sum_robots_in_quadrant(
    map: &Vec<Vec<i32>>,
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
) -> i32 {
    let mut sum = 0;
    for y in ymin..ymax {
        for x in xmin..xmax {
            sum += map[y][x];
        }
    }

    sum
}

fn compute_safety_factor(map: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    let xmid = (map[0].len() - 1) / 2;
    let xmax = map[0].len();
    let ymid = (map.len() - 1) / 2;
    let ymax = map.len();
    // eprintln!("{} {} {} {}", xmid, xmax, ymid, ymax);

    sum = sum_robots_in_quadrant(map, 0, xmid, 0, ymid);
    sum *= sum_robots_in_quadrant(map, xmid + 1, xmax, 0, ymid);
    sum *= sum_robots_in_quadrant(map, 0, xmid, ymid + 1, ymax);
    sum *= sum_robots_in_quadrant(map, xmid + 1, xmax, ymid + 1, ymax);

    sum
}

/* ---------------------------IMPORTED CODE FROM DAY 12 ---------------------------*/
#[derive(PartialEq, Clone, Copy)]
enum Direction {
    None,
    Left,
    Right,
    Up,
    Down,
}

fn check_array_bounds<T>(x: i32, y: i32, array: &Vec<Vec<T>>) -> bool {
    y >= 0 && y < array.len() as i32 && x >= 0 && x < array[0].len() as i32
}

fn get_next_position(position: Vector2d, direction: Direction) -> Vector2d {
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

fn try_next_plot_space(
    position: Vector2d,
    next_direction: Direction,
    farm: &Vec<Vec<i32>>,
    mut sum: i32,
    plot_spaces_visited: &mut HashSet<Vector2d>,
) -> i32 {
    let next_position = get_next_position(position, next_direction);

    // If the next space is not a member of the same sum, do not count it as visited.
    // Instead, increment the perimeter and return
    if !check_array_bounds(next_position.x, next_position.y, farm)
        || farm[next_position.y as usize][next_position.x as usize] == 0
    {
        return sum;
    }

    // This space is a member of the sum, so continue traversing
    sum = traverse_plot(
        farm,
        next_position,
        next_direction,
        sum,
        plot_spaces_visited,
    );

    sum
}

fn traverse_plot(
    farm: &Vec<Vec<i32>>,
    position: Vector2d,
    direction: Direction,
    mut sum: i32,
    plot_spaces_visited: &mut HashSet<Vector2d>,
) -> i32 {
    // If this space has already been visited, make no changes to the area.
    // Otherwise, mark this space as visited and increment the area.
    match plot_spaces_visited.get(&position) {
        Some(_) => return sum,
        None => (),
    }

    // This is a new space - increment area and mark this space as visited
    sum += 1;
    plot_spaces_visited.insert(position);

    if direction != Direction::Down {
        sum = try_next_plot_space(position, Direction::Up, farm, sum, plot_spaces_visited);
    }
    if direction != Direction::Up {
        sum = try_next_plot_space(position, Direction::Down, farm, sum, plot_spaces_visited);
    }
    if direction != Direction::Right {
        sum = try_next_plot_space(position, Direction::Left, farm, sum, plot_spaces_visited);
    }
    if direction != Direction::Left {
        sum = try_next_plot_space(position, Direction::Right, farm, sum, plot_spaces_visited);
    }

    sum
}
/* --------------------------------------------------------------------------------*/

fn print_map(map: &Vec<Vec<i32>>) {
    let mut buf = String::new();
    for i in map {
        for j in i {
            if *j > 0 {
                // print!("O");
                buf.push('O');
            } else {
                buf.push('.');
            }
        }
        buf.push('\n');
    }
    print!("{}", buf);
}

fn find_biggest_group_size(map: &Vec<Vec<i32>>) -> i32 {
    let mut visited_spaces = HashSet::new();
    let mut neighbors = 0;
    for i in 0..map[0].len() {
        for j in 0..map.len() {
            let position = Vector2d {
                x: j as i32,
                y: i as i32,
            };
            neighbors = neighbors.max(traverse_plot(
                map,
                position,
                Direction::None,
                0,
                &mut visited_spaces,
            ));
        }
    }

    neighbors
}

fn main() {
    let mut robots = load_robots_from_file(_INPUT);

    let map_width = 101;
    let map_height = 103;
    let mut map: Vec<Vec<i32>> = vec![vec![0; map_width]; map_height];

    let interval = Duration::from_secs(1) / 30;
    let mut next_time = Instant::now() + interval;

    let delta_t = 1;
    let mut seconds_elapsed: u32 = 0;
    let mut buf = String::new();
    loop {
        let mut map: Vec<Vec<i32>> = vec![vec![0; map_width]; map_height];
        for robot in &mut robots {
            update_robot_location(&map, robot, delta_t);
            map[robot.position.y as usize][robot.position.x as usize] += 1;
        }
        // next_time += interval;
        seconds_elapsed += 1;

        let biggest_group = find_biggest_group_size(&map);
        // println!("SECONDS: {} | Largest Group: {}", seconds_elapsed, biggest_group);
        if biggest_group > 10 {
            print!("\n\n\n\n\n\n\n\nSECONDS ELAPSED: {}\n\n", seconds_elapsed);
            print_map(&map);
        }
        // sleep(next_time - Instant::now());
    }

    println!("{}", compute_safety_factor(&map));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_robots_from_file() {
        let robots = load_robots_from_file(_TEST_INPUT);
        let robot = robots[0].clone();

        assert_eq!(robot.position.x, 0);
        assert_eq!(robot.position.y, 4);
        assert_eq!(robot.velocity.x, 3);
        assert_eq!(robot.velocity.y, -3);
    }

    #[test]
    fn test_website_example() {
        let robots = load_robots_from_file(_TEST_INPUT);

        let map_width = 11;
        let map_height = 7;
        let mut map: Vec<Vec<i32>> = vec![vec![0; map_width]; map_height];

        let delta_t = 100;
        for mut robot in robots {
            update_robot_location(&map, &mut robot, delta_t);
            map[robot.position.y as usize][robot.position.x as usize] += 1;
        }
        assert_eq!(compute_safety_factor(&map), 12);
    }
}
