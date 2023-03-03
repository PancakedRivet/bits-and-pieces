use std::fs;

fn main() {
    // Open the input file
    let file_path = r"D:\Git\bits-and-pieces\AdventOfCode22\Puzzle4\input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut fully_overlapped_assignment_count: i32 = 0;
    let mut any_overlap_assignment_count: i32 = 0;

    for line in contents.lines() {
        let split = line.split(",");
        let vec = split.collect::<Vec<&str>>();

        let elf_1_job_range = vec[0].split("-").collect::<Vec<&str>>();
        let elf_2_job_range = vec[1].split("-").collect::<Vec<&str>>();

        let elf_1_start = elf_1_job_range[0].parse::<i32>().unwrap();
        let elf_1_end = elf_1_job_range[1].parse::<i32>().unwrap();
        let elf_2_start = elf_2_job_range[0].parse::<i32>().unwrap();
        let elf_2_end = elf_2_job_range[1].parse::<i32>().unwrap();

        // If Elf 1 has lower starting number and a higher ending number than Elf 2
        if elf_1_end >= elf_2_end && elf_1_start <= elf_2_start {
            fully_overlapped_assignment_count += 1;
        }
        // If Elf 2 has lower starting number and a higher ending number  than Elf 1
        else if elf_2_end >= elf_1_end && elf_2_start <= elf_1_start {
            fully_overlapped_assignment_count += 1;
        }

        // If Elf 1's job assignment range contains the start or end of Elf 2's range
        if (elf_1_start <= elf_2_start && elf_2_start <= elf_1_end)
            || (elf_1_start <= elf_2_end && elf_2_end <= elf_1_end)
        {
            any_overlap_assignment_count += 1;
        }
        // If Elf 2's job assignment range contains the start or end of Elf 1's range
        else if (elf_2_start <= elf_1_start && elf_1_start <= elf_2_end)
            || (elf_2_start <= elf_1_end && elf_1_end <= elf_2_end)
        {
            any_overlap_assignment_count += 1;
        }
    }
    // Puzzle 4: https://adventofcode.com/2022/day/4
    println!("Overlapped Assignment Count: {fully_overlapped_assignment_count}");

    // Puzzle 4: Part 2: https://adventofcode.com/2022/day/4#part2
    println!("Any Overlap Assignment Count: {any_overlap_assignment_count}");
}
