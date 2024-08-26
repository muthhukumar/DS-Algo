/*
 * First solution. Very slow, more memory
 *
 */
function trap(height: number[]): number {
  let result = 0;

  let low = 0;
  let high = 0;

  for (let i = 0; i < height.length; i++) {
    if (height[i] > 0) {
      low = i;
      high = low + 1;
      break;
    }
  }

  let secondBiggest: number | null = null;

  while (low < height.length && high < height.length) {
    if (high === height.length - 1) {
      if (height[high] >= height[secondBiggest]) {
        secondBiggest = high;
      }

      result += calculateTrappedWater(height, low, secondBiggest);

      if (secondBiggest) {
        low = secondBiggest;
        high = low + 1;
        secondBiggest = null;
      }
    }

    if (height[high] >= height[low]) {
      result += calculateTrappedWater(height, low, high);

      low = high;
      high++;
      secondBiggest = null;
    } else {
      high++;

      if (secondBiggest === null || height[high] >= height[secondBiggest]) {
        secondBiggest = high;
      }

      continue;
    }
  }

  return result;
}

function calculateTrappedWater(
  height: number[],
  start: number,
  end: number,
): number {
  let result = 0;

  const min = Math.min(height[start], height[end]);

  for (let i = start + 1; i < end; i++) {
    if (height[i] <= min) {
      result += min - height[i];
    }
  }

  return result;
}
