package d7

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var seen = make(map[string]int)

func move(board *[]string, y int, x int, count int) int {
	key := strconv.Itoa(y) + "-" + strconv.Itoa(x)
	_, ok := seen[key]
	if ok {
		return seen[key]
	}

	if y >= len(*board) {
		return 1
	}

	if (*board)[y][x] == '^' {
		count = move(board, y+1, x-1, count)
		count += move(board, y+1, x+1, count)
	} else {
		count = move(board, y+1, x, count)
	}

	if !ok {
		seen[key] = count
	}
	return count
}

func PartTwo() {
	solution := 1

	file, _ := os.Open("d7/input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	var board []string
	for scanner.Scan() {
		line := scanner.Text()
		board = append(board, line)
	}

	startX := 0
	for i, c := range board[0] {
		if c == 'S' {
			startX = i
			break
		}
	}

	solution = move(&board, 0, startX, 0)

	fmt.Println(solution)
}
