import { Queue } from "~/DS";

export default function bfs(head: BinaryNode<number>, needle: number): boolean {
  const q = [head];

  while (q.length) {
    const val = q.shift();

    if (!val) continue;

    if (val.value === needle) return true;

    q.push(val.left);
    q.push(val.right);
  }

  return false;
}
