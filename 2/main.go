package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

func main() {
	input := "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
	acc := int64(0)
	for e := range strings.SplitSeq(input, ",") {
		ab := strings.Split(e, "-")
		a, e := strconv.ParseInt(ab[0], 10, 0)
		if e != nil {
			fmt.Println("bad input")
		}
		b, e := strconv.ParseInt(ab[1], 10, 0)
		if e != nil {
			fmt.Println("bad input")
		}
		for i := a; i <= b; i++ {
			if is_valid(i) {
				fmt.Printf("%d ", i)
				acc += i
			}
		}
		fmt.Println()
	}
	fmt.Println(acc)
}
func is_valid(n int64) bool {
	numdigits := int(math.Ceil(math.Log10(float64(n))))
	nstr := fmt.Sprintf("%d", n)
	for groupsize := numdigits / 2; groupsize > 0; groupsize-- {
		if numdigits%groupsize != 0 {
			continue
		}
		substr := nstr[0:groupsize]
		all := true
		for group_start_index := groupsize; group_start_index < numdigits; group_start_index += groupsize {
			window := nstr[group_start_index : group_start_index+groupsize]
			if substr != window {
				all = false
			}
		}
		if all == true {
			return true
		}
	}
	return false
}
