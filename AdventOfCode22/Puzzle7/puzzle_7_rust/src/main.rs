use std::fs;
use std::collections::HashMap;

fn main() {
    // Open the input file
    let file_path = r"/Users/patrick/Code/bits-and-pieces/AdventOfCode22/Puzzle7/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut current_dir_path: Vec<String> = Vec::new();
    let mut dir_sizes: HashMap<String, i32> = HashMap::new();

    for line in contents.lines() {
        let line_split = line.split_whitespace();
        let vec = line_split.collect::<Vec<&str>>();
        let command_1: String = vec[0].parse::<String>().unwrap();
        let command_2: String = vec[1].parse::<String>().unwrap();

        // println!("{line}");
        match (command_1.as_str(), command_2.as_str()) {
            ("$", "cd") => {
                let new_dir: String = vec[2].parse::<String>().unwrap();
                if new_dir == ".." {
                    current_dir_path.pop();
                } else {
                    current_dir_path.push(new_dir.clone());
                    if !dir_sizes.contains_key(&new_dir) {
                        dir_sizes.insert(new_dir, 0);
                    }
                    
                }
                // println!("Current: {:?}", &current_dir_path);
            },
            ("$", "ls") | ("dir", _) => (), // We don't care about when these commads are used
            _ => {
                // Only remaining match arm is where a file size is listed
                // println!("Before: {:?}", &dir_sizes);
                let file_size = command_1.parse::<i32>().unwrap();
                for dir in &current_dir_path {
                    dir_sizes.entry(dir.to_string()).and_modify(|size| *size += file_size);
                    // println!("Updating [{dir}] with [{file_size}]")
                }
                // println!("After: {:?}", &dir_sizes);
            }
        };
        // println!();
    };

    //println!("{:?}", &dir_sizes);
    let mut dir_sum = 0;
    for (_dir, size) in &dir_sizes {
        if *size <= 100000 {
            dir_sum += size;
            //println!("dir: [{dir}], size: [{size}], total size: [{dir_sum}]")
        }
    }  

    println!("Part 1: total size of directories < 100000 = [{dir_sum}]");

}
