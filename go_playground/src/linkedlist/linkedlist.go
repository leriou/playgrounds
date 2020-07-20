package linkedlist

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseList(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}
	var pre *ListNode
	cur := head
	nxt := head
	for cur != nil {
		nxt = nxt.Next
		cur.Next = pre
		pre = cur
		cur = nxt
	}
	return pre
}
