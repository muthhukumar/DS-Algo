function isValid(s: string): boolean {
  const stack: Array<string> = []

  const map = {
    "{": "}",
    "(": ")",
    "[": "]",
  }

  for (let i = 0; i < s.length; i++) {
    if (s[i] === "{" || s[i] === "(" || s[i] === "[") {
      stack.push(s[i])
    } else {
      if (map[stack.pop() ?? ""] === s[i]) {
        continue
      }

      return false
    }
  }

  return stack.length === 0
}
