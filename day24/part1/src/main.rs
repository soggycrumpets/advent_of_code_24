use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;

const _INPUT: &str = "input.txt";
const _EXAMPLE: &str = "example.txt";

#[derive(PartialEq, Debug, Eq, Hash)]
struct Gate<> {
    name: String,
    components: Option<[Rc<Gate>; 2]>,
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

    fn evaluate(&mut self) {
        // Gate has already been evaluated
        if let Some(_) = self.value {
            panic!("Cannot evaluate a gate that has already been evaluated!");
        }

        let gates = self.components.as_mut().unwrap();
        let (gate1, gate2) = gates.split_at_mut(0);

        let gate1 = Rc::get_mut(&mut gate1[0]).unwrap();
        let gate2 = Rc::get_mut(&mut gate2[0]).unwrap();

        // Recursively evaluate all prerequisite gates
        if let None = gate1.value {
            gate1.evaluate();
        }
        if let None = gate2.value {
            gate2.evaluate();
        }

        if let None = &self.operation {
            panic!("Tried to evaluate a gate with no operation!");
        }

        match self.operation.unwrap() {
            Operation::AND => self.value = Some(gates[0].value.unwrap() & gates[1].value.unwrap()),
            Operation::OR => self.value = Some(gates[0].value.unwrap() | gates[1].value.unwrap()),
            Operation::XOR => self.value = Some(gates[0].value.unwrap() ^ gates[1].value.unwrap()),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
enum Operation {
    AND,
    OR,
    XOR,
}

fn load_gates(name: &str) -> HashMap<&str, Rc<Gate>> {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut data_lines = data_string.lines();

    let mut gates: HashMap<&str, Rc<Gate>> = HashMap::new();

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
        gates.insert(name, Rc::new(gate));
    }

    // Declare the rest of the gates
    for line in data_lines.clone() {
        let name = line.split_whitespace().last().unwrap();
        let gate = Gate::new(name.to_string());
        gates.insert(name, Rc::new(gate));
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
            _ => panic!("Tried to parse invalid gate: {}", operation_string) 
        };

        let mut gate = gates.get_mut(strings[4]).unwrap();
        let gate = Rc::get_mut(gate).unwrap();
        gate.components = Some([component1, component2]);
    }

    gates
}

fn main() {
    let gates = load_gates(_EXAMPLE);
}
