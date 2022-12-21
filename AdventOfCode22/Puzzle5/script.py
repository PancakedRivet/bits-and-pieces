# Puzzle 5: https://adventofcode.com/2022/day/5

import copy

# Open the input file
with open('input.txt') as file:
    lines = file.readlines()

"""
# Starting position of crates:
    [P]                 [Q]     [T]
[F] [N]             [P] [L]     [M]
[H] [T] [H]         [M] [H]     [Z]
[M] [C] [P]     [Q] [R] [C]     [J]
[T] [J] [M] [F] [L] [G] [R]     [Q]
[V] [G] [D] [V] [G] [D] [N] [W] [L]
[L] [Q] [S] [B] [H] [B] [M] [L] [D]
[D] [H] [R] [L] [N] [W] [G] [C] [R]
 1   2   3   4   5   6   7   8   9 
"""

INITIAL_CRATE_STACKS = [
    ["D", "L", "V", "T", "M", "H", "F"],  # stack 1
    ["H", "Q", "G", "J", "C", "T", "N", "P"],  # stack 2
    ["R", "S", "D", "M", "P", "H"],  # stack 3
    ["L", "B", "V", "F"],  # stack 4
    ["N", "H", "G", "L", "O"],  # stack 5
    ["W", "B", "D", "G", "R", "M", "P"],  # stack 6
    ["G", "M", "N", "R", "C", "H", "L", "Q"],  # stack 7
    ["C", "L", "W"],  # stack 8
    ["R", "D", "L", "Q", "J", "Z", "M", "T"]  # stack 9
]

# Perform a deepcopy so as to not change the starting position
crate_stacks = copy.deepcopy(INITIAL_CRATE_STACKS)

for line in lines:
    # Remove the newline character from the line
    line_stripped = line.replace("\n", "")
    # Split the line by the comma
    line_split = line_stripped.split()

    # Extract values from the line
    quantity = int(line_split[1])
    from_stack_number = int(line_split[3]) - 1
    to_stack_number = int(line_split[5]) - 1

    # Repeat for the quantity in the instruction
    for _ in range(quantity):
        # Remove the crate from one stack
        crate = crate_stacks[from_stack_number].pop()
        # Add the crate to the other stack
        crate_stacks[to_stack_number].append(crate)

result = ""

# Add the top crate letter to a string
for idx in range(len(crate_stacks)):
    top_crate = crate_stacks[idx][-1]
    result += top_crate

print('Message for top crate of each stack = [%s].' % result)

# Puzzle 5: Part 2: https://adventofcode.com/2022/day/5#part2

# Perform a deepcopy so as to not change the starting position
crate_stacks = copy.deepcopy(INITIAL_CRATE_STACKS)

for line in lines:
    # Remove the newline character from the line
    line_stripped = line.replace("\n", "")
    # Split the line by the comma
    line_split = line_stripped.split()

    # Extract values from the line
    quantity = int(line_split[1])
    from_stack_number = int(line_split[3]) - 1
    to_stack_number = int(line_split[5]) - 1

    removed_crates = crate_stacks[from_stack_number][-quantity:]

    # Adjust the "from" stack
    new_from_stack = crate_stacks[from_stack_number][:-quantity]
    crate_stacks[from_stack_number] = new_from_stack

    # Adjust the "to" stack
    new_to_stack = crate_stacks[to_stack_number] + removed_crates
    crate_stacks[to_stack_number] = new_to_stack

result = ""

# Add the top crate letter to a string
for idx in range(len(crate_stacks)):
    top_crate = crate_stacks[idx][-1]
    result += top_crate

print(
    'Message for top crate of each stack with CrateMover 9001 = [%s].' % result)
