use std::fs;
use std::collections::HashMap;

fn main() {
    // Open the input file
    let file_path = r"/Users/patrick/Code/bits-and-pieces/AdventOfCode22/Puzzle7/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut current_dir_path: Vec<String> = Vec::new();
    let mut dir_sizes: HashMap<String, i32> = HashMap::new();

    static HOME_FOLDER: &str = "/home"; // Easier than just "/" in the path

    for line in contents.lines() {
        let line_split = line.split_whitespace();
        let vec = line_split.collect::<Vec<&str>>();
        let command_1: String = vec[0].parse::<String>().unwrap();
        let command_2: String = vec[1].parse::<String>().unwrap();

        match (command_1.as_str(), command_2.as_str()) {
            ("$", "cd") => {
                let mut new_dir: String = vec[2].parse::<String>().unwrap();
                if new_dir == ".." {
                    current_dir_path.pop();
                } else {
                    if new_dir == "/" {
                        new_dir = HOME_FOLDER.to_string();
                    } else {
                        new_dir = current_dir_path.last().unwrap().to_owned() + "/" + &new_dir;
                    }
                    current_dir_path.push(new_dir.clone());
                    if !dir_sizes.contains_key(&new_dir) {
                        dir_sizes.insert(new_dir.clone(), 0);
                    }
                }
            },
            ("$", "ls") | ("dir", _) => (), // We don't care about when these commads are used
            _ => {
                // Only remaining match arm is where a file size is listed
                let file_size = command_1.parse::<i32>().unwrap();
                for i in 0..current_dir_path.len() {
                    dir_sizes.entry(current_dir_path[i].to_string()).and_modify(|size| *size += file_size);
                }
            }
        };
    };

    let mut dir_sum: i32 = 0;

    let space_available: i32 = 70000000;
    let space_required: i32 = 30000000;
    let space_used: &i32 = dir_sizes.get(HOME_FOLDER).unwrap();
    let free_space: i32 = space_available - space_used;
    let mut directory_size_to_delete: &i32 = &space_available;

    for (_, size) in &dir_sizes {
        // Calculation for Part 1
        if *size <= 100000 {
            dir_sum += size;
        };
        // Calculation for Part 2
        if free_space + size >= space_required {
            if size < &directory_size_to_delete {
                directory_size_to_delete = size;
            }
        }
    };

    println!("Part 1: Total size of directories < 100000 = [{dir_sum}]");
    println!("Part 2: Size of directory to delete = [{directory_size_to_delete}]");
}
