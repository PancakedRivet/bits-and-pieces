use std::fs;

fn main() {
    // Open the input file
    let file_path = r"/Users/patrick/Code/bits-and-pieces/AdventOfCode22/Puzzle6/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // The signal is all in one line
    let signal = contents.lines().next().unwrap();
    let signal_chars: Vec<char> = signal.chars().collect();

    let part_1_index = get_start_of_signal_index(&signal_chars, 4);
    let part_2_index = get_start_of_signal_index(&signal_chars, 14);
    
    println!("Part 1: The signal starts after character: [{part_1_index}]");
    println!("Part 2: The message starts after character: [{part_2_index}]");
}

fn get_start_of_signal_index(signal: &Vec<char>, window_size: usize) -> usize {
    let mut index: usize = 0;
    let mut window_values: Vec<char> = Vec::new();

    for window in signal.windows(window_size) {
        
        window_values.push(window[0]);
        for i in 1..window_size {
            if !window_values.contains(&window[i]) {
                window_values.push(window[i]);
            }
        }

        if window_values.len() >= window_size {
            break;
        } else {
            window_values.clear();
            index += 1;
        }
    };
    index += window_size; // Accounting for the unique characters before the start of the signal
    return index;
}