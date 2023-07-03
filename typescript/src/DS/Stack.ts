class SNode<T> {
  value: T;
  prev?: SNode<T>;
  constructor(value: T, prev?: SNode<T>) {
    this.value = value;
    this.prev = prev;
  }
}

export default class Stack<T> {
  length: number;
  head?: SNode<T>;

  constructor() {
    this.head = undefined;
    this.length = 0;
  }

  push(value: T): void {
    const node = new SNode(value);

    this.length++;

    if (!this.head) {
      this.head = node;

      return;
    }

    node.prev = this.head;

    this.head = node;
  }

  pop(): T | undefined {
    if (!this.head) {
      return undefined;
    }

    this.length--;

    const head = this.head;

    this.head = this.head.prev;

    return head.value;
  }

  peek(): T | undefined {
    return this.head?.value;
  }
}
