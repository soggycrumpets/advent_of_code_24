use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
// use std::thread::sleep;
// use std::time::{Duration, Instant};

const _INPUT: &str = "input.txt";
const _EXAMPLE1: &str = "part1_example.txt";
const _EXAMPLE2: &str = "part2_example.txt";

struct CPU<'a> {
    rax: i64,
    rbx: i64,
    rcx: i64,
    rip: i64,

    output: &'a mut Vec<i64>,
}
impl<'a> CPU<'a> {
    fn new(output: &'a mut Vec<i64>) -> CPU<'a> {
        CPU {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rip: 0,
            output: output,
        }
    }

    fn init(&mut self, rax: i64, rbx: i64, rcx: i64) {
        self.rax = rax;
        self.rbx = rbx;
        self.rcx = rcx;
        self.rip = 0;
        self.output.clear();
    }

    fn run(&mut self, program: &Vec<i64>) {
        while self.rip < program.len() as i64 {
            self.op(program);
        }
    }

    fn op(&mut self, program: &Vec<i64>) {
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

    fn combo_operand(&self, operand: i64) -> i64 {
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

    fn adv(&mut self, operand: i64) {
        let numerator = self.rax;
        let divisor: i64 = 1 << self.combo_operand(operand);
        let result = numerator / divisor;

        self.rax = result;
        self.rip += 2;
    }

    fn bxl(&mut self, operand: i64) {
        let result = self.rbx ^ operand;

        self.rbx = result;
        self.rip += 2;
    }

    fn bst(&mut self, operand: i64) {
        let result = self.combo_operand(operand) % 8;

        self.rbx = result;
        self.rip += 2;
    }

    fn jnz(&mut self, operand: i64) {
        if 0 != self.rax {
            self.rip = operand;
        } else {
            self.rip += 2;
        }
    }

    fn bxc(&mut self, _operand: i64) {
        let result = self.rbx ^ self.rcx;

        self.rbx = result;
        self.rip += 2;
    }

    fn out(&mut self, operand: i64) {
        let result = self.combo_operand(operand) % 8;

        self.output.push(result);
        self.rip += 2
    }

    fn bdv(&mut self, operand: i64) {
        let numerator = self.rax;
        let divisor: i64 = 1 << self.combo_operand(operand);
        let result = numerator / divisor;

        self.rbx = result;
        self.rip += 2;
    }

    fn cdv(&mut self, operand: i64) {
        let numerator = self.rax;
        let divisor: i64 = 1 << self.combo_operand(operand);
        let result = numerator / divisor;

        self.rcx = result;
        self.rip += 2;
    }
}

struct Controller<'a> {
    target: &'a Vec<i64>,
    guess: i64,
    current_digit: usize,
    digit_states: Vec<i64>,
}
impl<'a> Controller<'a> {
    fn new(target: &Vec<i64>) -> Controller {
        Controller {
            target: target,
            guess: 8_i64.pow(target.len() as u32 - 1),
            current_digit: target.len() - 1,
            digit_states: vec![0; target.len()],
        }
    }

    fn get_next_guess(&mut self, output: &Vec<i64>) {
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
    }

    fn current_digit_matches(&mut self, output: &Vec<i64>) -> bool {
        output[self.current_digit] == self.target[self.current_digit]
    }

    fn current_digit_is_maxed(&mut self) -> bool {
        self.digit_states[self.current_digit] == 8
    }

    fn reset_current_digit(&mut self) {
        self.guess -= self.digit_states[self.current_digit] * 8_i64.pow(self.current_digit as u32);
        self.digit_states[self.current_digit] = 0;
    }

    fn increment_digit(&mut self) {
        self.guess += 8_i64.pow(self.current_digit as u32);
        self.digit_states[self.current_digit] += 1;
    }

}

fn load_program<'a>(filename: &str, cpu: &mut CPU) -> Vec<i64> {
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
    let program: Vec<i64> = get_instructions_from_line(lines.next());

    return program;

    fn get_num_from_line(line: Option<&str>) -> i64 {
        line.unwrap()
            .chars()
            .filter(|char| char.is_numeric())
            .collect::<String>()
            .parse::<i64>()
            .unwrap()
    }
    fn get_instructions_from_line(line: Option<&str>) -> Vec<i64> {
        line.unwrap()
            .chars()
            .filter(|c| c.is_numeric() || *c == ',')
            .collect::<String>()
            .split(',')
            .map(|str| str.parse().unwrap())
            .collect()
    }
}

fn _print_output(output: &Vec<i64>) {
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
    let mut rax;

    // let interval = Duration::from_secs(1) / 30;
    // let mut next_time = Instant::now() + interval;

    loop {
        rax = controller.guess;

        cpu.init(rax, rbx, rcx);
        cpu.run(&program);
        _print_output(&cpu.output);
        print!(" | Current Digit: {}", controller.current_digit);
        println!(" | Register A: {}", controller.guess);

        if *cpu.output == program {
            break;
        }
        controller.get_next_guess(cpu.output);

        // sleep(next_time - Instant::now());
        // next_time += interval;
    }
    print!("\nProgram copies itself at: Register A = {}\n\n", rax);
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
    let program = load_program(_INPUT, &mut cpu);
    let mut controller: Controller = Controller::new(&program);
    controller.current_digit = program.len() - 1;

    cpu.run(&program);

    let rbx = cpu.rbx;
    let rcx = cpu.rcx;
    let mut rax;

    loop {
        rax = controller.guess;

        cpu.init(rax, rbx, rcx);
        cpu.run(&program);
        _print_output(&cpu.output);
        print!("\t| Current Digit: {}", controller.current_digit);
        println!("\t| Register A: {}", controller.guess);

        if *cpu.output == program {
            break;
        }
        controller.get_next_guess(cpu.output);
    }

    assert_eq!(rax, 117440);
}
