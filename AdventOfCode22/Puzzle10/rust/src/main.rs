use std::fs;

// https://adventofcode.com/2022/day/10

fn main() {
    // Open the input file
    let file_path: &str = r"/Users/patrick/Code/bits-and-pieces/AdventOfCode22/Puzzle10/input.txt";
    let contents: String = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut signal_strength_total = 0;

    let mut current_cycle = 1;
    let mut current_register_value = 1;

    let mut is_performing_op = false;
    let mut op_value = 0;

    let mut lines = contents.lines();

    // Part 2:
    let mut screen_line: String = "".to_string();

    loop {

        // Part 2: "Rendering" to a line
        {
            // Determining what to print:
            let sprite_range: i32 = ((current_cycle - 1) % 40) - current_register_value;
            // println!("{}, {}, {}", current_cycle, current_cycle % 40, current_register_value);
            if  sprite_range.abs() <= 1 {
                screen_line = screen_line.to_owned() + "#";
            } else {
                screen_line = screen_line.to_owned() + ".";
            }

            // Print to the screen on every 40th cycle
            if current_cycle % 40 == 0 {
                println!("{}", screen_line);
                screen_line = "".to_string();
            }
        }

        // Part 1: 
        
        // Check the signal strength every 40 cycles (after 20)
        if (current_cycle + 20) % 40 == 0 {
            let signal_strength = current_cycle * current_register_value;
            signal_strength_total += signal_strength;
            // println!("Cycle: {}, Register: {}, Signal Strength: {}, Total: {}", current_cycle, current_register_value, signal_strength, signal_strength_total);
        }

        if !is_performing_op {

            // Stop when there are no more input lines to process
            let next_line = match lines.next() {
                None => break,
                Some(file_line) => file_line
            };

            let mut line_split = next_line.split_whitespace();

            let operation = line_split.next().unwrap();

            match operation {
                "noop" => (),
                "addx" => {
                    is_performing_op = true;
                    op_value = line_split.next().unwrap().parse::<i32>().unwrap();
                },
                _ => ()
            }
        } else {
            current_register_value = current_register_value + op_value;
            op_value = 0;
            is_performing_op = false;
        }

        // println!("Cycle: {}, CPU: {}, is_performing_op: {}, op_value: {}", current_cycle, current_register_value, is_performing_op, op_value);

        current_cycle += 1;
    }

    println!("Part 1: Signal strength sum: {}", signal_strength_total)

}