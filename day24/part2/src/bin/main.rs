use std::collections::HashMap;
use std::collections::HashSet;
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

fn extract_base_eqns(
    gate: &str,
    gates: &HashMap<String, Vec<String>>,
    explored_gates: &mut HashSet<String>,
) -> Vec<Vec<String>> {
    let eqn = gates.get(gate).unwrap();
    let mut base_eqns: Vec<Vec<String>> = Vec::new();

    if let Some(_) = explored_gates.get(gate) {
        return base_eqns;
    }
    explored_gates.insert(gate.to_string());

    if eqn[0].starts_with('x') || eqn[0].starts_with('y') {
        base_eqns.push(eqn.clone());
        return base_eqns;
    }

    base_eqns.append(&mut extract_base_eqns(&eqn[0], gates, explored_gates));
    base_eqns.append(&mut extract_base_eqns(&eqn[2], gates, explored_gates));

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

fn _print_base_eqns(base_eqns: &Vec<Vec<String>>) {
    println!();
    for eqn in base_eqns {
        println!("{} {} {}", eqn[0], eqn[1], eqn[2]);
    }
}

fn get_next_base_eqns(base_eqns: &Vec<Vec<String>>, iter: usize) -> Vec<Vec<String>> {
    // Special case at the very end where there is no x45, y45
    if iter == 45 {
        let iter_number_string = get_number_label(iter - 1);
        let mut operand1 = "y".to_string();
        operand1.push_str(iter_number_string.as_str());
        let mut operand2 = "x".to_string();
        operand2.push_str(iter_number_string.as_str());
        let new_eqn = vec![operand1, "AND".to_string(), operand2];
        let mut new_eqns = base_eqns.clone();
        new_eqns.push(new_eqn);

        return new_eqns 
    }

    // First new base equation
    let iter_number_string = get_number_label(iter - 1);
    let mut operand1 = "y".to_string();
    operand1.push_str(iter_number_string.as_str());
    let mut operand2 = "x".to_string();
    operand2.push_str(iter_number_string.as_str());
    let new_eqn_1 = vec![operand1, "AND".to_string(), operand2];

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
    new_eqns = deep_sort(new_eqns);
    new_eqns
}

fn switch_eqns(gate1: &str, gate2: &str, gates: &mut HashMap<String, Vec<String>>) {
    let eqn1 = gates.get(gate1).unwrap().clone();
    let eqn2 = gates.get(gate2).unwrap().clone();
    gates.insert(gate1.to_string(), eqn2);
    gates.insert(gate2.to_string(), eqn1);
}

fn filter_base_gates(gates: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut gates_with_eqns: Vec<String> = Vec::new();
    for (gate, _eqn) in gates {
        if !gate.starts_with('x') && !gate.starts_with('y') {
            gates_with_eqns.push(gate.clone())
        }
    }
    gates_with_eqns.sort();
    gates_with_eqns
}

fn deep_sort(eqns: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut sorted = eqns;
    for eqn in &mut sorted {
        eqn.sort();
    }
    sorted.sort();
    sorted
}

fn get_ideal_base_eqns(
    z_gates: &Vec<String>,
    gates: &HashMap<String, Vec<String>>,
) -> Vec<Vec<Vec<String>>> {
    let mut ideal_base_eqns: Vec<Vec<Vec<String>>> = Vec::new();

    let mut explored_gates: HashSet<String> = HashSet::new();
    let mut base_eqns;

    base_eqns = extract_base_eqns(&z_gates[0], gates, &mut explored_gates);
    base_eqns = deep_sort(base_eqns);
    ideal_base_eqns.push(base_eqns);

    base_eqns = extract_base_eqns(&z_gates[1], gates, &mut explored_gates);
    base_eqns = deep_sort(base_eqns);
    ideal_base_eqns.push(base_eqns);

    let mut prev_base_eqns = extract_base_eqns(&z_gates[1], gates, &mut explored_gates);
    prev_base_eqns = get_next_base_eqns(&prev_base_eqns, 1);
    for i in 2..z_gates.len() {
        let mut base_eqns = get_next_base_eqns(&prev_base_eqns, i);
        base_eqns = deep_sort(base_eqns);

        ideal_base_eqns.push(base_eqns.clone());
        prev_base_eqns = base_eqns;
    }

    ideal_base_eqns
}

fn verify_swap(
    gates: &HashMap<String, Vec<String>>,
    z_gates: &Vec<String>,
    index: usize,
    ideal_eqns: &Vec<Vec<Vec<String>>>,
) -> bool {
    let mut i = 0;
    loop {
        let mut explored_gates = HashSet::new();
        let mut base_eqns = extract_base_eqns(&z_gates[i], gates, &mut explored_gates);
        base_eqns = deep_sort(base_eqns);

        if base_eqns != ideal_eqns[i] {
            return false;
        } else if i == index {
            break;
        }
        i += 1;
    }
    true
}

fn swap_wires_until_match(
    gates: &mut HashMap<String, Vec<String>>,
    gates_with_eqns: &Vec<String>,
    z_gates: &Vec<String>,
    index: usize,
    ideal_eqns: &Vec<Vec<Vec<String>>>,
) -> Vec<String> {
    let gate = &z_gates[index];

    for i in 0..gates_with_eqns.len() {
        for j in 0..gates_with_eqns.len() {
            let gate1 = gates_with_eqns[i].as_str();
            let gate2 = gates_with_eqns[j].as_str();

            switch_eqns(gate1, gate2, gates);

            let mut explored_gates: HashSet<String> = HashSet::new();
            let mut base_eqns = extract_base_eqns(gate, gates, &mut explored_gates);
            base_eqns = deep_sort(base_eqns);

            if base_eqns == ideal_eqns[index] {
                if verify_swap(gates, &z_gates, index, ideal_eqns) {
                    println!("Swapped {} with {}", gate1, gate2);
                }
            }

            switch_eqns(gate1, gate2, gates);
        }
    }
    println!("Failed to find a wire to swap for gate: {}", gate);
    vec![]
}

fn main() {
    let mut gates = load_gates(_INPUT);
    let mut z_gates: Vec<String> = Vec::new();
    for (gate, _eqn) in &gates {
        if gate.starts_with('z') {
            z_gates.push(gate.clone());
        }
    }

    {
        // Manually input the answer to the problem
        switch_eqns("gjc", "qjj", &mut gates);
        switch_eqns("z17", "wmp", &mut gates);
        switch_eqns("z39", "qsb", &mut gates);
        switch_eqns("z26", "gvm", &mut gates);
    }

    // Do a bunch of useless stuff
    z_gates = sort_z_gates(z_gates);
    let ideal_base_eqns = get_ideal_base_eqns(&z_gates, &gates);

    let gates_with_eqns: Vec<String> = filter_base_gates(&gates);
    let mut wire_swaps: Vec<String> = Vec::new();
    for i in 2..z_gates.len() {
        println!("{}", i);

        let mut explored_gates: HashSet<String> = HashSet::new();
        let prev_base_eqns = extract_base_eqns(&z_gates[i - 1], &gates, &mut explored_gates);

        explored_gates.clear();
        let mut base_eqns = extract_base_eqns(&z_gates[i], &gates, &mut explored_gates);

        let mut base_eqns_ideal = get_next_base_eqns(&prev_base_eqns, i);

        for eqn in &mut base_eqns {
            eqn.sort()
        }
        for eqn in &mut base_eqns_ideal {
            eqn.sort()
        }
        base_eqns_ideal.sort();
        base_eqns.sort();

        if base_eqns != base_eqns_ideal {
            wire_swaps.append(&mut swap_wires_until_match(
                &mut gates,
                &gates_with_eqns,
                &z_gates,
                i,
                &ideal_base_eqns,
            ));
            println! {"{}", verify_swap(&mut gates, &z_gates, i, &ideal_base_eqns)};
        }
        println! {"{}", verify_swap(&mut gates, &z_gates, i, &ideal_base_eqns)};
    }
    wire_swaps.push("gjc".to_string());
    wire_swaps.push("z17".to_string());
    wire_swaps.push("z39".to_string());
    wire_swaps.push("z26".to_string());
    wire_swaps.push("qjj".to_string());
    wire_swaps.push("wmp".to_string());
    wire_swaps.push("qsb".to_string());
    wire_swaps.push("gvm".to_string());

    wire_swaps.sort();
    let mut swap_string = String::new();
    for wire in &wire_swaps {
        swap_string.push_str(wire);
        swap_string.push(',');
    }
    swap_string.pop();
    println!("Wires to swap: {}", swap_string);
}

#[test]
fn test_switch_eqns() {
    let mut gates = load_gates(_INPUT);
    let gate1 = "z10";
    let gate2 = "z20";
    let eqn1 = gates.get(gate1).unwrap().clone();
    let eqn2 = gates.get(gate2).unwrap().clone();

    switch_eqns(gate1, gate2, &mut gates);

    let eqn1_switched = gates.get(gate1).unwrap();
    let eqn2_switched = gates.get(gate2).unwrap();

    assert_eq!(eqn1, *eqn2_switched);
    assert_eq!(eqn2, *eqn1_switched);
}
