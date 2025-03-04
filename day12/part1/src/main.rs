use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::ops::{Add, AddAssign};
use std::path::Path;

const _INPUT: &str = "input.txt";
const _TEST_INPUT: &str = "test_input.txt";

// Reusing lots of code from day #10
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

#[derive(Clone, Copy)]
struct Plot {
    character: char,
    area: i32,
    perimeter: i32,
}
impl Plot {
    fn fresh(x: i32, y: i32, farm: &Vec<Vec<char>>) -> Plot {
        let character = farm[y as usize][x as usize];

        Plot {
            character: character,
            area: 0,
            perimeter: 0,
        }
    }
}
impl Add for Plot {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            character: self.character,
            area: self.area + other.area,
            perimeter: self.perimeter + other.perimeter,
        }
    }
}
impl AddAssign for Plot {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            character: self.character,
            area: self.area + other.area,
            perimeter: self.perimeter + other.perimeter,
        }
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

fn check_array_bounds<T>(x: i32, y: i32, array: &Vec<Vec<T>>) -> bool {
    y >= 0 && y < array.len() as i32 && x >= 0 && x < array[0].len() as i32
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

fn try_next_plot_space(
    position: Position,
    next_direction: Direction,
    farm: &Vec<Vec<char>>,
    mut plot: Plot,
    plot_spaces_visited: &mut HashSet<Position>,
) -> Plot {
    let next_position = get_next_position(position, next_direction);

    // If the next space is not a member of the same plot, do not count it as visited.
    // Instead, increment the perimeter and return
    if !check_array_bounds(next_position.x, next_position.y, farm)
        || farm[next_position.y as usize][next_position.x as usize] != plot.character
    {
        plot.perimeter += 1;
        return plot;
    }

    // This space is a member of the plot, so continue traversing
    plot = traverse_plot(
        farm,
        next_position,
        next_direction,
        plot,
        plot_spaces_visited,
    );

    plot
}

fn traverse_plot(
    farm: &Vec<Vec<char>>,
    position: Position,
    direction: Direction,
    mut plot: Plot,
    plot_spaces_visited: &mut HashSet<Position>,
) -> Plot {
    // If this space has already been visited, make no changes to the area.
    // Otherwise, mark this space as visited and increment the area.
    match plot_spaces_visited.get(&position) {
        Some(_) => return plot,
        None => (),
    }

    // This is a new plot space - increment area and mark this space as visited
    plot.area += 1;
    plot_spaces_visited.insert(position);

    if direction != Direction::Down {
        plot = try_next_plot_space(position, Direction::Up, farm, plot, plot_spaces_visited);
    }
    if direction != Direction::Up {
        plot = try_next_plot_space(position, Direction::Down, farm, plot, plot_spaces_visited);
    }
    if direction != Direction::Right {
        plot = try_next_plot_space(position, Direction::Left, farm, plot, plot_spaces_visited);
    }
    if direction != Direction::Left {
        plot = try_next_plot_space(position, Direction::Right, farm, plot, plot_spaces_visited);
    }

    plot
}

fn compute_fence_price_for_farm(farm: &Vec<Vec<char>>) -> i32 {
    let mut plot_spaces_visited: HashSet<Position> = HashSet::new();
    let mut price = 0;

    for i in 0..farm.len() {
        for j in 0..farm[i].len() {
            let mut plot = Plot::fresh(j as i32, i as i32, &farm);
            plot = traverse_plot(
                &farm,
                Position {
                    x: j as i32,
                    y: i as i32,
                },
                Direction::None,
                plot,
                &mut plot_spaces_visited,
            );

            price += plot.area * plot.perimeter;
        }
    }

    price
}

fn main() {
    let farm = read_file_to_array(_INPUT);
    let price = compute_fence_price_for_farm(&farm);
    println!("Price for all fences in this farm: {}", price);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_plot_area_and_perimeter() {
        let farm = read_file_to_array(_TEST_INPUT);

        let mut plot_spaces_visited: HashSet<Position> = HashSet::new();
        let mut plot = Plot::fresh(0, 0, &farm);

        plot = traverse_plot(
            &farm,
            Position { x: 0, y: 0 },
            Direction::None,
            plot,
            &mut plot_spaces_visited,
        );

        let plot_solution = Plot {
            character: 'R',
            area: 12,
            perimeter: 18,
        };

        assert_eq!(plot.character, plot_solution.character);
        assert_eq!(plot.area, plot_solution.area);
        assert_eq!(plot.perimeter, plot_solution.perimeter);
    }

    #[test]
    fn pass_website_example() {
        let farm = read_file_to_array(_TEST_INPUT);

        assert_eq!(compute_fence_price_for_farm(&farm), 1930);
    }
}
