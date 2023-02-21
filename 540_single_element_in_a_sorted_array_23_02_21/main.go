package main

func singleNonDuplicate(nums []int) int {
	// use human index
	getAnother := func(humanIndex int) int {
		if humanIndex%2 == 0 {
			return humanIndex - 1
		} else {
			return humanIndex + 1
		}
	}

	left, right := 1, len(nums) + 1

	for left < right {
		mid := (left + right) / 2

		if (mid-1 < 1 || nums[mid-1-1] != nums[mid-1]) && (mid+1 > len(nums) || nums[mid+1-1] != nums[mid-1]) {
			return mid - 1
		}

		if nums[mid-1] != nums[getAnother(mid) - 1] {
			right = mid - 1
		} else {
			left = mid
		}
	}

	return -1
}
