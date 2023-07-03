class QNode<T> {
  value: T;
  next?: QNode<T>;
  constructor(value: T, next?: QNode<T>) {
    this.value = value;
    this.next = next;
  }
}

export default class Queue<T> {
  head?: QNode<T>;
  tail?: QNode<T>;
  length: number;
  constructor() {
    this.head = this.tail = undefined;

    this.length = 0;
  }

  enqueue(value: T): void {
    this.length++;
    const node = new QNode(value);

    if (!this.tail) {
      this.head = this.tail = node;

      return;
    }

    this.tail.next = node;

    this.tail = node;
  }

  deque(): T | undefined {
    if (!this.head) {
      this.tail = undefined;
      this.length = 0;
      return undefined;
    }

    this.length--;

    const head = this.head;
    this.head = this.head.next;

    head.next = undefined;

    if (this.length === 0) {
      this.tail = undefined;
    }

    return head.value;
  }

  peek(): T | undefined {
    return this.head?.value;
  }
}
