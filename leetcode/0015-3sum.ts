/*
 * Final solution with the help of checking other solutions
 */
function threeSum(nums: number[]): number[][] {
  nums = nums.sort((a, b) => a - b);

  const result = new Set<string>();

  for (let i = 0; i < nums.length - 2; i++) {
    if (i > 0 && nums[i] === nums[i - 1]) continue;

    let low = i + 1;
    let high = nums.length - 1;

    while (low < high) {
      const sum = nums[i] + nums[low] + nums[high];

      if (sum === 0) {
        result.add(JSON.stringify([nums[i], nums[low], nums[high]]));
        low++;
        high--;
      } else if (sum > 0) {
        high--;
      } else {
        low++;
      }
    }
  }

  return [...result].map((num) => JSON.parse(num));
}

/*
 * Initial successful solution
 */

function threeSumFirst(nums: number[]): number[][] {
  sort(nums);

  const result = new Set<string>();

  for (let i = 0; i < nums.length - 2; i++) {
    const indexes = twoSum(nums, i + 1, 0 - nums[i]);

    if (indexes.length) {
      indexes.forEach((idx) => {
        result.add(`${nums[i]}|${nums[idx[0]]}|${nums[idx[1]]}`);
      });
    }
  }

  return [...result].map((num) => {
    return num.split("|").map(Number);
  });
}

function twoSum(
  nums: number[],
  startIdx: number,
  target: number,
): Array<[number, number]> {
  let low = startIdx;
  let high = nums.length - 1;

  const result = [] as Array<[number, number]>;

  while (low < high) {
    const sum = nums[low] + nums[high];

    if (sum === target) {
      result.push([low, high]);
      low++;
      high--;
    } else if (sum > target) {
      high--;
    } else {
      low++;
    }
  }

  return result;
}

function sort(nums: number[]) {
  for (let i = 0; i < nums.length; i++) {
    for (let j = i + 1; j < nums.length; j++) {
      if (nums[i] > nums[j]) {
        const temp = nums[i];

        nums[i] = nums[j];
        nums[j] = temp;
      }
    }
  }
}
