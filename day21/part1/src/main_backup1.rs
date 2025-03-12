use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input.txt";
const _EXAMPLE: &str = "example.txt";
const _EXAMPLE2: &str = "example2.txt";

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x: x, y: y }
    }
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

fn load_button_sequences(name: &str) -> Vec<String> {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();
    let mut button_sequences: Vec<String> = Vec::new();
    let data_lines = data_string.lines();
    for line in data_lines {
        button_sequences.push(line.to_string());
    }

    button_sequences
}

fn numerical_keypad(button: char) -> Position {
    match button {
        'A' => Position { x: 2, y: 3 },
        '0' => Position { x: 1, y: 3 },
        '1' => Position { x: 0, y: 2 },
        '2' => Position { x: 1, y: 2 },
        '3' => Position { x: 2, y: 2 },
        '4' => Position { x: 0, y: 1 },
        '5' => Position { x: 1, y: 1 },
        '6' => Position { x: 2, y: 1 },
        '7' => Position { x: 0, y: 0 },
        '8' => Position { x: 1, y: 0 },
        '9' => Position { x: 2, y: 0 },
        'X' => Position { x: 0, y: 3 },
        _ => panic!("Invalid button!"),
    }
}

#[derive(Clone, Copy)]
enum KeypadType {
    Numerical,
    Directional,
}

fn navigate_keypad(
    button: char,
    position: Position,
    keypad_type: KeypadType,
    last_direction_moved: char,
) -> (String, Position) {
    let mut current_position = position;
    let mut button_presses = String::new();

    let button_position: Position;
    let empty_space: Position;
    match keypad_type {
        KeypadType::Numerical => {
            button_position = numerical_keypad(button);
            empty_space = numerical_keypad('X');
        }
        KeypadType::Directional => {
            button_position = directional_keypad(button);
            empty_space = directional_keypad('X');
        }
    }
    // If we moved in a certain direction to get to the previous button, we should prioritize moving in that direction first.
    match last_direction_moved {
        '>' => move_robot_arm(&mut current_position, target_position, &mut button_presses, direction)
    }
    

    if empty_space.x != current_position.x {
        match_vertical_position(&mut current_position, button_position, &mut button_presses);
        assert_ne!(current_position, empty_space);
        match_horizontal_position(&mut current_position, button_position, &mut button_presses);
        assert_ne!(current_position, empty_space);
    } else {
        match_horizontal_position(&mut current_position, button_position, &mut button_presses);
        assert_ne!(current_position, empty_space);
        match_vertical_position(&mut current_position, button_position, &mut button_presses);
        assert_ne!(current_position, empty_space);
    }
    button_presses.push('A');

    (button_presses, current_position)
}

fn directional_keypad(button: char) -> Position {
    match button {
        'A' => Position { x: 2, y: 0 },
        '<' => Position { x: 0, y: 1 },
        'v' => Position { x: 1, y: 1 },
        '>' => Position { x: 2, y: 1 },
        '^' => Position { x: 1, y: 0 },
        'X' => Position { x: 0, y: 0 },
        _ => panic!("Invalid button!"),
    }
}

fn get_sequence_from_keypad(input_sequence: &str, keypad_type: KeypadType) -> String {
    let mut position: Position;
    match keypad_type {
        KeypadType::Numerical => position = numerical_keypad('A'),
        KeypadType::Directional => position = directional_keypad('A'),
    }

    let mut output_sequence: String = String::new();
    for button in input_sequence.chars() {
        let (directions, new_position) = navigate_keypad(button, position, keypad_type);
        output_sequence.push_str(&directions);
        position = new_position;
    }

    output_sequence
}

fn move_robot_arm(
    current_position: &mut Position,
    target_position: Position,
    button_presses: &mut String,
    direction: char,
) {
    match direction {
        '>' => {
            while current_position.x < target_position.x {
                *current_position = current_position.east(1);
                button_presses.push('>')
            }
        }
        '<' => {
            while current_position.x > target_position.x {
                *current_position = current_position.west(1);
                button_presses.push('<')
            } 
        }
        'v' => {
            while current_position.y < target_position.y {
                *current_position = current_position.south(1);
                button_presses.push('v')
            } 
        }
        '^' => {
            while current_position.y > target_position.y {
                *current_position = current_position.north(1);
                button_presses.push('^')
            } 
        }
        _ => panic!("Cannot move robot arm in direction: \"{}\"", direction),
    }
}

