package main

/**
 * Definition for singly-linked list.
 */

type ListNode struct {
	Val  int
	Next *ListNode
}

func hasCycle(head *ListNode) bool {
	return solution_v1(head)
}

func solution_v1(head *ListNode) bool {
	if head == nil || head.Next == nil {
		return false
	}
	slow := head.Next
	fast := head.Next.Next

	for {
		if fast == nil || slow == fast {
			break
		}

		slow = slow.Next
		if fast.Next == nil {
			fast = nil
		} else {
			fast = fast.Next.Next
		}
	}

	return slow == fast
}

func solution_v2(head *ListNode) bool {
	n := head
	for n != nil && n.Next != nil {
		n = n.Next.Next
		head = head.Next

		if n == head {
			return true
		}
	}

	return false
}
