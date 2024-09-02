function generateParenthesis(n: number): string[] {
  const result: Array<string> = [];

  function backtrack(left: number, right: number, path: string) {
    if (left === n && right === n) {
      result.push(path);
      return;
    }

    if (left < n) {
      backtrack(left + 1, right, path + "(");
    }

    if (right < left) {
      backtrack(left, right + 1, path + ")");
    }
  }

  backtrack(0, 0, "");

  return result;
}
