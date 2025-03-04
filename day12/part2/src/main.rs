use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input.txt";
const _TEST_INPUT: &str = "test_input.txt";
const _TEST_INPUT_2: &str = "test_input_2.txt";

// Reusing lots of code from day #10
#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    None,
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn get_reverse(&self) -> Self {
        match *self {
            Direction::None => Direction::None,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::None => write!(f, "None"),
        }
    }
}

#[derive(Clone)]
struct Plot {
    character: char,
    area: i32,
    borders: HashSet<(Position, Direction)>,
}
impl Plot {
    fn fresh(x: i32, y: i32, farm: &Vec<Vec<char>>) -> Plot {
        let character = farm[y as usize][x as usize];

        Plot {
            character: character,
            area: 0,
            borders: HashSet::new(),
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

    // If the next space is not a member of the same plot, mark this as a plot border and return
    // These borders will be important later for computing this plot's number of sides
    if !check_array_bounds(next_position.x, next_position.y, farm)
        || farm[next_position.y as usize][next_position.x as usize] != plot.character
    {
        plot.borders.insert((position, next_direction));
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
    // If this space has already been visited, do nothing and return.
    match plot_spaces_visited.get(&position) {
        Some(_) => return plot,
        None => (),
    }

    // This is a new plot space - increment area and mark this space as visited
    plot.area += 1;
    plot_spaces_visited.insert(position);

    // Check each of the surrounding plot spaces, except for the one that we came from
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

// Walk through the farm and compute price of each unique plot
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

            // Use the recorded borders to compute the number of sides in the plot
            let sides = get_sides_from_borders(&mut plot.borders);
            price += plot.area * sides;
        }
    }

    price
}

// Recursively remove all borders that are part of the same side
fn remove_connecting_borders(
    border: (Position, Direction),
    borders: &mut HashSet<(Position, Direction)>,
) {
    // First, remove this border
    borders.remove(&border);

    // If this is a HORIZONTAL border, traverse VERTICALLY.
    // If this is a VERTICAL border, traverse HORIZONTALLY.
    // Left vs Right and Up vs Down distinction does not matter, since both will be checked in either case.
    let border_traversal_direction = match border.1 {
        Direction::Up | Direction::Down => Direction::Right, // Vertical -> Horizontal (right)
        Direction::Left | Direction::Right => Direction::Up, // Horizontal -> Vertical (up)
        Direction::None => Direction::None,
    };

    // Get the border on one side
    let next_border = (
        get_next_position(border.0, border_traversal_direction),
        border.1,
    );
    // Get the border on the other side
    let previous_border = (
        get_next_position(border.0, border_traversal_direction.get_reverse()),
        border.1,
    );

    // Remove borders on one side
    match borders.get(&next_border) {
        Some(_) => remove_connecting_borders(next_border, borders),
        None => (),
    }
    // Remove borders on the other side
    match borders.get(&previous_border) {
        Some(_) => remove_connecting_borders(previous_border, borders),
        None => (),
    }
}

// This function picks an arbitrary border from the list of borders.
// Then, it removes every border that shares a side with it from the list.
// One full side has been removed, and the sides counter is incremented.
// It repeats this until there are no borders left and every side has been counted.
// Returns the number of sides counted.
fn get_sides_from_borders(borders: &mut HashSet<(Position, Direction)>) -> i32 {
    let mut sides = 0;
    loop {
        // first, find a random border
        let border = borders.iter().next();
        match border {
            Some(border) => {
                // Remove all connecting borders that share a side with this one
                remove_connecting_borders(*border, borders);
                sides += 1;
            }
            // If no borders are found, all of the sides have been counted
            None => break,
        }
    }

    sides
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
    fn get_plot_borders() {
        let farm = read_file_to_array(_TEST_INPUT_2);

        let mut plot_spaces_visited: HashSet<Position> = HashSet::new();
        let mut plot = Plot::fresh(0, 0, &farm);

        plot = traverse_plot(
            &farm,
            Position { x: 0, y: 0 },
            Direction::None,
            plot,
            &mut plot_spaces_visited,
        );

        for border in &plot.borders {
            eprintln!("Border: {}, {}", border.0, border.1);
        }

        let sides = get_sides_from_borders(&mut plot.borders);
        eprintln!("Sides: {}", sides);

        assert_eq!(sides, 12);
    }

    #[test]
    fn test_remove_sides() {
        let farm = read_file_to_array(_TEST_INPUT_2);

        let mut plot_spaces_visited: HashSet<Position> = HashSet::new();
        let mut plot = Plot::fresh(0, 0, &farm);

        plot = traverse_plot(
            &farm,
            Position { x: 0, y: 0 },
            Direction::None,
            plot,
            &mut plot_spaces_visited,
        );

        for border in &plot.borders {
            eprintln!("Border: {}, {}", border.0, border.1);
        }

        let sides = get_sides_from_borders(&mut plot.borders);
        eprintln!("Sides: {}", sides);

        assert_eq!(sides, 12);
    }

    #[test]
    fn pass_website_example() {
        let farm = read_file_to_array(_TEST_INPUT);

        assert_eq!(compute_fence_price_for_farm(&farm), 1206);
    }
}