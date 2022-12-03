# Puzzle 1: https://adventofcode.com/2022/day/1

# Open the input file
with open('input.txt') as file:
    lines = file.readlines()

# Prepare the running tally for calorie and highest calories counted so far
highest_calorie_elf = 0
running_calorie_total = 0

for line in lines:
    # If the next line is only a newline character, 
    # assume that everything before is carrie dby the same elf.
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
