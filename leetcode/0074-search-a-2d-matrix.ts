function searchMatrix(matrix: number[][], target: number): boolean {
  const rowSize = matrix[0].length;

  let low = 0;
  let high = matrix.length * rowSize;

  while (low < high) {
    const middle = Math.floor(low + (high - low) / 2);

    const row = Math.floor(middle / rowSize);
    const col = middle % rowSize;

    const value = matrix[row][col];

    if (value === target) {
      return true;
    } else if (target < value) {
      high = middle;
    } else {
      low = middle + 1;
    }
  }

  return false;
}
