function isPalindrome(x: number): boolean {
  if (x < 0) return false
  if (x <= 9) return true

  let num = x

  let nums: Array<number> = []

  while (num != 0) {
    nums.push(num % 10)
    num = Math.floor(num / 10)
  }

  let low = 0
  let high = nums.length - 1

  while (low < high) {
    if (nums[low] != nums[high]) {
      return false
    }

    low += 1
    high -= 1
  }

  return true
}
