use std::collections::HashMap;
use std::fs;

fn main() {
    let points_from_shape: HashMap<&str, i32> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let points_from_game: HashMap<&str, i32> = HashMap::from([
        ("A X", 3),
        ("A Y", 6),
        ("A Z", 0),
        ("B X", 0),
        ("B Y", 3),
        ("B Z", 6),
        ("C X", 6),
        ("C Y", 0),
        ("C Z", 3),
    ]);

    let points_from_game_with_correction: HashMap<&str, i32> = HashMap::from([
        ("A X", 3 + 0),
        ("A Y", 1 + 3),
        ("A Z", 2 + 6),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 2 + 0),
        ("C Y", 3 + 3),
        ("C Z", 1 + 6),
    ]);

    // Open the input file
    let file_path = r"D:\Git\bits-and-pieces\AdventOfCode22\Puzzle2\input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut round_shape_points: i32;
    let mut round_game_points: i32;

    let mut total_points_won: i32 = 0;
    let mut corrected_total_points_won: i32 = 0;

    for line in contents.lines() {
        // Splitting the line to get the points for choosing a shape
        let mut iter = line.split_whitespace();
        iter.next(); // Skip the first value, it's unused
        let chosen_shape = iter.next().unwrap();

        round_shape_points = *points_from_shape.get(&chosen_shape).unwrap();
        round_game_points = *points_from_game.get(&line).unwrap();

        total_points_won += round_game_points + round_shape_points;

        // Adding the points correction for Part Two:
        corrected_total_points_won += points_from_game_with_correction.get(&line).unwrap();
    }

    // Puzzle 2: Part 1: https://adventofcode.com/2022/day/2
    println!(
        "Expected score from the strategy guide = [{}] points.",
        total_points_won
    );

    // Puzzle 2: Part 2: https://adventofcode.com/2022/day/2#part2
    println!(
        "Expected score from the *corrected* strategy guide = [{}] points.",
        corrected_total_points_won,
    );
}
