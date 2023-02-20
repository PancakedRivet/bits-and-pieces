use std::collections::HashSet;
use std::fs;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

static ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn main() {
    // Open the input file
    let file_path = r"D:\Git\bits-and-pieces\AdventOfCode22\Puzzle3\input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let letter_priority_value: i32 = 0;

    for line in contents.lines() {
        let compartment_size: usize = line.len() / 2;
        let (compartment_1, compartment_2) = line.split_at(compartment_size);

        let set: HashSet<char> = compartment_1.chars().collect();

        //TODO: Convert this from a bool to a character
        let common_letter = compartment_2.chars().any(|c| set.contains(&c));
        println!("common_letter: [{}]", common_letter);
    }
}
