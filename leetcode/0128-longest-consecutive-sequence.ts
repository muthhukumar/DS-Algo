/**
 * After Watching the solution
 * @param {number[]} nums
 * @return {number}
 */
var longestConsecutive = function (nums) {
  const set = new Set(nums);

  let result = 0;

  for (const n of nums) {
    if (!set.has(n - 1)) {
      let count = 1;

      while (set.has(n + count)) {
        count += 1;
      }

      result = Math.max(result, count);
    }
  }

  return result;
};

/**
 * Initial solution without watching solution
 * @param {number[]} nums
 * @return {number}
 */
var longestConsecutive = function (nums) {
  const hashmap = {};

  for (let i = 0; i < nums.length; i++) {
    if (!(nums[i] in hashmap)) {
      hashmap[nums[i]] = i;
    }
  }

  // I later realized this is unnecessary. Tried to be smart but didn't worked out. This is a shortcut to the solution.
  for (let i = 0; i < nums.length; i++) {
    if (!(nums[i] - 1 in hashmap) && !(nums[i] + 1 in hashmap)) {
      delete hashmap[nums[i]];
    }
  }

  let result = 0;

  let min = null;
  let max = null;

  for (let i = 0; i < nums.length; i++) {
    let start = nums[i];

    if (min && max && start > min && start < max) continue;

    let count = 1;

    while (start + 1 in hashmap) {
      start = start + 1;
      count += 1;
    }

    if (count > result) {
      min = nums[i];
      max = start;
    }

    result = Math.max(result, count);
  }

  return result;
};
