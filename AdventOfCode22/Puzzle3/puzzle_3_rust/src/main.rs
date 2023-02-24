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

    let mut letter_priority_value: usize = 0;

    let mut common_letters = Vec::new();

    for line in contents.lines() {
        let compartment_size: usize = line.len() / 2;
        let (compartment_1, compartment_2) = line.split_at(compartment_size);

        // For each line, find the common letter
        'outer: for compartment_1_char in compartment_1.chars() {
            for compartment_2_char in compartment_2.chars() {
                if compartment_1_char == compartment_2_char {
                    common_letters.push(compartment_1_char);
                    // println!("common_letter: [{}]", compartment_1_char);
                    break 'outer;
                }
            }
        }
    }

    // With the letters collected, tally up their values.
    let mut index;
    for common_letter in common_letters {
        if common_letter.is_uppercase() {
            index = ASCII_UPPER
                .iter()
                .position(|&r| r == common_letter)
                .unwrap()
                + 26
                + 1;
        } else {
            index = ASCII_LOWER
                .iter()
                .position(|&r| r == common_letter)
                .unwrap()
                + 1;
        }
        letter_priority_value += index;
    }
    println!("letter_priority_value: [{}]", letter_priority_value);
}
