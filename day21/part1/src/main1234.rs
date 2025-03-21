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

fn button_to_position(button: char, keypad_type: Keypad) -> Position {
    match keypad_type {
        Keypad::Numerical => match button {
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
        },
        Keypad::Directional => match button {
            'A' => Position { x: 2, y: 0 },
            '<' => Position { x: 0, y: 1 },
            'v' => Position { x: 1, y: 1 },
            '>' => Position { x: 2, y: 1 },
            '^' => Position { x: 1, y: 0 },
            _ => panic!("Invalid button!"),
        },
    }
}

fn position_to_button(position: Position, keypad_type: Keypad) -> char {
    match keypad_type {
        Keypad::Numerical => match position {
            Position { x: 2, y: 3 } => 'A',
            Position { x: 1, y: 3 } => '0',
            Position { x: 0, y: 2 } => '1',
            Position { x: 1, y: 2 } => '2',
            Position { x: 2, y: 2 } => '3',
            Position { x: 0, y: 1 } => '4',
            Position { x: 1, y: 1 } => '5',
            Position { x: 2, y: 1 } => '6',
            Position { x: 0, y: 0 } => '7',
            Position { x: 1, y: 0 } => '8',
            Position { x: 2, y: 0 } => '9',
            _ => '\0',
        },
        Keypad::Directional => match position {
            Position { x: 2, y: 0 } => 'A',
            Position { x: 0, y: 1 } => '<',
            Position { x: 1, y: 1 } => 'v',
            Position { x: 2, y: 1 } => '>',
            Position { x: 1, y: 0 } => '^',
            _ => '\0',
        },
    }
}

#[derive(Clone, Copy)]
enum Keypad {
    Numerical,
    Directional,
}

fn navigate_keypad(
    position: Position,
    visited: &mut Vec<Position>,
    input: &Vec<char>,
    progress: usize,
    keypad: Keypad,
    sequence: &mut String,
    sequences: &mut Vec<String>,
) {

    visited.push(position);

    if progress >= input.len() {
        // One shortest sequence has been found
        sequences.push((*sequence).clone());

        sequence.pop();
        visited.pop();
        return;
    }

    let target = button_to_position(input[progress], keypad);
    if position == target {
        // Robot is hovering over the target button - begin on the next button
        sequence.push('A');
        navigate_keypad(position, visited, input, progress + 1, keypad, sequence, sequences);
    }

    let mut next_position: Position;

    // Try moving in directions that bring the robot immediately closer to the target button.
    next_position = position.east(1);
    if 
     position_to_button(next_position, keypad) != '\0' 
    && !visited.contains(&next_position) {

        sequence.push('>');
        navigate_keypad(next_position, visited, input, progress, keypad, sequence, sequences);
    }

    next_position = position.west(1);
    if 
     position_to_button(next_position, keypad) != '\0' 
    && !visited.contains(&next_position) {

        sequence.push('<');
        navigate_keypad(next_position, visited, input, progress, keypad, sequence, sequences);
    }

    next_position = position.south(1);
    if
     position_to_button(next_position, keypad) != '\0'
    && !visited.contains(&next_position) {
        sequence.push('v');
        navigate_keypad(next_position, visited, input, progress, keypad, sequence, sequences);
    }

    next_position = position.north(1);
    if 
     position_to_button(next_position, keypad) != '\0' 
    && !visited.contains(&next_position) {

        sequence.push('^');
        navigate_keypad(next_position, visited, input, progress, keypad, sequence, sequences);
    }

    
    sequence.pop();
    visited.pop();
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

fn main() {}

#[test]
fn test_example() {
    // let numpad_sequences = load_button_sequences(_EXAMPLE);

    // // Get the sequence to control the first robot
    // let sequence_1 = &numpad_sequences[0];
    // let sequence_2 = get_sequence_from_keypad(sequence_1, Keypad::Numerical);

    // assert!(
    //     sequence_2 == "<A^A>^^AvvvA"
    //         || sequence_2 == "<A^A^>^AvvvA"
    //         || sequence_2 == "<A^A^^>AvvvA",
    //     "First robot failed to find the shortest sequence!"
    // );

    // // Get the sequence to control the second robot
    // let sequence_3 = get_sequence_from_keypad(&sequence_2, Keypad::Directional);
    // assert_eq!(sequence_3.len(), "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());

    // // Get the sequence to control the third robot
    // let sequence_4 = get_sequence_from_keypad(&sequence_3, Keypad::Directional);
    // assert_eq!(
    //     sequence_4.len(),
    //     "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
    // );
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
fn test_example2() {}

#[test]
fn get_shortest_paths_for_first_robot() {
    let numpad_sequences = load_button_sequences(_EXAMPLE2);
    let input: Vec<char> = numpad_sequences[0].chars().collect();

    // First robot
    let position = button_to_position('A', Keypad::Numerical);
    let mut sequence = String::new();
    let mut sequences1: Vec<String> = Vec::new();
    let mut visited_positions: Vec<Position> = Vec::new();

    navigate_keypad(
        position,
        &mut visited_positions,
        &input,
        0,
        Keypad::Numerical,
        &mut sequence,
        &mut sequences1,
    );

    println!("\nShortest sequences for first robot:\n");
    for sequence in &sequences1 {
        println!("{}", sequence);
    }

    // // Second robot
    // let position = button_to_position('A', Keypad::Directional);
    // sequence.clear();
    // let mut sequences2: Vec<String> = Vec::new();
    // for sequence1 in &sequences1 {
    //     let input: Vec<char> = sequence1.chars().collect();
    //     navigate_keypad(
    //         position,
    //         &input,
    //         0,
    //         Keypad::Directional,
    //         &mut sequence,
    //         &mut sequences2,
    //     );
    // }

    // println!("\nShortest sequences for second robot:\n");
    // for sequence in &sequences2 {
    //     println!("{}", sequence);
    // }

    // // Third robot
    // let position = button_to_position('A', Keypad::Directional);
    // sequence.clear();
    // let mut sequences3: Vec<String> = Vec::new();
    // for sequence2 in &sequences2 {
    //     let input: Vec<char> = sequence2.chars().collect();
    //     navigate_keypad(
    //         position,
    //         &input,
    //         0,
    //         Keypad::Directional,
    //         &mut sequence,
    //         &mut sequences3,
    //     );
    // }

    // let mut shortest_sequence_sample: &str = "";

    // println!("\nToo many sequences for third robot to print all of them.\n");
    // let mut min_sequence_len: Option<usize> = None;
    // for sequence in &sequences3 {
    //     if let Some(min_len) = min_sequence_len {
    //         if sequence.len() > min_len {
    //             min_sequence_len = Some(sequence.len());
    //             shortest_sequence_sample = sequence;
    //             continue;
    //         }
    //     }
    //     min_sequence_len = Some(sequence.len());
    //     // println!("{}", sequence);
    // }

    // println!("\nShortest sequence length: {}", min_sequence_len.unwrap());
    // println!("Sample: {}", shortest_sequence_sample);

    // Get the complexity
    // let complexity = compute_complexity(numpad_sequences, final_sequences);
    // assert_eq!(complexity, 126384);
}
