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

    let mut common_letters_part_1 = Vec::new();
    let mut common_letters_part_2 = Vec::new();
    let mut rucksacks = Vec::new();

    for line in contents.lines() {
        let compartment_size: usize = line.len() / 2;
        let (compartment_1, compartment_2) = line.split_at(compartment_size);

        // For each line, find the common letter for Part 1
        'outer: for compartment_1_char in compartment_1.chars() {
            for compartment_2_char in compartment_2.chars() {
                if compartment_1_char == compartment_2_char {
                    common_letters_part_1.push(compartment_1_char);
                    break 'outer;
                }
            }
        }

        // For every 3rd line, calculate the common letter for the last 3 lines for Part 2
        rucksacks.push(line);
        if rucksacks.len() >= 3 {
            'outer: for rucksack1_char in rucksacks[0].chars() {
                for rucksack2_char in rucksacks[1].chars() {
                    // Only continue matching if the first 2 characters match
                    if rucksack1_char == rucksack2_char {
                        for rucksack3_char in rucksacks[2].chars() {
                            if rucksack1_char == rucksack3_char {
                                common_letters_part_2.push(rucksack1_char);
                                break 'outer;
                            }
                        }
                    }
                }
            }
            rucksacks.clear();
        }
    }

    // Puzzle 3: Part 1: https://adventofcode.com/2022/day/3
    let letter_priority_value_part_1 = calculate_priority_values(common_letters_part_1);
    println!(
        "letter_priority_value_part_1: [{:?}]",
        letter_priority_value_part_1
    );

    // Puzzle 3: Part 2: https://adventofcode.com/2022/day/3#part2
    let letter_priority_value_part_2 = calculate_priority_values(common_letters_part_2);
    println!(
        "letter_priority_value_part_2: [{:?}]",
        letter_priority_value_part_2
    );
}

fn calculate_priority_values(common_letters: Vec<char>) -> usize {
    let mut letter_priority_value: usize = 0;
    let mut index: usize;
    for common_letter in common_letters {
        if common_letter.is_uppercase() {
            index = ASCII_UPPER
                .iter()
                .position(|&r| r == common_letter)
                .unwrap()
                + 26 // Accounting for the lower case letters being 1-26
                + 1; // Accounting for aray indexing starting at 0
        } else {
            index = ASCII_LOWER
                .iter()
                .position(|&r| r == common_letter)
                .unwrap()
                + 1;
        }
        letter_priority_value += index;
    }
    return letter_priority_value;
}
