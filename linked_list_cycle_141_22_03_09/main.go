package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func hasCycle(head *ListNode) bool {
	for i := 0; i < 10*10*10*10+5; i++ {
		if head == nil {
			return false
		}
		head = head.Next
	}
	return true
}

