/**
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
