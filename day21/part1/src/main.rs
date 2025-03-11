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
        _ => panic!("Invalid button!"),
    }
}

fn navigate_numerical_keypad(button: char, position: Position) -> (String, Position) {
    let mut current_position = position;
    let button_position = numerical_keypad(button);
    let mut button_presses = String::new();

    // The buttons prioritized in a specific order as to prevent
    // going over the empty space
    while current_position != button_position {
        if current_position.x < button_position.x {
            current_position.x += 1;
            button_presses.push('>')
        } else if current_position.y > button_position.y {
            current_position.y -= 1;
            button_presses.push('^')
        } else if current_position.x > button_position.x {
            current_position.x -= 1;
            button_presses.push('<')
        } else {
            current_position.y += 1;
            button_presses.push('v');
        }
    }
    button_presses.push('A');

    (button_presses, current_position)
}

fn get_sequence_from_numerical_keypad(input_sequence: &str) -> String {
    let mut position = numerical_keypad('A');
    let mut output_sequence: String = String::new();
    for button in input_sequence.chars() {
        let (directions, new_position) = navigate_numerical_keypad(button, position);
        output_sequence.push_str(&directions);
        position = new_position;
    }
    output_sequence
}

fn directional_keypad(button: char) -> Position {
    match button {
        'A' => Position { x: 2, y: 0 },
        '<' => Position { x: 0, y: 1 },
        'v' => Position { x: 1, y: 1 },
        '>' => Position { x: 2, y: 1 },
        '^' => Position { x: 1, y: 0 },
        _ => panic!("Invalid button!"),
    }
}
// This is a separate function because different buttons must be prioritized to avoid the empty space.
// Robot should adjust horizontally and vertically separately, since hitting consecutive directions
//      will be faster for the robot that is controlling it.
fn navigate_directional_keypad(button: char, position: Position) -> (String, Position) {
    let mut current_position = position;
    let button_position = directional_keypad(button);
    let mut button_presses = String::new();

    while current_position.x != button_position.x {
        if current_position.y < button_position.y {
            current_position.y += 1;
            button_presses.push('v')
        } else if  current_position.x < button_position.x {
            current_position.x += 1;
            button_presses.push('>')
        } else if current_position.x > button_position.x {
            current_position.x -= 1;
            button_presses.push('<')
        } else {
            current_position.y -= 1;
            button_presses.push('^');
        }
    }
    button_presses.push('A');

    (button_presses, current_position)
}

fn get_sequence_from_directional_keypad(input_sequence: &str) -> String {
    let mut position = directional_keypad('A');
    let mut output_sequence: String = String::new();
    for button in input_sequence.chars() {
        let (directions, new_position) = navigate_directional_keypad(button, position);
        output_sequence.push_str(&directions);
        position = new_position;
    }

    output_sequence
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
        let sequence_2 = get_sequence_from_numerical_keypad(&sequence_1);
        let sequence_3 = get_sequence_from_directional_keypad(&sequence_2);
        let sequence_4 = get_sequence_from_directional_keypad(&sequence_3);
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
    let sequence_2 = get_sequence_from_numerical_keypad(sequence_1);

    assert!(
        sequence_2 == "<A^A>^^AvvvA"
            || sequence_2 == "<A^A^>^AvvvA"
            || sequence_2 == "<A^A^^>AvvvA",
        "First robot failed to find the shortest sequence!"
    );

    // Get the sequence to control the second robot
    let sequence_3 = get_sequence_from_directional_keypad(&sequence_2);
    assert_eq!(sequence_3.len(), "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());

    // Get the sequence to control the third robot
    let sequence_4 = get_sequence_from_directional_keypad(&sequence_3);
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
        let sequence_2 = get_sequence_from_numerical_keypad(&sequence_1);
        let sequence_3 = get_sequence_from_directional_keypad(&sequence_2);
        let sequence_4 = get_sequence_from_directional_keypad(&sequence_3);
        println!("{}: {}", sequence_4.len(), sequence_4);
        final_sequences.push(sequence_4);
    }

    // Get the complexity
    let complexity = compute_complexity(numpad_sequences, final_sequences);
    assert_eq!(complexity, 126384);
}