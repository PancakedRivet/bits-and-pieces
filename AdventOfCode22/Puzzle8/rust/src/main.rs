use std::fs;

fn main() {
    // Open the input file
    let file_path: &str = r"/Users/patrick/Code/bits-and-pieces/AdventOfCode22/Puzzle8/input2.txt";
    let contents: String = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut visibile_tree_count_total: i32 = 0;

    for line in contents.lines() {
        println!("{:}", line);
        let split_line_left = line.split("");
        let split_line_right = line.rsplit("");

        let mut max_tree_height: i32 = 0;
        let mut visibile_tree_count: i32 = 0;

        //  Searching forwards through the iterator:
        for tree in split_line_left {
            if tree == "" {
                continue;
            }

            // println!("{}", tree);

            let tree_height: i32 = tree.parse::<i32>().unwrap();
            if tree_height > max_tree_height {
                visibile_tree_count += 1;
                max_tree_height = tree_height;
            } 
            
        }

        visibile_tree_count_total += visibile_tree_count;

        println!("There are {:} visible trees on this line", visibile_tree_count)
        
    };

    println!("There are {:} visible trees in total", visibile_tree_count_total)

}

/* Ideas:
- Create a vector of booleans (false) for tree visibility
- split the string then collect it into a vector
- iterate the vector (collection needed for iterating forwards and backwards)
    - mark any visbile trees as true in the boolean vector (only needs to be visible form one side)
- sum the number of "true" values in the vector to get the total visible trees for the line

- TODO: do something for the columns (above and below)
    - possibly have a vector of the highest trees encountered while iterating down the input list
        - if the current column is taller, count it as visible and update it.

- include the trees on the outer perimeter in the calculation for total visible trees.
*/