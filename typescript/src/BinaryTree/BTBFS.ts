import { Queue } from "~/DS";

export default function bfs(head: BinaryNode<number>, needle: number): boolean {
  const queue = new Queue<BinaryNode<number>>();

  queue.enqueue(head);

  while (queue.length) {
    let head = queue.deque();

    if (!head) {
      continue;
    }

    if (head.value === needle) {
      return true;
    }

    queue.enqueue(head.left);
    queue.enqueue(head.right);
  }

  return false;
}
