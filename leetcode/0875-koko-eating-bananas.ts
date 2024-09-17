function minEatingSpeed(piles: number[], h: number): number {
  let low = 1;
  let high = max(piles);

  while (low < high) {
    const k = Math.floor(low + (high - low) / 2);

    let hours = 0;

    for (let pile of piles) {
      hours += Math.floor(pile / k);
      hours += pile % k > 0 ? 1 : 0;
    }

    if (hours <= h) {
      high = k;
    } else {
      low = k + 1;
    }
  }

  return low;
}

function max(nums: number[]) {
  let max = nums[0];

  for (let i = 1; i < nums.length; i++) {
    if (nums[i] > max) {
      max = nums[i];
    }
  }

  return max;
}
