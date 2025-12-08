package d8

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func PartTwo() {
	solution := 1

	file, _ := os.Open("d8/input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	set := make(map[string]struct{})
	data := [][]int{}
	for scanner.Scan() {
		line := scanner.Text()
		lineSplit := strings.Split(line, ",")
		x, _ := strconv.Atoi(lineSplit[0])
		y, _ := strconv.Atoi(lineSplit[1])
		z, _ := strconv.Atoi(lineSplit[2])

		key := strconv.Itoa(x) + "-" + strconv.Itoa(y) + "-" + strconv.Itoa(z)
		set[key] = struct{}{}

		data = append(data, []int{x, y, z})
	}

	distances := []pair{}
	for i := 0; i < len(data)-1; i++ {
		for j := i + 1; j < len(data); j++ {
			a := data[i]
			b := data[j]

			distance := math.Sqrt(
				math.Pow(float64(a[0]-b[0]), 2) + math.Pow(float64(a[1]-b[1]), 2) + math.Pow(float64(a[2]-b[2]), 2),
			)

			distances = append(distances, pair{
				distance: int(distance),
				a:        strconv.Itoa(a[0]) + "-" + strconv.Itoa(a[1]) + "-" + strconv.Itoa(a[2]),
				b:        strconv.Itoa(b[0]) + "-" + strconv.Itoa(b[1]) + "-" + strconv.Itoa(b[2]),
			})
		}
	}

	sort.Slice(distances, func(i, j int) bool {
		return distances[i].distance < distances[j].distance
	})

	connections := [][]string{}
	for i := range len(distances) {
		connections = append(connections, []string{distances[i].a, distances[i].b})
	}

	for _, c := range connections {
		delete(set, c[0])
		delete(set, c[1])

		if len(set) == 0 {
			split := strings.Split(c[0], "-")
			ax, _ := strconv.Atoi(split[0])
			split = strings.Split(c[1], "-")
			bx, _ := strconv.Atoi(split[0])
			solution = ax * bx
			break
		}
	}

	fmt.Println(solution)
}
