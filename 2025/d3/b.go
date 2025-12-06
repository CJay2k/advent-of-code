package d3

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func PartTwo() {
	solution := 0

	file, _ := os.Open("d3/input.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		var intList []int
		for _, digitRune := range line {
			digitInt, _ := strconv.Atoi(string(digitRune))
			intList = append(intList, digitInt)
		}

		maxJoltageStr := ""
		i := 0
		for j := -11; j <= 0; j++ {
			slice := intList[i : len(line)+j]
			maxDigitInSlice := slice[0]
			maxDigitInSliceIndex := 0
			for k := 1; k < len(slice); k++ {
				if slice[k] > maxDigitInSlice {
					maxDigitInSlice = slice[k]
					maxDigitInSliceIndex = k
				}
			}
			i += maxDigitInSliceIndex + 1
			maxJoltageStr += strconv.Itoa(maxDigitInSlice)
		}
		maxJoltage, _ := strconv.Atoi(maxJoltageStr)
		solution += maxJoltage
	}

	fmt.Println(solution)
}
