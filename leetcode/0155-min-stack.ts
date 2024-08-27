/*
 * Final solution only with next
 */

class LNode<T> {
  min: T;
  value: T;
  next: LNode<T> | null;
  constructor(value: T) {
    this.min = value;
    this.value = value;
    this.next = null;
  }
}

class MinStack {
  head: LNode<number> | null;
  constructor() {
    this.head = null;
  }

  push(val: number): void {
    const node = new LNode(val);

    if (!this.head) {
      this.head = node;

      return;
    }

    const head = this.head;

    node.min = Math.min(head.min, node.min);

    node.next = head;

    this.head = node;
  }

  pop(): void {
    if (!this.head) throw new Error("Head should never be null");

    const head = this.head;
    const next = head.next;

    if (!next) {
      this.head = null;
    } else {
      head.next = null;

      this.head = next;
    }
  }

  top(): number {
    if (!this.head) throw new Error("Head should never be null");

    return this.head.value;
  }

  getMin(): number {
    if (!this.head) throw new Error("Head should never be null");

    return this.head.min;
  }
}

/*
 * Initial solution with Prev and Next.
 */
class LNode2<T> {
  min: T;
  value: T;
  prev: LNode2<T> | null;
  next: LNode2<T> | null;
  constructor(value: T) {
    this.min = value;
    this.value = value;
    this.prev = null;
    this.next = null;
  }
}

class MinStack2 {
  head: LNode2<number> | null;
  constructor() {
    this.head = null;
  }

  push(val: number): void {
    const node = new LNode2(val);

    if (!this.head) {
      this.head = node;

      return;
    }

    const head = this.head;

    if (head.min < node.min) {
      node.min = head.min;
    }

    head.next = node;
    node.prev = head;

    this.head = node;
  }

  pop(): void {
    if (!this.head) throw new Error("Head should never be null");

    const head = this.head;
    const prev = head.prev;

    if (!prev) {
      this.head = null;
    } else {
      prev.next = null;
      head.prev = null;

      this.head = prev;
    }
  }

  top(): number {
    if (!this.head) throw new Error("Head should never be null");

    return this.head.value;
  }

  getMin(): number {
    if (!this.head) throw new Error("Head should never be null");

    return this.head.min;
  }
}
