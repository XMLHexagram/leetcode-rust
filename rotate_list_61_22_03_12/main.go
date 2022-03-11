package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func rotateRight(head *ListNode, k int) *ListNode {
	if head == nil || k == 0 {
		return head
	}
	var count int
	var tail *ListNode
	for p := head; p != nil; p = p.Next {
		count++
		tail = p
	}
	tail.Next = head
	k = count - (k % count )
	for i := 0; i < k; i++ {
		tail = tail.Next
	}
	head = tail.Next
	tail.Next = nil
	return head
}
