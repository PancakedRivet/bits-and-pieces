package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"unicode"
)

var priorityLetterValues = map[string]int{
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
	var currentLineLength, compartmentSize int
	var compartment1, compartment2 string
	var commonLetter string = ""
	var priorityLetterList []string
	var letterValue int

	for fileScanner.Scan() {
		currentLine = fileScanner.Text()
		currentLine = strings.ReplaceAll(currentLine, "\n", "")
		currentLineLength = len(currentLine)
		compartmentSize = int(currentLineLength / 2)

		// Get the compartments for a line
		compartment1 = currentLine[:compartmentSize]
		compartment2 = currentLine[compartmentSize:]

		// Reset common letters to none on each loop
		commonLetter = ""
		for _, letter := range compartment1 {
			// string(letter) needed as letter is of type "rune"
			stringLetter := string(letter)
			if strings.Contains(compartment1, stringLetter) && strings.Contains(compartment2, stringLetter) && !strings.Contains(commonLetter, stringLetter) {
				commonLetter += string(letter)
			}
		}

		// Sanity check to make sure that there is only 1 common letter per loop
		if len(commonLetter) > 1 {
			err := fmt.Errorf("Number of common letters is > 1: %s", commonLetter)
			check(err)
		}

		// Save the common letter on each loop
		priorityLetterList = append(priorityLetterList, commonLetter)

	}

	// Sum the value of the common letters using the value map above
	letterValue = sumCommonLetters(priorityLetterList)

	// Close the input file
	readFile.Close()

	// Puzzle 3: Part 1: https://adventofcode.com/2022/day/3
	fmt.Printf("Total value of priority letters = [%d].\n", letterValue)

	// Puzzle 3: Part 2: https://adventofcode.com/2022/day/3#part2
	// fmt.Printf("Expected score from the *corrected* strategy guide = [%d] points.\n", correctedTotalPointsWon)
}

func sumCommonLetters( letters []string) int {
	// Uses the value map to return an integer total
	runningTotal := 0
	var lowerCaseLetter string
	for _, letter := range letters {
		lowerCaseLetter = strings.ToLower(letter)
		runningTotal += priorityLetterValues[lowerCaseLetter]
		if IsUpper(letter) {
			// Only map values for lower case letters are listed
			// If it's an uppercase letter, add 26 as that's the lower to upper value difference
			runningTotal += 26
		}
	}
	return runningTotal
}

func IsUpper(s string) bool {
    for _, r := range s {
        if !unicode.IsUpper(r) && unicode.IsLetter(r) {
            return false
        }
    }
    return true
}
