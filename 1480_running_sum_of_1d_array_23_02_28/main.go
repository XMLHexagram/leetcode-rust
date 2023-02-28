package main

func runningSum(nums []int) []int {
	temp := 0
	res := make([]int, len(nums))

	for index, num := range nums {
		temp += num
		res[index] = temp
	}

	return res
}
