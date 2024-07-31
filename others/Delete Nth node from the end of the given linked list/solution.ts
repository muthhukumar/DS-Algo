class SLNode {
  next: SLNode | null;
  value: number;
  constructor(val: number) {
    this.next = null;
    this.value = val;
  }
}

const head = new SLNode(1);
const n2 = new SLNode(2);
const n3 = new SLNode(3);
const n4 = new SLNode(4);
const n5 = new SLNode(5);

head.next = n2;
n2.next = n3;
n3.next = n4;
n4.next = n5;

function findLength(head: SLNode | null) {
  let iter: SLNode | null = head;

  let count = 0;

  while (iter != null) {
    count += 1;
    iter = iter.next;
  }

  return count;
}

function deleteNthNodeFromEnd(head: SLNode | null, nth: number): number | null {
  const length = findLength(head);

  const nthElementFromStart = length - nth + 1;

  let curr: null | SLNode = null;

  let iter: SLNode | null = head;

  console.log("nth element from start", nthElementFromStart);

  for (let i = 1; i < nthElementFromStart; i++) {
    curr = iter;
    iter = iter?.next ?? null;
  }

  let result: null | number = null;

  if (curr === null) {
    result = head?.value ?? null;

    if (iter?.next) iter.next = null;
  } else {
    result = iter?.value ?? null;

    curr.next = curr?.next?.next ?? null;
  }

  return result;
}

function expect(expected: any, message?: string) {
  return {
    toBe(actual: any) {
      if (expected !== actual) {
        throw new Error(
          `Expected: ${actual} but found ${expected}, Message: ${message}`,
        );
      }

      console.log(`Passed: ${message}`);
    },
  };
}

function printList(head: SLNode | null) {
  while (head !== null) {
    console.log(head.value);
    head = head.next;
  }
}

expect(findLength(head), "length should be 5").toBe(5);

expect(deleteNthNodeFromEnd(head, 3), "remove value should be 3").toBe(3);
printList(head);
expect(deleteNthNodeFromEnd(head, 4), "remove value should be 1").toBe(1);
printList(head);
printList(n2);
