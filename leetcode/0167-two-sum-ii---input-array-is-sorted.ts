function twoSum(numbers: number[], target: number): number[] {
  let low = 0;
  let high = numbers.length - 1;

  while (low < high) {
    const sum = numbers[low] + numbers[high];

    if (sum === target) {
      return [low + 1, high + 1];
    } else if (sum > target) {
      high--;
    } else {
      low++;
    }
  }

  return [-1, -1];
}
