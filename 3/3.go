package main

import (
	"fmt"
	"math"
	"strings"
)

const bank_size = 15 // TODO! change me to 100 for the real input!
const required_batteries = 12

func main() {
	input := `987654321111111
811111111111119
234234234234278
818181911112111`
	// input_bytes, _ := (os.ReadFile("input"))
	// input := string(input_bytes)
	lines := strings.Split(input, "\n")
	sum := 0
	for _, line := range lines {
		//turn input to list of ints
		bank_as_runes := []rune(line)
		var bank [bank_size]int
		for i := range bank {
			bank[i] = int(bank_as_runes[i] - 48)
		}
		// choose the 12 right most digits
		// 234234234234278		818181911112111
		//    ************		   ************

		var chosen [required_batteries]int
		for i := range required_batteries {
			chosen[i] = i + (bank_size - required_batteries)
		}
		for i := range chosen {
			//move the left most chosen index to the biggest value batterie
			// 234234234234278		818181911112111
			//   * *********** 		*  ************

			right_bound := chosen[i]
			var left_bound int
			if i == 0 {
				left_bound = 0
			} else {
				left_bound = chosen[i-1] + 1
			}
			for availabe_index := right_bound; availabe_index >= left_bound; availabe_index-- {
				// 234234234234278		818181911112111
				//    ************		   ************
				// ^^^^					^^^^			 <- availabe indexes to choose
				if bank[availabe_index] >= bank[chosen[i]] {
					chosen[i] = availabe_index
				}
			}
			// final choices:
			// 234234234234278		818181911112111
			//   * ***********		* * * *********
		}
		// convert chosen indexes to joltage
		var joltage int
		for i, choice := range chosen {
			exponent := required_batteries - i - 1
			joltage += bank[choice] * int(math.Pow10(exponent))
		}
		sum += joltage
		fmt.Println(joltage)
	}
	fmt.Printf("\nsum: %d", sum)
}
