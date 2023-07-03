class ListNode<T> {
  prev: ListNode<T> | undefined;
  next: ListNode<T> | undefined;
  value: T;
  constructor(value: T) {
    this.value = value;
    this.prev = this.next = undefined;
  }
}

export default class DoublyLinkedList<T> {
  private head: ListNode<T> | undefined;
  private tail: ListNode<T> | undefined;
  public length: number;

  constructor() {
    this.head = this.tail = undefined;
    this.length = 0;
  }

  append(item: T): void {
    this.length++;

    const node = new ListNode(item);
    if (!this.tail) {
      this.tail = this.head = node;
      return;
    }

    const curr = this.tail;

    node.prev = curr;
    curr.next = node;

    this.tail = node;
  }

  prepend(item: T): void {
    this.length++;
    const node = new ListNode(item);

    if (!this.head) {
      this.head = this.tail = node;
      return;
    }

    const curr = this.head;

    node.next = curr;
    curr.prev = node;

    this.head = node;
  }

  insertAt(item: T, idx: number): void {
    if (idx === 0) {
      this.prepend(item);
      return;
    } else if (idx === this.length) {
      this.append(item);
      return;
    }

    let curr = this.getAt(idx);

    if (!curr) {
      throw new Error("can't possibly insert there");
    }

    this.length++;

    const node = new ListNode(item);

    node.next = curr;
    node.prev = curr.prev;

    if (curr.prev) {
      curr.prev.next = node;
    }

    curr.prev = node;
  }

  remove(item: T): T | undefined {
    let curr = this.head;

    for (let i = 0; curr && i < this.length; ++i) {
      if (item === curr.value) {
        break;
      }

      curr = curr.next;
    }

    if (!curr) {
      return;
    }

    this.length--;

    const value = curr.value;

    if (curr === this.tail) {
      this.tail = this.tail.prev;
      return value;
    }

    if (curr === this.head) {
      this.head = this.head.next;
      return value;
    }

    if (curr.prev) {
      curr.prev.next = curr.next;
    }

    if (curr.next) {
      curr.next.prev = curr.prev;
    }

    return value;
  }

  removeAt(idx: number): T | undefined {
    const curr = this.getAt(idx);

    if (!curr) {
      return undefined;
    }

    this.length--;

    let value = curr.value;

    if (curr === this.tail) {
      this.tail = this.tail.prev;
      return value;
    }

    if (curr === this.head) {
      this.head = this.head.next;
      return value;
    }

    if (curr.prev) {
      curr.prev.next = curr.next;
    }

    if (curr.next) {
      curr.next.prev = curr.prev;
    }

    return value;
  }

  get(idx: number): T | undefined {
    return this.getAt(idx)?.value;
  }

  getAt(idx: number): ListNode<T> | undefined {
    let curr = this.head;

    for (let i = 0; curr && i < idx; ++i) {
      curr = curr.next;
    }

    return curr;
  }
}
