package d1

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func PartOne() {
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

		position %= 100
		if position == 0 {
			solution += 1
		}
	}

	fmt.Println(solution)
}
