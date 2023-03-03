use std::fs;
use std::collections::HashMap;

fn main() {
    // Open the input file
    let file_path = r"/Users/patrick/Code/bits-and-pieces/AdventOfCode22/Puzzle7/example.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut current_dir_path: Vec<String> = Vec::new();
    let mut dir_sizes: HashMap<String, i32> = HashMap::new();
    let mut is_in_ls: bool = false;

    for line in contents.lines() {
        let line_split = line.split_whitespace();
        let vec = line_split.collect::<Vec<&str>>();

        let command_1: String = vec[0].parse::<String>().unwrap();

        if command_1 == "$" {
            is_in_ls = false;

            let command_2: String = vec[1].parse::<String>().unwrap();

            // Logic for changing where in the directory structure we are
            if command_2 == "cd" {
                let new_dir: String = vec[2].parse::<String>().unwrap();
                if new_dir == ".." {
                    current_dir_path.pop();
                } else {
                    current_dir_path.push(new_dir.clone());

                    if !dir_sizes.contains_key(&new_dir) {
                        dir_sizes.insert(new_dir, 0);
                    };
                }
            } else if command_2 == "ls" { 
                is_in_ls = true;
            }

        } else {
            if command_1 != "dir" {
                let file_size = command_1.parse::<i32>().unwrap();

                if is_in_ls {

                    for dir in &current_dir_path {
                        dir_sizes.entry(dir.to_string()).and_modify(|size| *size += file_size);
                    }
                }
            } 
        }
    };

    //println!("{:?}", &dir_sizes);

    let mut hash_vec: Vec<(&String, &i32)> = dir_sizes.iter().collect();
    //println!("{:?}", hash_vec);
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("Sorted: {:?}", hash_vec);

    let mut dir_sum = 0;
    for (dir, size) in &dir_sizes {
        if size <= &100000 {
            dir_sum += size;
            println!("dir: [{dir}], size: [{size}], total size: [{dir_sum}]")
        }
    }  

    println!("Part 1: total size of directories < 100000 = [{dir_sum}]");

}