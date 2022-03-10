package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func main() {
	res := new(ListNode)
	val := res
	val.Val = 1

	val.Next = new(ListNode)
	val = val.Next
	val.Val = 2

	val.Next = new(ListNode)
	val = val.Next
	val.Val = 3

	val.Next = new(ListNode)
	val = val.Next
	val.Val = 3

	val.Next = new(ListNode)
	val = val.Next
	val.Val = 4

	val.Next = new(ListNode)
	val = val.Next
	val.Val = 4

	val.Next = new(ListNode)
	val = val.Next
	val.Val = 5

	a := deleteDuplicates(res)

	fmt.Println(a)
	fmt.Println(a.Next)
	fmt.Println(a.Next.Next)
}

func deleteDuplicates(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}

	res := new(ListNode)
	val := res
	cache := -101

		for {
			if head == nil {
				return res.Next
			}

			if head.Next == nil {
				val.Next = new(ListNode)
				val = val.Next
				val.Val = head.Val
				return res.Next
			}

			cache = head.Val

			if head.Val == head.Next.Val {
				for {
					if head == nil {
						return res.Next
					}

					if head.Next == nil {
						// val.Val = head.Val
						return res.Next
					}

					head = head.Next

					if head.Val != cache {
						// fmt.Println("111",head)
						break
					}
				}
			} else {
				val.Next = new(ListNode)
				val = val.Next
				val.Val = cache
				head = head.Next
			}
		}
}


