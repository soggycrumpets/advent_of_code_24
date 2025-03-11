#![allow(arithmetic_overflow)]

use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::thread::current;

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

// Here's the general idea behind this function:
// One of the buttons is the "x-shifter" and the other is the "y-shifter"
// The x-shifter button is pressed/unpressed (yes, you can't actually un-press the buttons, but pretend you can) until the 
//      current x-position is as close as it can get to the prize's x-position
// Then, the y-shifter button is pressed/unpressed until the current y-position matches the prize's y-position
// This repeats for a while. If the current position converges to a single (x, y) coordinate, a solution has been found.
// The function returns the number of times each button was pressed to reach this point.
// This function is called twice, once with button A being the x-shifter and the other with B being x-shifter.
// If neither of these converge on a single coordinate, then there is no solution.
fn search_for_equilibrium(
    claw_machine: &ClawMachine,
    x_shifter: &Position,
    y_shifter: &Position,
) -> Option<(i64, i64)> {
    let mut current_position = Position { x: 0, y: 0 };
    let mut total_x_shifter_presses = 0;
    let mut total_y_shifter_presses = 0;

    // There are three outcomes that will break this loop:
    // 1 - the output has converged to an equilibrium - return the A and B presses used to get here
    // 2 - the output has diverged - output becomes so large that an integer overflow occurs. No solution.
    // 3 - the output dances around an equilibrium but never reaches a steady state (like a sine wave / periodic function). No solution.
    //      #3 is checked for by looking for duplicates in a list of visited positions. This is the only case where we visit the same position twice.

    let mut visited_positions: HashSet<Position> = HashSet::new();
    loop {
        // Force X position toward target
        let amount_to_move_x = claw_machine.prize.x - current_position.x;
        let x_shifter_presses = (amount_to_move_x as f64 / x_shifter.x as f64).floor() as i64; // Need to floor the first button_presses and ceil the second.
        total_x_shifter_presses += x_shifter_presses; // Otherwise, the program can get stuck right next to the solution.

        match i64::checked_mul(x_shifter_presses, x_shifter.x) {
            Some(mul) => current_position.x += mul,
            None => break,
        }
        match i64::checked_mul(x_shifter_presses, x_shifter.y) {
            Some(mul) => current_position.y += mul,
            None => break,
        }

        // Check break conditions
        if current_position == claw_machine.prize {
            break
        }
        if visited_positions.get(&current_position) != None {
            return None
        }
        visited_positions.insert(current_position);


        // Force Y position toward target
        let amount_to_move_y = claw_machine.prize.y - current_position.y;
        let y_shifter_presses = (amount_to_move_y as f64 / y_shifter.y as f64).ceil() as i64;
        total_y_shifter_presses += y_shifter_presses;

        match i64::checked_mul(y_shifter_presses, y_shifter.x) {
            Some(mul) => current_position.x += mul,
            None => break,
        }
        match i64::checked_mul(y_shifter_presses, y_shifter.y) {
            Some(mul) => current_position.y += mul,
            None => break,
        }

        // Check break conditions
        if current_position == claw_machine.prize {
            break;
        }
        if visited_positions.get(&current_position) != None {
            return None
        }
        visited_positions.insert(current_position);

    }

    if current_position == claw_machine.prize {
        return Some((total_x_shifter_presses, total_y_shifter_presses));
    } else {
        return None;
    }
}

fn compute_minimum_cost(claw_machine: ClawMachine) -> i64 {
    let a_press_cost = 3;
    let b_press_cost = 1;

    // A as x-shifter converges on a solution
    match search_for_equilibrium(&claw_machine, &claw_machine.a, &claw_machine.b) {
        Some((a_presses, b_presses)) => return a_presses * a_press_cost + b_presses * b_press_cost,
        None => (),
    }
    // B as x-shifter converges on a solution
    match search_for_equilibrium(&claw_machine, &claw_machine.b, &claw_machine.a) {
        Some((b_presses, a_presses)) => return a_presses * a_press_cost + b_presses * b_press_cost,
        None => (),
    }

    // No solution
    return 0;
}

fn compute_minimum_cost_sum(claw_machines: Vec<ClawMachine>) -> i64 {
    let mut cost_sum = 0;
    for claw_machine in claw_machines {
        cost_sum += compute_minimum_cost(claw_machine);
    }

    cost_sum
}

fn main() {
let Timer

    let claw_machines = load_claw_machines_from_file(_INPUT, true);
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

    #[test]
    fn test_website_example_part2() {
        let claw_machines = load_claw_machines_from_file(_TEST_INPUT, true);
        assert!(compute_minimum_cost(claw_machines[0].clone()) == 0);
        assert!(compute_minimum_cost(claw_machines[1].clone()) > 0);
        assert!(compute_minimum_cost(claw_machines[2].clone()) == 0);
        assert!(compute_minimum_cost(claw_machines[3].clone()) > 0);
    }

    #[test]
    fn test_problematic_claw_machines() {
        let claw_machines = load_claw_machines_from_file(_INPUT, false);

        // 48
        let minimum_total_cost = compute_minimum_cost(claw_machines[47].clone());
        assert_eq!(minimum_total_cost, 157);

        // 72
        let minimum_total_cost = compute_minimum_cost(claw_machines[71].clone());
        assert_eq!(minimum_total_cost, 279);

        // 91
        let minimum_total_cost = compute_minimum_cost(claw_machines[90].clone());
        assert_eq!(minimum_total_cost, 244);

        // 92
        let minimum_total_cost = compute_minimum_cost(claw_machines[91].clone());
        assert_eq!(minimum_total_cost, 287);
    }
}