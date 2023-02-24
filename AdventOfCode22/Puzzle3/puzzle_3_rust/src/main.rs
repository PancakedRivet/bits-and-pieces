use std::fs;

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

fn calculate_priority_values(common_letters: Vec<char>) -> u32 {
    let mut letter_priority_value: u32 = 0;
    let mut index: u32 = 0;
    for common_letter in common_letters {
        match common_letter {
            'a'..='z' => index = (common_letter as u32) - 97 + 1, // 'a' = ASCII 97, 'a' = puzzle code 1
            'A'..='Z' => index = (common_letter as u32) - 65 + 27, // 'A' = ASCII 65, 'a' = puzzle code 27
            _ => println!("common_letter = [{common_letter}] not matched!"),
        }
        letter_priority_value += index;
    }
    return letter_priority_value;
}
