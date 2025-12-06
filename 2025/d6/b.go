package d6

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"unicode"
)

func PartTwo() {
	solution := 0

	file, _ := os.Open("d6/input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	var data []string
	for scanner.Scan() {
		line := scanner.Text()
		data = append(data, line)
	}

	var nums []int
	operator := '+'
	for x := len(data[0]) - 1; x >= 0; x-- {
		numStr := ""
		for y := 0; y < len(data); y++ {
			char := rune(data[y][x])
			if unicode.IsDigit(char) {
				numStr += string(char)
			}
			switch char {
			case '*':
				operator = '*'
			case '+':
				operator = '+'
			}
		}
		num, err := strconv.Atoi(numStr)
		if err == nil {
			nums = append(nums, num)
		}

		if numStr == "" || x == 0 {
			partSolution := 0
			if operator == '*' {
				partSolution = 1
			}

			for _, num := range nums {
				if operator == '*' {
					partSolution *= num
				} else {
					partSolution += num
				}
			}

			solution += partSolution
			nums = []int{}
			continue
		}

	}

	fmt.Println(solution)
}
