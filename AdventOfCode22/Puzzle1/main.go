package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

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
	var calorieLine, calorieValue int
	var calorieList []int

	// To find the top N elves we need an N+1 array
	highestCalories := []int{0, 0, 0, 0}

	for fileScanner.Scan() {
		// If the next line is only a newline character,
		// assume that everything before is carried by the same elf.
		currentLine = fileScanner.Text()

		if currentLine == "" {
			// Sum the list of items carried by an elf
			calorieValue = sumArray(calorieList)

			// Insert the value into the array in the "dummy" slot
			highestCalories[0] = calorieValue

			// Determine the 3 highest calorie elves from the 4 values
			highestCalories = compareCalorieCount(highestCalories)

			// Reset the running total
			calorieList = nil
		} else {
			calorieLine, err = strconv.Atoi(currentLine)
			check(err)
			calorieList = append(calorieList, calorieLine)
		}
	}
	// Close the input file
	readFile.Close()

	// Puzzle 1: Part 1: https://adventofcode.com/2022/day/1
	fmt.Printf("Most caloric elf is carrying [%d] calories.\n", highestCalories[3])

	// Puzzle 1: Part 2: https://adventofcode.com/2022/day/1#part2
	// Sum the array of highest calories (the 4th value is always 0 so it's only summing 3)
	calorieValue = sumArray(highestCalories)
	fmt.Printf("Calories carried by top 3 elves = [%d] calories.\n", calorieValue)
}

func sumArray(values []int) int {
	// Sum the values of an array
	calorieCount := 0
	for _, v := range values {
		calorieCount += v
	}
	return calorieCount
}

func compareCalorieCount(values []int) []int {
	// Sort the values from low to high and remove (zero) the lowest one
	sort.Ints(values)
	values[0] = 0
	return values
}
