# Puzzle 6: https://adventofcode.com/2022/day/6

# Open the input file
with open('input.txt') as file:
    lines = file.readlines()

signal = lines[0]  # The signal is a single line in the text file
counter = 0
signal_datastream = []

for letter in signal:
    counter += 1

    # If the next letter is already in the datastream
    if letter in signal_datastream:
        # Find the index of the existing letter
        index = signal_datastream.index(letter)
        # Keep the datastream past where the conflict occurred
        signal_datastream = signal_datastream[index + 1:]

    signal_datastream.append(letter)

    if len(signal_datastream) >= 4:
        break

print("Start of packet marker detected at position = [%s]." % counter)
print("Packet = %s." % signal_datastream)

# Puzzle 6: Part 2: https://adventofcode.com/2022/day/6#part2

signal = lines[0]  # The signal is a single line in the text file
counter = 0
signal_datastream = []

for letter in signal:
    counter += 1

    # If the next letter is already in the datastream
    if letter in signal_datastream:
        # Find the index of the existing letter
        index = signal_datastream.index(letter)
        # Keep the datastream past where the conflict occurred
        signal_datastream = signal_datastream[index + 1:]

    signal_datastream.append(letter)

    if len(signal_datastream) >= 14:
        break

print("Start of packet marker detected at position = [%s]." % counter)
print("Packet = %s." % signal_datastream)
