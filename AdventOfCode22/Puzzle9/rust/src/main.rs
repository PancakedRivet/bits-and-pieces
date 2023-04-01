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

    let mut rope: Vec<Coordinate> = Vec::new();

    const ROPE_PIECES: u8 = 2;

    for _ in 0..ROPE_PIECES {
        rope.push(Coordinate { x: 0, y: 0 });
    }

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
                Direction::Up => rope[0].y += 1,
                Direction::Down => rope[0].y += -1,
                Direction::Left => rope[0].x += -1,
                Direction::Right => rope[0].x += 1,
            }

            let prev_rope_piece = rope[0].clone();

            move_rope_piece(&prev_rope_piece, &mut rope[1], &direction);

            if !visited_coords.contains_key(&rope[1].key()) {
                visited_coords.insert(rope[1].key(), rope[1].clone());
            }

            // println!("{}, {}", rope_head, rope_tail);
        }
    }

    println!("Part 1: Number of locations visited by the rope tail: {}", visited_coords.len());

    // https://adventofcode.com/2022/day/9#part2

}

fn move_rope_piece(prev_rope_piece: &Coordinate, rope_piece: &mut Coordinate, direction: &Direction) {

    // Update the rope piece position if the distance is too great from the previous rope piece
    let x_diff = prev_rope_piece.x - rope_piece.x;
    let y_diff = prev_rope_piece.y - rope_piece.y;
    let should_move_tail = x_diff.abs() > 1 || y_diff.abs() > 1;

    if should_move_tail {
        if prev_rope_piece.x - rope_piece.x == 0 || prev_rope_piece.y - rope_piece.y == 0 {
            // Move along the axis
            match direction {
                Direction::Up => rope_piece.y += 1,
                Direction::Down => rope_piece.y += -1,
                Direction::Left => rope_piece.x += -1,
                Direction::Right => rope_piece.x += 1,
            }
        } else {
            // Move diagonally
            match (direction, x_diff, y_diff) {
                (Direction::Up, d, _) if d < 0 => {
                    rope_piece.x += -1;
                    rope_piece.y += 1;
                },
                (Direction::Up, d, _) if d > 0 => {
                    rope_piece.x += 1;
                    rope_piece.y += 1;
                },

                (Direction::Down, d, _) if d < 0 => {
                    rope_piece.x += -1;
                    rope_piece.y += -1;
                },
                (Direction::Down, d, _) if d > 0 => {
                    rope_piece.x += 1;
                    rope_piece.y += -1;
                },

                (Direction::Left, _, d) if d < 0 => {
                    rope_piece.y += -1;
                    rope_piece.x += -1;
                },
                (Direction::Left, _, d) if d > 0 => {
                    rope_piece.y += 1;
                    rope_piece.x += -1;
                },

                (Direction::Right, _, d) if d < 0 => {
                    rope_piece.y += -1;
                    rope_piece.x += 1;
                },
                (Direction::Right, _, d) if d > 0 => {
                    rope_piece.y += 1;
                    rope_piece.x += 1;
                },
                (_,_,_) => ()
            }
        }
    }
}