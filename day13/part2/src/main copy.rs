#![allow(arithmetic_overflow)]

use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


const _INPUT: &str = "input.txt";
const _TEST_INPUT: &str = "test_input.txt";
const _TEST_INPUT_2: &str = "test_input_2.txt";

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
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
            a: Position { x: 0, y: 0 },
            b: Position { x: 0, y: 0 },
            prize: Position { x: 0, y: 0 },
        }
    }
}

fn load_claw_machines_from_file(name: &str, part2: bool) -> Vec<ClawMachine> {
    let path = Path::new(name);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut claw_machines_vec: Vec<ClawMachine> = Vec::new();
    let mut i = 0;
    let mut claw_machine = ClawMachine::new();

    for line in data_string.lines() {
        if i % 4 == 3 {
            i += 1;
            continue;
        }

        let numbers_string = line
            .chars()
            .filter(|char| char.is_numeric() || char.is_whitespace())
            .collect::<String>();
        let mut number_string_iter = numbers_string.split_whitespace();
        let x: i64 = number_string_iter.next().unwrap().parse::<i64>().unwrap();
        let y: i64 = number_string_iter.next().unwrap().parse::<i64>().unwrap();

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
                if part2 {
                    claw_machine.prize.x = x + 10000000000000;
                    claw_machine.prize.y = y + 10000000000000;
                } else {
                    claw_machine.prize.x = x;
                    claw_machine.prize.y = y;
                }
                claw_machines_vec.push(claw_machine.clone());
            }
            _ => panic!("Mod math is broken!"),
        }

        i += 1;
    }

    claw_machines_vec
}

fn compute_minimum_cost(claw_machine: ClawMachine) -> i64 {
    let minimum_cost: Option<i64> = None;
    let a_press_cost = 3;
    let b_press_cost = 1;
    let a_presses;
    let b_presses;

    let x_dominant_button: Position;
    let y_dominant_button: Position;
    let mut total_x_dominant_button_presses = 0;
    let mut total_y_dominant_button_presses = 0;
    if claw_machine.a.x >= claw_machine.a.y {
        x_dominant_button = claw_machine.a;
        y_dominant_button = claw_machine.b;
    }
    else {
        y_dominant_button = claw_machine.a;
        x_dominant_button = claw_machine.b;
    }

    let mut current_position = Position { x: 0, y: 0 };

    for _i in 0..10 {
        let amount_to_move_x = claw_machine.prize.x - current_position.x;
        let mut x_dominant_button_presses = (amount_to_move_x as f64 / x_dominant_button.x as f64).round() as i64;
        total_x_dominant_button_presses += x_dominant_button_presses;
        // if total_x_dominant_button_presses < 0 {
        //     x_dominant_button_presses *= -1;
        //     total_x_dominant_button_presses += 2*x_dominant_button_presses;
        // }
        current_position.x += x_dominant_button_presses * x_dominant_button.x;
        current_position.y += x_dominant_button_presses * x_dominant_button.y;

        eprintln!("{}", current_position);

        let amount_to_move_y = claw_machine.prize.y - current_position.y;
        let mut y_dominant_button_presses= (amount_to_move_y as f64 / y_dominant_button.y as f64).round() as i64;
        total_y_dominant_button_presses += y_dominant_button_presses;
        // if total_y_dominant_button_presses < 0 {
        //     y_dominant_button_presses *= -1;
        //     total_y_dominant_button_presses += 2*y_dominant_button_presses;
        // }
        current_position.x += y_dominant_button_presses * y_dominant_button.x;
        current_position.y += y_dominant_button_presses * y_dominant_button.y;

        eprintln!("{}", current_position);
    }

    if x_dominant_button == claw_machine.a {
        a_presses = total_x_dominant_button_presses;
        b_presses = total_y_dominant_button_presses;
    } else {
        b_presses = total_x_dominant_button_presses;
        a_presses = total_y_dominant_button_presses;
    }

    let cost = a_presses* a_press_cost + b_presses * b_press_cost;
    if current_position == claw_machine.prize {
        println!("SUCCESS | A presses: {}, B presses: {}, Cost: {}", a_presses, b_presses, cost);
        return cost
    }
    else {
        println!("FAILURE | A presses: {}, B presses: {}, Cost: {}", a_presses, b_presses, cost);
        return 0
    }
}

fn compute_minimum_cost_sum(claw_machines: Vec<ClawMachine>) -> i64 {
    let mut cost_sum = 0;
    for claw_machine in claw_machines {
        cost_sum += compute_minimum_cost(claw_machine);
    }

    cost_sum
}

fn main() {
    let claw_machines = load_claw_machines_from_file(_INPUT, false);
    let minimum_total_cost = compute_minimum_cost_sum(claw_machines);
    println!("Minimum total cost: {}", minimum_total_cost);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_minimum_cost() {
        let claw_machines = load_claw_machines_from_file(_TEST_INPUT, false);

        let minimum_cost = compute_minimum_cost(claw_machines[0].clone());
        assert_eq!(minimum_cost, 280);

        let minimum_cost = compute_minimum_cost(claw_machines[2].clone());
        assert_eq!(minimum_cost, 200);
    }

    #[test]
    fn test_load_claw_machines_from_file() {
        let claw_machines = load_claw_machines_from_file(_TEST_INPUT, true);
        assert_eq!(claw_machines[0].a.x, 94);
        assert_eq!(claw_machines[0].a.y, 34);
        assert_eq!(claw_machines[0].b.x, 22);
        assert_eq!(claw_machines[0].b.y, 67);
        assert_eq!(claw_machines[0].a.x, 94);
        assert_eq!(claw_machines[0].prize.x, 8400 + 10000000000000);
        assert_eq!(claw_machines[0].prize.y, 5400 + 10000000000000);
    }

    #[test]
    fn test_website_example_part1() {
        let claw_machines = load_claw_machines_from_file(_TEST_INPUT, false);
        let minimum_total_cost = compute_minimum_cost_sum(claw_machines);
        assert_eq!(minimum_total_cost, 480);
    }

    #[test]
    fn test_part_1() {
        let claw_machines = load_claw_machines_from_file(_INPUT, false);
        let minimum_total_cost = compute_minimum_cost_sum(claw_machines);
        assert_eq!(minimum_total_cost, 34393);
    }

    // #[test]
    fn test_website_example_part2() {
    let claw_machines = load_claw_machines_from_file(_TEST_INPUT, true);
    assert!(compute_minimum_cost(claw_machines[0].clone()) == 0);
    assert!(compute_minimum_cost(claw_machines[1].clone()) > 0);
    assert!(compute_minimum_cost(claw_machines[2].clone()) == 0);
    assert!(compute_minimum_cost(claw_machines[3].clone()) > 0);
    }

    #[test]
    fn test_claw_machine_48() {
        let claw_machines = load_claw_machines_from_file(_INPUT, false);
        let minimum_total_cost = compute_minimum_cost(claw_machines[47].clone());
        assert_eq!(minimum_total_cost, 157);
    }
}
