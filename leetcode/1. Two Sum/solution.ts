function twoSum(nums: number[], target: number): number[] {
  for (let i = 0; i < nums.length; i++) {
    for (let j = i + 1; j < nums.length; j++) {
      if (nums[i] + nums[j] === target) {
        return [i, j]
      }
    }
  }

  return [-1, -1]
}

// Solution 2: using hashmap
function twoSumHashMap(nums: number[], target: number): number[] {
    const hashmap = {}

    for (let i = 0; i < nums.length; i++) {
        const diff = target - nums[i]

        if (hashmap[diff] >= 0) {
            return [hashmap[diff], i]
        } else {
            hashmap[nums[i]] = i
        }
    }

    return [-1, -1]
};
