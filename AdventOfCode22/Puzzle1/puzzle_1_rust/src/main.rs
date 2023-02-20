use std::fs;

fn main() {
    // Open the input file
    let file_path = r"D:\Git\bits-and-pieces\AdventOfCode22\Puzzle1\input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut calorie_value: u32 = 0;
    let mut calorie_list: Vec<u32> = Vec::new();

    for line in contents.lines() {
        match line.parse::<u32>() {
            Ok(n) => {
                // Add the value of the line to the current calorie value
                calorie_value += n;
            }
            _ => {
                // If the line was not able to be parsed to an int, then it must be an empty line
                calorie_list.push(calorie_value);
                // Reset the calorie counter
                calorie_value = 0;
            }
        }
    }

    // Sort Ascending
    calorie_list.sort();
    // Sort Descending (easier to slice)
    calorie_list.reverse();

    // Part One
    println!("Part One: Calories of highest elf: {:?}", calorie_list[0]);

    // Part Two
    let sum: u32 = calorie_list[0..=2].iter().sum();
    println!("Part Two: Calories of top 3 elves: {:?}", sum);
}
