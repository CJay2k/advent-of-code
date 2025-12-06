package d4

import (
	"bufio"
	"fmt"
	"os"
)

func PartTwo() {
	solution := 0

	file, _ := os.Open("d4/input.txt")
	defer file.Close()

	var board []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		board = append(board, line)
	}

	y := 0
	for y < len(board) {
		removed := false
		for x := 0; x < len(board[y]); x++ {
			if canBeAccessed(&board, y, x) {
				runes := []rune(board[y])
				runes[x] = 'x'
				board[y] = string(runes)

				solution += 1
				removed = true
			}
		}
		if removed {
			y = max(y-1, 0)
		} else {
			y++
		}
	}

	fmt.Println(solution)
}
