use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input.txt";
const _EXAMPLE: &str = "example.txt";

fn load_gates(name: &str) -> HashMap<String, Vec<String>> {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut data_lines = data_string.lines();

    let mut gates: HashMap<String, Vec<String>> = HashMap::new();
    // Initialize the set of gates with predetermined values
    loop {
        let line = data_lines.next().unwrap();

        if line.trim().is_empty() {
            break;
        }

        let mut string_iter = line.split(':');
        let name: &str = string_iter.next().unwrap();

        gates.insert(name.to_string(), Vec::new());
    }

    // Initialize the rest of the gates
    for line in data_lines {
        let strings: Vec<&str> = line.split_ascii_whitespace().collect();
        let name = strings[4].to_string();
        let component1 = strings[0].to_string();
        let operation: String = strings[1].to_string();
        let component2 = strings[2].to_string();

        gates.insert(name, vec![component1, operation, component2]);
    }

    gates
}

fn sort_z_gates(z_gates: Vec<String>) -> Vec<String> {
    let mut z_gates_sorted: Vec<String> = vec![String::new(); z_gates.len()];

    for i in 0..z_gates.len() {
        let index: usize = z_gates[i]
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap();

        z_gates_sorted[index] = z_gates[i].clone();
    }

    z_gates_sorted
}

fn extract_base_eqns(gate: &str, gates: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let eqn = gates.get(gate).unwrap();
    let mut base_eqns: Vec<Vec<String>> = Vec::new();

    if eqn[0].starts_with('x') || eqn[0].starts_with('y') {
        base_eqns.push(eqn.clone());
        return base_eqns;
    }

    base_eqns.append(&mut extract_base_eqns(&eqn[0], gates));
    base_eqns.append(&mut extract_base_eqns(&eqn[2], gates));

    return base_eqns;
}

fn get_number_label(i: usize) -> String {
    match i > 9 {
        true => i.to_string(),
        false => vec!['0', i.to_string().chars().next().unwrap()]
            .into_iter()
            .collect(),
    }
}

fn get_next_base_equations(base_eqns: &Vec<Vec<String>>, iter: usize) -> Vec<Vec<String>> {
    // First new base equation
    let iter_number_string = get_number_label(iter);
    let mut operand1 = "y".to_string();
    operand1.push_str(iter_number_string.as_str());
    let mut operand2 = "x".to_string();
    operand2.push_str(iter_number_string.as_str());
    let new_eqn_1 = vec![operand1, "XOR".to_string(), operand2];

    // Second new base equation
    let prev_iter_number_string = get_number_label(iter);
    let mut operand1 = "y".to_string();
    operand1.push_str(prev_iter_number_string.as_str());
    let mut operand2 = "x".to_string();
    operand2.push_str(prev_iter_number_string.as_str());
    let new_eqn_2 = vec![operand1, "XOR".to_string(), operand2];

    let mut new_eqns = base_eqns.clone();
    new_eqns.push(new_eqn_1);
    new_eqns.push(new_eqn_2);
    new_eqns
}

fn main() {
    let gates = load_gates(_INPUT);
    let mut z_gates: Vec<String> = Vec::new();
    for (gate, eqn) in &gates {
        if gate.starts_with('z') {
            z_gates.push(gate.clone());
        }
    }

    z_gates = sort_z_gates(z_gates);

    let i: usize = 2;
    let prev_base_eqns = extract_base_eqns(&z_gates[i], &gates);
    let mut base_eqns_ideal = get_next_base_equations(&prev_base_eqns, i);
    base_eqns_ideal.sort();

    let mut base_eqns = extract_base_eqns(&z_gates[i + 1], &gates);
    base_eqns.sort();

    for eqn in prev_base_eqns {
        println!("{} {} {}", eqn[0], eqn[1], eqn[2]);
    }
    println!();

    for eqn in base_eqns_ideal {
        println!("{} {} {}", eqn[0], eqn[1], eqn[2]);
    }
    println!();

    for eqn in base_eqns {
        println!("{} {} {}", eqn[0], eqn[1], eqn[2]);
    }

    // let base_eqns = extract_base_eqns(&z_gates[1], &gates);
    // for eqn in base_eqns {
    //     print!("{} {} {} | ", eqn[0], eqn[1], eqn[2]);
    // }
    // println!();

    // let base_eqns = extract_base_eqns(&z_gates[2], &gates);
    // for eqn in base_eqns {
    //     print!("{} {} {} | ", eqn[0], eqn[1], eqn[2]);
    // }
    // println!();

    // let base_eqns = extract_base_eqns(&z_gates[3], &gates);
    // for eqn in base_eqns {
    //     print!("{} {} {} | ", eqn[0], eqn[1], eqn[2]);
    // }
    // println!();
}
