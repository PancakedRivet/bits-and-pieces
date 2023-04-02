use std::fs;
use std::fmt;
use std::collections::HashSet;

// https://adventofcode.com/2022/day/9

struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn key(self: &Self) -> String {
        let key = format!("({},{})", self.x, self.y);
        key
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {

    // Open the input file
    // let file_path: &str = r"/Users/patrick/Code/bits-and-pieces/AdventOfCode22/Puzzle9/input.txt";
    let file_path: &str = r"C:\SourceCode\bits-and-pieces\AdventOfCode22\Puzzle9\input.txt";
    let contents: String = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut visited_coords: HashSet<String> = HashSet::new();

    let mut rope: Vec<Coordinate> = Vec::new();

    const ROPE_PIECES: usize = 10;

    for _ in 0..ROPE_PIECES {
        rope.push(Coordinate { x: 0, y: 0 });
    }

    for line in contents.lines() {

        let mut next_move = line.split_whitespace();

        let direction = next_move.next().unwrap();

        let amount: i32 = next_move.next().unwrap().parse::<i32>().unwrap();

        for _ in 1..=amount {

            // Move the head of the rope
            match direction {
                "U" => rope[0].y += 1,
                "D" => rope[0].y += -1,
                "L" => rope[0].x += -1,
                "R" => rope[0].x += 1,
                _ => ()
            }

            for i in 1..ROPE_PIECES {
                
                let x_diff = rope[i-1].x - rope[i].x;
                let y_diff = rope[i-1].y - rope[i].y;

                // Update the rope piece position if the distance is too great from the previous rope piece
                if x_diff.abs() > 1 || y_diff.abs() > 1 {

                    match x_diff {
                        d if d > 0 => {
                            rope[i].x += 1;
                        }
                        d if d < 0 => {
                            rope[i].x += -1;
                        }
                        _ => ()
                    }

                    match y_diff {
                        d if d > 0 => {
                            rope[i].y += 1;
                        }
                        d if d < 0 => {
                            rope[i].y += -1;
                        }
                        _ => ()
                    }

                } else {
                    // Stop iterating through the rope pieces if there's no more movement
                    break;
                }
            }

            // Check if the end of the rope is in a unique position and save it
            if !visited_coords.contains(&rope[ROPE_PIECES-1].key()) {
                visited_coords.insert(rope[ROPE_PIECES-1].key());
            }
        }

        // println!("{}", line);
        // for i in 0..ROPE_PIECES {
        //     println!("Rope: {}, {}", i, rope[i])
        // }

    }

    println!("Number of locations visited by the rope tail: {}", visited_coords.len());

}
