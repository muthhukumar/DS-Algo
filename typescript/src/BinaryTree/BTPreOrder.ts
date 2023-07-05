function walk(head: BinaryNode<number> | null, path: number[]): number[] {
  if (!head) {
    return path;
  }

  path.push(head.value);
  walk(head.left, path);
  walk(head.right, path);

  return path;
}

export default function preOrderSearch(head: BinaryNode<number>): number[] {
  return walk(head, []);
}
