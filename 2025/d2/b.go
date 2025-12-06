package d2

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func PartTwo() {
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

			for i := numLength / 2; i > 0; i-- {
				set := make(map[string]struct{})
				for j := 0; j < len(numAsString); j += i {
					end := min(j+i, len(numAsString))
					set[numAsString[j:end]] = struct{}{}
				}

				if len(set) == 1 {
					solution += num
					break
				}
			}
		}
	}

	fmt.Println(solution)
}
