# Puzzle 4: https://adventofcode.com/2022/day/4

# Open the input file
with open('input.txt') as file:
    lines = file.readlines()

overlapped_assingments = 0

for line in lines:
    # Remove the newline character from the line
    line_stripped = line.replace("\n", "")
    # Split the line by the comma
    line_split = line_stripped.split(",")

    elf_1 = line_split[0].split("-")
    elf_2 = line_split[1].split("-")

    elf_1_start = int(elf_1[0])
    elf_1_end = int(elf_1[1])
    elf_2_start = int(elf_2[0])
    elf_2_end = int(elf_2[1])

    # If Elf 1 has lower starting number and a higher ending number than Elf 2
    if elf_1_end >= elf_2_end and elf_1_start <= elf_2_start:
        overlapped_assingments += 1

    # If Elf 2 has lower starting number and a higher ending number  than Elf 1
    elif elf_2_end >= elf_1_end and elf_2_start <= elf_1_start:
        overlapped_assingments += 1

print(
    'Number of overlapped assignment sections = [%s].' % overlapped_assingments)

# Puzzle 4: Part 2: https://adventofcode.com/2022/day/4#part2

overlapped_assingments = 0

for line in lines:
    # Remove the newline character from the line
    line_stripped = line.replace("\n", "")
    # Split the line by the comma
    line_split = line_stripped.split(",")

    elf_1 = line_split[0].split("-")
    elf_2 = line_split[1].split("-")

    elf_1_start = int(elf_1[0])
    elf_1_end = int(elf_1[1])
    elf_2_start = int(elf_2[0])
    elf_2_end = int(elf_2[1])

    # If Elf 1 has an ending number between the boundaries of Elf 2:
    if elf_2_start <= elf_1_end <= elf_2_end:
        overlapped_assingments += 1

    # If Elf 2 has an ending number between the boundaries of Elf 1:
    elif elf_1_start <= elf_2_end <= elf_1_end:
        overlapped_assingments += 1

    # If Elf 1 has an startomg number between the boundaries of Elf 2:
    elif elf_2_start <= elf_1_start <= elf_2_end:
        overlapped_assingments += 1

    # If Elf 2 has an starting number between the boundaries of Elf 1:
    elif elf_1_start <= elf_2_start <= elf_1_end:
        overlapped_assingments += 1

print(
    'Number of assignment sections with any overlap = [%s].' % overlapped_assingments)
