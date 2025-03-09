use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Display, Path};
use std::thread::sleep;
use std::time::{Duration, Instant};

const _INPUT: &str = "input.txt";
const _EXAMPLE1: &str = "part1_example.txt";
const _EXAMPLE2: &str = "part2_example.txt";

struct CPU<'a> {
    rax: i128,
    rbx: i128,
    rcx: i128,
    rip: i128,

    output: &'a mut Vec<i128>,
}
impl<'a> CPU<'a> {
    fn new(output: &'a mut Vec<i128>) -> CPU<'a> {
        CPU {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rip: 0,
            output: output,
        }
    }

    fn init(&mut self, rax: i128, rbx: i128, rcx: i128) {
        self.rax = rax;
        self.rbx = rbx;
        self.rcx = rcx;
        self.rip = 0;
        self.output.clear();
    }

    fn run(&mut self, program: &Vec<i128>) {
        while self.rip < program.len() as i128 {
            self.op(program);
        }
    }

    fn op(&mut self, program: &Vec<i128>) {
        let opcode = program[self.rip as usize];
        let operand = program[self.rip as usize + 1];

        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("Invalid opcode: {}", opcode),
        }
    }

    fn combo_operand(&self, operand: i128) -> i128 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.rax,
            5 => self.rbx,
            6 => self.rcx,
            7 => panic!("Reserved combo operation: {}", operand),
            _ => panic!("Invalid combo operation: {}", operand),
        }
    }

    fn adv(&mut self, operand: i128) {
        let numerator = self.rax;
        let divisor: i128 = 1 << self.combo_operand(operand);
        let result = numerator / divisor;

        self.rax = result;
        self.rip += 2;
    }

    fn bxl(&mut self, operand: i128) {
        let result = self.rbx ^ operand;

        self.rbx = result;
        self.rip += 2;
    }

    fn bst(&mut self, operand: i128) {
        let result = self.combo_operand(operand) % 8;

        self.rbx = result;
        self.rip += 2;
    }

    fn jnz(&mut self, operand: i128) {
        if 0 != self.rax {
            self.rip = operand;
        } else {
            self.rip += 2;
        }
    }

    fn bxc(&mut self, _operand: i128) {
        let result = self.rbx ^ self.rcx;

        self.rbx = result;
        self.rip += 2;
    }

    fn out(&mut self, operand: i128) {
        let result = self.combo_operand(operand) % 8;

        self.output.push(result);
        self.rip += 2
    }

    fn bdv(&mut self, operand: i128) {
        let numerator = self.rax;
        let divisor: i128 = 1 << self.combo_operand(operand);
        let result = numerator / divisor;

        self.rbx = result;
        self.rip += 2;
    }

    fn cdv(&mut self, operand: i128) {
        let numerator = self.rax;
        let divisor: i128 = 1 << self.combo_operand(operand);
        let result = numerator / divisor;

        self.rcx = result;
        self.rip += 2;
    }
}

struct Controller<'a> {
    target: &'a Vec<i128>,
    state: State,
    guess: i128,
    matches: i32,
    current_digit: usize,
    digit_states: Vec<i32>,
}
impl<'a> Controller<'a> {
    fn new(target: &Vec<i128>) -> Controller {
        Controller {
            target: target,
            state: State::Forward,
            guess: 8_i128.pow(target.len() as u32 - 1),
            matches: 0,
            current_digit: target.len() - 1,
            digit_states: vec![0; target.len()],
        }
    }

    fn get_next_guess(&mut self, output: &Vec<i128>) {

        if self.current_digit_is_maxed() {
            while self.current_digit_is_maxed() {
                self.reset_current_digit();
                self.current_digit += 1;
            }
            self.increment_digit();
        } else if self.current_digit_matches(output) {
            self.current_digit -= 1;
        } else {
            self.increment_digit();
        }

        // match self.state {
        //     State::Forward => {
        //         if self.current_digit_is_maxed() {
        //             while self.current_digit_is_maxed() {
        //                 self.reset_current_digit();
        //                 self.current_digit += 1;
        //             }
        //             self.increment_digit();
        //             self.state = State::Backward;
        //         } else if self.current_digit_matches(output) {
        //             self.current_digit -= 1;
        //         } else {
        //             self.increment_digit();
        //         }
        //     }
        //     State::Backward => {
        //         if self.current_digit_is_maxed() {
        //             self.reset_current_digit();
        //             self.current_digit += 1;
        //         } else if self.current_digit_matches(output) {
        //             self.current_digit -= 1;
        //             self.state = State::Forward;
        //         }
        //         else {
        //             self.increment_digit();
        //         }
        //     }
        // }

        // current digit = last digit
        // while current digit does not match, increment current digit.
        // current digit = next digit
        // ...
        // if current digit is incremented > 7 times
        // zero current digit
        // current digit = previous digit
        // increment current digit and while current digit does not match, increment current digit
        // if current digit is incremented > 7 times
        // zero current digit
        // current digit = previus digit
        // ...
        // if current digit matches
        // current digit = next digit
        // while current does not match, increment current digit
        // ...
    }

    fn current_digit_matches(&mut self, output: &Vec<i128>) -> bool {
        output[self.current_digit] == self.target[self.current_digit]
    }

    fn current_digit_is_maxed(&mut self) -> bool {
        if self.guess % 8_i128.pow(1 + self.current_digit as u32)
            == 7 * 8_i128.pow(self.current_digit as u32)
        {
            return true;
        } else {
            return false;
        }
    }

    fn reset_current_digit(&mut self) {
        // Digit states are 0-7
        let digit_state = self.guess % 8_i128.pow(1 + self.current_digit as u32);
        self.guess -= 7 * 8_i128.pow(self.current_digit as u32);
    }

