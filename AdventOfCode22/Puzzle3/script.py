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
    # print(line)

    # Get the length of the line (minus the newline character)
    line_length = len(line) - 1
    compartment_size = int(line_length / 2)

    # Get the compartments for each line
    compartment_1 = line[:compartment_size]
    compartment_2 = line[compartment_size:]

    common_letter = ""
    for i in compartment_1:
        if i in compartment_2 and not i in common_letter:
            common_letter += i

    if len(common_letter) > 1:
        print(line)
        print(compartment_1)
        print(compartment_2)
        print(common_letter)
        raise Exception("There should only be one letter in common")

    priority_letter_list.append(common_letter)

    # print(compartment_1)
    # print(compartment_2)
    # print(common_letter)
    # print("")

priority_letter_value_list = []

for letter in priority_letter_list:
    value = 0
    try:
        value = priority_letter_values[letter]
    except:
        if priority_letter_values[letter.lower()] > 0:
            value = priority_letter_values[letter.lower()] + 26
        else:
            raise Exception(
                "Could not generate value for letter [%s]" % letter)
    priority_letter_value_list.append(value)

priority_letter_sum = sum(priority_letter_value_list)

print(
    'Total value of priority letters = [%s].' % priority_letter_sum)
