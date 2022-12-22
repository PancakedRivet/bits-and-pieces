package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	const file string = "../input.txt"

	readFile, err := os.Open(file)
	check(err)

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	var currentLine string
	var calorieLine, calorieValue, highestCalories int
	var calorieList []int

	for fileScanner.Scan() {
		currentLine = fileScanner.Text()

		if currentLine == "" {
			calorieValue = countCalories(calorieList)
			if calorieValue > highestCalories {
				highestCalories = calorieValue
			}
			calorieList = nil
		} else {
			calorieLine, err = strconv.Atoi(currentLine)
			check(err)
			calorieList = append(calorieList, calorieLine)
		}
	}
	readFile.Close()

	fmt.Printf("Most caloric elf is carrying [%d] calories.\n", highestCalories)
}

func countCalories(values []int) int {
	calorieCount := 0
	for _, v := range values {
		calorieCount += v
	}
	return calorieCount
}
