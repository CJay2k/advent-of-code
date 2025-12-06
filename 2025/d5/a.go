package d5

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func PartOne() {
	solution := 0

	file, _ := os.Open("d5/input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	var freshRanges [][]int
	var ids []int
	emptyLineFound := false
	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			emptyLineFound = true
			continue
		}

		if !emptyLineFound {
			startAndEnd := strings.Split(line, "-")
			start, _ := strconv.Atoi(startAndEnd[0])
			end, _ := strconv.Atoi(startAndEnd[1])
			freshRanges = append(freshRanges, []int{start, end})
		} else {
			num, _ := strconv.Atoi(line)
			ids = append(ids, num)
		}
	}

	for _, id := range ids {
		for _, _range := range freshRanges {
			start := _range[0]
			end := _range[1]
			if id >= start && id <= end {
				solution++
				break
			}
		}
	}

	fmt.Println(solution)
}
