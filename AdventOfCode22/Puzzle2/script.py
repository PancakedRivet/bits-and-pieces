# Puzzle 2: https://adventofcode.com/2022/day/2

# Open the input file
with open('input.txt') as file:
    lines = file.readlines()

# Create the resulting values for shapes chosen and game outcomes
chosen_shape_values = {
    "X": 1,  # Rock
    "Y": 2,  # Paper
    "Z": 3   # Scissors
}

game_outcome_values = {
    "A X": 3,
    "A Y": 6,
    "A Z": 0,
    "B X": 0,
    "B Y": 3,
    "B Z": 6,
    "C X": 6,
    "C Y": 0,
    "C Z": 3,
}

total_points_won = 0

for line in lines:
    # Remove the newline character from the line
    line_stripped = line.replace("\n", "")
    # Split the line by the whitespace
    line_split = line_stripped.split()

    # Determine the players choide using the second character in the line
    players_choice = line_split[1]
    points_for_choosing = chosen_shape_values[players_choice]

    # Determine the outcome of the game based on both players choices
    points_for_outcome = game_outcome_values[line_stripped]

    # Calculate the points earned for that round
    total_points_for_round = points_for_choosing + points_for_outcome
    # Increment the total points based on the points won that round
    total_points_won += total_points_for_round

print(
    'Expected score from the strategy guide = [%s] points.' % total_points_won)

# Puzzle 2: Part 2: https://adventofcode.com/2022/day/2#part2

# Open the input file
with open('input.txt') as file:
    lines = file.readlines()

# Create the resulting values for shapes chosen and game outcomes
game_outcome_values = {
    # Using choice + outcome
    "A X": 3 + 0,
    "A Y": 1 + 3,
    "A Z": 2 + 6,
    "B X": 1 + 0,
    "B Y": 2 + 3,
    "B Z": 3 + 6,
    "C X": 2 + 0,
    "C Y": 3 + 3,
    "C Z": 1 + 6,
}

total_points_won = 0

for line in lines:
    # Remove the newline character from the line
    line_stripped = line.replace("\n", "")

    # Determine the outcome of the game based on both players choices
    points_for_outcome = game_outcome_values[line_stripped]

    # Increment the total points based on the points won that round
    total_points_won += points_for_outcome

print(
    'Expected score from the *corrected* strategy guide = [%s] points.' % total_points_won)
