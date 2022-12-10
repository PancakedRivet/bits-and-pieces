# Puzzle 3: https://adventofcode.com/2022/day/3

# Open the input file
with open('input.txt') as file:
    lines = file.readlines()

# Create the list of priority values
priority_letter_values = {
    "a": 1,
    "b": 2,
    "c": 3,
    "d": 4,
    "e": 5,
    "f": 6,
    "g": 7,
    "h": 8,
    "i": 9,
    "j": 10,
    "k": 11,
    "l": 12,
    "m": 13,
    "n": 14,
    "o": 15,
    "p": 16,
    "q": 17,
    "r": 18,
    "s": 19,
    "t": 20,
    "u": 21,
    "v": 22,
    "w": 23,
    "x": 24,
    "y": 25,
    "z": 26,
}

priority_letter_list = []

for line in lines:
    # Get the length of the line
    line_length = len(line)
    if line[-1] == "\n":
        # Remove the newline character if it exists at the end of the line
        line_length -= 1
    compartment_size = int(line_length / 2)

    # Get the compartments for a line
    compartment_1 = line[:compartment_size]
    compartment_2 = line[compartment_size:]

    common_letter = ""
    for i in compartment_1:
        if i in compartment_2 and not i in common_letter:
            # Add the common letter if it isn't already in the list
            common_letter += i

    # Make sure that there's only 1 common letter between compartments
    if len(common_letter) > 1:
        raise Exception(
            "There should only be one letter in common between compartment 1 = [%s] and compartment 2 = [%s]" % (compartment_1, compartment_2))

    priority_letter_list.append(common_letter)

priority_letter_value_list = []

for letter in priority_letter_list:
    value = 0

    # Try generate the value for a lower case letter
    try:
        value = priority_letter_values[letter]
    # If there is no value returned, the letter must be uppercase
    except:
        if priority_letter_values[letter.lower()] > 0:
            # Adjust the value of the upper case letters by adding 26
            value = priority_letter_values[letter.lower()] + 26
        else:
            # Stop executing if there is a letter without a value
            raise Exception(
                "Could not generate value for letter [%s]" % letter)

    priority_letter_value_list.append(value)

# Sum the list of priority letters
priority_letter_sum = sum(priority_letter_value_list)

print(
    'Total value of priority letters = [%s].' % priority_letter_sum)

# Puzzle 3: Part 2: https://adventofcode.com/2022/day/3#part2

badge_letter_list = []

# Calcualte the expected number of groups for the numer of lines given
number_of_groups = int(len(lines) / 3)

for idx in range(number_of_groups):
    # Define the line for each elf in the group
    elf_1 = lines[idx * 3 + 0]
    elf_2 = lines[idx * 3 + 1]
    elf_3 = lines[idx * 3 + 2]

    # Remove the newline character if it exists at the end of the line
    if elf_1[-1] == "\n":
        elf_1 = elf_1[:-1]
    if elf_2[-1] == "\n":
        elf_2 = elf_2[:-1]
    if elf_3[-1] == "\n":
        elf_3 = elf_3[:-1]

    common_letter = ""
    for i in elf_1:
        if i in elf_2 and i in elf_3 and not i in common_letter:
            # Add the common letter if it isn't already in the list
            common_letter += i

    # Make sure that there's only 1 common letter between elves
    if len(common_letter) > 1:
        raise Exception(
            "There should only be one letter in common between elf 1 = [%s], elf 2 = [%s] and elf 3 = [%s]" % (elf_1, elf_2, elf_3))

    badge_letter_list.append(common_letter)

priority_letter_value_list = []

for letter in badge_letter_list:
    value = 0

    # Try generate the value for a lower case letter
    try:
        value = priority_letter_values[letter]
    # If there is no value returned, the letter must be uppercase
    except:
        if priority_letter_values[letter.lower()] > 0:
            # Adjust the value of the upper case letters by adding 26
            value = priority_letter_values[letter.lower()] + 26
        else:
            # Stop executing if there is a letter without a value
            raise Exception(
                "Could not generate value for letter [%s]" % letter)

    priority_letter_value_list.append(value)

# Sum the list of priority letters
priority_letter_sum = sum(priority_letter_value_list)

print(
    'Total value of badge letters = [%s].' % priority_letter_sum)
