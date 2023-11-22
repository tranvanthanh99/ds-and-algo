package main

/**
 * Definition for a Node.
 */
type Node struct {
	Val    int
	Next   *Node
	Random *Node
}

func copyRandomList(head *Node) *Node {
	if head == nil {
		return nil
	}

	// Step 1: Create new nodes
	for current := head; current != nil; current = current.Next.Next {
		newNode := &Node{Val: current.Val, Next: current.Next}
		current.Next = newNode
	}

	// Step 2: Set random pointers
	for current := head; current != nil; current = current.Next.Next {
		if current.Random != nil {
			current.Next.Random = current.Random.Next
		}
	}

	// Step 3: Separate the two lists
	newHead := head.Next
	for current := head; current != nil; current = current.Next {
		temp := current.Next
		current.Next = current.Next.Next
		if temp.Next != nil {
			temp.Next = temp.Next.Next
		}
	}

	return newHead
}
