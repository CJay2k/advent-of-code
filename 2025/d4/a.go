package d4

import (
	"bufio"
	"fmt"
	"os"
)

func canBeAccessed(board *[]string, y int, x int) bool {
	if (*board)[y][x] != '@' {
		return false
	}

	moves := [][]int{
		{-1, -1},
		{-1, 0},
		{-1, 1},
		{0, -1},
		{0, 1},
		{1, -1},
		{1, 0},
		{1, 1},
	}

	count := 0
	for _, row := range moves {
		currY := y - row[0]
		currX := x - row[1]

		if currY < 0 || currY >= len(*board) || currX < 0 || currX >= len((*board)[0]) {
			continue
		}

		if (*board)[currY][currX] == '@' {
			count++
		}

		if count >= 4 {
			return false
		}
	}

	return true
}

func PartOne() {
	solution := 0

	file, _ := os.Open("d4/input.txt")
	defer file.Close()

	var board []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		board = append(board, line)
	}

	for y := 0; y < len(board); y++ {
		for x := 0; x < len(board[y]); x++ {
			if canBeAccessed(&board, y, x) {
				solution += 1
			}
		}
	}

	fmt.Println(solution)
}
