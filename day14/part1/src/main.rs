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

#[derive(Clone)]
struct Robot {
    position: Vector2d,
    velocity: Vector2d,
}
// impl Robot {
//     fn new() -> Robot {
//         Robot {
//             position: Vector2d { x: 0, y: 0 },
//             velocity: Vector2d { x: 0, y: 0 },
//         }
//     }
// }

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

fn main() {
    let robots = load_robots_from_file(_INPUT);

    let map_width = 101;
    let map_height = 103;
    let mut map: Vec<Vec<i32>> = vec![vec![0; map_width]; map_height];

    let delta_t = 100;
    for mut robot in robots {
        update_robot_location(&map, &mut robot, delta_t);
        map[robot.position.y as usize][robot.position.x as usize] += 1;
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

        for i in &map {
            for j in i {
                if *j > 0 {
                    eprint!("{}", j);
                } else {
                    eprint!(".")
                }
            }
            eprintln!();
        }

        assert_eq!(compute_safety_factor(&map), 12);
    }
}
