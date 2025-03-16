use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;

const _INPUT: &str = "input.txt";
const _EXAMPLE: &str = "example.txt";

#[derive(PartialEq, Debug, Eq)]
struct Gate {
    name: String,
    components: Option<[Rc<RefCell<Gate>>; 2]>,
    operation: Option<Operation>,
    value: Option<bool>,
}
impl Gate {
    fn new(name: String) -> Gate {
        Gate {
            name: name.to_string(),
            components: None,
            operation: None,
            value: None,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
enum Operation {
    AND,
    OR,
    XOR,
}

fn evaluate(gate: &Rc<RefCell<Gate>>) -> bool {

        // Gate has already been evaluated
        if let Some(bool) = gate.borrow().value {
            return bool
        }

        let binding = gate.borrow();
        let gates = binding.components.as_ref().unwrap();

        // Recursively evaluate all prerequisite gates
        println!("{}", gate.borrow().name);
    
        if let None = &gate.borrow().operation {
            panic!("Tried to evaluate a gate with no operation!");
        }

        match gate.borrow().operation.unwrap() {
            Operation::AND => {
                    return evaluate(&gates[0]) & evaluate(&gates[1]);
            }
            Operation::OR => {
                    return evaluate(&gates[0]) | evaluate(&gates[1]);
            }
            Operation::XOR => {
                    return evaluate(&gates[0]) ^ evaluate(&gates[1]);
            }
        }
    }

fn load_gates(name: &str) -> HashMap<String, Rc<RefCell<Gate>>> {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut data_lines = data_string.lines();

    let mut gates: HashMap<String, Rc<RefCell<Gate>>> = HashMap::new();

    // Initialize the set of gates with predetermined values
    loop {
        let line = data_lines.next().unwrap();

        if line.trim().is_empty() {
            break;
        }

        let mut string_iter = line.split(':');
        let name: &str = string_iter.next().unwrap();
        let value: bool = string_iter.next().unwrap().trim().parse::<i32>().unwrap() > 0;

        let mut gate = Gate::new(name.to_string());
        gate.value = Some(value);
        gates.insert(name.to_string(), Rc::new(RefCell::new(gate)));
    }

    // Declare the rest of the gates
    for line in data_lines.clone() {
        let name = line.split_whitespace().last().unwrap();
        let gate = Gate::new(name.to_string());
        gates.insert(name.to_string(), Rc::new(RefCell::new(gate)));
    }

    // Initialize the rest of the gates
    for line in data_lines {
        let strings: Vec<&str> = line.split_ascii_whitespace().collect();
        let component1 = gates.get(strings[0]).unwrap().clone();
        let component2 = gates.get(strings[2]).unwrap().clone();

        let operation_string = strings[1];
        let operation: Operation = match operation_string {
            "AND" => Operation::AND,
            "OR" => Operation::OR,
            "XOR" => Operation::XOR,
            _ => panic!("Tried to parse invalid gate: {}", operation_string),
        };

        print!("Strings[4]: {}", strings[4]);
        let mut gate = gates.get(strings[4]).unwrap().borrow_mut();
        println!("| {}", strings[4]);
        gate.components = Some([component1, component2]);
        gate.operation = Some(operation);
    }

    gates
}

fn main() {
    let gates = load_gates(_EXAMPLE);
    for (name, gate) in gates {
        if !name.starts_with('z') {
            continue
        }
        println!("{}", evaluate(&gate));
    }
}
