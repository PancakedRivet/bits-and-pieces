use std::fs;

// https://adventofcode.com/2022/day/11

struct Monkey {
    items: Vec<i32>,
    operation: Box<dyn Fn(i32) -> i32>,
    test_divisor: i32,
    test_true: i32,
    test_false: i32
}

fn main() {
    // Open the input file
    // let file_path: &str = r"/Users/patrick/Code/bits-and-pieces/AdventOfCode22/Puzzle11/input2.txt";
    // let contents: String = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let monkeys = monkeys_input2();
}

fn monkeys_input2() -> Vec<Monkey> {
    let monkey0: Monkey = Monkey {
        items: vec![79, 98],
        operation: Box::new(|x| x * 19),
        test_divisor: 23,
        test_true: 2,
        test_false: 3
    };
    let monkey1: Monkey = Monkey {
        items: vec![79, 98],
        operation: Box::new(|x| x * 19),
        test_divisor: 19,
        test_true: 2,
        test_false: 3
    };
    let monkey2: Monkey = Monkey {
        items: vec![79, 98],
        operation: Box::new(|x| x * 19),
        test_divisor: 13,
        test_true: 2,
        test_false: 3
    };
    let monkey3: Monkey = Monkey {
        items: vec![79, 98],
        operation: Box::new(|x| x * 19),
        test_divisor: 17,
        test_true: 2,
        test_false: 3
    };
    let monkeys: Vec<Monkey> = vec![monkey0,monkey1,monkey2,monkey3];
    
    monkeys
}