package d2

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func PartOne() {
	solution := 0

	content, _ := os.ReadFile("d2/input.txt")
	ranges := strings.SplitSeq(string(content), ",")

	for rang := range ranges {
		startAndEnd := strings.Split(rang, "-")
		start, _ := strconv.Atoi(startAndEnd[0])
		end, _ := strconv.Atoi(startAndEnd[1])

		for num := start; num <= end; num++ {
			numAsString := strconv.Itoa(num)
			numLength := len(numAsString)

			if numAsString[:numLength/2] == numAsString[numLength/2:] {
				solution += num
			}
		}
	}

	fmt.Println(solution)
}
