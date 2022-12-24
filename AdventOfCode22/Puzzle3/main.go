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
	var compartments [2]string
	var commonLetter string = ""
	var priorityLetterList []string
	var letterValue int

	var elfLetters []string
	var badeLetterList []string
	lineNumber := 1

	for fileScanner.Scan() {
		currentLine = fileScanner.Text()
		currentLine = strings.ReplaceAll(currentLine, "\n", "")
		currentLineLength = len(currentLine)
		compartmentSize = int(currentLineLength / 2)

		// Get the compartments for a line
		compartments[0] = currentLine[:compartmentSize]
		compartments[1] = currentLine[compartmentSize:]

		commonLetter = findCommonLetter(compartments[:])

		// Save the common letter on each loop
		priorityLetterList = append(priorityLetterList, commonLetter)

		// Part 2:
		// Append the current line to an array (representing an elf)
		elfLetters = append(elfLetters, currentLine)

		if lineNumber % 3 == 0 {
			commonLetter = findCommonLetter(elfLetters[:])
			badeLetterList = append(badeLetterList, commonLetter)
			elfLetters = nil
		}

		lineNumber++
	}

	// Close the input file
	readFile.Close()

	// Sum the value of the common letters using the value map above
	letterValue = sumCommonLetters(priorityLetterList)
	// Puzzle 3: Part 1: https://adventofcode.com/2022/day/3
	fmt.Printf("Total value of priority letters = [%d].\n", letterValue)

	// Sum the value of the common letters using the value map above
	letterValue = sumCommonLetters(badeLetterList)
	// Puzzle 3: Part 2: https://adventofcode.com/2022/day/3#part2
	fmt.Printf("Total value of badge letters = [%d].\n", letterValue)
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

func findCommonLetter(stringList []string) string {
	commonLetter, testedLetters := "", ""
	isCommonLetter := true
	listSize := len(stringList)

	for _, letter := range stringList[0] {
        // string(letter) needed as letter is of type "rune"
		stringLetter := string(letter)
		// Reset isCommonLetter to true for each new letter tested
		isCommonLetter = true

		// Short circuit evaluation if the letter has been tested
		if strings.Contains(commonLetter, stringLetter) || strings.Contains(testedLetters, stringLetter) {
			continue
		}

		// Test all other elements in the list to see if it contains the letter
		for idx := 1; idx < listSize; idx ++ {
			if !strings.Contains(stringList[idx], stringLetter) {
				isCommonLetter = false
				break
			}
		}

		// Add the tested letter to a list so it's not tested again
		testedLetters += stringLetter

		if isCommonLetter {
			// If the common letter is found, save it
			commonLetter += stringLetter
		}
	}

	// Sanity check to make sure that there is only 1 common letter per loop
	if len(commonLetter) != 1 {
		err := fmt.Errorf("Number of common letters is not 1: letters = [%s]", commonLetter)
		check(err)
	}

	return commonLetter
}