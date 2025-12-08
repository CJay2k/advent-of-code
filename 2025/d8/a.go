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

type pair struct {
	distance int
	a        string
	b        string
}

func PartOne() {
	solution := 1

	file, _ := os.Open("d8/input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	data := [][]int{}
	for scanner.Scan() {
		line := scanner.Text()
		lineSplit := strings.Split(line, ",")
		x, _ := strconv.Atoi(lineSplit[0])
		y, _ := strconv.Atoi(lineSplit[1])
		z, _ := strconv.Atoi(lineSplit[2])

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
	for i := range 1000 {
		connections = append(connections, []string{distances[i].a, distances[i].b})
	}

	for {
		mergedConnections := [][]string{}

		for i := 0; i < len(connections); i++ {
			for j := 0; j < len(connections[i]); j++ {
				mathFound := false
				for k := 0; k < len(mergedConnections); k++ {
					for l := 0; l < len(mergedConnections[k]); l++ {
						if connections[i][j] == mergedConnections[k][l] {
							mathFound = true
							break
						}
					}

					if mathFound {
						mergedConnections[k] = append(mergedConnections[k], connections[i]...)
						set := make(map[string]struct{})
						for z := 0; z < len(mergedConnections[k]); z++ {
							set[mergedConnections[k][z]] = struct{}{}
						}
						mergedConnections[k] = make([]string, 0, len(set))
						for x := range set {
							mergedConnections[k] = append(mergedConnections[k], x)
						}
						break
					}
				}
				if !mathFound {
					mergedConnections = append(mergedConnections, connections[i])
				}
			}
		}

		if len(connections) == len(mergedConnections) {
			break
		}

		connections = mergedConnections
	}

	sort.Slice(connections, func(i, j int) bool {
		return len(connections[i]) > len(connections[j])
	})

	for _, x := range connections[:3] {
		solution *= len(x)
	}

	fmt.Println(solution)
}
