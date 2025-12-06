package d3

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func PartOne() {
	solution := 0

	file, _ := os.Open("d3/input.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		maxJoltage := 0
		for i := 0; i < len(line)-1; i++ {
			for j := i + 1; j < len(line); j++ {
				numberStr := fmt.Sprintf("%c%c", line[i], line[j])
				num, _ := strconv.Atoi(numberStr)
				maxJoltage = max(num, maxJoltage)
			}
		}
		solution += maxJoltage
	}

	fmt.Println(solution)
}
