function search(nums: number[], target: number): number {
  let lo = 0;
  let high = nums.length;

  while (lo < high) {
    const middle = Math.floor(lo + (high - lo) / 2);

    const value = nums[middle];

    if (value === target) {
      return middle;
    } else if (target < value) {
      high = middle;
    } else {
      lo = middle + 1;
    }
  }

  return -1;
}
