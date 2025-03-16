use std::collections::HashMap;
use std::collections::HashSet;
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

fn confirm_connections(party: &HashSet<String>, connection_list: &Vec<String>) -> bool {
    for computer2 in party {
        if !connection_list.contains(computer2) {
            return false;
        }
    }
    return true;
}

fn add_to_party(
    party: &mut HashSet<String>,
    computer: &str,
    connections: &HashMap<String, Vec<String>>,
) {
    let connected_computers = connections.get(computer).unwrap();


    // Don't add the same computer to the party twice
    if let Some(_) = party.get(computer) {
        return;
    }
    // Don't add this computer to the party if it isn't connected to all of the current party members
    if !confirm_connections(party, connected_computers) {
        return;
    }

    party.insert(computer.to_string());
    for connection in connected_computers {
        add_to_party(party, connection, connections);
    }
}

fn main() {
    let connections = load_connections(_INPUT);

    let mut parties: Vec<HashSet<String>> = Vec::new();
    let mut computers_in_party: HashSet<String> = HashSet::new();

    for (computer, _connection_list) in connections.clone() {

        // Skip this computer if it's already been added to a party
        if computers_in_party.get(&computer) != None {
           continue 
        }

        let mut party: HashSet<String> = HashSet::new();
        add_to_party(&mut party, computer.as_str(), &connections);
        parties.push(party.clone());

        for computer in party {
            computers_in_party.insert(computer);
        }
    }

    let mut biggest_party: Option<HashSet<String>> = None;
    for party in parties {
        if let Some(big_party) = &biggest_party {
            if big_party.len() < party.len() {
                biggest_party = Some(party);
            }
        } else {
            biggest_party = Some(party);
        }
    }

    let mut biggest_party: Vec<String> = biggest_party.unwrap().into_iter().collect();
    biggest_party.sort();
        let mut party_str = String::new();
        for computer in biggest_party {
            party_str.push_str(computer.as_str());
            party_str.push(',');
        }
        party_str.pop();
        println!("{}", party_str);
}