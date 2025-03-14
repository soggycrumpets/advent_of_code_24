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
    input: &Vec<char>,
    progress: usize,
    keypad: Keypad,
    sequence: &mut String,
    sequences: &mut Vec<String>,
) {
    if progress >= input.len() {
        // One shortest sequence has been found
        sequences.push((*sequence).clone());
        return;
    }

    let target = button_to_position(input[progress], keypad);
    if position == target {
        // Robot is hovering over the target button - begin on the next button
        sequence.push('A');
        navigate_keypad(position, input, progress + 1, keypad, sequence, sequences);
        sequence.pop();
        return;
    }

    let mut next_position: Position;

    // Try moving in directions that bring the robot immediately closer to the target button.
    next_position = position.east(1);
    if position.x < target.x && position_to_button(next_position, keypad) != '\0' {
        sequence.push('A');
        navigate_keypad(next_position, input, progress, keypad, sequence, sequences);
        sequence.pop();
    }

    next_position = position.west(1);
    if position.x > target.x && position_to_button(next_position, keypad) != '\0' {
        sequence.push('<');
        navigate_keypad(next_position, input, progress, keypad, sequence, sequences);
        sequence.pop();
    }

    next_position = position.south(1);
    if position.y < target.y && position_to_button(next_position, keypad) != '\0' {
        sequence.push('v');
        navigate_keypad(next_position, input, progress, keypad, sequence, sequences);
        sequence.pop();
    }

    next_position = position.north(1);
    if position.y > target.y && position_to_button(next_position, keypad) != '\0' {
        sequence.push('^');
        navigate_keypad(next_position, input, progress, keypad, sequence, sequences);
        sequence.pop();
    }
}

fn compute_complexity(numpad_codes: &Vec<String>, final_lengths: &Vec<usize>) -> usize {

    let mut complexity = 0;
    let mut lengths_iter = final_lengths.iter();
    for code in numpad_codes {
        let mut filtered_code = code.clone();
        filtered_code.retain(|c| c.is_numeric());

        let length = lengths_iter.next().unwrap();

        complexity += (filtered_code.parse::<usize>().unwrap()) * length;
    }

    complexity
}

fn get_shortest_sequences(input: &Vec<char>, keypad: Keypad) -> Vec<String> {
    let position = button_to_position('A', keypad);
    let mut sequence = String::new();
    let mut sequences: Vec<String> = Vec::new();
    navigate_keypad(position, &input, 0, keypad, &mut sequence, &mut sequences);

    sequences
}

fn get_shortest_sequence_len(input: &Vec<char>) -> usize {
    // Find the set of shortest sequences for the first robot
    let sequences1 = get_shortest_sequences(&input, Keypad::Numerical);

    // Find the set of shortest sequences for the second robot to produce all of the first robot's sequences
    let mut sequences2: Vec<String> = Vec::new();
    for sequence in &sequences1 {
        let input: Vec<char> = sequence.chars().collect();
        sequences2.append(&mut get_shortest_sequences(&input, Keypad::Directional));
    }

    // Find the set of shortest sequences for the third robot to produce all of the second robot's sequences
    let mut sequences3: Vec<String> = Vec::new();
    for sequence in &sequences2 {
        let input: Vec<char> = sequence.chars().collect();
        sequences3.append(&mut get_shortest_sequences(&input, Keypad::Directional));
    }

    // Find the shortest length of all the sequences for the third set
    let mut min_sequence_len: Option<usize> = None;
    for sequence in &sequences3 {
        if let Some(min_len) = min_sequence_len {
            min_sequence_len = Some(min_len.min(sequence.len()));
        } else {
            min_sequence_len = Some(sequence.len());
        }
    }

    min_sequence_len.unwrap()
}

fn main() {
    let numpad_sequences = load_button_sequences(_INPUT);

   // Compute the lengths of all the shortest button press sequences
    let mut lengths: Vec<usize> = Vec::new();
    for sequence in &numpad_sequences {
        let input: Vec<char> = sequence.chars().collect();
        lengths.push(get_shortest_sequence_len(&input));
    }

    // Get the complexity
    let complexity = compute_complexity(&numpad_sequences, &lengths);
    println!("Complexity: {}", complexity);
}

#[test]
fn test_example() {
    let numpad_sequences = load_button_sequences(_EXAMPLE2);

    // Compute the lengths of all the shortest button press sequences
    let mut lengths: Vec<usize> = Vec::new();
    for sequence in &numpad_sequences {
        let input: Vec<char> = sequence.chars().collect();
        lengths.push(get_shortest_sequence_len(&input));
    }

    assert_eq!(lengths[0], 68);
    assert_eq!(lengths[1], 60);
    assert_eq!(lengths[2], 68);
    assert_eq!(lengths[3], 64);
    assert_eq!(lengths[4], 64);

    // Get the complexity
    let complexity = compute_complexity(&numpad_sequences, &lengths);
    assert_eq!(complexity, 126384);
}