fn match_horizontal_position(
    current_position: &mut Position,
    target_position: Position,
    button_presses: &mut String,
) {
    while current_position.x != target_position.x {
        if current_position.x < target_position.x {
            *current_position = current_position.east(1);
            button_presses.push('>')
        } else {
            *current_position = current_position.west(1);
            button_presses.push('<')
        }
    }
}

fn match_vertical_position(
    current_position: &mut Position,
    target_position: Position,
    button_presses: &mut String,
) {
    while current_position.y != target_position.y {
        if current_position.y < target_position.y {
            *current_position = current_position.south(1);
            button_presses.push('v')
        } else {
            *current_position = current_position.north(1);
            button_presses.push('^')
        }
    }
}

fn compute_complexity(numpad_sequences: Vec<String>, final_sequences: Vec<String>) -> i32 {
    let mut complexity = 0;
    let mut numpad_codes = numpad_sequences.into_iter();
    for sequence in &final_sequences {
        let mut code = numpad_codes.next().unwrap();
        code.retain(|c| c.is_numeric());
        complexity += (code.parse::<usize>().unwrap()) * sequence.len();
    }
    complexity as i32
}

fn main() {
    let numpad_sequences = load_button_sequences(_INPUT);
    let mut final_sequences: Vec<String> = Vec::new();
    for sequence_1 in &numpad_sequences {
        let sequence_2 = get_sequence_from_keypad(&sequence_1, KeypadType::Numerical);
        let sequence_3 = get_sequence_from_keypad(&sequence_2, KeypadType::Directional);
        let sequence_4 = get_sequence_from_keypad(&sequence_3, KeypadType::Directional);
        println!("{}: {}", sequence_4.len(), sequence_4);
        final_sequences.push(sequence_4);
    }

    // Get the complexity
    let complexity = compute_complexity(numpad_sequences, final_sequences);
    println!("Total complexity: {}", complexity)
}

#[test]
fn test_example() {
    let numpad_sequences = load_button_sequences(_EXAMPLE);

    // Get the sequence to control the first robot
    let sequence_1 = &numpad_sequences[0];
    let sequence_2 = get_sequence_from_keypad(sequence_1, KeypadType::Numerical);

    assert!(
        sequence_2 == "<A^A>^^AvvvA"
            || sequence_2 == "<A^A^>^AvvvA"
            || sequence_2 == "<A^A^^>AvvvA",
        "First robot failed to find the shortest sequence!"
    );

    // Get the sequence to control the second robot
    let sequence_3 = get_sequence_from_keypad(&sequence_2, KeypadType::Directional);
    assert_eq!(sequence_3.len(), "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());

    // Get the sequence to control the third robot
    let sequence_4 = get_sequence_from_keypad(&sequence_3, KeypadType::Directional);
    assert_eq!(
        sequence_4.len(),
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
    );
}

#[test]
fn test_compute_complexity() {
    let numpad_sequences = vec![
        "029A".to_string(),
        "980A".to_string(),
        "179A".to_string(),
        "456A".to_string(),
        "379A".to_string(),
    ];
    let final_sequences = vec![
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".to_string(),
        "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".to_string(),
        "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".to_string(),
        "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".to_string(),
        "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".to_string(),
    ];

    let complexity = compute_complexity(numpad_sequences, final_sequences);
    assert_eq!(complexity, 126384);
}

#[test]
fn test_example2() {
    let numpad_sequences = load_button_sequences(_EXAMPLE2);
    let mut final_sequences: Vec<String> = Vec::new();
    for sequence_1 in &numpad_sequences {
        let sequence_2 = get_sequence_from_keypad(&sequence_1, KeypadType::Numerical);
        let sequence_3 = get_sequence_from_keypad(&sequence_2, KeypadType::Directional);
        let sequence_4 = get_sequence_from_keypad(&sequence_3, KeypadType::Directional);
        println!("{}: {}", sequence_4.len(), sequence_4);
        final_sequences.push(sequence_4);
    }

    // Get the complexity
    let complexity = compute_complexity(numpad_sequences, final_sequences);
    assert_eq!(complexity, 126384);
}
