function longestCommonPrefix(strs: string[]): string {
  let smallest: null | string = null

  for (const str of strs) {
    if (!smallest) {
      smallest = str
    } else if (smallest.length <= str.length) {
      smallest = str
    }
  }

  let pattern = ""

  for (let j = 1; smallest && j <= smallest.length; j++) {
    const substr = smallest.substring(0, j)

    let stop = false

    for (const str of strs) {
      if (str.substring(0, j) !== substr) {
        stop = true
        break
      }
    }

    if (!stop) {
      pattern = substr
    }
  }

  return pattern
}
