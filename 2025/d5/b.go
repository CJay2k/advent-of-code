package d5

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func PartTwo() {
	solution := 0

	file, _ := os.Open("d5/input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	var freshRanges [][]int
	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			break
		}

		startAndEnd := strings.Split(line, "-")
		start, _ := strconv.Atoi(startAndEnd[0])
		end, _ := strconv.Atoi(startAndEnd[1])
		freshRanges = append(freshRanges, []int{start, end})
	}

	for i := 0; i < len(freshRanges); i++ {
		for j := 0; j < len(freshRanges); j++ {
			if freshRanges[i][1] >= freshRanges[j][0] && freshRanges[i][0] <= freshRanges[j][1] {
				newStart := min(freshRanges[i][0], freshRanges[j][0])
				newEnd := max(freshRanges[i][1], freshRanges[j][1])
				freshRanges[i][0] = newStart
				freshRanges[i][1] = newEnd
				freshRanges[j][0] = newStart
				freshRanges[j][1] = newEnd
			}
		}
	}

	seen := make(map[string]struct{})
	for _, _range := range freshRanges {
		key := strconv.Itoa(_range[0]) + "-" + strconv.Itoa(_range[1])
		if _, ok := seen[key]; !ok {
			seen[key] = struct{}{}
			solution += _range[1] + 1 - _range[0]
		}
	}

	fmt.Println(solution)
}
