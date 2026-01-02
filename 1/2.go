package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	// input_bytes, err := os.ReadFile("input")
	// if err != nil {
	// 	fmt.Errorf("cannot read file")
	// }
	// input := string(input_bytes)
	input := `L68
	L30
	R48
	L5
	R60
	L55
	L1
	L99
	R14
	L82`
	replacer := strings.NewReplacer(
		"R", "",
		"L", "-",
	)
	input = replacer.Replace(input)

	var rotation int64 = 50
	times_zero := 0
	for line := range strings.Lines(input) {
		line = strings.Trim(line, "\n")
		n, err := strconv.ParseInt(line, 10, 64)
		if err != nil {
			fmt.Printf("\"%s\" cannot be parsed", line)
		}
		rotation += n
		for rotation < 0 {
			rotation += 100
			times_zero++
		}
		for rotation >= 100 {
			rotation -= 100
			times_zero++
		}
		fmt.Println("---")
		fmt.Println(rotation)
		fmt.Println(times_zero)
	}
}
