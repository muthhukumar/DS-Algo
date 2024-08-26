function trap(height) {
  if (height.length <= 2) return 0;

  let result = 0;

  const arr = [];

  let lastSmallest = null;
  let lastGreatest = null;
  let isLastSmall = null;

  if (height[0] === 0) {
    lastSmallest = height[0];
    isLastSmall = true;
  } else {
    arr.push(0);
    lastGreatest = height[0];
    isLastSmall = false;
  }

  for (let i = 1; i < height.length; i++) {
    if (isLastSmall && height[i] >= lastSmallest) {
      arr.push(i);
      lastGreatest = height[i];

      isLastSmall = false;
    } else if (height[i] >= lastGreatest) {
      arr.push(i);
      lastGreatest = height[i];

      isLastSmall = false;
    } else {
      lastSmallest = height[i];
      isLastSmall = true;
    }
  }

  console.log(arr);
  for(const n of arr) {
    console.log(height[n])
  }

  let low = arr[0];
  let high = arr[1];

  let i = 1;
  while (low < high) {
    i += 1;

    while (high + 1 === arr[i]) {
      high += 1;
      i += 1;
    }

    console.log(low, high);

    result += calculateTrappedWater(height, low, high);

    low = high;
    high = arr[i];

    console.log("after", low, high);
  }

  return result;
}

//console.log(trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]) === 6);
//console.log(trap([9, 6, 8, 8, 5, 6, 3]) === 3);
console.log(trap([4, 2, 0, 3, 2, 5]) === 9);
//console.log(trap([4, 2, 3]) === 1);
//console.log(trap([5, 4, 1, 2]) === 1);

function calculateTrappedWater(height, start, end) {
  let result = 0;

  const min = Math.min(height[start], height[end]);

  for (let i = start + 1; i < end; i++) {
    if (height[i] <= min) {
      result += min - height[i];
    }
  }

  return result;
}
