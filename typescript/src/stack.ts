type Node<T> = {
  value: T,
  next?: Node<T>,
}

export default class Stack<T> {
  public length: number;
  private head?: Node<T>;

  constructor() {
    this.head = undefined;
    this.length = 0;
  }

  push(item: T): void {
    let newNodo = { value: item } as Node<T>;
    
    this.length++;

    // if (this.length === 0) {
    if (!this.head) {
      this.head = newNodo;
      return;
    }

    newNodo.next = this.head;
    this.head = newNodo;
  }
  
  pop(): T | undefined {
    let curHead = this.head;
    if (!curHead) {
      return undefined;
    }

    this.length--;

    this.head = curHead.next;

    // free() or similar on lower lvl languages (with no G.C.)
    curHead.next = undefined;

    return curHead.value;
  }
  
  peek(): T | undefined {
    // in like rust we shhould return a NOT mut pointer... i think
    return this.head?.value;
  }
}
