
// #[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}


// Queue
// - Specific implementation of a single linked list
// - First in, first out ---- A line!
struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> Queue<T> {

    pub fn new(&mut self) -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }    
    }

    // Adds a node at the end of the que
    pub fn enqueue(&mut self, data: T) {
        // current tail, point to E
        // and set the tail to E
        
        // self.tail.next = T /* our last element had tail None, so now points to our new elemnt */
        // self.tail = T /* And set our tail as the new element */

        // let cur_tail = self.tail.take();

        // let new_node = Box::new(Node {
        //     value: data,
        //     next: None,
        // });

        // if let Some(mut tail) = self.tail.take() {
        //     tail.next = Some(new_node);
        // } else {
        //     self.head = Some(new_node);
        //     self.tail = self.head;
        // }
        // self.length += 1;

        // self.length += 1;

        // let cur_tail = self.tail.take();

        // if let Some(mut tail) = cur_tail {
        //     tail.next = new;
        //     self.tail = tail.next;
        // } else {
        //     self.tail = new;
        // }
    }

    // Removes first Element, and returns its value
    pub fn deque(&mut self) -> Option<T> {
        /*
        * let curHead = self.head -- Save reference to our current head 
        * self.head = curHead.next -- Here we set the new head, to the elemnt our 'old' head was
        * pointing
        * curHead.next = None
        * return curHead.value
        */
        // if self.head.is_none() {
        //     return None
        // }

        // self.length -= 1;

        // // Current head, save value and just unwrap it since we check is_none before
        // let mut head = self.head.take().unwrap();

        // // This is why i like Rust, take() will
        // // - set head.next to None, i think
        // // - return the value wrapped in an Option if exists
        // self.head = head.next;

        // // self.head = match(head.next.take()) {
        // //     Some(node) => node,
        // //     None => None,
        // // };
        // // i think i dont need to do this sicne head.next.take() should have done that?
        // head.next = None;
        // 
        // Some(head.value)
        None
    }

    // Returns the current (first) elemnt
    pub fn peek(&self) -> Option<&T> {
        /*
        * Returning a reference allows us to avoid copying the value of the head node, which could be expensive for large or complex types.
        * It also ensures that the caller cannot mutate the value of the head node directly, which could lead to unexpected behavior or data corruption.
        *
        * Note that returning a reference also requires us to ensure that the lifetime of the reference is valid, which can be tricky in some cases.
        * In this example, we're returning a reference to a field of a struct that we know will live as long as the linked list itself, so the lifetime
        * of the reference is guaranteed to be valid
        */
        // match &self.head {
        //     Some(node) => Some(&node.value),
        //     None => None,
        // }

        None
    }
}

/*
About BOX:

In Rust, Box is a smart pointer that is used to allocate values on the heap rather than the stack.
When a value is allocated on the stack, it is stored directly in the memory allocated for the current function call
and its lifetime is tied to the lifetime of the function. When a value is allocated on the heap, however, it can be accessed
and shared across multiple functions or threads, and its lifetime is managed by Rust's memory management system.

In the implementation of a singly linked list in Rust, we use Box to allocate each node of the list on the heap.
This is because the size of each node is not known at compile time and can vary depending on the size of the data
it holds, and therefore it cannot be stored on the stack.

Since each node in a singly linked list points to the next node in the list, it needs to hold a reference to the nex
node's memory location. We could use a raw pointer (*mut Node<T>) to represent this reference, but that would not be safe
because it could be null, dangling or incorrectly aliased.

Instead, we use a Box smart pointer to wrap each node and allocate its memory on the heap. A Box pointer guarantees that the memory
it points to is valid, non-null and non-dangling, and it is automatically deallocated when the Box goes out of scope.

Therefore, using Box is a safe way to allocate memory for the nodes of a singly linked list and ensure that each node points to a valid and owned memory location.

Note that there are other smart pointers in Rust, such as Rc and Arc, that can also be used to allocate values on the heap and share
ownership of them, but they are not suitable for a singly linked list, since they do not allow for mutable or exclusive ownership.
*/
