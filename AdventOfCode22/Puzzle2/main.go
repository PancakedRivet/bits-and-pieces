package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

var shapePoints = map[string]int{
	"X": 1,
	"Y": 2,
	"Z": 3,
}

var gamePoints = map[string]int{
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

var correctedGamePoints = map[string]int{
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

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {

	// Open the input file
	readFile, err := os.Open("./input.txt")
	check(err)
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	var currentLine string
	var currentLineSplit []string
	var roundShapePoints, roundGamePoints int
	var totalPointsWon, correctedTotalPointsWon int = 0, 0

	for fileScanner.Scan() {
		// Read the next line and split it into seperate words
		currentLine = fileScanner.Text()
		currentLineSplit = strings.Fields(currentLine)
		
		// Calculate the points earned based on the shape chosen and game outcome
		roundShapePoints = shapePoints[currentLineSplit[1]]
		roundGamePoints = gamePoints[currentLine]

		// Increase the total points counts by the results
		totalPointsWon += roundShapePoints + roundGamePoints
		correctedTotalPointsWon += correctedGamePoints[currentLine]
	}
	// Close the input file
	readFile.Close()

	// Puzzle 2: Part 1: https://adventofcode.com/2022/day/2
	fmt.Printf("Expected score from the strategy guide = [%d] points.\n", totalPointsWon)

	// Puzzle 2: Part 2: https://adventofcode.com/2022/day/2#part2
	fmt.Printf("Expected score from the *corrected* strategy guide = [%d] points.\n", correctedTotalPointsWon)
}
