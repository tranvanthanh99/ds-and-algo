package lru

// runtime: 397 ms, memory: 67.61 MB
type LinkedNode struct {
	key  int
	Val  int
	Next *LinkedNode
	Prev *LinkedNode
}

type LRUCache struct {
	cap    int
	length int
	cache  map[int]*LinkedNode
	head   *LinkedNode
	tail   *LinkedNode
}

func Constructor(capacity int) LRUCache {
	node := LinkedNode{key: -1, Val: -1, Next: nil, Prev: nil}
	return LRUCache{
		cap:   capacity,
		cache: make(map[int]*LinkedNode),
		head:  &node,
		tail:  &node,
	}
}

func (this *LRUCache) Get(key int) int {
	v, ok := this.cache[key]

	if !ok {
		return -1
	}

	if v.Next != nil {
		v.Prev.Next = v.Next
		v.Next.Prev = v.Prev

		v.Prev = this.tail
		v.Next = nil

		this.tail.Next = v
		this.tail = v
	}

	return v.Val
}

func (this *LRUCache) Put(key int, value int) {
	v, ok := this.cache[key]

	if !ok {
		node := LinkedNode{
			key:  key,
			Val:  value,
			Prev: this.tail,
			Next: nil,
		}
		this.cache[key] = &node
		this.tail.Next = &node
		this.tail = &node

		if this.length < this.cap {
			this.length++
			return
		}

		delete(this.cache, this.head.Next.key)

		this.head.Next.Next.Prev = this.head
		this.head.Next = this.head.Next.Next

		return
	}

	v.Val = value
	if v.Next != nil {
		v.Prev.Next = v.Next
		v.Next.Prev = v.Prev

		v.Prev = this.tail
		v.Next = nil

		this.tail.Next = v
		this.tail = v
	}
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * obj := Constructor(capacity);
 * param_1 := obj.Get(key);
 * obj.Put(key,value);
 */
