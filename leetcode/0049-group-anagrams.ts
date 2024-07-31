function groupAnagrams(strs: string[]): string[][] {
  const result = {}

  for (let i = 0; i < strs.length; i++) {
    const sorted = strs[i].split("").sort().join("")

    if (result[sorted] === undefined) {
      result[sorted] = []
    }

    result[sorted].push(strs[i])
  }

  return Object.values(result)
}
