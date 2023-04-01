use std::fs;
use std::fmt;
use std::clone::Clone;
use std::collections::HashMap;

// https://adventofcode.com/2022/day/9

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
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
    let file_path: &str = r"/Users/patrick/Code/bits-and-pieces/AdventOfCode22/Puzzle9/input.txt";
    let contents: String = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut visited_coords: HashMap<String, Coordinate> = HashMap::new();

    let mut rope_head: Coordinate = Coordinate { x: 0, y: 0 };
    let mut rope_tail: Coordinate = rope_head.clone();

    for line in contents.lines() {
        // println!("{}", line);

        let mut next_move = line.split_whitespace();

        let direction = match next_move.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => Direction::Up
        };

        let amount: i32 = next_move.next().unwrap().parse::<i32>().unwrap();

        for _ in 1..=amount {
            match direction {
                Direction::Up => rope_head.y += 1,
                Direction::Down => rope_head.y += -1,
                Direction::Left => rope_head.x += -1,
                Direction::Right => rope_head.x += 1,
            }

            // Update the tail position if the distance is too great from the head
            let x_diff = rope_head.x - rope_tail.x;
            let y_diff = rope_head.y - rope_tail.y;
            let should_move_tail = x_diff.abs() > 1 || y_diff.abs() > 1;

            if should_move_tail {
                if rope_head.x - rope_tail.x == 0 || rope_head.y - rope_tail.y == 0 {
                    // Move along the axis
                    match direction {
                        Direction::Up => rope_tail.y += 1,
                        Direction::Down => rope_tail.y += -1,
                        Direction::Left => rope_tail.x += -1,
                        Direction::Right => rope_tail.x += 1,
                    }
                } else {
                    match direction {
                        Direction::Up => {
                            match rope_head.x - rope_tail.x {
                                d if d < 0 => rope_tail.x += -1,
                                d if d > 0 => rope_tail.x += 1,
                                _   => (),
                            };
                            rope_tail.y += 1;
                        }
                        Direction::Down => {
                            match rope_head.x - rope_tail.x {
                                d if d < 0 => rope_tail.x += -1,
                                d if d > 0 => rope_tail.x += 1,
                                _   => (),
                            };
                            rope_tail.y += -1;
                        },
                        Direction::Left => {
                            match rope_head.y - rope_tail.y {
                                d if d < 0 => rope_tail.y += -1,
                                d if d > 0 => rope_tail.y += 1,
                                _   => (),
                            };
                            rope_tail.x += -1;
                        }
                        Direction::Right => {
                            match rope_head.y - rope_tail.y {
                                d if d < 0 => rope_tail.y += -1,
                                d if d > 0 => rope_tail.y += 1,
                                _   => (),
                            };
                            rope_tail.x += 1;
                        },
                    }
                }
            }

            if !visited_coords.contains_key(&rope_tail.key()) {
                visited_coords.insert(rope_tail.key().clone(), rope_tail.clone());
            }

            // println!("{}, {}", rope_head, rope_tail);
        }
    }

    println!("Part 1: Number of locations visited by the rope tail: {}", visited_coords.len());

    // https://adventofcode.com/2022/day/9#part2

}
