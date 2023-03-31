use std::fs;
use std::iter::zip;

// https://adventofcode.com/2022/day/8

enum Direction {
    Top,
    Bottom,
    Left,
    Right
}

fn main() {
    // Open the input file
    let file_path: &str = r"/Users/patrick/Code/bits-and-pieces/AdventOfCode22/Puzzle8/input.txt";
    let contents: String = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let rev_contents: String = contents.clone().chars().rev().collect();

    let mut all_visible_trees_vec = create_visible_tree_vec(&contents);
    let rev_all_visible_trees_vec = create_visible_tree_vec(&rev_contents);

    println!("Dimensions: {:} x {:}", all_visible_trees_vec.len(), all_visible_trees_vec[0].len());
    
    for i in 0..all_visible_trees_vec.len() {
        for j in 0..all_visible_trees_vec[0].len() {
            let top_left_visible = all_visible_trees_vec[i][j];
            // println!("top_left_visible: ({:},{:}) = {}", i, j, top_left_visible);

            // Continue to the next tree if it's already visible
            if top_left_visible {
                continue;
            }

            let rev_i = all_visible_trees_vec.len() - i - 1;
            let rev_j = all_visible_trees_vec[0].len() - j - 1;
            let bottom_right_visible = rev_all_visible_trees_vec[rev_i][rev_j];
            // println!("bottom_right_visible: ({:},{:}) = {}", rev_i, rev_j, bottom_right_visible);

            if bottom_right_visible {
                all_visible_trees_vec[i][j] = true;
            }
        }
    }

    // println!("{:?}", all_visible_trees_vec);

    let mut visibile_tree_count_total: i32 = 0;

    // Calculate the number of visible trees in each line
    for tree_line in all_visible_trees_vec {
        let total_true = tree_line.into_iter().filter(|b| *b).count();
        visibile_tree_count_total += total_true as i32;
    }

    println!("Part 1: There are {:} visible trees in total", visibile_tree_count_total);


    // Part 2: https://adventofcode.com/2022/day/8#part2

    let mut all_trees_vec = Vec::<Vec<i32>>::new();
    let mut rev_all_trees_vec = Vec::<Vec<i32>>::new();

    let file_iter = zip(contents.lines(), rev_contents.lines());

    for (line, rev_line) in file_iter {
        let line_vec = create_int_height_vec_from_str(line);
        all_trees_vec.push(line_vec);

        let rev_line_vec = create_int_height_vec_from_str(rev_line);
        rev_all_trees_vec.push(rev_line_vec);
    }

    // println!("{:?}", all_trees_vec);

    let mut max_scenic_value: i32 = 0;

    let max_i = all_trees_vec.len();
    let max_j = all_trees_vec[0].len();

    for i in 0..max_i {
        for j in 0..max_j {

            let rev_i = all_trees_vec.len() - i - 1;
            let rev_j = all_trees_vec[0].len() - j - 1;

            // Find vec of tree heights to the right of the value
            let tree_counter_right: usize = calculate_scenic_score(&all_trees_vec, j, max_j, i, Direction::Right);
            let tree_counter_bottom: usize = calculate_scenic_score(&all_trees_vec, i, max_i, j, Direction::Bottom);
            let tree_counter_left: usize = calculate_scenic_score(&rev_all_trees_vec, rev_j, max_j, rev_i, Direction::Left);
            let tree_counter_top: usize = calculate_scenic_score(&rev_all_trees_vec, rev_i, max_i, rev_j, Direction::Top);

            let tree_scenic_value = tree_counter_right as i32 * tree_counter_bottom as i32 * tree_counter_left as i32 * tree_counter_top as i32;

            // Indexing into the 2d vec looks backwards because the second index is the row index, not the column index (based on how it was pushed)
            // println!("Tree ({},{}): height={}, [r,b,t,l]=[{},{},{},{}], total={}", j, i, all_trees_vec[i][j], tree_counter_right, tree_counter_bottom, tree_counter_left, tree_counter_top, tree_scenic_value);

            if tree_scenic_value > max_scenic_value {
                max_scenic_value = tree_scenic_value;
            }
        }
    }
    println!("Part 2: Scenic value maximum: {}", max_scenic_value);

}

fn create_visible_tree_vec(contents: &String) -> Vec<Vec<bool>> {
    let mut visible_trees_vec = Vec::<Vec<bool>>::new();

    let mut file_lines = contents.lines();
    
    // Get the first line as this is used to create the initial vector or tree heights as seen from the top
    let first_line = file_lines.next().unwrap();
    let mut tree_height_vec_top: Vec<i32> = top_boundary_trees(first_line);
    visible_trees_vec.push(vec![true; tree_height_vec_top.len()]);

    for line in file_lines {
        //.ok().unwrap().as_str()
        let visibile_tree_vec = calculate_visible_trees(line, &mut tree_height_vec_top);
        visible_trees_vec.push(visibile_tree_vec);
    }

    visible_trees_vec
}

fn create_int_height_vec_from_str(line: &str) -> Vec<i32> {
    let line_split: Vec<&str> = line.split("").collect();
    // filter out the empty string vec elements (artifact of the split("") call)
    let line_split_not_empty = line_split.iter().filter(|x| !x.is_empty()).copied();
    let line_split_values = line_split_not_empty.map(|x| x.parse::<i32>().unwrap()).collect();
    line_split_values
}

fn calculate_scenic_score(all_trees_vec: &Vec<Vec<i32>>, counter: usize, max_counter: usize, fixed_dimension: usize, direction: Direction) -> usize {
    
    let starting_tree_height = match direction {
        Direction::Top | Direction::Bottom => all_trees_vec[counter][fixed_dimension],
        Direction::Left | Direction::Right => all_trees_vec[fixed_dimension][counter],
    };
    
    // Find the number of shorter trees in one direction
    let mut tree_counter: usize = 0;
    if counter < max_counter {
        for k in counter+1..max_counter {

            let tree_height = match direction {
                Direction::Top | Direction::Bottom => all_trees_vec[k][fixed_dimension],
                Direction::Left | Direction::Right => all_trees_vec[fixed_dimension][k],
            };

            // println!("starting_tree_height={}, tree_height={}, counter={}, max_counter={}, k={}", starting_tree_height, tree_height, counter, max_counter, k);
            if tree_height >= starting_tree_height {
                tree_counter = k - counter;
                // println!("BREAK starting_tree_height={}, tree_height={}, counter={}, max_counter={}, k={}", starting_tree_height, tree_height, counter, max_counter, k);
                break;
            }
            if k == max_counter - 1 {
                tree_counter = k - counter;
            }
        }
    }
    tree_counter
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
