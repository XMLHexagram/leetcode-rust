package main

import (
	"fmt"
	"sort"
)

func main() {
	fmt.Println(test([]int{3, 4, 3, 2, 4}))
	fmt.Println(test([]int{1, 6, 3, 2, 4}))
}

func test(buildings []int) (res int) {
	cache := make([]int, len(buildings))
	now := 0

	for k, v := range buildings {
		temp := now - v
		now = v
		res += temp
		cache[k] = res

	}

	fmt.Println(cache)
	sort.Ints(cache)
	fmt.Println(cache)

	return -cache[0]
}
