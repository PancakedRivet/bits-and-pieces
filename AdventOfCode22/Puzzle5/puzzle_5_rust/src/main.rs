use std::fs;

fn main() {
    // Open the input file
    let file_path = r"/Users/patrick/Code/bits-and-pieces/AdventOfCode22/Puzzle5/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut crates: Vec<Vec<&str>> = create_stack();
    let mut crates_part2: Vec<Vec<&str>> = create_stack();

    for line in contents.lines() {
        let line_split = line.split_whitespace();
        let vec = line_split.collect::<Vec<&str>>();

        let quantity: usize = vec[1].parse::<usize>().unwrap();
        // -1 to account for the stack starting at 1 but index starting at 0
        let from_stack_id: usize = vec[3].parse::<usize>().unwrap() - 1;
        let to_stack_id: usize = vec[5].parse::<usize>().unwrap() - 1;

        crates = move_crate(crates, from_stack_id, to_stack_id, quantity);
        crates_part2 = move_crate_group(crates_part2, from_stack_id, to_stack_id, quantity);
    };

    // Puzzle 5: https://adventofcode.com/2022/day/5
    let crate_string = generate_crate_string(crates);
    println!("Part 1 Answer: {crate_string}");

    // Puzzle 5: Part 2: https://adventofcode.com/2022/day/5#part2
    let crate_string_part2 = generate_crate_string(crates_part2);
    println!("Part 2 Answer: {crate_string_part2}");
}

/* Starting position of crates:
    [P]                 [Q]     [T]
[F] [N]             [P] [L]     [M]
[H] [T] [H]         [M] [H]     [Z]
[M] [C] [P]     [Q] [R] [C]     [J]
[T] [J] [M] [F] [L] [G] [R]     [Q]
[V] [G] [D] [V] [G] [D] [N] [W] [L]
[L] [Q] [S] [B] [H] [B] [M] [L] [D]
[D] [H] [R] [L] [N] [W] [G] [C] [R]
 1   2   3   4   5   6   7   8   9 
*/
fn create_stack() -> Vec<Vec<&'static str>> {
    // Create the stacks
    let mut crates: Vec<Vec<&str>> = Vec::new();
    let stack_1: Vec<&str> = vec!["D", "L", "V", "T", "M", "H", "F"];
    let stack_2: Vec<&str> = vec!["H", "Q", "G", "J", "C", "T", "N", "P"];
    let stack_3: Vec<&str> = vec!["R", "S", "D", "M", "P", "H"];
    let stack_4: Vec<&str> = vec!["L", "B", "V", "F"];
    let stack_5: Vec<&str> = vec!["N", "H", "G", "L", "O"];
    let stack_6: Vec<&str> = vec!["W", "B", "D", "G", "R", "M", "P"];
    let stack_7: Vec<&str> = vec!["G", "M", "N", "R", "C", "H", "L", "Q"];
    let stack_8: Vec<&str> = vec!["C", "L", "W"];
    let stack_9: Vec<&str> = vec!["R", "D", "L", "Q", "J", "Z", "M", "T"];
    crates.push(stack_1);
    crates.push(stack_2);
    crates.push(stack_3);
    crates.push(stack_4);
    crates.push(stack_5);
    crates.push(stack_6);
    crates.push(stack_7);
    crates.push(stack_8);
    crates.push(stack_9);

    return crates;
}

fn move_crate(mut crates: Vec<Vec<&'static str>>, from_stack_id: usize, to_stack_id: usize, quantity: usize ) -> Vec<Vec<&'static str>> {
    for _ in 0..quantity {
        let crate_being_moved: &str = crates[from_stack_id].pop().unwrap();
        crates[to_stack_id].push(crate_being_moved);
    }
    return crates;
}

fn move_crate_group(mut crates: Vec<Vec<&'static str>>, from_stack_id: usize, to_stack_id: usize, quantity: usize ) -> Vec<Vec<&'static str>> {
    let mut temp_stack: Vec<&str> = Vec::new();
    for _ in 0..quantity {
        let crate_being_moved: &str = crates[from_stack_id].pop().unwrap();
        temp_stack.push(crate_being_moved);
    }
    temp_stack.reverse(); // Order must be reversed to achieve LILO queue for moving crates
    crates[to_stack_id].append(&mut temp_stack);
    return crates;
}

fn generate_crate_string(crates: Vec<Vec<&'static str>>) -> String {
    let mut crate_string: String = "".to_string();
    for i in 0..crates.len() {
        crate_string += crates[i].last().unwrap();
    }
    return crate_string;
}