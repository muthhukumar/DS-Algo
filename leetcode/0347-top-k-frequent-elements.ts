function topKFrequent(nums: number[], k: number): number[] {
  const hash = new Map()

  for (const num of nums) {
    hash.set(num, (hash.get(num) || 0) + 1)
  }

  const keys = [...hash.keys()]

  quickSort(keys, hash)

  return keys.slice(keys.length - k, keys.length)
}

function qs(nums: number[], lo: number, high: number, hash): void {
  if (lo >= high) {
    return
  }

  const partitionIdx = partition(nums, lo, high, hash)

  qs(nums, lo, partitionIdx - 1, hash)
  qs(nums, partitionIdx + 1, high, hash)
}

function partition(nums: number[], lo: number, high: number, hash): number {
  const pivot = nums[high]

  let idx = lo - 1

  for (let i = lo; i < high; ++i) {
    if (hash.get(nums[i]) <= hash.get(pivot)) {
      idx++
      const temp = nums[i]

      nums[i] = nums[idx]
      nums[idx] = temp
    }
  }

  idx++

  nums[high] = nums[idx]
  nums[idx] = pivot

  return idx
}

function quickSort(nums: number[], hash): void {
  qs(nums, 0, nums.length - 1, hash)
}
