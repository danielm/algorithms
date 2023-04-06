type Node<T> = {
  value: T,
  next?: Node<T>,
}



export default class Queue<T> {
  public length: number;
  private head?: Node<T>;
  private tail?: Node<T>;

  constructor() {
    this.head = this.tail = undefined;
    this.length = 0;
  }

  enqueue(item: T): void {
    // current tail, point to E
    // and set the tail to E
    
    // self.tail.next = T /* our last element had tail None, so now points to our new elemnt */
    // self.tail = T /* And set our tail as the new element */
    let newNodo = { value: item } as Node<T>;
    
    this.length++;

    // if (this.length === 0) {
    if (!this.tail) {
      this.tail = this.head = newNodo;
      return;
    }

    this.tail.next = newNodo;
    this.tail = newNodo;

    // This fixes the 69 test, but the best solution seems to add the last condition in deque
    // if (!this.head) {
    //   this.head = newNodo;
    // }

  }
  
  deque(): T | undefined {
    /*
     * let curHead = self.head -- Save reference to our current head 
     * self.head = curHead.next -- Here we set the new head, to the elemnt our 'old' head was
     * pointing
     * curHead.next = None
     * return curHead.value
     */

    let curHead = this.head;
    if (!curHead) {
      return undefined;
    }

    this.length--;

    this.head = curHead.next;

    // free() or similar on lower lvl languages (with no G.C.)
    curHead.next = undefined;

    if (this.length == 0) {
      this.tail = undefined;
    }

    return curHead.value;
  }
  
  peek(): T | undefined {
    // if (!this.head) {
    //   return undefined;
    // }

    // return this.head.value;
    
    // in like rust we shhould return a NOT mut pointer... i think
    return this.head?.value;
  }
}