    fn increment_digit(&mut self) {
        self.guess += 8_i128.pow(self.current_digit as u32);
    }

    // fn count_matching_digits_from_front(&mut self, output: &Vec<i128>) -> i32 {
    //     let mut matches = 0;
    //     for i in 0..output.len() {
    //         if output[i] == self.target[i] {
    //             matches += 1;
    //         } else {
    //             break;
    //         }
    //     }
    //     matches
    // }

    // fn count_matching_digits_from_back(&mut self, output: &Vec<i128>) -> i32 {
    //     let mut matches = 0;
    //     for i in (0..output.len()).rev() {
    //         if output[i] == self.target[i] {
    //             matches += 1;
    //         } else {
    //             break;
    //         }
    //     }
    //     matches
    // }

    // fn all_but_first_two_digits_match(&mut self, output: &Vec<i128>) -> bool {
    //     for i in 0..output.len() - 2 {
    //         if output[output.len() - 1 - i] != self.target[self.target.len() - 1 - i] {
    //             return false;
    //         }
    //     }
    //     return true;
    // }

    // fn all_digits_match(&mut self, output: &Vec<i128>) -> bool {
    //     for i in 0..output.len() {
    //         if output[output.len() - 1 - i] != self.target[self.target.len() - 1 - i] {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
}
#[derive(PartialEq)]
enum State {
    Forward,
    Backward,
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Forward => write!(f, "Forward"),
            State::Backward => write!(f, "Backward"),
        }
    }
}

fn load_program<'a>(filename: &str, cpu: &mut CPU) -> Vec<i128> {
    let path = Path::new(filename);
    let mut file = File::open(&path).unwrap();
    let mut data_string: String = String::new();
    file.read_to_string(&mut data_string).unwrap().to_string();

    let mut lines = data_string.lines();

    let rax = get_num_from_line(lines.next());
    let rbx = get_num_from_line(lines.next());
    let rcx = get_num_from_line(lines.next());
    cpu.init(rax, rbx, rcx);
    lines.next();
    let program: Vec<i128> = get_instructions_from_line(lines.next());

    return program;

    fn get_num_from_line(line: Option<&str>) -> i128 {
        line.unwrap()
            .chars()
            .filter(|char| char.is_numeric())
            .collect::<String>()
            .parse::<i128>()
            .unwrap()
    }
    fn get_instructions_from_line(line: Option<&str>) -> Vec<i128> {
        line.unwrap()
            .chars()
            .filter(|c| c.is_numeric() || *c == ',')
            .collect::<String>()
            .split(',')
            .map(|str| str.parse().unwrap())
            .collect()
    }
}

fn _print_output(output: &Vec<i128>) {
    let mut buf = String::new();
    for num in output {
        buf.push_str(num.to_string().as_str());
        buf.push(',');
    }
    buf.pop();
    print!("Output: {}", buf);
}

fn main() {
    let mut output = Vec::new();
    let mut cpu = CPU::new(&mut output);
    let program = load_program(_INPUT, &mut cpu);
    let mut controller: Controller = Controller::new(&program);
    controller.current_digit = program.len() - 1;

    cpu.run(&program);

    let rbx = cpu.rbx;
    let rcx = cpu.rcx;
    let mut rax: i128 = cpu.rax;

    let interval = Duration::from_secs(1) / 30;
    let mut next_time = Instant::now() + interval;

    loop {
        rax = controller.guess;

        cpu.init(rax, rbx, rcx);
        cpu.run(&program);
        _print_output(&cpu.output);
        print!(" | {}", controller.matches);
        print!(" | {}", controller.current_digit);
        print!(" | {}", controller.state);
        print!(" | {}", controller.current_digit_is_maxed());
        println!(" | {}", controller.guess);

        if *cpu.output == program {
            break;
        }
        controller.get_next_guess(cpu.output);

        sleep(next_time - Instant::now());
        next_time += interval;
    }
    println!("Program copies itself for rax = {}", rax);
}

#[test]
fn test_website_examples() {
    let mut output = Vec::new();
    let mut cpu = CPU::new(&mut output);

    {
        cpu.init(0, 0, 9);
        let program = vec![2, 6];
        cpu.run(&program);
        assert_eq!(cpu.rbx, 1);
    }
    {
        cpu.init(10, 0, 0);
        let program = vec![5, 0, 5, 1, 5, 4];
        cpu.run(&program);
        assert_eq!(*cpu.output, vec![0, 1, 2]);
    }
    {
        cpu.init(2024, 0, 0);
        let program = vec![0, 1, 5, 4, 3, 0];
        cpu.run(&program);
        assert_eq!(*cpu.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(cpu.rax, 0);
    }
    {
        cpu.init(0, 29, 0);
        let program = vec![1, 7];
        cpu.run(&program);
        assert_eq!(cpu.rbx, 26);
    }
    {
        cpu.init(0, 2024, 43690);
        let program = vec![4, 0];
        cpu.run(&program);
        assert_eq!(cpu.rbx, 44354);
    }
    {
        let program = load_program(_EXAMPLE1, &mut cpu);
        cpu.run(&program);
        assert_eq!(output, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }
}

#[test]
fn test_website_example_part2() {
    let mut output = Vec::new();
    let mut cpu = CPU::new(&mut output);

    let program = load_program(_EXAMPLE2, &mut cpu);

    let mut rax = 1;
    let rbx = cpu.rbx;
    let rcx = cpu.rcx;

    loop {
        cpu.init(rax, rbx, rcx);
        cpu.run(&program);
        if *cpu.output == program {
            break;
        }
        rax += 1;
    }
    assert_eq!(rax, 117440);
}
