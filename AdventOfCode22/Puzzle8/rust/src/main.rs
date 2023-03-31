use std::fs;

fn main() {
    // Open the input file
    let file_path: &str = r"/Users/patrick/Code/bits-and-pieces/AdventOfCode22/Puzzle8/input2.txt";
    let contents: String = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut file_lines = contents.lines();

    // Get the first line as this is used to create the initial vector or tree heights as seen from the top
    let first_line = file_lines.next().unwrap();
    println!("{:}", first_line);

    let mut visibile_tree_count_total: i32 = 0;
    let mut tree_height_vec_top: Vec<i32> = top_boundary_trees(&mut visibile_tree_count_total, first_line);

    for line in file_lines {
        println!("{:}", line);

        let mut visible_tree_count = 0;

        let mut max_tree_height_left: i32 = -1;

        // let split_line_left = line.split("");
        // let split_line_right = line.rsplit("");

        let line_split: Vec<&str> = line.split("").collect();

        // filter out the empty string vec elements (artifact of the split("") call)
        let line_split_not_empty = line_split.iter().filter(|x| !x.is_empty()).copied();

        // let line_vec: Vec<bool> = vec![false; line.len()];

        for (i, tree_height) in line_split_not_empty.enumerate() {

            let tree_height: i32 = tree_height.parse::<i32>().unwrap();
            
            if tree_height > max_tree_height_left { // Can the tree be seen from the left?
                println!("{} > {} Left", tree_height, max_tree_height_left);
                visible_tree_count += 1;
                max_tree_height_left = tree_height;
            } else if tree_height > tree_height_vec_top[i] { // Can the tree be seen from the top?
                println!("{} > {} Top", tree_height, tree_height_vec_top[i]);
                visible_tree_count += 1;
                tree_height_vec_top[i] = tree_height;
            } else {
                println!("{}", tree_height);
            }
        }

        println!("{} trees visible", visible_tree_count);

        visibile_tree_count_total += visible_tree_count
    };

    println!("There are {:} visible trees in total", visibile_tree_count_total)

}

fn top_boundary_trees(visibile_tree_count_total: &mut i32, first_line: &str) -> Vec<i32> {
    let first_line_split: Vec<&str> = first_line.split("").collect();
    // filter out the empty string vec elements (artifact of the split("") call)
    let first_line_split_not_empty = first_line_split.iter().filter(|x| !x.is_empty()).copied();

    let mut tree_height_vec_top = vec![-1; first_line.len()];

    // Trees on the boundary are always seen, so add every tree as visible form the top
    for (i, tree_height) in first_line_split_not_empty.enumerate() {
        let tree_height: i32 = tree_height.parse::<i32>().unwrap();

        if tree_height > tree_height_vec_top[i] { // Can the tree be seen from the top?
            println!("{} > {} Top", tree_height, tree_height_vec_top[i]);
            *visibile_tree_count_total += 1;
            tree_height_vec_top[i] = tree_height;
        }
    }

    println!("{} trees visible", visibile_tree_count_total);

    tree_height_vec_top
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