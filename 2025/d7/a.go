package d7

import (
	"bufio"
	"fmt"
	"os"
)

func PartOne() {
	solution := 0

	file, _ := os.Open("d7/input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	var previousLine string
	for scanner.Scan() {
		line := scanner.Text()

		if previousLine == "" {
			previousLine = line
			continue
		}

		for i := 0; i < len(line); i++ {
			if previousLine[i] == 'S' || previousLine[i] == '|' {
				runes := []rune(line)
				switch line[i] {
				case '.':
					runes[i] = '|'
				case '^':
					if runes[i-1] == '.' || runes[i+1] == '.' {
						solution++
					}
					runes[i-1] = '|'
					runes[i+1] = '|'
				}
				line = string(runes)
			}
		}
		previousLine = line
	}

	fmt.Println(solution)
}
