use std::fs;

// https://adventofcode.com/2022/day/8

fn main() {
    // Open the input file
    let file_path: &str = r"/Users/patrick/Code/bits-and-pieces/AdventOfCode22/Puzzle8/input.txt";
    let contents: String = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let rev_contents: String = contents.clone().chars().rev().collect();

    let mut file_lines = contents.lines();
    let mut rev_file_lines = rev_contents.lines();

    let mut combined_visible_trees_vec = Vec::<Vec<bool>>::new();
    let mut all_visible_trees_vec = Vec::<Vec<bool>>::new();
    let mut rev_all_visible_trees_vec = Vec::<Vec<bool>>::new();

    // Get the first line as this is used to create the initial vector or tree heights as seen from the top
    let first_line = file_lines.next().unwrap();
    let mut tree_height_vec_top: Vec<i32> = top_boundary_trees(first_line);
    all_visible_trees_vec.push(vec![true; tree_height_vec_top.len()]);
    combined_visible_trees_vec.push(vec![false; tree_height_vec_top.len()]);

    // Get the first line as this is used to create the initial vector or tree heights as seen from the top
    let rev_first_line = rev_file_lines.next().unwrap();
    let mut tree_height_vec_bottom: Vec<i32> = top_boundary_trees(rev_first_line);
    rev_all_visible_trees_vec.push(vec![true; tree_height_vec_bottom.len()]);

    for line in file_lines {
        let visibile_tree_vec = calculate_visible_trees(line, &mut tree_height_vec_top);
        all_visible_trees_vec.push(visibile_tree_vec);
        combined_visible_trees_vec.push(vec![false; tree_height_vec_top.len()]);
    }

    for line in rev_file_lines {
        let visibile_tree_vec = calculate_visible_trees(line, &mut tree_height_vec_bottom);
        rev_all_visible_trees_vec.push(visibile_tree_vec);
    }

    println!("Dimensions: {:} x {:}", combined_visible_trees_vec.len(), combined_visible_trees_vec[0].len());
    
    for i in 0..all_visible_trees_vec.len() {
        for j in 0..all_visible_trees_vec[0].len() {
            let top_left_visible = all_visible_trees_vec[i][j];
            // println!("top_left_visible: ({:},{:}) = {}", i, j, top_left_visible);

            let rev_i = all_visible_trees_vec.len() - i - 1;
            let rev_j = all_visible_trees_vec[0].len() - j - 1;
            let bottom_right_visible = rev_all_visible_trees_vec[rev_i][rev_j];
            // println!("bottom_right_visible: ({:},{:}) = {}", rev_i, rev_j, bottom_right_visible);
            if top_left_visible || bottom_right_visible {
                combined_visible_trees_vec[i][j] = true;
            }
        }
    }

    // println!("{:?}", all_visible_trees_vec);
    // println!("{:?}", rev_all_visible_trees_vec);
    // println!("{:?}", combined_visible_trees_vec);

    let mut visibile_tree_count_total: i32 = 0;

    for tree_line in combined_visible_trees_vec {
        let total_true = tree_line.into_iter().filter(|b| *b).count();
        visibile_tree_count_total += total_true as i32;
    }

    println!("There are {:} visible trees in total", visibile_tree_count_total);

}

fn top_boundary_trees(first_line: &str) -> Vec<i32> {
    let first_line_split: Vec<&str> = first_line.split("").collect();
    // filter out the empty string vec elements (artifact of the split("") call)
    let first_line_split_not_empty = first_line_split.iter().filter(|x| !x.is_empty()).copied();

    let mut tree_height_vec_top = vec![-1; first_line.len()];

    // Trees on the boundary are always seen, so add every tree as visible form the top
    for (i, tree_height) in first_line_split_not_empty.enumerate() {
        let tree_height: i32 = tree_height.parse::<i32>().unwrap();

        if tree_height > tree_height_vec_top[i] { // Can the tree be seen from the top?
            tree_height_vec_top[i] = tree_height;
        }
    }

    tree_height_vec_top
}

fn calculate_visible_trees(line: &str, tree_height_vec_top: &mut Vec<i32>) -> Vec<bool> {
    let mut max_tree_height_left: i32 = -1;

    let mut tree_height_is_visible_vec = vec![false; tree_height_vec_top.len()];

    let line_split: Vec<&str> = line.split("").collect();
    // filter out the empty string vec elements (artifact of the split("") call)
    let line_split_not_empty = line_split.iter().filter(|x| !x.is_empty()).copied();

    for (i, tree_height) in line_split_not_empty.enumerate() {

        let tree_height: i32 = tree_height.parse::<i32>().unwrap();

        if tree_height > tree_height_vec_top[i] { // Can the tree be seen from the top?
            tree_height_vec_top[i] = tree_height;
            tree_height_is_visible_vec[i] = true;
        } 
        if tree_height > max_tree_height_left { // Can the tree be seen from the left?
            max_tree_height_left = tree_height;
            tree_height_is_visible_vec[i] = true;
        } 
    }

    tree_height_is_visible_vec
}
