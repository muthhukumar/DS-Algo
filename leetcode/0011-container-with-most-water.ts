/*
 * First solution. More memory, Less speed
 */
function maxArea0(height: number[]): number {
  let low = 0;
  let high = height.length - 1;

  let result = 0;

  while (low < high) {
    const [left, right] = [
      { value: height[low], idx: low, id: "low" },
      { idx: high, value: height[high], id: "high" },
    ];

    const min = left.value > right.value ? right : left;

    const mostWater = min.value * (right.idx - left.idx);

    if (mostWater > result) {
      result = mostWater;
    }

    if (min.id === "low") {
      low++;
    } else {
      high--;
    }
  }

  return result;
}

/*
 * Fast, Bit memory
 */
function maxArea1(height: number[]): number {
  let low = 0;
  let high = height.length - 1;

  let result = 0;

  while (low < high) {
    let mostWater = 0;

    const lv = height[low];
    const hv = height[high];

    if (lv < hv) {
      mostWater = lv * (high - low);
      low++;
    } else {
      mostWater = hv * (high - low);
      high--;
    }

    if (mostWater > result) {
      result = mostWater;
    }
  }

  return result;
}

/*
 * Less memory, medium speed
 */
function maxArea3(height: number[]): number {
  let low = 0;
  let high = height.length - 1;

  let result = 0;

  while (low < high) {
    const lv = height[low];
    const hv = height[high];

    result = Math.max(Math.min(lv, hv) * (high - low), result);

    if (lv < hv) {
      low++;
    } else {
      high--;
    }
  }

  return result;
}
