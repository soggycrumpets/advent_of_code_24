use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const _INPUT: &str = "input.txt";
const _TEST_INPUT: &str = "test_input.txt";
const _TEST_INPUT_2: &str = "test_input_2.txt";

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone)]
struct ClawMachine {
    a: Position,
    b: Position,
    prize: Position,
}
impl ClawMachine {
    fn new() -> ClawMachine {
        ClawMachine {
            a: Position {x: 0, y: 0},
            b: Position {x: 0, y: 0},
            prize: Position {x: 0, y: 0},
        }
    }
}

fn load_claw_machines_from_file(name: &str) -> Vec<ClawMachine> {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut claw_machines_vec: Vec<ClawMachine> = Vec::new();
    let mut i = 0;
    let mut claw_machine = ClawMachine::new();

    for line in data_string.lines() {
        if i % 4 == 3  {
            i += 1;
            continue
        }

        let numbers_string= line.chars().filter(|char| char.is_numeric() || char.is_whitespace()).collect::<String>();
        let mut number_string_iter = numbers_string.split_whitespace();
        let x = number_string_iter.next().unwrap().parse().unwrap();
        let y = number_string_iter.next().unwrap().parse().unwrap();

        match i % 4 {
            0 => {
                claw_machine.a.x = x;
                claw_machine.a.y = y;
            }
            1 => {
                claw_machine.b.x = x;
                claw_machine.b.y = y;
            }
            2 => {
                claw_machine.prize.x = x;
                claw_machine.prize.y = y;
                claw_machines_vec.push(claw_machine.clone());
            }
            _ => panic!("Mod math is broken!"),
        }

        i += 1;
    }

    claw_machines_vec
}

fn compute_minimum_cost(claw_machine: ClawMachine) -> i32 {
    let mut minimum_cost: Option<i32> = None;    
    let maximum_button_presses = 100;
    let a_press_cost = 3;
    let b_press_cost = 1;

    let mut a_min = 0;
    let mut b_min = 0;
    for a_presses in 0..(maximum_button_presses+1) {
        let mut crane_position = Position {
            x: claw_machine.a.x * a_presses, 
            y: claw_machine.a.y * a_presses
        };
        // For every legal number of a presses (< 100),
        // press b until either a prize is found or the legal number of presses is exceeded.
        for b_presses in 0..(maximum_button_presses+1) {
            if crane_position == claw_machine.prize {
                let cost = a_presses * a_press_cost + b_presses * b_press_cost;
                match minimum_cost {
                    Some(min) => {
                        if cost < min {
                            a_min = a_presses;
                            b_min = b_presses;
                        }
                        minimum_cost = Some(cost.min(min));
                    }
                    None => {
                        minimum_cost = Some(cost);
                        a_min = a_presses;
                        b_min = b_presses;
                    }
                }
                break
            }
            crane_position.x += claw_machine.b.x;
            crane_position.y += claw_machine.b.y;
        }
    }
    match minimum_cost {
        Some(min) => {
            // println!("SUCCESS | A presses: {}, B presses: {}, Cost: {}", a_min, b_min, min);
        },
        None => {
            println!("FAILURE");
        },
    }

    match minimum_cost {
        Some(min) => min,
        None => 0,
    }
}

fn compute_minimum_cost_sum(claw_machines: Vec<ClawMachine>) -> i32 {
    let mut cost_sum = 0;
    for claw_machine in claw_machines {
        cost_sum += compute_minimum_cost(claw_machine);
    }

    cost_sum
}

fn main() {
    let claw_machines = load_claw_machines_from_file(_INPUT);
    let minimum_total_cost = compute_minimum_cost_sum(claw_machines);
    println!("Minimum total cost: {}", minimum_total_cost);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_minimum_cost() {
        let claw_machine = ClawMachine {
            a: Position {x: 94, y: 34},
            b: Position {x: 22, y: 67},
            prize: Position {x: 8400, y: 5400},
        };

        let minimum_cost = compute_minimum_cost(claw_machine);
        assert_eq!(minimum_cost, 280);
    }

    #[test]
    fn test_load_claw_machines_from_file() {
        let claw_machines = load_claw_machines_from_file(_TEST_INPUT);
        assert_eq!(claw_machines[0].a.x, 94);
        assert_eq!(claw_machines[0].a.y, 34);
        assert_eq!(claw_machines[0].b.x, 22);
        assert_eq!(claw_machines[0].b.y, 67);
        assert_eq!(claw_machines[0].a.x, 94);
        assert_eq!(claw_machines[0].prize.x, 8400);
        assert_eq!(claw_machines[0].prize.y, 5400);
    }

    #[test]
    fn test_website_example() {
        let claw_machines = load_claw_machines_from_file(_TEST_INPUT);
        let minimum_total_cost = compute_minimum_cost_sum(claw_machines);
        assert_eq!(480, minimum_total_cost);
    }
}
