use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::hash::Hash;
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

#[derive(Clone, Copy, PartialEq)]
enum Keypad {
    Numerical,
    Directional,
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

fn get_sequence_from_keypad(input_sequence: &str, keypad: Keypad) -> String {
    let mut position = button_to_position('A', keypad);

    let mut output_sequence: String = String::new();

    // Each iteration of the loop is one button of the input sequence being pressed
    for button in input_sequence.chars() {
        let (directions, new_position) = navigate_keypad(button, position, keypad);
        output_sequence.push_str(&directions);
        position = new_position;
    }

    output_sequence
}

fn navigate_keypad(button: char, position: Position, keypad: Keypad) -> (String, Position) {
    let mut current_position = position;
    let mut button_presses = String::new();

    let target_position = button_to_position(button, keypad);

    while current_position != target_position {
        move_until_matching(
            &mut current_position,
            target_position,
            &mut button_presses,
            keypad,
            '<',
        );
        move_until_matching(
            &mut current_position,
            target_position,
            &mut button_presses,
            keypad,
            'v',
        );
        move_until_matching(
            &mut current_position,
            target_position,
            &mut button_presses,
            keypad,
            '^',
        );
        move_until_matching(
            &mut current_position,
            target_position,
            &mut button_presses,
            keypad,
            '>',
        );
    }
    button_presses.push('A');

    (button_presses, current_position)
}

fn move_until_matching(
    current_position: &mut Position,
    target_position: Position,
    button_presses: &mut String,
    keypad: Keypad,
    direction: char,
) {
    let mut next_position = current_position.clone();
    let mut next_button_presses = String::new();
    match direction {
        '>' => {
            while next_position.x < target_position.x {
                next_position = next_position.east(1);
                next_button_presses.push('>')
            }
        }
        '<' => {
            while next_position.x > target_position.x {
                next_position = next_position.west(1);
                next_button_presses.push('<')
            }
        }
        'v' => {
            while next_position.y < target_position.y {
                next_position = next_position.south(1);
                next_button_presses.push('v')
            }
        }
        '^' => {
            while next_position.y > target_position.y {
                next_position = next_position.north(1);
                next_button_presses.push('^')
            }
        }
        _ => panic!("Cannot move robot arm in direction: \"{}\"", direction),
    }

    // This movement is only made if the end position is not over the empty space
    if position_to_button(next_position, keypad) != '\0' {
        *current_position = next_position;
        button_presses.push_str(&next_button_presses);
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

// A transition is a string of two characters describing the transition from one instruction to another
fn define_transitions() -> Vec<String> {
    let mut transitions: Vec<String> = Vec::new();

    let buttons = vec!['>', 'v', '<', '^', 'A'];
    for button1 in &buttons {
        for button2 in &buttons {
            let mut transition = String::new();
            transition.push(*button1);
            transition.push(*button2);
            transitions.push(transition);
        }
    }
    transitions
}

// A translation is the set of inputs requried to get the best path to get from one instruction to another
fn define_translations(transitions: &Vec<String>) -> HashMap<String, String> {
    let mut translations: HashMap<String, String> = HashMap::new();
    for transition in transitions {
        let instructions: Vec<char> = transition.chars().collect();
        let (mut translation, _) = navigate_keypad(
            instructions[1],
            button_to_position(instructions[0], Keypad::Directional),
            Keypad::Directional,
        );
        translation.pop();
        translations.insert(transition.clone(), translation);
    }
    translations
}

fn extract_transitions_from_sequence(sequence: &str) -> HashMap<String, u64> {
    let chars: Vec<char> = sequence.chars().collect();
    let mut transitions: HashMap<String, u64> = HashMap::new();

    // The first transition is a special case; it's always A -> first button press
    let first_transition: String = vec!['A', chars[0]].into_iter().collect();
    transitions.insert(first_transition.clone(), 1);

    for i in 0..chars.len() - 1 {
        let mut transition = String::new();
        transition.push(chars[i]);
        transition.push(chars[i + 1]);
        if let Some(occurances) = transitions.get_mut(&transition) {
            *occurances += 1;
        } else {
            transitions.insert(transition, 1);
        }
    }

    if first_transition.is_empty() {
        panic!("Failed to find a sequence's first transition!")
    }
    transitions
}

fn compute_next_instruction_set(
    transitions: &HashMap<String, u64>,
    translations: &HashMap<String, String>,
) -> HashMap<String, u64> {
    let mut next_transitions: HashMap<String, u64> = HashMap::new();

    for (transition, num_transitions) in transitions {
        let mut translation_sequence = translations.get(transition).unwrap().clone();
        translation_sequence = vec!["A", translation_sequence.as_str(), "A"]
            .into_iter()
            .collect();

        for i in 0..translation_sequence.len() - 1 {
            let mut transition = String::new();
            let translation: Vec<char> = translation_sequence.chars().collect();
            transition.push(translation[i]);
            transition.push(translation[i + 1]);

            if let Some(occurances) = next_transitions.get_mut(&transition) {
                *occurances += num_transitions;
            } else {
                next_transitions.insert(transition, *num_transitions);
            }
        }
    }

    next_transitions
}

fn compute_length_from_transitions(transitions: &HashMap<String, u64>) -> u64 {
    let mut len: u64 = 0;
    for (_transition, occurances) in transitions {
        len += occurances;
    }
    len
}

fn main() {
    let transitions = define_transitions();
    let translations = define_translations(&transitions);
    let sequence = "<^<^";

    let mut transitions = extract_transitions_from_sequence(sequence);
    for i in 0..25 {
        transitions = compute_next_instruction_set(&transitions, &translations);
        println!("Number of instructions: {}", compute_length_from_transitions(&transitions));
    }

    let mut len = 0;
    println!("{}", len);
    len = 0;
    
    /* -------------------------------------------------------------------------------- */

    // let numpad_sequences = load_button_sequences(_INPUT);

    //     let mut final_sequences: Vec<String> = Vec::new();
    //     for sequence_1 in &numpad_sequences {
    //         let numpad_sequence = get_sequence_from_keypad(&sequence_1, Keypad::Numerical);
    //         let numpad_sequence = "<^<^".to_string();
    //         let mut dirpad_sequences: Vec<String> = Vec::new();

    //         let mut next_sequence = numpad_sequence.clone();
    //         println!("{}", next_sequence);
    //         println!("{}", next_sequence.len());
    //         for _i in 0..20 {
    //             next_sequence = (get_sequence_from_keypad(&next_sequence, Keypad::Directional));
    //             dirpad_sequences.push(next_sequence.clone());

    //             // println!("{}", next_sequence);
    //             println!("{}", next_sequence.len());
    //         }
    //         println!();
    //         final_sequences.push(dirpad_sequences.last().unwrap().clone());
    //     }

    //     // Get the complexity
    //     let complexity = compute_complexity(numpad_sequences, final_sequences);
    //     println!("Total complexity: {}", complexity)
}

#[test]
fn test_example() {
    let numpad_sequences = load_button_sequences(_EXAMPLE);

    // Get the sequence to control the first robot
    let sequence_1 = &numpad_sequences[0];
    let sequence_2 = get_sequence_from_keypad(sequence_1, Keypad::Numerical);

    assert!(
        sequence_2 == "<A^A>^^AvvvA"
            || sequence_2 == "<A^A^>^AvvvA"
            || sequence_2 == "<A^A^^>AvvvA",
        "First robot failed to find the shortest sequence!"
    );

    // Get the sequence to control the second robot
    let sequence_3 = get_sequence_from_keypad(&sequence_2, Keypad::Directional);
    assert_eq!(sequence_3.len(), "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());

    // Get the sequence to control the third robot
    let sequence_4 = get_sequence_from_keypad(&sequence_3, Keypad::Directional);
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
        let sequence_2 = get_sequence_from_keypad(&sequence_1, Keypad::Numerical);
        let sequence_3 = get_sequence_from_keypad(&sequence_2, Keypad::Directional);
        let sequence_4 = get_sequence_from_keypad(&sequence_3, Keypad::Directional);
        println!("{}: {}", sequence_4.len(), sequence_4);
        final_sequences.push(sequence_4);
    }

    // Get the complexity
    let complexity = compute_complexity(numpad_sequences, final_sequences);
    assert_eq!(complexity, 126384);
}

#[test]
fn test_part1() {
    let numpad_sequences = load_button_sequences(_INPUT);
    let mut final_sequences: Vec<String> = Vec::new();
    for sequence_1 in &numpad_sequences {
        let sequence_2 = get_sequence_from_keypad(&sequence_1, Keypad::Numerical);
        let sequence_3 = get_sequence_from_keypad(&sequence_2, Keypad::Directional);
        let sequence_4 = get_sequence_from_keypad(&sequence_3, Keypad::Directional);
        println!("{}: {}", sequence_4.len(), sequence_4);
        final_sequences.push(sequence_4);
    }

    // Get the complexity
    let complexity = compute_complexity(numpad_sequences, final_sequences);
    assert_eq!(complexity, 157892);
}
