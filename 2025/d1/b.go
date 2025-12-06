package d1

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func PartTwo() {
	position := 50
	solution := 0

	file, _ := os.Open("d1/input.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		direction := line[:1]
		number, _ := strconv.Atoi(line[1:])

		if direction == "L" {
			position -= number
		} else {
			position += number
		}

		for !(position < 100 && position >= 0) {
			if position < 0 {
				position += 100
			} else {
				position -= 100
			}
			solution += 1
		}
	}

	fmt.Println(solution)
}
