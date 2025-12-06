package d6

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func PartOne() {
	solution := 0

	file, _ := os.Open("d6/input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	var data [][]string
	for scanner.Scan() {
		line := scanner.Text()
		var row []string
		for numStr := range strings.SplitSeq(line, " ") {
			if numStr != "" {
				row = append(row, numStr)
			}
		}
		data = append(data, row)
	}

	for x := 0; x < len(data[0]); x++ {
		var operator string
		partSolution := 1
		for y := 0; y < len(data)-1; y++ {
			operator = strings.TrimSpace(data[len(data)-1][x])
			num, _ := strconv.Atoi(strings.TrimSpace(data[y][x]))
			if operator == "*" {
				partSolution *= num
			} else {
				partSolution += num
			}
		}
		if operator == "+" {
			partSolution -= 1
		}
		solution += partSolution
	}

	fmt.Println(solution)
}
