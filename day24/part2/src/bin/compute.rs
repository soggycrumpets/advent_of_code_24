use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;

const _INPUT: &str = "input.txt";
const _SWAPPED: &str = "swapped.txt";
const _EXAMPLE: &str = "example.txt";

#[derive(PartialEq, Debug, Eq)]
struct Gate {
    name: String,
    components: Option<[Rc<RefCell<Gate>>; 2]>,
    operation: Option<String>,
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

struct Device {
    gates: HashMap<String, Rc<RefCell<Gate>>>,
    num1: u64,
    num2: u64,
}

fn evaluate(gate: &Rc<RefCell<Gate>>) -> bool {
    // Gate has already been evaluated
    if let Some(bool) = gate.borrow().value {
        return bool;
    }

    let gates= gate.borrow();
    let gates = gates.components.as_ref().unwrap();

    if let None = &gate.borrow().operation {
        panic!("Tried to evaluate a gate with no operation!");
    }

    // Recursively evaluate all prerequisite gates
    match gate.borrow().operation.as_ref().unwrap().as_str() {
        "AND" => {
            return evaluate(&gates[0]) & evaluate(&gates[1]);
        }
        "OR" => {
            return evaluate(&gates[0]) | evaluate(&gates[1]);
        }
        "XOR" => {
            return evaluate(&gates[0]) ^ evaluate(&gates[1]);
        }
        _ => panic!("Invalid operation"),
    }
}

fn load_gates(name: &str) -> Device {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut data_lines = data_string.lines();

    let mut gates: HashMap<String, Rc<RefCell<Gate>>> = HashMap::new();
    let mut num1 = String::new();
    let mut num2 = String::new();

    // Initialize the set of gates with predetermined values
    loop {
        let line = data_lines.next().unwrap();

        if line.trim().is_empty() {
            break;
        }

        let mut string_iter = line.split(':');
        let name: &str = string_iter.next().unwrap();
        let value: bool = string_iter.next().unwrap().trim().parse::<i32>().unwrap() > 0;

        if name.starts_with('x') {
            num1.push(u64::from(value).to_string().chars().next().unwrap());
        } else if name.starts_with('y') {
            num2.push(u64::from(value).to_string().chars().next().unwrap());
        }

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

        let operation= strings[1];

        let mut gate = gates.get(strings[4]).unwrap().borrow_mut();
        gate.components = Some([component1, component2]);
        gate.operation = Some(operation.to_string());
    }

    num1 = num1.chars().rev().collect();
    num2 = num2.chars().rev().collect();
    Device {
        gates: gates,
        num1: u64::from_str_radix(num1.as_str(), 2).unwrap(),
        num2: u64::from_str_radix(num2.as_str(), 2).unwrap(),
    }
}

fn evaluate_gates(gates: &HashMap<String, Rc<RefCell<Gate>>>) -> u64 {
    let mut indeces: Vec<usize> = Vec::new();
    let mut scrambled_values: Vec<bool> = Vec::new();
    for (name, gate) in gates {
        if !name.starts_with('z') {
            continue;
        }
        let mut index: String = name.clone();
        index.retain(|c| c.is_numeric());
        indeces.push(index.parse().unwrap());
        scrambled_values.push(evaluate(&gate));
    }

    let mut values: Vec<bool> = vec![false; indeces.len()];
    for i in 0..values.len() {
        values[indeces[i]] = scrambled_values[i];
    }
    values.reverse();

    let mut values_string: String = String::new();
    for value in values {
        if value {
            values_string.push('1')
        } else {
            values_string.push('0');
        }
    }

    u64::from_str_radix(values_string.as_str(), 2).unwrap()
}

fn get_base_components(gate: &Rc<RefCell<Gate>>) -> Vec<String> {
    let mut components = vec![String::new()];

    if let None = gate.borrow().components {
        components.push(gate.borrow().name.clone());
        return components;
    }

    let gates= gate.borrow();
    let gates = gates.components.as_ref().unwrap();
    components.append(&mut get_base_components(&gates[0]));
    components.append(&mut vec![gate.borrow().operation.as_ref().unwrap().clone()]);
    components.append(&mut get_base_components(&gates[1]));

    components
}

fn get_intermediate_components(gate: &Rc<RefCell<Gate>>) -> Vec<String> {
    let mut components = vec![String::new()];

    if let None = gate.borrow().components {
        return components;
    }

    components.push(gate.borrow().name.clone());

    let gates= gate.borrow();
    let gates = gates.components.as_ref().unwrap();
    components.append(&mut get_intermediate_components(&gates[0]));
    components.append(&mut get_intermediate_components(&gates[1]));

    components
}

fn main() {
    let device = load_gates(_SWAPPED);
    let gates = device.gates;
    let result = evaluate_gates(&gates);
    let expected_result = device.num1 + device.num2;
    println!("{:046b} | Result", result);
    println!("{:046b} | Expected", expected_result);
    println!("{:046b} | Diff", result ^ expected_result);

//     let z_base= get_base_components(gates.get("z04").unwrap());
//     for comp in z_base {
//         print!("{} ", comp)        
//     }
//     println!();

// let z_int = get_intermediate_components(gates.get("z04").unwrap());
//     for comp in z_int {
//         print!("{} ", comp)        
//     }
//     println!();
}

#[test]
fn test_example() {
    let device = load_gates(_EXAMPLE);
    let gates = device.gates;

    assert_eq!(evaluate_gates(&gates), 2024);
}
