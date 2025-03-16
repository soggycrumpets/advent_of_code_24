use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input.txt";
const _EXAMPLE: &str = "example.txt";

fn load_connections(name: &str) -> HashMap<String, Vec<String>> {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    let data_lines = data_string.lines();
    for line_str in data_lines {
        let mut computers = line_str.split('-');
        let computer1: String = computers.next().unwrap().to_string();
        let computer2: String = computers.next().unwrap().to_string();
        add_computer_to_connections(&computer1, &computer2, &mut connections);
        add_computer_to_connections(&computer2, &computer1, &mut connections);
    }
    connections
}

fn add_computer_to_connections(
    computer1: &str,
    computer2: &str,
    connections: &mut HashMap<String, Vec<String>>,
) {
    if let Some(connection_list) = connections.get_mut(computer1) {
        connection_list.push(computer2.to_string());
    } else {
        connections.insert(computer1.to_string(), vec![computer2.to_string()]);
    }
}

fn main() {
    let connections = load_connections(_INPUT);
    let mut triangular_connections: Vec<Vec<&str>> = Vec::new();
    for (computer1, computer1_connections) in &connections {
        for computer2 in computer1_connections {
            let computer2_connections = connections.get(computer2).unwrap();
            for computer3 in computer2_connections {
                let computer3_connections = connections.get(computer3).unwrap();
                for computer4 in computer3_connections {
                    if computer1 == computer4 {
                        triangular_connections.push(vec![computer1, computer2, computer3]);
                    }
                }
            }
        }
    }

    let mut triangular_connections_filtered: Vec<Vec<&str>> = Vec::new();
    for computers in triangular_connections {
        if computers[0].starts_with('t')
            || computers[1].starts_with('t')
            || computers[2].starts_with('t')
        {
            triangular_connections_filtered.push(computers);
        }
    }

    println!(
        "Number of triangular connections that contain a t computer: {}",
        triangular_connections_filtered.len() / 3 / 2
    );
}
