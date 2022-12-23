# Puzzle 1: https://adventofcode.com/2022/day/1

import os.path

input_file = os.path.dirname(__file__) + '/input.txt'

# Open the input file
with open(input_file)as file:
    lines = file.readlines()

# Prepare the running tally for calorie and highest calories counted so far
highest_calorie_elf = 0
running_calorie_total = 0

for line in lines:
    # If the next line is only a newline character,
    # assume that everything before is carried by the same elf.
    if line == "\n":
        # Check whether the current calorie total is the new highest value
        if running_calorie_total > highest_calorie_elf:
            highest_calorie_elf = running_calorie_total
        # Reset the running total
        running_calorie_total = 0
    else:
        # If it's not a newline, increment the running total
        calorie_value = int(line[:-1])
        running_calorie_total += calorie_value

print('Most caloric elf is carrying [%s] calories.' % highest_calorie_elf)

# Puzzle 1: Part 2: https://adventofcode.com/2022/day/1#part2

# Open the input file
with open(input_file) as file:
    lines = file.readlines()

# Prepare the running tally for calorie and highest calories counted so far
elf_calorie_count = []
running_calorie_total = 0

for line in lines:
    # If the next line is only a newline character,
    # assume that everything before is carried by the same elf.
    if line == "\n":
        # Add the calorie count to the array
        elf_calorie_count.append(running_calorie_total)
        # Reset the running total
        running_calorie_total = 0
    else:
        # If it's not a newline, increment the running total
        calorie_value = int(line[:-1])
        running_calorie_total += calorie_value

# Sort the list of calorie totals
elf_calorie_count.sort(reverse=True)

# Sum the 3 most caloric elves
top_3_elves_calories = sum(elf_calorie_count[0:3])

print(
    'Calories carried by top 3 elves = [%s] calories.' % top_3_elves_calories)
